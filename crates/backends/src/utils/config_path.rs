#[allow(unused_imports)]
use anyhow::{Error, Result, anyhow};
use std::path::PathBuf;

#[cfg(target_os = "android")]
pub fn get_config_dir() -> Result<PathBuf, Error> {
    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::objects::{JObject, JString};

    // Get the JVM from the current thread
    let vm = unsafe {
        JavaVM::from_raw(ndk_context::android_context().vm().cast())
            .map_err(|e| anyhow!("Failed to get JavaVM: {:?}", e))?
    };

    let mut env = vm
        .attach_current_thread()
        .map_err(|e| anyhow!("Failed to attach thread: {:?}", e))?;

    // Get context from ndk_context
    let context = ndk_context::android_context().context().cast();
    let context = unsafe { JObject::from_raw(context) };

    // Call getFilesDir() on the context
    let file_dir = env.call_method(&context, "getFilesDir", "()Ljava/io/File;", &[])?;
    let file_obj = JObject::from(file_dir.l()?);
    let path_str = env.call_method(&file_obj, "getAbsolutePath", "()Ljava/lang/String;", &[])?;
    let path_jstring = JString::from(path_str.l()?);
    let path: String = env.get_string(&path_jstring)?.into();

    Ok(PathBuf::from(path).join("config"))
}

#[cfg(target_os = "ios")]
pub fn get_config_dir() -> Result<PathBuf, Error> {
    use objc::runtime::Object;
    use objc::{msg_send, sel, sel_impl};

    let ns_document_dir: i32 = 9; // NSDocumentDirectory
    let ns_user_domain: i32 = 1; // NSUserDomainMask

    let paths: *mut Object = unsafe {
        let search_paths: *mut Object = msg_send![
            Class::get("NSSearchPathForDirectoriesInDomains").unwrap(),
            searchPathForDirectoriesInDomains: ns_document_dir,
            inDomains: ns_user_domain
        ];
        search_paths
    };

    if paths.is_null() {
        return Err(anyhow!("Failed to get document directory"));
    }

    let path: *mut Object = unsafe { msg_send![paths, firstObject] };
    let path_str: *mut Object = unsafe { msg_send![path, path] };
    let c_str: *const std::os::raw::c_char = unsafe { msg_send![path_str, UTF8String] };
    let path = unsafe { std::ffi::CStr::from_ptr(c_str).to_str()?.to_owned() };

    Ok(PathBuf::from(path).join("config"))
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn get_config_dir() -> Result<PathBuf, Error> {
    dirs::config_dir().ok_or(anyhow::anyhow!("Cannot locate config_dir!"))
}
// 在其他平台使用默认实现
