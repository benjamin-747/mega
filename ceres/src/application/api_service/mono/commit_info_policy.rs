//! Routing policy for tree commit-info / latest-commit attribution.
//!
//! Keeps `/third-party/**` on the import commit DAG and restricts monorepo
//! history (which surfaces `cl merge generated commit`) to true monorepo paths.

use std::path::Path;

/// How [`super::MonoApiService`] should resolve last-modification commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommitInfoRoute {
    /// Registered import repo under `import_dir` — use [`ImportApiService`](crate::application::api_service::import_api_service::ImportApiService).
    ImportRepo,
    /// Under `import_dir` but no `git_repo` row — denormalized stamps only; never monorepo history.
    ImportDirDenormalizedOnly,
    /// True monorepo paths (e.g. `/project/**`) — denormalized when complete, else history.
    MonorepoDenormalizedOrHistory,
}

/// `path` is a proper child of `import_dir` (not the import root itself).
pub(crate) fn is_path_under_import_dir(path: &Path, import_dir: &Path) -> bool {
    path.starts_with(import_dir) && path != import_dir
}

/// Select commit-info attribution strategy.
///
/// `has_matching_git_repo` is true when `find_git_repo_like_path(path)` returns a repo
/// (typically the longest prefix under `/third-party/...`).
pub(crate) fn commit_info_route(
    path: &Path,
    import_dir: &Path,
    has_matching_git_repo: bool,
) -> CommitInfoRoute {
    if is_path_under_import_dir(path, import_dir) {
        if has_matching_git_repo {
            CommitInfoRoute::ImportRepo
        } else {
            CommitInfoRoute::ImportDirDenormalizedOnly
        }
    } else {
        CommitInfoRoute::MonorepoDenormalizedOrHistory
    }
}

/// Whether monorepo history BFS is allowed for this route.
pub(crate) fn allows_monorepo_history(route: CommitInfoRoute) -> bool {
    matches!(route, CommitInfoRoute::MonorepoDenormalizedOrHistory)
}

/// Default tree view (empty / `main` / `master`) can use denormalized stamps;
/// tag or commit-SHA browsing must stay refs-aware (history).
pub(crate) fn prefer_denormalized_for_refs(reference: Option<&str>) -> bool {
    let Some(refs) = reference.map(str::trim).filter(|r| !r.is_empty()) else {
        return true;
    };
    let branch = refs.strip_prefix("refs/heads/").unwrap_or(refs);
    branch == "main" || branch == "master"
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const IMPORT_DIR: &str = "/third-party";

    #[test]
    fn third_party_with_git_repo_routes_to_import() {
        let route = commit_info_route(
            Path::new("/third-party/rust/crates/a3/mo/a3mo_lib"),
            Path::new(IMPORT_DIR),
            true,
        );
        assert_eq!(route, CommitInfoRoute::ImportRepo);
        assert!(!allows_monorepo_history(route));
    }

    #[test]
    fn third_party_without_git_repo_forbids_monorepo_history() {
        // Regression: Mono history blamed `cl merge generated commit` on import trees.
        let route = commit_info_route(
            Path::new("/third-party/rust/crates/vo/om/voom"),
            Path::new(IMPORT_DIR),
            false,
        );
        assert_eq!(route, CommitInfoRoute::ImportDirDenormalizedOnly);
        assert!(!allows_monorepo_history(route));
    }

    #[test]
    fn import_dir_root_is_not_treated_as_import_child() {
        let route = commit_info_route(Path::new("/third-party"), Path::new(IMPORT_DIR), false);
        assert_eq!(route, CommitInfoRoute::MonorepoDenormalizedOrHistory);
        assert!(allows_monorepo_history(route));
    }

    #[test]
    fn project_paths_allow_monorepo_history_fallback() {
        // Regression: /project/dagrs blank/stale stamps after CL merge object reuse.
        let route = commit_info_route(Path::new("/project/dagrs"), Path::new(IMPORT_DIR), false);
        assert_eq!(route, CommitInfoRoute::MonorepoDenormalizedOrHistory);
        assert!(allows_monorepo_history(route));
    }

    #[test]
    fn is_path_under_import_dir_matches_children_only() {
        let import = Path::new(IMPORT_DIR);
        assert!(is_path_under_import_dir(
            Path::new("/third-party/rust"),
            import
        ));
        assert!(!is_path_under_import_dir(Path::new("/third-party"), import));
        assert!(!is_path_under_import_dir(
            Path::new("/project/dagrs"),
            import
        ));
        assert!(!is_path_under_import_dir(
            Path::new("/third-party-extra"),
            import
        ));
    }

    #[test]
    fn prefer_denormalized_for_default_and_main_refs() {
        assert!(prefer_denormalized_for_refs(None));
        assert!(prefer_denormalized_for_refs(Some("")));
        assert!(prefer_denormalized_for_refs(Some("main")));
        assert!(prefer_denormalized_for_refs(Some("refs/heads/main")));
        assert!(prefer_denormalized_for_refs(Some("master")));
        assert!(!prefer_denormalized_for_refs(Some("v1.0.0")));
        assert!(!prefer_denormalized_for_refs(Some(
            "0a9c39e9174641fb3890904a7c82a07da1f7d378"
        )));
    }
}
