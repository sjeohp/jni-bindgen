// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-biometrics-BiometricPrompt_CryptoObject"))]
__jni_bindgen! {
    /// public final class [BiometricPrompt.CryptoObject](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html)
    ///
    /// Required feature: android-hardware-biometrics-BiometricPrompt_CryptoObject
    public final class BiometricPrompt_CryptoObject ("android/hardware/biometrics/BiometricPrompt$CryptoObject") extends crate::java::lang::Object {

        /// [CryptoObject](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html#CryptoObject(java.security.Signature))
        ///
        /// Required features: "java-security-Signature"
        #[cfg(any(feature = "all", all(feature = "java-security-Signature")))]
        pub fn new_Signature<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Signature>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::biometrics::BiometricPrompt_CryptoObject>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/biometrics/BiometricPrompt$CryptoObject", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/security/Signature;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/biometrics/BiometricPrompt$CryptoObject\0", "<init>\0", "(Ljava/security/Signature;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CryptoObject](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html#CryptoObject(javax.crypto.Cipher))
        ///
        /// Required features: "javax-crypto-Cipher"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-Cipher")))]
        pub fn new_Cipher<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::crypto::Cipher>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::biometrics::BiometricPrompt_CryptoObject>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/biometrics/BiometricPrompt$CryptoObject", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljavax/crypto/Cipher;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/biometrics/BiometricPrompt$CryptoObject\0", "<init>\0", "(Ljavax/crypto/Cipher;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CryptoObject](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html#CryptoObject(javax.crypto.Mac))
        ///
        /// Required features: "javax-crypto-Mac"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-Mac")))]
        pub fn new_Mac<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::crypto::Mac>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::biometrics::BiometricPrompt_CryptoObject>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/biometrics/BiometricPrompt$CryptoObject", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljavax/crypto/Mac;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/biometrics/BiometricPrompt$CryptoObject\0", "<init>\0", "(Ljavax/crypto/Mac;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSignature](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html#getSignature())
        ///
        /// Required features: "java-security-Signature"
        #[cfg(any(feature = "all", all(feature = "java-security-Signature")))]
        pub fn getSignature<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Signature>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/biometrics/BiometricPrompt$CryptoObject", java.flags == PUBLIC, .name == "getSignature", .descriptor == "()Ljava/security/Signature;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/biometrics/BiometricPrompt$CryptoObject\0", "getSignature\0", "()Ljava/security/Signature;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCipher](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html#getCipher())
        ///
        /// Required features: "javax-crypto-Cipher"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-Cipher")))]
        pub fn getCipher<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::Cipher>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/biometrics/BiometricPrompt$CryptoObject", java.flags == PUBLIC, .name == "getCipher", .descriptor == "()Ljavax/crypto/Cipher;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/biometrics/BiometricPrompt$CryptoObject\0", "getCipher\0", "()Ljavax/crypto/Cipher;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMac](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt.CryptoObject.html#getMac())
        ///
        /// Required features: "javax-crypto-Mac"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-Mac")))]
        pub fn getMac<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::Mac>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/biometrics/BiometricPrompt$CryptoObject", java.flags == PUBLIC, .name == "getMac", .descriptor == "()Ljavax/crypto/Mac;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/biometrics/BiometricPrompt$CryptoObject\0", "getMac\0", "()Ljavax/crypto/Mac;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
