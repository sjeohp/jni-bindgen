// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-drm-DrmStore_DrmObjectType"))]
__jni_bindgen! {
    /// public class [DrmStore.DrmObjectType](https://developer.android.com/reference/android/drm/DrmStore.DrmObjectType.html)
    ///
    /// Required feature: android-drm-DrmStore_DrmObjectType
    public class DrmStore_DrmObjectType ("android/drm/DrmStore$DrmObjectType") extends crate::java::lang::Object {

        /// [DrmObjectType](https://developer.android.com/reference/android/drm/DrmStore.DrmObjectType.html#DrmObjectType())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::drm::DrmStore_DrmObjectType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/drm/DrmStore$DrmObjectType", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/drm/DrmStore$DrmObjectType\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CONTENT](https://developer.android.com/reference/android/drm/DrmStore.DrmObjectType.html#CONTENT)
        pub const CONTENT : i32 = 1;

        /// public static final [RIGHTS_OBJECT](https://developer.android.com/reference/android/drm/DrmStore.DrmObjectType.html#RIGHTS_OBJECT)
        pub const RIGHTS_OBJECT : i32 = 2;

        /// public static final [TRIGGER_OBJECT](https://developer.android.com/reference/android/drm/DrmStore.DrmObjectType.html#TRIGGER_OBJECT)
        pub const TRIGGER_OBJECT : i32 = 3;

        /// public static final [UNKNOWN](https://developer.android.com/reference/android/drm/DrmStore.DrmObjectType.html#UNKNOWN)
        pub const UNKNOWN : i32 = 0;
    }
}
