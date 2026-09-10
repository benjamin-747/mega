use std::time::Duration;

use callisto::orion_vm_image;
use common::errors::MegaError;
use io_orbit::{
    factory::MegaObjectStorageWrapper,
    object_storage::{ObjectKey, ObjectNamespace},
};
use reqwest::Method;

use crate::storage::{
    base_storage::{BaseStorage, StorageConnector},
    orion_vm_image_storage::{OrionVmImageStorage, UpsertOrionVmImage},
};

/// Presigned GET TTL for scheduler first-pull of multi-GB qcow2 images.
pub const ORION_IMAGE_PRESIGN_TTL_SECS: u64 = 6 * 60 * 60;

#[derive(Clone)]
pub struct OrionVmImageService {
    st: OrionVmImageStorage,
    obj_storage: MegaObjectStorageWrapper,
}

impl OrionVmImageService {
    pub fn new(base: BaseStorage, obj_storage: MegaObjectStorageWrapper) -> Self {
        Self {
            st: OrionVmImageStorage { base },
            obj_storage,
        }
    }

    pub fn mock() -> Self {
        Self::new(BaseStorage::mock(), MegaObjectStorageWrapper::mock())
    }

    pub fn supports_presigned_urls(&self) -> bool {
        self.obj_storage.supports_presigned_urls()
    }

    pub async fn list(&self) -> Result<Vec<orion_vm_image::Model>, MegaError> {
        self.st.list_all().await
    }

    pub async fn get(&self, id: &str) -> Result<Option<orion_vm_image::Model>, MegaError> {
        self.st.find_by_id(id).await
    }

    pub async fn upsert(
        &self,
        input: UpsertOrionVmImage,
    ) -> Result<orion_vm_image::Model, MegaError> {
        self.st.upsert(input).await
    }

    /// Delete catalog row and best-effort remove qcow2 + sidecar objects.
    pub async fn delete(&self, id: &str) -> Result<Option<orion_vm_image::Model>, MegaError> {
        let Some(model) = self.st.delete_by_id(id).await? else {
            return Ok(None);
        };
        let qcow2 = ObjectKey {
            namespace: ObjectNamespace::OrionImage,
            key: model.object_key.clone(),
        };
        if let Err(e) = self.obj_storage.inner.delete(&qcow2).await {
            tracing::warn!(
                "failed to delete orion image object {}: {}",
                qcow2.default_sharding(),
                e
            );
        }
        if let Some(info_key) = &model.info_object_key {
            let info = ObjectKey {
                namespace: ObjectNamespace::OrionImage,
                key: info_key.clone(),
            };
            if let Err(e) = self.obj_storage.inner.delete(&info).await {
                tracing::warn!(
                    "failed to delete orion image info object {}: {}",
                    info.default_sharding(),
                    e
                );
            }
        }
        Ok(Some(model))
    }

    pub async fn signed_get_url(&self, model: &orion_vm_image::Model) -> Result<String, MegaError> {
        let key = ObjectKey {
            namespace: ObjectNamespace::OrionImage,
            key: model.object_key.clone(),
        };
        let url = self
            .obj_storage
            .inner
            .signed_url(
                &key,
                Method::GET,
                Duration::from_secs(ORION_IMAGE_PRESIGN_TTL_SECS),
            )
            .await?;
        url.ok_or_else(|| {
            MegaError::ObjStorage(
                "object storage does not support presigned URLs; configure S3-compatible RustFS"
                    .into(),
            )
        })
    }
}

/// Strip optional `sha256:` / `sha512:` prefix for object-key layout.
pub fn digest_hex(digest: &str) -> &str {
    digest
        .strip_prefix("sha256:")
        .or_else(|| digest.strip_prefix("sha512:"))
        .unwrap_or(digest)
}
