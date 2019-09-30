// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accounts-AbstractAccountAuthenticator"))]
__jni_bindgen! {
    /// public class [AbstractAccountAuthenticator](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html)
    ///
    /// Required feature: android-accounts-AbstractAccountAuthenticator
    public class AbstractAccountAuthenticator ("android/accounts/AbstractAccountAuthenticator") extends crate::java::lang::Object {

        /// [AbstractAccountAuthenticator](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#AbstractAccountAuthenticator(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::accounts::AbstractAccountAuthenticator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIBinder](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#getIBinder())
        ///
        /// Required features: "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-os-IBinder")))]
        pub fn getIBinder<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | FINAL, .name == "getIBinder", .descriptor == "()Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "getIBinder\0", "()Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [editProperties](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#editProperties(android.accounts.AccountAuthenticatorResponse,%20java.lang.String))
        ///
        /// Required features: "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn editProperties<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "editProperties", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "editProperties\0", "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAccount](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#addAccount(android.accounts.AccountAuthenticatorResponse,%20java.lang.String,%20java.lang.String,%20java.lang.String%5B%5D,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn addAccount<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "addAccount", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "addAccount\0", "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [confirmCredentials](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#confirmCredentials(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle")))]
        pub fn confirmCredentials<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "confirmCredentials", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "confirmCredentials\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAuthToken](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#getAuthToken(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn getAuthToken<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "getAuthToken", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "getAuthToken\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAuthTokenLabel](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#getAuthTokenLabel(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAuthTokenLabel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "getAuthTokenLabel", .descriptor == "(Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "getAuthTokenLabel\0", "(Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateCredentials](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#updateCredentials(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn updateCredentials<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "updateCredentials", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "updateCredentials\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasFeatures](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#hasFeatures(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20java.lang.String%5B%5D))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn hasFeatures<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC | ABSTRACT, .name == "hasFeatures", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;[Ljava/lang/String;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "hasFeatures\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;[Ljava/lang/String;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccountRemovalAllowed](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#getAccountRemovalAllowed(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle")))]
        pub fn getAccountRemovalAllowed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "getAccountRemovalAllowed", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "getAccountRemovalAllowed\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccountCredentialsForCloning](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#getAccountCredentialsForCloning(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle")))]
        pub fn getAccountCredentialsForCloning<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "getAccountCredentialsForCloning", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "getAccountCredentialsForCloning\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAccountFromCredentials](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#addAccountFromCredentials(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle")))]
        pub fn addAccountFromCredentials<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "addAccountFromCredentials", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "addAccountFromCredentials\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startAddAccountSession](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#startAddAccountSession(android.accounts.AccountAuthenticatorResponse,%20java.lang.String,%20java.lang.String,%20java.lang.String%5B%5D,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn startAddAccountSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "startAddAccountSession", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "startAddAccountSession\0", "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startUpdateCredentialsSession](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#startUpdateCredentialsSession(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn startUpdateCredentialsSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "startUpdateCredentialsSession", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "startUpdateCredentialsSession\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [finishSession](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#finishSession(android.accounts.AccountAuthenticatorResponse,%20java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn finishSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "finishSession", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "finishSession\0", "(Landroid/accounts/AccountAuthenticatorResponse;Ljava/lang/String;Landroid/os/Bundle;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isCredentialsUpdateSuggested](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#isCredentialsUpdateSuggested(android.accounts.AccountAuthenticatorResponse,%20android.accounts.Account,%20java.lang.String))
        ///
        /// Required features: "android-accounts-Account", "android-accounts-AccountAuthenticatorResponse", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-accounts-Account", feature = "android-accounts-AccountAuthenticatorResponse", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn isCredentialsUpdateSuggested<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::AccountAuthenticatorResponse>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accounts::Account>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accounts/AbstractAccountAuthenticator", java.flags == PUBLIC, .name == "isCredentialsUpdateSuggested", .descriptor == "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accounts/AbstractAccountAuthenticator\0", "isCredentialsUpdateSuggested\0", "(Landroid/accounts/AccountAuthenticatorResponse;Landroid/accounts/Account;Ljava/lang/String;)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [KEY_CUSTOM_TOKEN_EXPIRY](https://developer.android.com/reference/android/accounts/AbstractAccountAuthenticator.html#KEY_CUSTOM_TOKEN_EXPIRY)
        pub const KEY_CUSTOM_TOKEN_EXPIRY : &'static str = "android.accounts.expiry";
    }
}
