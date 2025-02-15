// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-mbms-DownloadProgressListener"))]
__jni_bindgen! {
    /// public class [DownloadProgressListener](https://developer.android.com/reference/android/telephony/mbms/DownloadProgressListener.html)
    ///
    /// Required feature: android-telephony-mbms-DownloadProgressListener
    public class DownloadProgressListener ("android/telephony/mbms/DownloadProgressListener") extends crate::java::lang::Object {

        /// [DownloadProgressListener](https://developer.android.com/reference/android/telephony/mbms/DownloadProgressListener.html#DownloadProgressListener())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::mbms::DownloadProgressListener>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/DownloadProgressListener", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/DownloadProgressListener\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onProgressUpdated](https://developer.android.com/reference/android/telephony/mbms/DownloadProgressListener.html#onProgressUpdated(android.telephony.mbms.DownloadRequest,%20android.telephony.mbms.FileInfo,%20int,%20int,%20int,%20int))
        ///
        /// Required features: "android-telephony-mbms-DownloadRequest", "android-telephony-mbms-FileInfo"
        #[cfg(any(feature = "all", all(feature = "android-telephony-mbms-DownloadRequest", feature = "android-telephony-mbms-FileInfo")))]
        pub fn onProgressUpdated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::mbms::DownloadRequest>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::mbms::FileInfo>>, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/DownloadProgressListener", java.flags == PUBLIC, .name == "onProgressUpdated", .descriptor == "(Landroid/telephony/mbms/DownloadRequest;Landroid/telephony/mbms/FileInfo;IIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/DownloadProgressListener\0", "onProgressUpdated\0", "(Landroid/telephony/mbms/DownloadRequest;Landroid/telephony/mbms/FileInfo;IIII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
