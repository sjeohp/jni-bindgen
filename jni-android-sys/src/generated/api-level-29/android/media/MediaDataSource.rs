// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaDataSource"))]
__jni_bindgen! {
    /// public class [MediaDataSource](https://developer.android.com/reference/android/media/MediaDataSource.html)
    ///
    /// Required feature: android-media-MediaDataSource
    public class MediaDataSource ("android/media/MediaDataSource") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        /// [MediaDataSource](https://developer.android.com/reference/android/media/MediaDataSource.html#MediaDataSource())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaDataSource>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaDataSource", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaDataSource\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readAt](https://developer.android.com/reference/android/media/MediaDataSource.html#readAt(long,%20byte%5B%5D,%20int,%20int))
        pub fn readAt<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaDataSource", java.flags == PUBLIC | ABSTRACT, .name == "readAt", .descriptor == "(J[BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaDataSource\0", "readAt\0", "(J[BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSize](https://developer.android.com/reference/android/media/MediaDataSource.html#getSize())
        pub fn getSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaDataSource", java.flags == PUBLIC | ABSTRACT, .name == "getSize", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaDataSource\0", "getSize\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
