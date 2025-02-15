// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-backup-RestoreObserver"))]
__jni_bindgen! {
    /// public class [RestoreObserver](https://developer.android.com/reference/android/app/backup/RestoreObserver.html)
    ///
    /// Required feature: android-app-backup-RestoreObserver
    public class RestoreObserver ("android/app/backup/RestoreObserver") extends crate::java::lang::Object {

        /// [RestoreObserver](https://developer.android.com/reference/android/app/backup/RestoreObserver.html#RestoreObserver())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::backup::RestoreObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/RestoreObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/RestoreObserver\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [restoreStarting](https://developer.android.com/reference/android/app/backup/RestoreObserver.html#restoreStarting(int))
        pub fn restoreStarting<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/RestoreObserver", java.flags == PUBLIC, .name == "restoreStarting", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/RestoreObserver\0", "restoreStarting\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onUpdate](https://developer.android.com/reference/android/app/backup/RestoreObserver.html#onUpdate(int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onUpdate<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/RestoreObserver", java.flags == PUBLIC, .name == "onUpdate", .descriptor == "(ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/RestoreObserver\0", "onUpdate\0", "(ILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [restoreFinished](https://developer.android.com/reference/android/app/backup/RestoreObserver.html#restoreFinished(int))
        pub fn restoreFinished<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/RestoreObserver", java.flags == PUBLIC, .name == "restoreFinished", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/RestoreObserver\0", "restoreFinished\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
