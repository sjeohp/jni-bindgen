// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-drm-DrmUtils"))]
__jni_bindgen! {
    /// public class [DrmUtils](https://developer.android.com/reference/android/drm/DrmUtils.html)
    ///
    /// Required feature: android-drm-DrmUtils
    public class DrmUtils ("android/drm/DrmUtils") extends crate::java::lang::Object {

        /// [DrmUtils](https://developer.android.com/reference/android/drm/DrmUtils.html#DrmUtils())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::drm::DrmUtils>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/drm/DrmUtils", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/drm/DrmUtils\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtendedMetadataParser](https://developer.android.com/reference/android/drm/DrmUtils.html#getExtendedMetadataParser(byte%5B%5D))
        ///
        /// Required features: "android-drm-DrmUtils_ExtendedMetadataParser"
        #[cfg(any(feature = "all", all(feature = "android-drm-DrmUtils_ExtendedMetadataParser")))]
        pub fn getExtendedMetadataParser<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::drm::DrmUtils_ExtendedMetadataParser>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/drm/DrmUtils", java.flags == PUBLIC | STATIC, .name == "getExtendedMetadataParser", .descriptor == "([B)Landroid/drm/DrmUtils$ExtendedMetadataParser;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/drm/DrmUtils\0", "getExtendedMetadataParser\0", "([B)Landroid/drm/DrmUtils$ExtendedMetadataParser;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
