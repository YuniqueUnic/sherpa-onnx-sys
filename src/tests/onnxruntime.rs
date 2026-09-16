use std::ffi::CStr;

#[test]
fn sherpa_uses_the_linked_onnxruntime() {
    // SAFETY: The getter returns a borrowed, process-lifetime C string.
    let sherpa_version_ptr = unsafe { crate::SherpaOnnxGetOnnxruntimeVersionStr() };
    assert!(
        !sherpa_version_ptr.is_null(),
        "Sherpa's ORT runtime version"
    );

    // SAFETY: The non-null pointer refers to a NUL-terminated version string.
    let sherpa_version = unsafe { CStr::from_ptr(sherpa_version_ptr) };
    assert!(
        !sherpa_version.to_bytes().is_empty(),
        "ORT version is present"
    );

    #[cfg(all(
        not(feature = "shared"),
        any(target_os = "linux", target_os = "macos", target_os = "windows")
    ))]
    {
        // SAFETY: ORT exposes a process-lifetime API table; as_ref handles null.
        let api_base = unsafe { ort_sys::OrtGetApiBase().as_ref() }.expect("ORT C API base");

        // SAFETY: GetApi accepts a version and returns null when unsupported.
        // Sherpa 1.13.8's prebuilt libraries were compiled against ORT API 28.
        let api = unsafe { (api_base.GetApi)(28) };
        assert!(!api.is_null(), "the native runtime must support ORT API 28");

        // SAFETY: The getter returns a borrowed, process-lifetime C string.
        let ort_version_ptr = unsafe { (api_base.GetVersionString)() };
        assert!(!ort_version_ptr.is_null(), "ORT runtime version");
        // SAFETY: The non-null pointer refers to a NUL-terminated version string.
        let ort_version = unsafe { CStr::from_ptr(ort_version_ptr) };
        assert_eq!(
            sherpa_version, ort_version,
            "both APIs must use the same ORT version"
        );
        assert_eq!(
            sherpa_version_ptr, ort_version_ptr,
            "both APIs must return the same native runtime's version string"
        );
    }
}
