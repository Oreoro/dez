use std::{path::Path, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkspaceEvidenceHost {
    Local,
    Remote,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkspaceEvidenceProvenance {
    VisibleWorktree,
    OpenSurface,
    TerminalSession { session_id: Arc<str> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkspaceEvidenceConfidence {
    Authoritative,
    Observed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkspaceEvidenceLifecycle {
    Current,
    Stale,
    Unresolved,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkspaceEvidenceKind {
    WorkspaceRoot,
    OpenFile,
    TerminalWorkingDirectory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceEvidenceRecord {
    pub id: Arc<str>,
    pub kind: WorkspaceEvidenceKind,
    pub path: Arc<Path>,
    pub provenance: WorkspaceEvidenceProvenance,
    pub confidence: WorkspaceEvidenceConfidence,
    pub host: WorkspaceEvidenceHost,
    pub lifecycle: WorkspaceEvidenceLifecycle,
    pub truncated: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkspaceEvidenceSet {
    revision: u64,
    records: Vec<WorkspaceEvidenceRecord>,
    truncated: bool,
}

impl WorkspaceEvidenceSet {
    const MAX_OPEN_FILE_RECORDS: usize = 256;

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn records(&self) -> &[WorkspaceEvidenceRecord] {
        &self.records
    }

    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    pub fn replace_visible_worktree_roots(
        &mut self,
        workspace_id: Option<i64>,
        roots: impl IntoIterator<Item = Arc<Path>>,
        host: WorkspaceEvidenceHost,
    ) -> bool {
        let identity_prefix = workspace_id.map_or_else(
            || "workspace:pending".to_owned(),
            |workspace_id| format!("workspace:{workspace_id}"),
        );
        let root_records = roots
            .into_iter()
            .map(|path| WorkspaceEvidenceRecord {
                id: format!("{identity_prefix}:root:{}", path.to_string_lossy()).into(),
                kind: WorkspaceEvidenceKind::WorkspaceRoot,
                path,
                provenance: WorkspaceEvidenceProvenance::VisibleWorktree,
                confidence: WorkspaceEvidenceConfidence::Authoritative,
                host: host.clone(),
                lifecycle: WorkspaceEvidenceLifecycle::Current,
                truncated: false,
            })
            .collect::<Vec<_>>();
        let records = root_records
            .into_iter()
            .chain(
                self.records
                    .iter()
                    .filter(|record| record.kind != WorkspaceEvidenceKind::WorkspaceRoot)
                    .cloned(),
            )
            .collect::<Vec<_>>();

        if self.records == records {
            return false;
        }
        self.records = records;
        self.revision = self.revision.saturating_add(1);
        true
    }

    pub fn set_terminal_working_directory(
        &mut self,
        workspace_id: Option<i64>,
        session_id: impl Into<Arc<str>>,
        path: Option<Arc<Path>>,
        host: WorkspaceEvidenceHost,
    ) -> bool {
        let session_id = session_id.into();
        let previous = self.records.clone();
        self.records.retain(|record| {
            !matches!(
                &record.provenance,
                WorkspaceEvidenceProvenance::TerminalSession {
                    session_id: existing,
                } if existing == &session_id
            )
        });
        if let Some(path) = path {
            let identity_prefix = workspace_id.map_or_else(
                || "workspace:pending".to_owned(),
                |workspace_id| format!("workspace:{workspace_id}"),
            );
            self.records.push(WorkspaceEvidenceRecord {
                id: format!("{identity_prefix}:terminal:{session_id}:cwd").into(),
                kind: WorkspaceEvidenceKind::TerminalWorkingDirectory,
                path,
                provenance: WorkspaceEvidenceProvenance::TerminalSession { session_id },
                confidence: WorkspaceEvidenceConfidence::Observed,
                host,
                lifecycle: WorkspaceEvidenceLifecycle::Current,
                truncated: false,
            });
        }
        if self.records == previous {
            return false;
        }
        self.revision = self.revision.saturating_add(1);
        true
    }

    pub fn replace_open_files(
        &mut self,
        workspace_id: Option<i64>,
        paths: impl IntoIterator<Item = Arc<Path>>,
        host: WorkspaceEvidenceHost,
    ) -> bool {
        let identity_prefix = workspace_id.map_or_else(
            || "workspace:pending".to_owned(),
            |workspace_id| format!("workspace:{workspace_id}"),
        );
        let mut paths = paths.into_iter().collect::<Vec<_>>();
        paths.sort_by(|left, right| left.as_os_str().cmp(right.as_os_str()));
        paths.dedup();
        let truncated = paths.len() > Self::MAX_OPEN_FILE_RECORDS;
        paths.truncate(Self::MAX_OPEN_FILE_RECORDS);
        let file_records = paths.into_iter().map(|path| WorkspaceEvidenceRecord {
            id: format!("{identity_prefix}:file:{}", path.to_string_lossy()).into(),
            kind: WorkspaceEvidenceKind::OpenFile,
            path,
            provenance: WorkspaceEvidenceProvenance::OpenSurface,
            confidence: WorkspaceEvidenceConfidence::Authoritative,
            host: host.clone(),
            lifecycle: WorkspaceEvidenceLifecycle::Current,
            truncated: false,
        });
        let records = self
            .records
            .iter()
            .filter(|record| record.kind != WorkspaceEvidenceKind::OpenFile)
            .cloned()
            .chain(file_records)
            .collect::<Vec<_>>();
        if self.records == records && self.truncated == truncated {
            return false;
        }
        self.records = records;
        self.truncated = truncated;
        self.revision = self.revision.saturating_add(1);
        true
    }

    pub fn set_terminal_lifecycle(
        &mut self,
        session_id: &str,
        lifecycle: WorkspaceEvidenceLifecycle,
    ) -> bool {
        let mut changed = false;
        for record in &mut self.records {
            if matches!(
                &record.provenance,
                WorkspaceEvidenceProvenance::TerminalSession {
                    session_id: existing,
                } if existing.as_ref() == session_id
            ) && record.lifecycle != lifecycle
            {
                record.lifecycle = lifecycle.clone();
                changed = true;
            }
        }
        if changed {
            self.revision = self.revision.saturating_add(1);
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_roots_have_stable_identity_and_revision() {
        let mut evidence = WorkspaceEvidenceSet::default();
        assert!(evidence.replace_visible_worktree_roots(
            Some(7),
            [Arc::<Path>::from(Path::new("/repo"))],
            WorkspaceEvidenceHost::Local,
        ));
        let id = evidence.records()[0].id.clone();
        assert_eq!(evidence.revision(), 1);
        assert!(!evidence.is_truncated());

        assert!(!evidence.replace_visible_worktree_roots(
            Some(7),
            [Arc::<Path>::from(Path::new("/repo"))],
            WorkspaceEvidenceHost::Local,
        ));
        assert_eq!(evidence.revision(), 1);
        assert_eq!(evidence.records()[0].id, id);

        assert!(evidence.replace_visible_worktree_roots(
            Some(7),
            [Arc::<Path>::from(Path::new("/repo-2"))],
            WorkspaceEvidenceHost::Remote,
        ));
        assert_eq!(evidence.revision(), 2);
        assert_ne!(evidence.records()[0].id, id);
        assert_eq!(evidence.records()[0].host, WorkspaceEvidenceHost::Remote);
    }

    #[test]
    fn replacing_roots_preserves_terminal_session_evidence() {
        let mut evidence = WorkspaceEvidenceSet::default();
        evidence.replace_visible_worktree_roots(
            Some(7),
            [Arc::<Path>::from(Path::new("/repo"))],
            WorkspaceEvidenceHost::Local,
        );
        assert!(evidence.set_terminal_working_directory(
            Some(7),
            "session-1",
            Some(Arc::<Path>::from(Path::new("/repo/subdir"))),
            WorkspaceEvidenceHost::Local,
        ));
        assert_eq!(evidence.records().len(), 2);

        assert!(evidence.replace_visible_worktree_roots(
            Some(7),
            [Arc::<Path>::from(Path::new("/repo-2"))],
            WorkspaceEvidenceHost::Local,
        ));
        assert_eq!(evidence.records().len(), 2);
        assert!(evidence.records().iter().any(|record| {
            record.kind == WorkspaceEvidenceKind::TerminalWorkingDirectory
                && record.path.as_ref() == Path::new("/repo/subdir")
        }));
    }

    #[test]
    fn terminal_lifecycle_changes_without_dropping_evidence() {
        let mut evidence = WorkspaceEvidenceSet::default();
        evidence.set_terminal_working_directory(
            Some(7),
            "session-1",
            Some(Arc::<Path>::from(Path::new("/repo"))),
            WorkspaceEvidenceHost::Local,
        );
        assert!(evidence.set_terminal_lifecycle("session-1", WorkspaceEvidenceLifecycle::Stale,));
        assert_eq!(evidence.records().len(), 1);
        assert_eq!(
            evidence.records()[0].lifecycle,
            WorkspaceEvidenceLifecycle::Stale
        );
        assert!(!evidence.set_terminal_lifecycle("session-1", WorkspaceEvidenceLifecycle::Stale,));
    }

    #[test]
    fn open_files_are_stable_deduplicated_and_bounded() {
        let mut evidence = WorkspaceEvidenceSet::default();
        assert!(evidence.replace_open_files(
            Some(7),
            [
                Arc::<Path>::from(Path::new("/repo/b.rs")),
                Arc::<Path>::from(Path::new("/repo/a.rs")),
                Arc::<Path>::from(Path::new("/repo/a.rs")),
            ],
            WorkspaceEvidenceHost::Local,
        ));
        assert_eq!(evidence.records().len(), 2);
        assert_eq!(evidence.records()[0].path.as_ref(), Path::new("/repo/a.rs"));
        assert!(!evidence.is_truncated());

        let revision = evidence.revision();
        assert!(!evidence.replace_open_files(
            Some(7),
            [
                Arc::<Path>::from(Path::new("/repo/a.rs")),
                Arc::<Path>::from(Path::new("/repo/b.rs")),
            ],
            WorkspaceEvidenceHost::Local,
        ));
        assert_eq!(evidence.revision(), revision);

        assert!(
            evidence.replace_open_files(
                Some(7),
                (0..=WorkspaceEvidenceSet::MAX_OPEN_FILE_RECORDS)
                    .map(|index| Arc::<Path>::from(Path::new(&format!("/repo/{index:03}.rs")))),
                WorkspaceEvidenceHost::Local,
            )
        );
        assert_eq!(
            evidence.records().len(),
            WorkspaceEvidenceSet::MAX_OPEN_FILE_RECORDS
        );
        assert!(evidence.is_truncated());
    }
}
