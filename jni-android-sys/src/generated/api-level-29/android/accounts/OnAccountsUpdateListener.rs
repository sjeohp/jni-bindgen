// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accounts-OnAccountsUpdateListener"))]
__jni_bindgen! {
    /// public interface [OnAccountsUpdateListener](https://developer.android.com/reference/android/accounts/OnAccountsUpdateListener.html)
    ///
    /// Required feature: android-accounts-OnAccountsUpdateListener
    public interface OnAccountsUpdateListener ("android/accounts/OnAccountsUpdateListener") extends crate::java::lang::Object {

        /// [onAccountsUpdated](https://developer.android.com/reference/android/accounts/OnAccountsUpdateListener.html#onAccountsUpdated(android.accounts.Account%5B%5D))
        ///
        /// Required features: "android-accounts-Account"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account")))]
        pub fn onAccountsUpdated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::accounts::Account, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/OnAccountsUpdateListener", java.flags == PUBLIC | ABSTRACT, .name == "onAccountsUpdated", .descriptor == "([Landroid/accounts/Account;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/OnAccountsUpdateListener\0", "onAccountsUpdated\0", "([Landroid/accounts/Account;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
