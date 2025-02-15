// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-Observable"))]
__jni_bindgen! {
    /// public class [Observable](https://developer.android.com/reference/android/database/Observable.html)
    ///
    /// Required feature: android-database-Observable
    public class Observable ("android/database/Observable") extends crate::java::lang::Object {

        /// [Observable](https://developer.android.com/reference/android/database/Observable.html#Observable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::Observable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/Observable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/Observable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerObserver](https://developer.android.com/reference/android/database/Observable.html#registerObserver(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn registerObserver<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/Observable", java.flags == PUBLIC, .name == "registerObserver", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/Observable\0", "registerObserver\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterObserver](https://developer.android.com/reference/android/database/Observable.html#unregisterObserver(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn unregisterObserver<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/Observable", java.flags == PUBLIC, .name == "unregisterObserver", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/Observable\0", "unregisterObserver\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterAll](https://developer.android.com/reference/android/database/Observable.html#unregisterAll())
        pub fn unregisterAll<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/Observable", java.flags == PUBLIC, .name == "unregisterAll", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/Observable\0", "unregisterAll\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// **get** protected final [mObservers](https://developer.android.com/reference/android/database/Observable.html#mObservers)
        // ///
        // /// Required feature: java-util-ArrayList
        // #[cfg(any(feature = "all", feature = "java-util-ArrayList"))]
        // pub fn mObservers<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ArrayList>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/database/Observable\0", "mObservers\0", "Ljava/util/ArrayList;\0");
        //         env.get_object_field(class, field)
        //     }
        // }
    }
}
