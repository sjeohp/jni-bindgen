// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaDrm_KeyStatus"))]
__jni_bindgen! {
    /// public final class [MediaDrm.KeyStatus](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html)
    ///
    /// Required feature: android-media-MediaDrm_KeyStatus
    public final class MediaDrm_KeyStatus ("android/media/MediaDrm$KeyStatus") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [KeyStatus](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#KeyStatus(byte%5B%5D,%20int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaDrm_KeyStatus>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/MediaDrm$KeyStatus", java.flags == (empty), .name == "<init>", .descriptor == "([BI)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaDrm$KeyStatus\0", "<init>\0", "([BI)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getStatusCode](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#getStatusCode())
        pub fn getStatusCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaDrm$KeyStatus", java.flags == PUBLIC, .name == "getStatusCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaDrm$KeyStatus\0", "getStatusCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyId](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#getKeyId())
        pub fn getKeyId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaDrm$KeyStatus", java.flags == PUBLIC, .name == "getKeyId", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaDrm$KeyStatus\0", "getKeyId\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [STATUS_EXPIRED](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#STATUS_EXPIRED)
        pub const STATUS_EXPIRED : i32 = 1;

        /// public static final [STATUS_INTERNAL_ERROR](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#STATUS_INTERNAL_ERROR)
        pub const STATUS_INTERNAL_ERROR : i32 = 4;

        /// public static final [STATUS_OUTPUT_NOT_ALLOWED](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#STATUS_OUTPUT_NOT_ALLOWED)
        pub const STATUS_OUTPUT_NOT_ALLOWED : i32 = 2;

        /// public static final [STATUS_PENDING](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#STATUS_PENDING)
        pub const STATUS_PENDING : i32 = 3;

        /// public static final [STATUS_USABLE](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#STATUS_USABLE)
        pub const STATUS_USABLE : i32 = 0;

        /// public static final [STATUS_USABLE_IN_FUTURE](https://developer.android.com/reference/android/media/MediaDrm.KeyStatus.html#STATUS_USABLE_IN_FUTURE)
        pub const STATUS_USABLE_IN_FUTURE : i32 = 5;
    }
}
