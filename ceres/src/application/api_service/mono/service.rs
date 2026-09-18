//! # Mono API Service
//!
//! Thin facade over monorepo operation modules. See sibling `mono_*_ops` modules for
//! CLA, buck upload, merge queue, sync, tags, entries, diffs, branch updates, and merges.

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};

use api_model::{common::Pagination, git::commit::LatestCommitInfo};
use async_trait::async_trait;
use common::errors::MegaError;
use git_internal::{
    errors::GitError,
    internal::object::{
        commit::Commit,
        tree::{Tree, TreeItem, TreeItemMode},
    },
};
use jupiter::{storage::Storage, utils::converter::FromMegaModel};

use super::{commit_info_policy, context::ServiceContext, logic::MonoServiceLogic};
use crate::{
    application::{
        api_service::{
            ApiHandler, cache::GitObjectCache, commit_ops, history,
            import_api_service::ImportApiService, tree_ops,
        },
        build_trigger::SharedBuildDispatch,
    },
    infra::TransportContext,
    model::git::{CreateEntryInfo, CreateEntryResult, EditFilePayload, EditFileResult},
    transport::protocol::repo::Repo,
};

/// Git-domain API service (tags, commits, sync, buck upload, entry edit).
pub type GitApplicationService = MonoApiService;

#[derive(Clone)]
pub struct MonoApiService {
    ctx: ServiceContext,
}

impl MonoApiService {
    pub fn new(ctx: TransportContext) -> Self {
        Self {
            ctx: ServiceContext::from_transport(ctx),
        }
    }

    pub fn storage(&self) -> &Storage {
        self.ctx.storage()
    }

    pub(crate) fn build_dispatch(&self) -> Option<SharedBuildDispatch> {
        self.ctx.build_dispatch()
    }

    pub fn git_object_cache(&self) -> Arc<GitObjectCache> {
        self.ctx.git_object_cache().clone()
    }

    pub(crate) async fn trigger_build_for_cl(
        &self,
        editor: &crate::application::code_edit::on_edit::OneditCodeEdit,
        cl: &callisto::mega_cl::Model,
        username: &str,
    ) -> Result<(), GitError> {
        let Some(build_dispatch) = self.build_dispatch() else {
            return Ok(());
        };
        editor
            .trigger_build_and_check(
                self.storage().clone(),
                self.git_object_cache(),
                build_dispatch,
                cl,
                username,
            )
            .await?;
        Ok(())
    }

    /// Default tree view (empty/`main`/`master`) can use denormalized stamps.
    fn prefer_denormalized_commit_map(reference: Option<&str>) -> bool {
        commit_info_policy::prefer_denormalized_for_refs(reference)
    }

    fn path_under_import_dir(&self, path: &Path) -> bool {
        commit_info_policy::is_path_under_import_dir(path, &self.import_dir())
    }

    /// Prefer the import-repo handler for `/third-party/**` so commit-info shows
    /// `Import …` commits, not monorepo `cl merge generated commit`.
    async fn import_handler_for_path(
        &self,
        path: &Path,
    ) -> Result<Option<ImportApiService>, MegaError> {
        if !self.path_under_import_dir(path) {
            return Ok(None);
        }
        let path_str = path
            .to_str()
            .ok_or_else(|| MegaError::Other(format!("invalid path: {}", path.display())))?;
        let Some(model) = self.find_git_repo_like_path(path_str).await? else {
            return Ok(None);
        };
        Ok(Some(ImportApiService::new(
            self.storage().clone(),
            Repo::from(model),
            self.git_object_cache(),
        )))
    }

