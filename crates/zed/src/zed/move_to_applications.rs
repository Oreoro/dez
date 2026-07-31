use anyhow::{Context as _, Result};
use gpui::{App, AsyncWindowContext, PromptButton, PromptLevel};
use release_channel::RELEASE_CHANNEL;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use util::ResultExt;
use util::command::new_command;
use workspace::MultiWorkspace;

static PROMPTED_THIS_SESSION: AtomicBool = AtomicBool::new(false);

pub fn init(cx: &mut App) {
    let Some(request) = MoveToApplicationsRequest::new(cx).log_err().flatten() else {
        return;
    };

    cx.observe_new(move |_workspace: &mut MultiWorkspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        if PROMPTED_THIS_SESSION.swap(true, Ordering::AcqRel) {
            return;
        }

        let request = request.clone();
        cx.spawn_in(window, async move |_workspace, cx| {
            request.prompt(cx).await.log_err();
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
        "{} is running from a temporary location. Install it in /Applications and relaunch before opening Workspaces or durable terminals.",
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

    async fn prompt(self, cx: &mut AsyncWindowContext) -> Result<()> {
        let app_name = RELEASE_CHANNEL.display_name();
        let prompt_title = format!("Install {app_name} before continuing");
        let prompt_description = installation_required_message();
        let response = cx
            .prompt(
                PromptLevel::Warning,
                &prompt_title,
                Some(&prompt_description),
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
            _ => {
                cx.update(|_window, cx| cx.quit())?;
            }
        }

        Ok(())
    }
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
