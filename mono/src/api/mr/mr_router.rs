use std::collections::HashMap;
use crate::api::mr::{FilesChangedItem, FilesChangedList, MRDetail, MRStatusParams, MrInfoItem};
    Router::new().nest(
        "/mr",
        Router::new()
            .route("/list", post(fetch_mr_list))
            .route("/{link}/detail", get(mr_detail))
            .route("/{link}/merge", post(merge))
            .route("/{link}/close", post(close_mr))
            .route("/{link}/reopen", post(reopen_mr))
            .route("/{link}/files-changed", get(get_mr_files_changed))
            .route("/{link}/comment", post(save_comment))
            .route("/comment/{conv_id}/delete", post(delete_comment)),
    )
                let conversations = state.mr_stg().get_mr_conversations(&link).await.unwrap();
                detail.conversations = conversations.into_iter().map(|x| x.into()).collect();
) -> Result<Json<CommonResult<FilesChangedList>>, ApiError> {
        Ok(data) => {
            let diff_files = extract_files_with_status(&data);
            let mut diff_list: Vec<FilesChangedItem> = vec![];
            for (path, status) in diff_files {
                diff_list.push(FilesChangedItem { path, status });
            }

            CommonResult::success(Some(FilesChangedList {
                files: diff_list,
                content: data,
            }))
        }


fn extract_files_with_status(diff_output: &str) -> HashMap<String, String> {
    let mut files = HashMap::new();

    let chunks: Vec<&str> = diff_output.split("diff --git ").collect();

    for chunk in chunks.iter().skip(1) {
        let lines: Vec<&str> = chunk.split_whitespace().collect();
        if lines.len() >= 2 {
            let current_file = lines[0].trim_start_matches("a/").to_string();
            files.insert(current_file.clone(), "modified".to_string()); // 默认状态为修改
            if chunk.contains("new file mode") {
                files.insert(current_file, "new".to_string());
            } else if chunk.contains("deleted file mode") {
                files.insert(current_file, "deleted".to_string());
            }
        }
    }
    files
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::api::mr::mr_router::extract_files_with_status;

    #[test]
    fn test_parse_diff_result_to_filelist() {
        let diff_output = r#"
        diff --git a/ceres/src/api_service/mono_api_service.rs b/ceres/src/api_service/mono_api_service.rs
        new file mode 100644
        index 0000000..561296a1
        @@ -1,0 +1,595 @@
        fn main() {
            println!("Hello, world!");
        }
        diff --git a/ceres/src/lib.rs b/ceres/src/lib.rs
        index 1234567..89abcdef 100644
        --- a/ceres/src/lib.rs
        +++ b/ceres/src/lib.rs
        @@ -10,7 +10,8 @@
        diff --git a/ceres/src/removed.rs b/ceres/src/removed.rs
        deleted file mode 100644
        "#;
        let files_with_status = extract_files_with_status(diff_output);
        println!("Files with status:");
        for (file, status) in &files_with_status {
            println!("{} ({})", file, status);
        }

        let mut expected = HashMap::new();
        expected.insert(
            "ceres/src/api_service/mono_api_service.rs".to_string(),
            "new".to_string(),
        );
        expected.insert("ceres/src/lib.rs".to_string(), "modified".to_string());
        expected.insert("ceres/src/removed.rs".to_string(), "deleted".to_string());

        assert_eq!(files_with_status, expected);
    }
}