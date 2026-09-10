use std::path::{Path, PathBuf};

/// Candidate payload roots after an archive has been unpacked into its own cache directory.
///
/// Some releases put the platform payload directly at the extraction root, while desktop
/// tarballs wrap it in a directory named after the archive. Both are valid release layouts.
pub fn payload_roots(extracted_dir: &Path, archive_stem: &str) -> [PathBuf; 2] {
    [extracted_dir.to_path_buf(), extracted_dir.join(archive_stem)]
}

/// Find the linkable platform directory without coupling cache layout to archive layout.
pub fn find_prebuilt_lib_dir(
    extracted_dir: &Path,
    archive_stem: &str,
    lib_dir_name: &str,
    android_abi: &str,
) -> Option<PathBuf> {
    for root in payload_roots(extracted_dir, archive_stem) {
        let lib_dir = root.join(lib_dir_name);
        if lib_dir.is_dir() {
            return Some(lib_dir);
        }

        let android_lib_dir = root.join("jniLibs").join(android_abi);
        if android_lib_dir.is_dir() {
            return Some(android_lib_dir);
        }
    }

    None
}
