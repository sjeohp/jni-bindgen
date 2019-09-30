// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accounts-AccountManagerFuture"))]
__jni_bindgen! {
    /// public interface [AccountManagerFuture](https://developer.android.com/reference/android/accounts/AccountManagerFuture.html)
    ///
    /// Required feature: android-accounts-AccountManagerFuture
    public interface AccountManagerFuture ("android/accounts/AccountManagerFuture") extends crate::java::lang::Object {

        /// [cancel](https://developer.android.com/reference/android/accounts/AccountManagerFuture.html#cancel(boolean))
        pub fn cancel<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AccountManagerFuture", java.flags == PUBLIC | ABSTRACT, .name == "cancel", .descriptor == "(Z)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AccountManagerFuture\0", "cancel\0", "(Z)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isCancelled](https://developer.android.com/reference/android/accounts/AccountManagerFuture.html#isCancelled())
        pub fn isCancelled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AccountManagerFuture", java.flags == PUBLIC | ABSTRACT, .name == "isCancelled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AccountManagerFuture\0", "isCancelled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDone](https://developer.android.com/reference/android/accounts/AccountManagerFuture.html#isDone())
        pub fn isDone<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AccountManagerFuture", java.flags == PUBLIC | ABSTRACT, .name == "isDone", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AccountManagerFuture\0", "isDone\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResult](https://developer.android.com/reference/android/accounts/AccountManagerFuture.html#getResult())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getResult<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AccountManagerFuture", java.flags == PUBLIC | ABSTRACT, .name == "getResult", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AccountManagerFuture\0", "getResult\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResult](https://developer.android.com/reference/android/accounts/AccountManagerFuture.html#getResult(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-lang-Object", "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-concurrent-TimeUnit")))]
        pub fn getResult_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AccountManagerFuture", java.flags == PUBLIC | ABSTRACT, .name == "getResult", .descriptor == "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AccountManagerFuture\0", "getResult\0", "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
