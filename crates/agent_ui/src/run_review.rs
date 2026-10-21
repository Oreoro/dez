use std::path::PathBuf;

/// Lifecycle truth projected from the surface or session that owns a Run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunReviewState {
    Draft,
    Running,
    WaitingForPermission,
    WaitingForInput,
    Idle,
    Completed,
    Failed,
    Detached,
    Reconnecting,
    Exited,
    Missing,
    Incompatible,
    Saved,
    Resumable,
}

impl RunReviewState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Running => "Running",
            Self::WaitingForPermission => "Waiting for permission",
            Self::WaitingForInput => "Waiting for input",
            Self::Idle => "Idle",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Detached => "Detached",
            Self::Reconnecting => "Reconnecting",
            Self::Exited => "Exited",
            Self::Missing => "Missing",
            Self::Incompatible => "Incompatible",
            Self::Saved => "Saved",
            Self::Resumable => "Resumable",
        }
    }

    fn is_incomplete(self) -> bool {
        matches!(
            self,
            Self::Draft
                | Self::Running
                | Self::WaitingForPermission
                | Self::WaitingForInput
                | Self::Idle
                | Self::Reconnecting
                | Self::Missing
                | Self::Incompatible
                | Self::Resumable
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservedRunCheckStatus {
    Running,
    Passed,
    Failed,
}

impl ObservedRunCheckStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Passed => "Passed",
            Self::Failed => "Failed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedRunCheck {
    pub name: String,
    pub status: ObservedRunCheckStatus,
    pub source_path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedRunCommand {
    pub command: String,
    pub exit_code: Option<i32>,
    pub source_path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedRunActivity {
    pub sequence: u64,
    pub summary: String,
    pub source_path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkspaceEvidenceKind {
    WorkspaceRoot,
    OpenFile,
    UserSelectedPath,
    TerminalWorkingDirectory,
}

impl WorkspaceEvidenceKind {
    fn label(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "Workspace root",
            Self::OpenFile => "Open file",
            Self::UserSelectedPath => "Selected path",
            Self::TerminalWorkingDirectory => "Terminal working directory",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObservedWorkspaceEvidence {
    pub kind: WorkspaceEvidenceKind,
    pub path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObservedRepositoryEvidence {
    pub worktree_path: PathBuf,
    pub main_worktree_path: Option<PathBuf>,
    pub branch: Option<String>,
    pub changed_paths: Vec<PathBuf>,
    pub changed_path_count: usize,
    pub conflict_count: usize,
    pub untracked_count: usize,
    pub linked_worktree: bool,
    pub truncated: bool,
}

impl ObservedRunCheck {
    /// Conservatively classifies a completed structured command as a check.
    /// Unknown commands and missing exit status stay unclassified rather than
    /// being presented as validation evidence.
    pub fn from_command(command: &ObservedRunCommand) -> Option<Self> {
        let exit_code = command.exit_code?;
        let normalized = command.command.trim().to_ascii_lowercase();
        let is_check = [
            "cargo test",
            "cargo nextest",
            "cargo check",
            "cargo clippy",
            "cargo fmt --check",
            "./script/clippy",
            "pytest",
            "python -m pytest",
            "python3 -m pytest",
            "go test",
            "npm test",
            "npm run test",
            "npm run lint",
            "npm run typecheck",
            "pnpm test",
            "pnpm run test",
            "pnpm lint",
            "pnpm typecheck",
            "yarn test",
            "yarn lint",
            "yarn typecheck",
        ]
        .iter()
        .any(|prefix| {
            normalized == *prefix
                || normalized
                    .strip_prefix(prefix)
                    .is_some_and(|suffix| suffix.chars().next().is_some_and(char::is_whitespace))
        });
        is_check.then(|| Self {
            name: command.command.clone(),
            status: if exit_code == 0 {
                ObservedRunCheckStatus::Passed
            } else {
                ObservedRunCheckStatus::Failed
            },
            source_path: command.source_path.clone(),
        })
    }
}

/// A value projection over authoritative terminal, thread, action-log, and
/// workspace facts. It deliberately owns no lifecycle state and is not a
/// second persistence store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunReviewBrief {
    pub run_label: String,
    pub actor: String,
    pub state: RunReviewState,
    pub host: Option<String>,
    pub session: Option<String>,
    pub workspace_evidence: Vec<ObservedWorkspaceEvidence>,
    pub repository_evidence: Vec<ObservedRepositoryEvidence>,
    pub lines_added: u64,
    pub lines_removed: u64,
    pub changed_files: Vec<PathBuf>,
    pub file_targets: Vec<PathBuf>,
    pub file_targets_truncated: bool,
    pub activity: Vec<ObservedRunActivity>,
    pub commands: Vec<ObservedRunCommand>,
    pub checks: Vec<ObservedRunCheck>,
    pub observed_risks: Vec<String>,
}

impl RunReviewBrief {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Review brief: ");
        markdown.push_str(&heading_text(&self.run_label));
        markdown.push_str("\n\n> Deterministic projection of observed Dez evidence. Missing evidence is stated explicitly; no check is inferred.\n\n");
        markdown.push_str("[Decision](#review-decision) · [Workspace evidence](#workspace-evidence) · [Repository evidence](#repository-evidence) · [Changes](#changes) · [Activity](#recent-activity) · [Commands](#commands) · [Checks](#checks) · [Risks](#risks-and-unresolved-evidence)\n\n");

        markdown.push_str("## Run\n\n");
        markdown.push_str("| Fact | Observed value |\n| --- | --- |\n");
        push_table_row(&mut markdown, "Objective", &self.run_label);
        push_table_row(&mut markdown, "Actor", &self.actor);
        push_table_row(&mut markdown, "State", self.state.label());
        push_table_row(
            &mut markdown,
            "Host",
            self.host.as_deref().unwrap_or("Not observed"),
        );
        push_table_row(
            &mut markdown,
            "Session",
            self.session.as_deref().unwrap_or("Not observed"),
        );

        markdown.push_str("\n## Review decision\n\n");
        markdown.push_str(
            "This is an editable reviewer-owned checklist. Selecting an outcome records a note in this buffer; it does not mutate, stop, or resolve the authoritative Run.\n\n",
        );
        markdown.push_str("- [ ] **Continue** — return to the owning session and keep working.\n");
        markdown.push_str(
            "- [ ] **Request changes** — record feedback, then return to the owning session.\n",
        );
        markdown.push_str(
            "- [ ] **Accept as reviewed** — the observed evidence is sufficient for this review.\n",
        );

        markdown.push_str("\n## Workspace evidence\n\n");
        let mut evidence = self.workspace_evidence.clone();
        evidence.sort_by(|left, right| {
            left.path
                .cmp(&right.path)
                .then_with(|| left.kind.cmp(&right.kind))
        });
        evidence.dedup();
        if evidence.is_empty() {
            markdown.push_str("No path-bearing workspace evidence was observed.\n");
        } else {
            for evidence in evidence {
                markdown.push_str("- **");
                markdown.push_str(evidence.kind.label());
                markdown.push_str(":** ");
                markdown.push_str(&path_link(&evidence.path));
                markdown.push('\n');
            }
        }

        markdown.push_str("\n## Repository evidence\n\n");
        let mut repositories = self.repository_evidence.clone();
        repositories.sort();
        repositories.dedup();
        if repositories.is_empty() {
            markdown.push_str(
                "No owning Git repository was observed. Repository cleanliness and branch state are unknown.\n",
            );
        } else {
            markdown.push_str(
                "These are current Git-store observations for the owning Workspace or terminal directory. Changed paths are not attributed to this Run unless another source says so.\n\n",
            );
            for repository in repositories {
                markdown.push_str("### ");
                markdown.push_str(&inline_code(
                    &repository.worktree_path.display().to_string(),
                ));
                markdown.push_str("\n\n");
                markdown.push_str("- **Branch:** ");
                markdown.push_str(
                    &repository
                        .branch
                        .as_deref()
                        .map(inline_code)
                        .unwrap_or_else(|| "Not observed".to_owned()),
                );
                markdown.push('\n');
                markdown.push_str("- **Worktree:** ");
                markdown.push_str(&path_link(&repository.worktree_path));
                markdown.push('\n');
                if repository.linked_worktree {
                    markdown.push_str("- **Linked worktree:** Yes");
                    if let Some(main_worktree_path) = &repository.main_worktree_path {
                        markdown.push_str("; main worktree ");
                        markdown.push_str(&path_link(main_worktree_path));
                    }
                    markdown.push('\n');
                }
                markdown.push_str(&format!(
                    "- **Observed status:** {} changed path(s), {} conflict(s), {} untracked\n",
                    repository.changed_path_count,
                    repository.conflict_count,
                    repository.untracked_count
                ));
                if repository.changed_paths.is_empty() {
                    markdown
                        .push_str("- **Changed paths:** None observed in the bounded projection\n");
                } else {
                    markdown.push_str("- **Changed paths:**\n");
                    for path in &repository.changed_paths {
                        markdown.push_str("  - ");
                        markdown.push_str(&path_link(path));
                        markdown.push('\n');
                    }
                }
                if repository.truncated {
                    markdown.push_str(
                        "- **Truncation:** Additional changed paths exist outside this brief\n",
                    );
                }
                markdown.push('\n');
            }
        }

        markdown.push_str("\n## Changes\n\n");
        if self.lines_added == 0 && self.lines_removed == 0 {
            markdown.push_str("No tracked line changes were observed. This does not prove the worktree is clean.\n");
        } else {
            markdown.push_str(&format!(
                "Observed tracked diff: **+{} / -{}** lines.\n",
                self.lines_added, self.lines_removed
            ));
        }
        let mut changed_files = self.changed_files.clone();
        changed_files.sort();
        changed_files.dedup();
        if changed_files.is_empty() {
            markdown.push_str("\nNo changed-file paths were observed.\n");
        } else {
            markdown.push_str("\nObserved changed files:\n\n");
            for path in changed_files {
                markdown.push_str("- ");
                markdown.push_str(&path_link(&path));
                markdown.push('\n');
            }
        }
        let mut file_targets = self.file_targets.clone();
        file_targets.sort();
        file_targets.dedup();
        if !file_targets.is_empty() || self.file_targets_truncated {
            markdown.push_str(
                "\nObserved adapter file targets (these identify intended scope, not proof that a change succeeded):\n\n",
            );
            for path in file_targets {
                markdown.push_str("- ");
                markdown.push_str(&path_link(&path));
                markdown.push('\n');
            }
            if self.file_targets_truncated {
                markdown.push_str("- Additional adapter file targets were truncated.\n");
            }
        }

        markdown.push_str("\n## Recent activity\n\n");
        if self.activity.is_empty() {
            markdown.push_str("No structured activity events were observed.\n");
        } else {
            for event in &self.activity {
                markdown.push_str(&format!(
                    "- **#{}** {}",
                    event.sequence,
                    markdown_text(&event.summary)
                ));
                if let Some(source_path) = &event.source_path {
                    markdown.push_str(" — ");
                    markdown.push_str(&path_link(source_path));
                }
                markdown.push('\n');
            }
        }

        markdown.push_str("\n## Commands\n\n");
        if self.commands.is_empty() {
            markdown.push_str("No structured command events were observed.\n");
        } else {
            for command in &self.commands {
                let outcome = command
                    .exit_code
                    .map(|code| format!("exit {code}"))
                    .unwrap_or_else(|| "outcome not observed".to_owned());
                markdown.push_str(&format!(
                    "- {} — {}",
                    inline_code(&command.command),
                    outcome
                ));
                if let Some(source_path) = &command.source_path {
                    markdown.push_str(" — cwd ");
                    markdown.push_str(&path_link(source_path));
                }
                markdown.push('\n');
            }
        }

        markdown.push_str("\n## Checks\n\n");
        if self.checks.is_empty() {
            markdown.push_str(
                "No structured check results were observed. This brief does not claim validation passed.\n",
            );
        } else {
            for check in &self.checks {
                markdown.push_str(&format!(
                    "- **{}:** {}",
                    markdown_text(&check.name),
                    check.status.label()
                ));
                if let Some(source_path) = &check.source_path {
                    markdown.push_str(" — cwd ");
                    markdown.push_str(&path_link(source_path));
                }
                markdown.push('\n');
            }
        }

        markdown.push_str("\n## Risks and unresolved evidence\n\n");
        let mut risks = self.observed_risks.clone();
        if self.state.is_incomplete() {
            risks.push(format!(
                "Run state is {}; results may still change.",
                self.state.label()
            ));
        }
        if (self.lines_added > 0 || self.lines_removed > 0) && self.checks.is_empty() {
            risks.push("Tracked changes have no structured check result.".to_owned());
        }
        if self.file_targets_truncated {
            risks.push("Adapter file-target evidence is truncated.".to_owned());
        }
        risks.sort();
        risks.dedup();
        if risks.is_empty() {
            markdown.push_str("No structured risk was observed. This is not a safety guarantee.\n");
        } else {
            for risk in risks {
                markdown.push_str("- ");
                markdown.push_str(&markdown_text(&risk));
                markdown.push('\n');
            }
        }

        markdown
    }
}

fn push_table_row(markdown: &mut String, label: &str, value: &str) {
    markdown.push_str("| ");
    markdown.push_str(&table_text(label));
    markdown.push_str(" | ");
    markdown.push_str(&table_text(value));
    markdown.push_str(" |\n");
}

fn heading_text(value: &str) -> String {
    value.replace(['\r', '\n'], " ").trim().to_owned()
}

fn markdown_text(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace(['\r', '\n'], " ")
        .replace('*', "\\*")
        .replace('_', "\\_")
}

fn table_text(value: &str) -> String {
    markdown_text(value).replace('|', "\\|")
}

fn inline_code(value: &str) -> String {
    format!("`{}`", value.replace('`', "'").replace(['\r', '\n'], " "))
}

fn path_link(path: &std::path::Path) -> String {
    let label = inline_code(&path.display().to_string());
    url::Url::from_file_path(path)
        .ok()
        .map_or(label.clone(), |url| format!("[{label}]({url})"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_brief_never_infers_checks_or_cleanliness() {
        let brief = RunReviewBrief {
            run_label: "Refactor terminal\nstate".to_owned(),
            actor: "Codex".to_owned(),
            state: RunReviewState::Completed,
            host: None,
            session: None,
            workspace_evidence: Vec::new(),
            repository_evidence: Vec::new(),
            lines_added: 0,
            lines_removed: 0,
            changed_files: Vec::new(),
            file_targets: Vec::new(),
            file_targets_truncated: false,
            activity: vec![ObservedRunActivity {
                sequence: 4,
                summary: "Codex requested permission".to_owned(),
                source_path: Some(PathBuf::from("/workspace")),
            }],
            commands: Vec::new(),
            checks: Vec::new(),
            observed_risks: Vec::new(),
        };

        let markdown = brief.to_markdown();
        assert!(markdown.contains("No tracked line changes were observed"));
        assert!(markdown.contains("does not prove the worktree is clean"));
        assert!(markdown.contains("No structured command events were observed"));
        assert!(markdown.contains("does not claim validation passed"));
        assert!(markdown.contains("**#4** Codex requested permission"));
        assert!(markdown.contains("file:///workspace"));
        assert!(markdown.contains("**Accept as reviewed**"));
        assert!(markdown.contains("does not mutate, stop, or resolve the authoritative Run"));
        assert!(!markdown.contains("Checks passed"));
    }

    #[test]
    fn review_brief_sorts_evidence_and_surfaces_unchecked_changes() {
        let brief = RunReviewBrief {
            run_label: "Ship review".to_owned(),
            actor: "Codex".to_owned(),
            state: RunReviewState::WaitingForPermission,
            host: Some("local".to_owned()),
            session: Some("session-7".to_owned()),
            workspace_evidence: vec![
                ObservedWorkspaceEvidence {
                    kind: WorkspaceEvidenceKind::WorkspaceRoot,
                    path: PathBuf::from("/z"),
                },
                ObservedWorkspaceEvidence {
                    kind: WorkspaceEvidenceKind::TerminalWorkingDirectory,
                    path: PathBuf::from("/a"),
                },
                ObservedWorkspaceEvidence {
                    kind: WorkspaceEvidenceKind::UserSelectedPath,
                    path: PathBuf::from("/review.rs"),
                },
            ],
            repository_evidence: vec![ObservedRepositoryEvidence {
                worktree_path: PathBuf::from("/z"),
                main_worktree_path: Some(PathBuf::from("/main/z")),
                branch: Some("codex/review".to_owned()),
                changed_paths: vec![PathBuf::from("/z/file.rs")],
                changed_path_count: 2,
                conflict_count: 1,
                untracked_count: 0,
                linked_worktree: true,
                truncated: true,
            }],
            lines_added: 12,
            lines_removed: 3,
            changed_files: vec![PathBuf::from("/z/file.rs"), PathBuf::from("/a/file.rs")],
            file_targets: vec![PathBuf::from("/z/target.rs")],
            file_targets_truncated: true,
            activity: Vec::new(),
            commands: Vec::new(),
            checks: Vec::new(),
            observed_risks: vec!["Permission decision pending.".to_owned()],
        };

        let markdown = brief.to_markdown();
        let first_path = markdown.find("`/a`").unwrap_or(usize::MAX);
        let second_path = markdown.find("`/z`").unwrap_or_default();
        assert!(first_path < second_path);
        assert!(markdown.contains("Selected path: `/review.rs`"));
        assert!(markdown.contains("Terminal working directory"));
        assert!(markdown.contains("Workspace root"));
        assert!(markdown.contains("file:///a"));
        assert!(markdown.contains("[Commands](#commands)"));
        assert!(markdown.contains("[Repository evidence](#repository-evidence)"));
        assert!(markdown.contains("`codex/review`"));
        assert!(markdown.contains("2 changed path(s), 1 conflict(s), 0 untracked"));
        assert!(markdown.contains("Additional changed paths exist"));
        assert!(markdown.contains("**+12 / -3**"));
        assert!(markdown.contains("file:///a/file.rs"));
        assert!(markdown.contains("intended scope, not proof that a change succeeded"));
        assert!(markdown.contains("file:///z/target.rs"));
        assert!(markdown.contains("Adapter file-target evidence is truncated"));
        assert!(markdown.contains("Run state is Waiting for permission"));
        assert!(markdown.contains("Tracked changes have no structured check result"));
    }

    #[test]
    fn check_classification_requires_known_command_and_observed_exit() {
        let passed = ObservedRunCommand {
            command: "cargo test -p terminal".to_owned(),
            exit_code: Some(0),
            source_path: Some(PathBuf::from("/repo")),
        };
        assert_eq!(
            ObservedRunCheck::from_command(&passed),
            Some(ObservedRunCheck {
                name: passed.command.clone(),
                status: ObservedRunCheckStatus::Passed,
                source_path: passed.source_path.clone(),
            })
        );

        let unknown = ObservedRunCommand {
            command: "deploy production".to_owned(),
            exit_code: Some(0),
            source_path: None,
        };
        let missing_outcome = ObservedRunCommand {
            command: "pytest".to_owned(),
            exit_code: None,
            source_path: None,
        };
        assert_eq!(ObservedRunCheck::from_command(&unknown), None);
        assert_eq!(ObservedRunCheck::from_command(&missing_outcome), None);
    }
}
