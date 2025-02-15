// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-drm-DrmInfoEvent"))]
__jni_bindgen! {
    /// public class [DrmInfoEvent](https://developer.android.com/reference/android/drm/DrmInfoEvent.html)
    ///
    /// Required feature: android-drm-DrmInfoEvent
    public class DrmInfoEvent ("android/drm/DrmInfoEvent") extends crate::android::drm::DrmEvent {

        /// [DrmInfoEvent](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#DrmInfoEvent(int,%20int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_int_int_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::drm::DrmInfoEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/drm/DrmInfoEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/drm/DrmInfoEvent\0", "<init>\0", "(IILjava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DrmInfoEvent](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#DrmInfoEvent(int,%20int,%20java.lang.String,%20java.util.HashMap))
        ///
        /// Required features: "java-lang-String", "java-util-HashMap"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-HashMap")))]
        pub fn new_int_int_String_HashMap<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::HashMap>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::drm::DrmInfoEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/drm/DrmInfoEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IILjava/lang/String;Ljava/util/HashMap;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/drm/DrmInfoEvent\0", "<init>\0", "(IILjava/lang/String;Ljava/util/HashMap;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [TYPE_ACCOUNT_ALREADY_REGISTERED](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#TYPE_ACCOUNT_ALREADY_REGISTERED)
        pub const TYPE_ACCOUNT_ALREADY_REGISTERED : i32 = 5;

        /// public static final [TYPE_ALREADY_REGISTERED_BY_ANOTHER_ACCOUNT](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#TYPE_ALREADY_REGISTERED_BY_ANOTHER_ACCOUNT)
        pub const TYPE_ALREADY_REGISTERED_BY_ANOTHER_ACCOUNT : i32 = 1;

        /// public static final [TYPE_REMOVE_RIGHTS](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#TYPE_REMOVE_RIGHTS)
        pub const TYPE_REMOVE_RIGHTS : i32 = 2;

        /// public static final [TYPE_RIGHTS_INSTALLED](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#TYPE_RIGHTS_INSTALLED)
        pub const TYPE_RIGHTS_INSTALLED : i32 = 3;

        /// public static final [TYPE_RIGHTS_REMOVED](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#TYPE_RIGHTS_REMOVED)
        pub const TYPE_RIGHTS_REMOVED : i32 = 6;

        /// public static final [TYPE_WAIT_FOR_RIGHTS](https://developer.android.com/reference/android/drm/DrmInfoEvent.html#TYPE_WAIT_FOR_RIGHTS)
        pub const TYPE_WAIT_FOR_RIGHTS : i32 = 4;
    }
}