    /// Returns `Ok(Some(map))` only when every tree item has a resolvable stamp.
    /// `Ok(None)` means caller should fall back to history traversal.
    async fn denormalized_item_to_commit_map(
        &self,
        path: &Path,
        reference: Option<&str>,
    ) -> Result<Option<HashMap<TreeItem, Option<Commit>>>, GitError> {
        let Some(tree) = tree_ops::search_tree_by_path(self, path, reference)
            .await
            .map_err(|e| GitError::CustomError(e.to_string()))?
        else {
            return Ok(Some(HashMap::new()));
        };

        if tree.tree_items.is_empty() {
            return Ok(Some(HashMap::new()));
        }

        let storage = self.storage().mono_storage();
        let mut item_to_commit = HashMap::new();

        let tree_hashes: Vec<String> = tree
            .tree_items
            .iter()
            .filter(|x| x.mode == TreeItemMode::Tree)
            .map(|x| x.id.to_string())
            .collect();
        if !tree_hashes.is_empty() {
            let trees = storage
                .get_trees_by_hashes(tree_hashes)
                .await
                .map_err(|e| GitError::CustomError(e.to_string()))?;
            for t in trees {
                if t.commit_id.is_empty() {
                    return Ok(None);
                }
                item_to_commit.insert(t.tree_id, t.commit_id);
            }
        }

        let blob_hashes: Vec<String> = tree
            .tree_items
            .iter()
            .filter(|x| x.mode == TreeItemMode::Blob)
            .map(|x| x.id.to_string())
            .collect();
        if !blob_hashes.is_empty() {
            let blobs = storage
                .get_mega_blobs_by_hashes(blob_hashes)
                .await
                .map_err(|e| GitError::CustomError(e.to_string()))?;
            for blob in blobs {
                if blob.commit_id.is_empty() {
                    return Ok(None);
                }
                item_to_commit.insert(blob.blob_id, blob.commit_id);
            }
        }

        // Every tree item must have a stamp (missing row or empty → history).
        for item in &tree.tree_items {
            if !item_to_commit.contains_key(&item.id.to_string()) {
                return Ok(None);
            }
        }

        let commit_ids: HashSet<String> = item_to_commit.values().cloned().collect();
        let commits = self
            .get_commits_by_hashes(commit_ids.into_iter().collect())
            .await?;
        let commit_map: HashMap<String, Commit> =
            commits.into_iter().map(|x| (x.id.to_string(), x)).collect();

        // If a stamp points at a missing commit, fall back to history.
        for commit_id in item_to_commit.values() {
            if !commit_map.contains_key(commit_id) {
                return Ok(None);
            }
        }

        Ok(Some(MonoServiceLogic::map_tree_items_to_commits(
            tree,
            &item_to_commit,
            &commit_map,
        )))
    }
}

impl From<TransportContext> for MonoApiService {
    fn from(ctx: TransportContext) -> Self {
        Self::new(ctx)
    }
}

#[async_trait]
impl ApiHandler for MonoApiService {
    fn get_context(&self) -> Storage {
        self.storage().clone()
    }

    fn object_cache(&self) -> &GitObjectCache {
        self.ctx.git_object_cache()
    }

    async fn get_root_commit(&self) -> Result<Commit, MegaError> {
        let storage = self.storage().mono_storage();
        let refs = storage.get_main_ref("/").await.unwrap().unwrap();
        self.get_commit_by_hash(&refs.ref_commit_hash).await
    }

    async fn save_file_edit(&self, payload: EditFilePayload) -> Result<EditFileResult, GitError> {
        self.save_file_edit_impl(payload).await
    }

    async fn create_monorepo_entry(
        &self,
        entry_info: CreateEntryInfo,
    ) -> Result<CreateEntryResult, GitError> {
        self.create_monorepo_entry_impl(entry_info).await
    }

    fn strip_relative(&self, path: &Path) -> Result<PathBuf, MegaError> {
        Ok(path.to_path_buf())
    }

    async fn get_root_tree(&self, refs: Option<&str>) -> Result<Tree, MegaError> {
        let refs = refs.unwrap_or("").trim();

        if refs.is_empty() {
            let storage = self.storage().mono_storage();
            let refs = storage.get_main_ref("/").await.unwrap().unwrap();
            return self.get_tree_by_hash(&refs.ref_tree_hash).await;
        }

        if refs.len() == 40 && refs.chars().all(|c| c.is_ascii_hexdigit()) {
            let commit = self.get_commit_by_hash(refs).await?;
            return self.get_tree_by_hash(&commit.tree_id.to_string()).await;
        }

        if let Ok(Some(tag)) = self.get_tag(None, refs.to_string()).await {
            let commit = self.get_commit_by_hash(&tag.object_id).await?;
            return self.get_tree_by_hash(&commit.tree_id.to_string()).await;
        }

        Err(MegaError::Other(format!(
            "Invalid refs: '{}' is not a valid commit hash or tag",
            refs
        )))
    }

    async fn get_tree_by_hash(&self, hash: &str) -> Result<Tree, MegaError> {
        let model = self
            .storage()
            .mono_storage()
            .get_tree_by_hash(hash)
            .await?
            .ok_or_else(|| MegaError::NotFound(format!("tree not found: {}", hash)))?;
        Ok(Tree::from_mega_model(model))
    }

