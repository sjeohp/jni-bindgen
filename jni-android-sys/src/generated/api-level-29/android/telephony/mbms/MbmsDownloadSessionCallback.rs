// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-mbms-MbmsDownloadSessionCallback"))]
__jni_bindgen! {
    /// public class [MbmsDownloadSessionCallback](https://developer.android.com/reference/android/telephony/mbms/MbmsDownloadSessionCallback.html)
    ///
    /// Required feature: android-telephony-mbms-MbmsDownloadSessionCallback
    public class MbmsDownloadSessionCallback ("android/telephony/mbms/MbmsDownloadSessionCallback") extends crate::java::lang::Object {

        /// [MbmsDownloadSessionCallback](https://developer.android.com/reference/android/telephony/mbms/MbmsDownloadSessionCallback.html#MbmsDownloadSessionCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::mbms::MbmsDownloadSessionCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/MbmsDownloadSessionCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/MbmsDownloadSessionCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onError](https://developer.android.com/reference/android/telephony/mbms/MbmsDownloadSessionCallback.html#onError(int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onError<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/MbmsDownloadSessionCallback", java.flags == PUBLIC, .name == "onError", .descriptor == "(ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/MbmsDownloadSessionCallback\0", "onError\0", "(ILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFileServicesUpdated](https://developer.android.com/reference/android/telephony/mbms/MbmsDownloadSessionCallback.html#onFileServicesUpdated(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn onFileServicesUpdated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/MbmsDownloadSessionCallback", java.flags == PUBLIC, .name == "onFileServicesUpdated", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/MbmsDownloadSessionCallback\0", "onFileServicesUpdated\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onMiddlewareReady](https://developer.android.com/reference/android/telephony/mbms/MbmsDownloadSessionCallback.html#onMiddlewareReady())
        pub fn onMiddlewareReady<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/MbmsDownloadSessionCallback", java.flags == PUBLIC, .name == "onMiddlewareReady", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/MbmsDownloadSessionCallback\0", "onMiddlewareReady\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
