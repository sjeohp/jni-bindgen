// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-DownloadManager_Query"))]
__jni_bindgen! {
    /// public class [DownloadManager.Query](https://developer.android.com/reference/android/app/DownloadManager.Query.html)
    ///
    /// Required feature: android-app-DownloadManager_Query
    public class DownloadManager_Query ("android/app/DownloadManager$Query") extends crate::java::lang::Object {

        /// [Query](https://developer.android.com/reference/android/app/DownloadManager.Query.html#Query())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::DownloadManager_Query>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DownloadManager$Query", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DownloadManager$Query\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFilterById](https://developer.android.com/reference/android/app/DownloadManager.Query.html#setFilterById(long...))
        ///
        /// Required features: "android-app-DownloadManager_Query"
        #[cfg(any(feature = "all", all(feature = "android-app-DownloadManager_Query")))]
        pub fn setFilterById<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::DownloadManager_Query>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DownloadManager$Query", java.flags == PUBLIC | VARARGS, .name == "setFilterById", .descriptor == "([J)Landroid/app/DownloadManager$Query;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DownloadManager$Query\0", "setFilterById\0", "([J)Landroid/app/DownloadManager$Query;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFilterByStatus](https://developer.android.com/reference/android/app/DownloadManager.Query.html#setFilterByStatus(int))
        ///
        /// Required features: "android-app-DownloadManager_Query"
        #[cfg(any(feature = "all", all(feature = "android-app-DownloadManager_Query")))]
        pub fn setFilterByStatus<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::DownloadManager_Query>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DownloadManager$Query", java.flags == PUBLIC, .name == "setFilterByStatus", .descriptor == "(I)Landroid/app/DownloadManager$Query;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DownloadManager$Query\0", "setFilterByStatus\0", "(I)Landroid/app/DownloadManager$Query;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
