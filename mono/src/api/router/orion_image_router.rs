use anyhow::anyhow;
use api_model::common::CommonResult;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use ceres::model::orion_image::{
    OrionVmImageListResponse, OrionVmImageResponse, RegisterOrionVmImageRequest,
};
use jupiter::storage::orion_vm_image_storage::UpsertOrionVmImage;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::{
    MonoApiServiceState, api_common::group_permission::ensure_admin, api_doc::ORION_RUNNER_TAG,
    error::ApiError, oauth::model::LoginUser,
};

pub fn routers() -> OpenApiRouter<MonoApiServiceState> {
    OpenApiRouter::new().nest(
        "/orion/images",
        OpenApiRouter::new()
            .routes(routes!(list_orion_images))
            .routes(routes!(register_orion_image))
            .routes(routes!(delete_orion_image)),
    )
}

fn to_response(
    id: String,
    digest: String,
    object_key: String,
    info_object_key: Option<String>,
    image_name: Option<String>,
    built_at: Option<String>,
    rust: Option<String>,
    buck2: Option<String>,
    python: Option<String>,
    kernel: Option<String>,
    size_bytes: Option<i64>,
    label: Option<String>,
    created_at: chrono::NaiveDateTime,
) -> OrionVmImageResponse {
    OrionVmImageResponse {
        id,
        digest,
        object_key,
        info_object_key,
        image_name,
        built_at,
        rust,
        buck2,
        python,
        kernel,
        size_bytes,
        label,
        created_at: created_at.and_utc().to_rfc3339(),
    }
}

macro_rules! model_to_response {
    ($m:expr) => {
        to_response(
            $m.id,
            $m.digest,
            $m.object_key,
            $m.info_object_key,
            $m.image_name,
            $m.built_at,
            $m.rust,
            $m.buck2,
            $m.python,
            $m.kernel,
            $m.size_bytes,
            $m.label,
            $m.created_at,
        )
    };
}

/// List registered Orion VM images (toolchain metadata for the UI catalog).
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = CommonResult<OrionVmImageListResponse>, content_type = "application/json"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only"),
    ),
    tag = ORION_RUNNER_TAG
)]
async fn list_orion_images(
    user: LoginUser,
    State(state): State<MonoApiServiceState>,
) -> Result<Json<CommonResult<OrionVmImageListResponse>>, ApiError> {
    ensure_admin(&state, &user).await?;
    let images = state
        .services()
        .storage()
        .orion_vm_image_service
        .list()
        .await
        .map_err(ApiError::from)?;
    let images: Vec<_> = images.into_iter().map(|m| model_to_response!(m)).collect();
    let count = images.len();
    Ok(Json(CommonResult::success(Some(
        OrionVmImageListResponse { count, images },
    ))))
}

/// Register (upsert) an image after build-script upload to RustFS.
#[utoipa::path(
    post,
    path = "/",
    request_body = RegisterOrionVmImageRequest,
    responses(
        (status = 200, body = CommonResult<OrionVmImageResponse>, content_type = "application/json"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only"),
    ),
    tag = ORION_RUNNER_TAG
)]
async fn register_orion_image(
    user: LoginUser,
    State(state): State<MonoApiServiceState>,
    Json(req): Json<RegisterOrionVmImageRequest>,
) -> Result<Json<CommonResult<OrionVmImageResponse>>, ApiError> {
    ensure_admin(&state, &user).await?;
    let digest = req.digest.trim().to_string();
    if digest.is_empty() || req.object_key.trim().is_empty() {
        return Err(ApiError::bad_request(anyhow!(
            "digest and object_key are required"
        )));
    }
    if !(digest.starts_with("sha256:") || digest.starts_with("sha512:")) {
        return Err(ApiError::bad_request(anyhow!(
            "digest must start with sha256: or sha512:"
        )));
    }

    let model = state
        .services()
        .storage()
        .orion_vm_image_service
        .upsert(UpsertOrionVmImage {
            digest,
            object_key: req.object_key.trim().trim_start_matches('/').to_string(),
            info_object_key: req
                .info_object_key
                .map(|k| k.trim().trim_start_matches('/').to_string())
                .filter(|k| !k.is_empty()),
            image_name: req.image_name,
            built_at: req.built_at,
            rust: req.rust,
            buck2: req.buck2,
            python: req.python,
            kernel: req.kernel,
            size_bytes: req.size_bytes,
            label: req.label,
        })
        .await
        .map_err(ApiError::from)?;

    Ok(Json(CommonResult::success(Some(model_to_response!(model)))))
}

/// Delete a catalog entry and its RustFS objects.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Catalog image id")
    ),
    responses(
        (status = 200, body = CommonResult<OrionVmImageResponse>, content_type = "application/json"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Image still in use by a runner"),
    ),
    tag = ORION_RUNNER_TAG
)]
async fn delete_orion_image(
    user: LoginUser,
    State(state): State<MonoApiServiceState>,
    Path(id): Path<String>,
) -> Result<Json<CommonResult<OrionVmImageResponse>>, ApiError> {
    ensure_admin(&state, &user).await?;

    let svc = &state.services().storage().orion_vm_image_service;
    let existing =
        svc.get(&id).await.map_err(ApiError::from)?.ok_or_else(|| {
            ApiError::with_status(StatusCode::NOT_FOUND, anyhow!("image not found"))
        })?;

    if let Some(client) = state.orion_scheduler_client() {
        if let Ok(list) = client.list_vms().await {
            let in_use = list.vms.iter().any(|vm| {
                vm.image_digest
                    .as_deref()
                    .is_some_and(|d| d == existing.digest)
            });
            if in_use {
                return Err(ApiError::with_status(
                    StatusCode::CONFLICT,
                    anyhow!(
                        "image {} is still referenced by a tracked runner",
                        existing.digest
                    ),
                ));
            }
        }
    }

    let deleted = svc
        .delete(&id)
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::with_status(StatusCode::NOT_FOUND, anyhow!("image not found")))?;

    Ok(Json(CommonResult::success(Some(model_to_response!(
        deleted
    )))))
}
