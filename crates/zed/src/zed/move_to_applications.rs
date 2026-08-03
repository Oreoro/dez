use anyhow::{Context as _, Result};
use gpui::{App, AsyncWindowContext, Context, PromptButton, PromptLevel, Window};
use paths::APP_NAME;
use release_channel::RELEASE_CHANNEL;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use util::ResultExt as _;
use util::command::new_command;
use workspace::{MultiWorkspace, Toast, Workspace, notifications::NotificationId};

static PROMPTED_UPSTREAM_ZED_THIS_SESSION: AtomicBool = AtomicBool::new(false);
static DEZ_INSTALL_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

pub fn init(cx: &mut App) {
    if APP_NAME != "Zed" {
        return;
    }
    let Some(request) = MoveToApplicationsRequest::new(cx).log_err().flatten() else {
        return;
    };

    cx.observe_new(move |_workspace: &mut MultiWorkspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        if PROMPTED_UPSTREAM_ZED_THIS_SESSION.swap(true, Ordering::AcqRel) {
            return;
        }

        let request = request.clone();
        cx.spawn_in(window, async move |_workspace, cx| {
            request.prompt_upstream_zed(cx).await.log_err();
        })
        .detach();
    })
    .detach();
}

pub fn installation_required(cx: &App) -> bool {
    MoveToApplicationsRequest::new(cx)
        .map(|request| request.is_some())
        .unwrap_or(false)
}

pub fn installation_required_message() -> String {
    format!(
        "{} is running from a temporary location. Install it in /Applications and relaunch before opening a Workspace or starting durable terminals.",
        RELEASE_CHANNEL.display_name()
    )
}

#[derive(Clone)]
struct MoveToApplicationsRequest {
    app_path: PathBuf,
}

impl MoveToApplicationsRequest {
    fn new(cx: &App) -> Result<Option<Self>> {
        let app_path = match cx.app_path() {
            Ok(app_path) => app_path,
            Err(_) => return Ok(None),
        };

        if !should_require_installation(&app_path) {
            return Ok(None);
        }

        Ok(Some(Self { app_path }))
    }

    async fn prompt_upstream_zed(self, cx: &mut AsyncWindowContext) -> Result<()> {
        let app_name = RELEASE_CHANNEL.display_name();
        let response = cx
            .prompt(
                PromptLevel::Warning,
                &format!("Install {app_name} before continuing"),
                Some(&installation_required_message()),
                &[
                    PromptButton::ok("Install and Relaunch"),
                    PromptButton::cancel("Quit"),
                ],
            )
            .await?;

        match response {
            0 => {
                if let Err(error) = move_to_applications(&self.app_path, cx).await {
                    cx.prompt(
                        PromptLevel::Critical,
                        &format!("Failed to install {app_name}"),
                        Some(&error.to_string()),
                        &["OK"],
                    )
                    .await
                    .log_err();
                }
            }
            _ => cx.update(|_window, cx| cx.quit())?,
        }
        Ok(())
    }
}

pub fn install_and_relaunch(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    struct InstallAndRelaunchToast;

    let request = match MoveToApplicationsRequest::new(cx) {
        Ok(Some(request)) => request,
        Ok(None) => return,
        Err(error) => {
            log::error!("could not determine the Dez installation location: {error:#}");
            return;
        }
    };
    if DEZ_INSTALL_IN_PROGRESS.swap(true, Ordering::AcqRel) {
        workspace.show_toast(
            Toast::new(
                NotificationId::unique::<InstallAndRelaunchToast>(),
                "Installing Dez in /Applications…",
            )
            .autohide(),
            cx,
        );
        return;
    }

    workspace.show_toast(
        Toast::new(
            NotificationId::unique::<InstallAndRelaunchToast>(),
            format!(
                "Installing {} in /Applications…",
                RELEASE_CHANNEL.display_name()
            ),
        ),
        cx,
    );

    cx.spawn_in(window, async move |workspace, cx| {
        if let Err(error) = move_to_applications(&request.app_path, cx).await {
            DEZ_INSTALL_IN_PROGRESS.store(false, Ordering::Release);
            log::error!("failed to install Dez in /Applications: {error:#}");
            workspace
                .update_in(cx, |workspace, _, cx| {
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<InstallAndRelaunchToast>(),
                            format!("Could not install Dez: {error}"),
                        ),
                        cx,
                    );
                })
                .log_err();
        }
    })
    .detach();
}

fn should_require_installation(app_path: &Path) -> bool {
    app_path
        .extension()
        .is_some_and(|extension| extension == "app")
        && !app_path.starts_with(Path::new("/Applications"))
}

async fn move_to_applications(app_path: &Path, cx: &mut AsyncWindowContext) -> Result<()> {
    let destination_path = install_destination(app_path).await?;
    restart_into(destination_path, cx)
}

async fn install_destination(app_path: &Path) -> Result<PathBuf> {
    let app_name = app_path
        .file_name()
        .context("invalid app path: missing app bundle name")?;

    let system_destination = Path::new("/Applications").join(app_name);
    copy_app_bundle(app_path, &system_destination)
        .await
        .with_context(|| format!("failed to install app at {}", system_destination.display()))?;
    Ok(system_destination)
}

async fn copy_app_bundle(source: &Path, destination: &Path) -> Result<()> {
    let parent = destination
        .parent()
        .context("invalid destination path: missing parent directory")?;
    smol::fs::create_dir_all(parent)
        .await
        .with_context(|| format!("failed to create {}", parent.display()))?;

    let mut source_with_contents: OsString = source.into();
    source_with_contents.push("/");
    let mut destination_with_contents: OsString = destination.into();
    destination_with_contents.push("/");

    let mut command = new_command("rsync");
    command
        .args(["-a", "--delete"])
        .arg(&source_with_contents)
        .arg(&destination_with_contents);
    let output = command
        .output()
        .await
        .with_context(|| format!("failed to run rsync for {}", source.display()))?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy app bundle: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}

fn restart_into(app_path: PathBuf, cx: &mut AsyncWindowContext) -> Result<()> {
    cx.update(|_window, cx| {
        cx.set_restart_path(app_path);
        cx.restart();
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_bundles_require_system_installation() {
        assert!(!should_require_installation(Path::new(
            "/Applications/Dez.app"
        )));
        assert!(should_require_installation(Path::new(
            "/Volumes/Dez/Dez.app"
        )));
        assert!(should_require_installation(Path::new(
            "/private/tmp/dez-build/Dez.app"
        )));
        assert!(should_require_installation(Path::new(
            "/Users/test/Applications/Dez.app"
        )));
        assert!(!should_require_installation(Path::new(
            "/Users/test/src/target/debug/dez"
        )));
    }
}
