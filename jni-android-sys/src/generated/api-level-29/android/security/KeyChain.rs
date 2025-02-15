// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-security-KeyChain"))]
__jni_bindgen! {
    /// public final class [KeyChain](https://developer.android.com/reference/android/security/KeyChain.html)
    ///
    /// Required feature: android-security-KeyChain
    public final class KeyChain ("android/security/KeyChain") extends crate::java::lang::Object {

        /// [KeyChain](https://developer.android.com/reference/android/security/KeyChain.html#KeyChain())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::security::KeyChain>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyChain\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createInstallIntent](https://developer.android.com/reference/android/security/KeyChain.html#createInstallIntent())
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn createInstallIntent<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Intent>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "createInstallIntent", .descriptor == "()Landroid/content/Intent;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "createInstallIntent\0", "()Landroid/content/Intent;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [choosePrivateKeyAlias](https://developer.android.com/reference/android/security/KeyChain.html#choosePrivateKeyAlias(android.app.Activity,%20android.security.KeyChainAliasCallback,%20java.lang.String%5B%5D,%20java.security.Principal%5B%5D,%20java.lang.String,%20int,%20java.lang.String))
        ///
        /// Required features: "android-app-Activity", "android-security-KeyChainAliasCallback", "java-lang-String", "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-security-KeyChainAliasCallback", feature = "java-lang-String", feature = "java-security-Principal")))]
        pub fn choosePrivateKeyAlias_Activity_KeyChainAliasCallback_String_array_Principal_array_String_int_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::security::KeyChainAliasCallback>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::Principal, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg5: i32, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "choosePrivateKeyAlias", .descriptor == "(Landroid/app/Activity;Landroid/security/KeyChainAliasCallback;[Ljava/lang/String;[Ljava/security/Principal;Ljava/lang/String;ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "choosePrivateKeyAlias\0", "(Landroid/app/Activity;Landroid/security/KeyChainAliasCallback;[Ljava/lang/String;[Ljava/security/Principal;Ljava/lang/String;ILjava/lang/String;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [choosePrivateKeyAlias](https://developer.android.com/reference/android/security/KeyChain.html#choosePrivateKeyAlias(android.app.Activity,%20android.security.KeyChainAliasCallback,%20java.lang.String%5B%5D,%20java.security.Principal%5B%5D,%20android.net.Uri,%20java.lang.String))
        ///
        /// Required features: "android-app-Activity", "android-net-Uri", "android-security-KeyChainAliasCallback", "java-lang-String", "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-net-Uri", feature = "android-security-KeyChainAliasCallback", feature = "java-lang-String", feature = "java-security-Principal")))]
        pub fn choosePrivateKeyAlias_Activity_KeyChainAliasCallback_String_array_Principal_array_Uri_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::security::KeyChainAliasCallback>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::Principal, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "choosePrivateKeyAlias", .descriptor == "(Landroid/app/Activity;Landroid/security/KeyChainAliasCallback;[Ljava/lang/String;[Ljava/security/Principal;Landroid/net/Uri;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "choosePrivateKeyAlias\0", "(Landroid/app/Activity;Landroid/security/KeyChainAliasCallback;[Ljava/lang/String;[Ljava/security/Principal;Landroid/net/Uri;Ljava/lang/String;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrivateKey](https://developer.android.com/reference/android/security/KeyChain.html#getPrivateKey(android.content.Context,%20java.lang.String))
        ///
        /// Required features: "android-content-Context", "java-lang-String", "java-security-PrivateKey"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String", feature = "java-security-PrivateKey")))]
        pub fn getPrivateKey<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::PrivateKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "getPrivateKey", .descriptor == "(Landroid/content/Context;Ljava/lang/String;)Ljava/security/PrivateKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "getPrivateKey\0", "(Landroid/content/Context;Ljava/lang/String;)Ljava/security/PrivateKey;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertificateChain](https://developer.android.com/reference/android/security/KeyChain.html#getCertificateChain(android.content.Context,%20java.lang.String))
        ///
        /// Required features: "android-content-Context", "java-lang-String", "java-security-cert-X509Certificate"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String", feature = "java-security-cert-X509Certificate")))]
        pub fn getCertificateChain<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::X509Certificate, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "getCertificateChain", .descriptor == "(Landroid/content/Context;Ljava/lang/String;)[Ljava/security/cert/X509Certificate;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "getCertificateChain\0", "(Landroid/content/Context;Ljava/lang/String;)[Ljava/security/cert/X509Certificate;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isKeyAlgorithmSupported](https://developer.android.com/reference/android/security/KeyChain.html#isKeyAlgorithmSupported(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn isKeyAlgorithmSupported<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "isKeyAlgorithmSupported", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "isKeyAlgorithmSupported\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isBoundKeyAlgorithm](https://developer.android.com/reference/android/security/KeyChain.html#isBoundKeyAlgorithm(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn isBoundKeyAlgorithm<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyChain", java.flags == PUBLIC | STATIC, .name == "isBoundKeyAlgorithm", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/security/KeyChain\0", "isBoundKeyAlgorithm\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_KEYCHAIN_CHANGED](https://developer.android.com/reference/android/security/KeyChain.html#ACTION_KEYCHAIN_CHANGED)
        pub const ACTION_KEYCHAIN_CHANGED : &'static str = "android.security.action.KEYCHAIN_CHANGED";

        /// public static final [ACTION_KEY_ACCESS_CHANGED](https://developer.android.com/reference/android/security/KeyChain.html#ACTION_KEY_ACCESS_CHANGED)
        pub const ACTION_KEY_ACCESS_CHANGED : &'static str = "android.security.action.KEY_ACCESS_CHANGED";

        /// public static final [ACTION_STORAGE_CHANGED](https://developer.android.com/reference/android/security/KeyChain.html#ACTION_STORAGE_CHANGED)
        #[deprecated] pub const ACTION_STORAGE_CHANGED : &'static str = "android.security.STORAGE_CHANGED";

        /// public static final [ACTION_TRUST_STORE_CHANGED](https://developer.android.com/reference/android/security/KeyChain.html#ACTION_TRUST_STORE_CHANGED)
        pub const ACTION_TRUST_STORE_CHANGED : &'static str = "android.security.action.TRUST_STORE_CHANGED";

        /// public static final [EXTRA_CERTIFICATE](https://developer.android.com/reference/android/security/KeyChain.html#EXTRA_CERTIFICATE)
        pub const EXTRA_CERTIFICATE : &'static str = "CERT";

        /// public static final [EXTRA_KEY_ACCESSIBLE](https://developer.android.com/reference/android/security/KeyChain.html#EXTRA_KEY_ACCESSIBLE)
        pub const EXTRA_KEY_ACCESSIBLE : &'static str = "android.security.extra.KEY_ACCESSIBLE";

        /// public static final [EXTRA_KEY_ALIAS](https://developer.android.com/reference/android/security/KeyChain.html#EXTRA_KEY_ALIAS)
        pub const EXTRA_KEY_ALIAS : &'static str = "android.security.extra.KEY_ALIAS";

        /// public static final [EXTRA_NAME](https://developer.android.com/reference/android/security/KeyChain.html#EXTRA_NAME)
        pub const EXTRA_NAME : &'static str = "name";

        /// public static final [EXTRA_PKCS12](https://developer.android.com/reference/android/security/KeyChain.html#EXTRA_PKCS12)
        pub const EXTRA_PKCS12 : &'static str = "PKCS12";
    }
}
