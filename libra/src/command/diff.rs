    path::{Path, PathBuf},
        let old_index = old_hash.map_or("0000000".to_string(), |h| h.to_string()[0..8].to_string());
        let new_index = new_hash.map_or("0000000".to_string(), |h| h.to_string()[0..8].to_string());
        let old_type = infer::get(&old_content);
        let new_type = infer::get(&new_content);
                    file_display(&file, old_hash, old_type),
                    file_display(&file, new_hash, new_type)
// display file with type
fn file_display(file: &Path, hash: Option<&SHA1>, file_type: Option<infer::Type>) -> String {
    let file_name = match hash {
        Some(_) => file.display().to_string(),
        None => "dev/null".to_string(),
    };

    if let Some(file_type) = file_type {
        // Check if the file type is displayable in browser, like image, audio, video, etc.
        if matches!(
            file_type.matcher_type(),
            infer::MatcherType::Audio | infer::MatcherType::Video | infer::MatcherType::Image
        ) {
            return format!("{} ({})", file_name, file_type.mime_type()).to_string();
        }
    }
    file_name
}