    async fn get_commit_by_hash(&self, hash: &str) -> Result<Commit, MegaError> {
        let model = self
            .storage()
            .mono_storage()
            .get_commit_by_hash(hash)
            .await?
            .ok_or_else(|| MegaError::NotFound(format!("commit not found: {}", hash)))?;
        Ok(Commit::from_mega_model(model))
    }

    async fn get_latest_commit(
        &self,
        path: PathBuf,
        refs: Option<&str>,
    ) -> Result<LatestCommitInfo, GitError> {
        // `/third-party/**` is direct import — never attribute monorepo CL merges.
        if let Some(import) = self
            .import_handler_for_path(&path)
            .await
            .map_err(|e| GitError::CustomError(e.to_string()))?
        {
            return commit_ops::get_latest_commit(&import, path, refs).await;
        }
        commit_ops::get_latest_commit(self, path, refs).await
    }

    async fn item_to_commit_map(
        &self,
        path: PathBuf,
        reference: Option<&str>,
    ) -> Result<HashMap<TreeItem, Option<Commit>>, GitError> {
        let import_dir = self.import_dir();
        let has_git_repo = if commit_info_policy::is_path_under_import_dir(&path, &import_dir) {
            let path_str = path.to_str().ok_or_else(|| {
                GitError::CustomError(format!("invalid path: {}", path.display()))
            })?;
            self.find_git_repo_like_path(path_str)
                .await
                .map_err(|e| GitError::CustomError(e.to_string()))?
                .is_some()
        } else {
            false
        };
        let route = commit_info_policy::commit_info_route(&path, &import_dir, has_git_repo);

        match route {
            // Import repos are attached into the monorepo tree, but last-mod must
            // come from the import commit DAG (`Import …`), not monorepo history
            // (`cl merge generated commit`).
            commit_info_policy::CommitInfoRoute::ImportRepo => {
                let import = self
                    .import_handler_for_path(&path)
                    .await
                    .map_err(|e| GitError::CustomError(e.to_string()))?
                    .ok_or_else(|| {
                        GitError::CustomError(
                            "import repo expected for commit-info route".to_string(),
                        )
                    })?;
                import.item_to_commit_map(path, reference).await
            }
            // Under import_dir without a registered git_repo: never walk monorepo
            // history (would still blame CL merges). Prefer denormalized stamps only.
            commit_info_policy::CommitInfoRoute::ImportDirDenormalizedOnly => Ok(self
                .denormalized_item_to_commit_map(&path, reference)
                .await?
                .unwrap_or_default()),
            // `/project/**` and other monorepo paths: denormalized when complete,
            // otherwise history (fixes blank/stale stamps after CL merge reuse).
            commit_info_policy::CommitInfoRoute::MonorepoDenormalizedOrHistory => {
                if Self::prefer_denormalized_commit_map(reference)
                    && let Some(map) = self
                        .denormalized_item_to_commit_map(&path, reference)
                        .await?
                {
                    return Ok(map);
                }
                // Policy gate: only this route may run monorepo history BFS.
                assert!(
                    commit_info_policy::allows_monorepo_history(route),
                    "monorepo history must not run for import_dir paths"
                );
                history::item_to_commit_map(self, path, reference).await
            }
        }
    }

    async fn get_commits_by_hashes(&self, c_hashes: Vec<String>) -> Result<Vec<Commit>, GitError> {
        let commits = self
            .storage()
            .mono_storage()
            .get_commits_by_hashes(&c_hashes)
            .await
            .unwrap();
        Ok(commits.into_iter().map(Commit::from_mega_model).collect())
    }

    async fn create_tag(
        &self,
        repo_path: Option<String>,
        name: String,
        target: Option<String>,
        tagger_name: Option<String>,
        tagger_email: Option<String>,
        message: Option<String>,
    ) -> Result<crate::model::tag::TagInfo, GitError> {
        self.create_tag_impl(repo_path, name, target, tagger_name, tagger_email, message)
            .await
    }

    async fn list_tags(
        &self,
        repo_path: Option<String>,
        pagination: Pagination,
    ) -> Result<(Vec<crate::model::tag::TagInfo>, u64), GitError> {
        self.list_tags_impl(repo_path, pagination).await
    }

    async fn get_tag(
        &self,
        repo_path: Option<String>,
        name: String,
    ) -> Result<Option<crate::model::tag::TagInfo>, GitError> {
        self.get_tag_impl(repo_path, name).await
    }

    async fn delete_tag(&self, repo_path: Option<String>, name: String) -> Result<(), GitError> {
        self.delete_tag_impl(repo_path, name).await
    }
}
