use std::path::Path;

use crate::tasks::workflows::{
    release::ReleaseBundleJobs,
    runners::{Arch, Platform},
    steps::{FluentBuilder, IfNoFilesFound, NamedJob, UploadArtifactStep, dependant_job, named},
    vars::{assets, bundle_envs},
};

use super::{runners, steps};
use gh_workflow::*;
use indoc::indoc;

pub fn run_bundling() -> Workflow {
    let bundle = ReleaseBundleJobs {
        linux_aarch64: bundle_linux(Arch::AARCH64, &[]),
        linux_x86_64: bundle_linux(Arch::X86_64, &[]),
        mac_aarch64: bundle_mac(Arch::AARCH64, &[]),
        mac_x86_64: bundle_mac(Arch::X86_64, &[]),
        windows_aarch64: bundle_windows(Arch::AARCH64, &[]),
        windows_x86_64: bundle_windows(Arch::X86_64, &[]),
    };
    named::workflow()
        .on(Event::default().pull_request(
            PullRequest::default().types([PullRequestType::Labeled, PullRequestType::Synchronize]),
        ))
        .concurrency(
            Concurrency::new(Expression::new(
                "${{ github.workflow }}-${{ github.head_ref || github.ref }}",
            ))
            .cancel_in_progress(true),
        )
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"))
        .map(|mut workflow| {
            for job in bundle.into_jobs() {
                workflow = workflow.add_job(job.name, job.job);
            }
            workflow
        })
}

fn bundle_job(deps: &[&NamedJob]) -> Job {
    dependant_job(deps)
        .when(deps.len() == 0, |job|
            job.cond(Expression::new(
                indoc! {
                    r#"(github.event.action == 'labeled' && github.event.label.name == 'run-bundling') ||
                    (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'run-bundling'))"#,
                })))
        .timeout_minutes(60u32)
}

pub(crate) fn bundle_mac(arch: Arch, deps: &[&NamedJob]) -> NamedJob {
    pub fn bundle_mac(arch: Arch) -> Step<Run> {
        named::bash(&format!("./script/bundle-mac {arch}-apple-darwin"))
    }
    let platform = Platform::Mac;
    let artifact_name = match arch {
        Arch::X86_64 => assets::MAC_X86_64,
        Arch::AARCH64 => assets::MAC_AARCH64,
    };
    let remote_server_artifact_name = match arch {
        Arch::X86_64 => assets::REMOTE_SERVER_MAC_X86_64,
        Arch::AARCH64 => assets::REMOTE_SERVER_MAC_AARCH64,
    };
    NamedJob {
        name: format!("bundle_mac_{arch}"),
        job: bundle_job(deps)
            .runs_on(runners::MAC_DEFAULT)
            .envs(bundle_envs(platform))
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_node())
            .add_step(steps::clear_target_dir_if_large(runners::Platform::Mac))
            .add_step(bundle_mac(arch))
            .add_step(upload_artifact(&format!(
                "target/{arch}-apple-darwin/release/{artifact_name}"
            )))
            .add_step(upload_artifact(&format!(
                "target/{remote_server_artifact_name}"
            ))),
    }
}

pub fn upload_artifact(path: &str) -> UploadArtifactStep {
    let name = Path::new(path).file_name().unwrap().to_str().unwrap();
    steps::upload_artifact(name, path).if_no_files_found(IfNoFilesFound::Error)
}

pub(crate) fn bundle_linux(arch: Arch, deps: &[&NamedJob]) -> NamedJob {
    let platform = Platform::Linux;
    let artifact_name = match arch {
        Arch::X86_64 => assets::LINUX_X86_64,
        Arch::AARCH64 => assets::LINUX_AARCH64,
    };
    let remote_server_artifact_name = match arch {
        Arch::X86_64 => assets::REMOTE_SERVER_LINUX_X86_64,
        Arch::AARCH64 => assets::REMOTE_SERVER_LINUX_AARCH64,
    };
    let (deb_artifact_name, rpm_artifact_name) = match arch {
        Arch::X86_64 => ("dez-linux-x86_64.deb", "dez-linux-x86_64.rpm"),
        Arch::AARCH64 => ("dez-linux-aarch64.deb", "dez-linux-aarch64.rpm"),
    };
    NamedJob {
        name: format!("bundle_linux_{arch}"),
        job: bundle_job(deps)
            .runs_on(arch.linux_bundler())
            .envs(bundle_envs(platform))
            .add_env(Env::new("CC", "clang-18"))
            .add_env(Env::new("CXX", "clang++-18"))
            .add_step(steps::checkout_repo())
            .map(steps::install_linux_dependencies)
            .add_step(
                named::bash(indoc! {r#"
                sudo apt-get update
                sudo apt-get install -y cpio desktop-file-utils rpm
                ./script/install-nfpm
            "#})
                .name("Install Linux packaging tools"),
            )
            .add_step(steps::script("./script/test-package-linux"))
            .add_step(steps::script("./script/bundle-linux"))
            .add_step(steps::script("./script/package-linux"))
            .add_step(upload_artifact(&format!("target/release/{artifact_name}")))
            .add_step(upload_artifact(&format!(
                "target/release/{deb_artifact_name}"
            )))
            .add_step(upload_artifact(&format!(
                "target/release/{rpm_artifact_name}"
            )))
            .add_step(upload_artifact(&format!(
                "target/{remote_server_artifact_name}"
            ))),
    }
}

pub(crate) fn bundle_windows(arch: Arch, deps: &[&NamedJob]) -> NamedJob {
    let platform = Platform::Windows;
    pub fn bundle_windows(arch: Arch) -> Step<Run> {
        let step = match arch {
            Arch::X86_64 => named::pwsh("script/bundle-windows.ps1 -Architecture x86_64"),
            Arch::AARCH64 => named::pwsh("script/bundle-windows.ps1 -Architecture aarch64"),
        };
        step.working_directory("${{ env.ZED_WORKSPACE }}")
    }
    let artifact_name = match arch {
        Arch::X86_64 => assets::WINDOWS_X86_64,
        Arch::AARCH64 => assets::WINDOWS_AARCH64,
    };
    let remote_server_artifact_name = match arch {
        Arch::X86_64 => assets::REMOTE_SERVER_WINDOWS_X86_64,
        Arch::AARCH64 => assets::REMOTE_SERVER_WINDOWS_AARCH64,
    };
    NamedJob {
        name: format!("bundle_windows_{arch}"),
        job: bundle_job(deps)
            .runs_on(runners::WINDOWS_DEFAULT)
            .envs(bundle_envs(platform))
            .add_step(steps::checkout_repo())
            .add_step(bundle_windows(arch))
            .add_step(upload_artifact(&format!("target/{artifact_name}")))
            .add_step(upload_artifact(&format!(
                "target/{remote_server_artifact_name}"
            ))),
    }
}
