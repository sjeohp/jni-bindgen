// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaCodec_CryptoInfo_Pattern"))]
__jni_bindgen! {
    /// public final class [MediaCodec.CryptoInfo.Pattern](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.Pattern.html)
    ///
    /// Required feature: android-media-MediaCodec_CryptoInfo_Pattern
    public final class MediaCodec_CryptoInfo_Pattern ("android/media/MediaCodec$CryptoInfo$Pattern") extends crate::java::lang::Object {

        /// [Pattern](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.Pattern.html#Pattern(int,%20int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaCodec_CryptoInfo_Pattern>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo$Pattern", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo$Pattern\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.Pattern.html#set(int,%20int))
        pub fn set<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo$Pattern", java.flags == PUBLIC, .name == "set", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo$Pattern\0", "set\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSkipBlocks](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.Pattern.html#getSkipBlocks())
        pub fn getSkipBlocks<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo$Pattern", java.flags == PUBLIC, .name == "getSkipBlocks", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo$Pattern\0", "getSkipBlocks\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncryptBlocks](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.Pattern.html#getEncryptBlocks())
        pub fn getEncryptBlocks<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo$Pattern", java.flags == PUBLIC, .name == "getEncryptBlocks", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo$Pattern\0", "getEncryptBlocks\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
