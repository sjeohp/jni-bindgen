// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaCodec_CodecException"))]
__jni_bindgen! {
    /// public final class [MediaCodec.CodecException](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html)
    ///
    /// Required feature: android-media-MediaCodec_CodecException
    public final class MediaCodec_CodecException ("android/media/MediaCodec$CodecException") extends crate::java::lang::IllegalStateException {

        // // Not emitting: Non-public method
        // /// [CodecException](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#CodecException(int,%20int,%20java.lang.String))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaCodec_CodecException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/MediaCodec$CodecException", java.flags == (empty), .name == "<init>", .descriptor == "(IILjava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CodecException\0", "<init>\0", "(IILjava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isTransient](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#isTransient())
        pub fn isTransient<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CodecException", java.flags == PUBLIC, .name == "isTransient", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CodecException\0", "isTransient\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRecoverable](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#isRecoverable())
        pub fn isRecoverable<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CodecException", java.flags == PUBLIC, .name == "isRecoverable", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CodecException\0", "isRecoverable\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getErrorCode](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#getErrorCode())
        pub fn getErrorCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CodecException", java.flags == PUBLIC, .name == "getErrorCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CodecException\0", "getErrorCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDiagnosticInfo](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#getDiagnosticInfo())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getDiagnosticInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CodecException", java.flags == PUBLIC, .name == "getDiagnosticInfo", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CodecException\0", "getDiagnosticInfo\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ERROR_INSUFFICIENT_RESOURCE](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#ERROR_INSUFFICIENT_RESOURCE)
        pub const ERROR_INSUFFICIENT_RESOURCE : i32 = 1100;

        /// public static final [ERROR_RECLAIMED](https://developer.android.com/reference/android/media/MediaCodec.CodecException.html#ERROR_RECLAIMED)
        pub const ERROR_RECLAIMED : i32 = 1101;
    }
}
