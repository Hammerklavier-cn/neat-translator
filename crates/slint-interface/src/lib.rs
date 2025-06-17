mod slint_interface;

slint::include_modules!();

#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    slint_interface::run().unwrap();
}
