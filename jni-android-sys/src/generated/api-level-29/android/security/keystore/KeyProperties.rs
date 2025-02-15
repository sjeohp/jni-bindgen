// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-security-keystore-KeyProperties"))]
__jni_bindgen! {
    /// public class [KeyProperties](https://developer.android.com/reference/android/security/keystore/KeyProperties.html)
    ///
    /// Required feature: android-security-keystore-KeyProperties
    public class KeyProperties ("android/security/keystore/KeyProperties") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [KeyProperties](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KeyProperties())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::security::keystore::KeyProperties>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/security/keystore/KeyProperties", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyProperties\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [BLOCK_MODE_CBC](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#BLOCK_MODE_CBC)
        pub const BLOCK_MODE_CBC : &'static str = "CBC";

        /// public static final [BLOCK_MODE_CTR](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#BLOCK_MODE_CTR)
        pub const BLOCK_MODE_CTR : &'static str = "CTR";

        /// public static final [BLOCK_MODE_ECB](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#BLOCK_MODE_ECB)
        pub const BLOCK_MODE_ECB : &'static str = "ECB";

        /// public static final [BLOCK_MODE_GCM](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#BLOCK_MODE_GCM)
        pub const BLOCK_MODE_GCM : &'static str = "GCM";

        /// public static final [DIGEST_MD5](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_MD5)
        pub const DIGEST_MD5 : &'static str = "MD5";

        /// public static final [DIGEST_NONE](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_NONE)
        pub const DIGEST_NONE : &'static str = "NONE";

        /// public static final [DIGEST_SHA1](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_SHA1)
        pub const DIGEST_SHA1 : &'static str = "SHA-1";

        /// public static final [DIGEST_SHA224](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_SHA224)
        pub const DIGEST_SHA224 : &'static str = "SHA-224";

        /// public static final [DIGEST_SHA256](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_SHA256)
        pub const DIGEST_SHA256 : &'static str = "SHA-256";

        /// public static final [DIGEST_SHA384](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_SHA384)
        pub const DIGEST_SHA384 : &'static str = "SHA-384";

        /// public static final [DIGEST_SHA512](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#DIGEST_SHA512)
        pub const DIGEST_SHA512 : &'static str = "SHA-512";

        /// public static final [ENCRYPTION_PADDING_NONE](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ENCRYPTION_PADDING_NONE)
        pub const ENCRYPTION_PADDING_NONE : &'static str = "NoPadding";

        /// public static final [ENCRYPTION_PADDING_PKCS7](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ENCRYPTION_PADDING_PKCS7)
        pub const ENCRYPTION_PADDING_PKCS7 : &'static str = "PKCS7Padding";

        /// public static final [ENCRYPTION_PADDING_RSA_OAEP](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ENCRYPTION_PADDING_RSA_OAEP)
        pub const ENCRYPTION_PADDING_RSA_OAEP : &'static str = "OAEPPadding";

        /// public static final [ENCRYPTION_PADDING_RSA_PKCS1](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ENCRYPTION_PADDING_RSA_PKCS1)
        pub const ENCRYPTION_PADDING_RSA_PKCS1 : &'static str = "PKCS1Padding";

        /// public static final [KEY_ALGORITHM_3DES](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_3DES)
        #[deprecated] pub const KEY_ALGORITHM_3DES : &'static str = "DESede";

        /// public static final [KEY_ALGORITHM_AES](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_AES)
        pub const KEY_ALGORITHM_AES : &'static str = "AES";

        /// public static final [KEY_ALGORITHM_EC](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_EC)
        pub const KEY_ALGORITHM_EC : &'static str = "EC";

        /// public static final [KEY_ALGORITHM_HMAC_SHA1](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_HMAC_SHA1)
        pub const KEY_ALGORITHM_HMAC_SHA1 : &'static str = "HmacSHA1";

        /// public static final [KEY_ALGORITHM_HMAC_SHA224](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_HMAC_SHA224)
        pub const KEY_ALGORITHM_HMAC_SHA224 : &'static str = "HmacSHA224";

        /// public static final [KEY_ALGORITHM_HMAC_SHA256](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_HMAC_SHA256)
        pub const KEY_ALGORITHM_HMAC_SHA256 : &'static str = "HmacSHA256";

        /// public static final [KEY_ALGORITHM_HMAC_SHA384](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_HMAC_SHA384)
        pub const KEY_ALGORITHM_HMAC_SHA384 : &'static str = "HmacSHA384";

        /// public static final [KEY_ALGORITHM_HMAC_SHA512](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_HMAC_SHA512)
        pub const KEY_ALGORITHM_HMAC_SHA512 : &'static str = "HmacSHA512";

        /// public static final [KEY_ALGORITHM_RSA](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#KEY_ALGORITHM_RSA)
        pub const KEY_ALGORITHM_RSA : &'static str = "RSA";

        /// public static final [ORIGIN_GENERATED](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ORIGIN_GENERATED)
        pub const ORIGIN_GENERATED : i32 = 1;

        /// public static final [ORIGIN_IMPORTED](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ORIGIN_IMPORTED)
        pub const ORIGIN_IMPORTED : i32 = 2;

        /// public static final [ORIGIN_SECURELY_IMPORTED](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ORIGIN_SECURELY_IMPORTED)
        pub const ORIGIN_SECURELY_IMPORTED : i32 = 8;

        /// public static final [ORIGIN_UNKNOWN](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#ORIGIN_UNKNOWN)
        pub const ORIGIN_UNKNOWN : i32 = 4;

        /// public static final [PURPOSE_DECRYPT](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#PURPOSE_DECRYPT)
        pub const PURPOSE_DECRYPT : i32 = 2;

        /// public static final [PURPOSE_ENCRYPT](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#PURPOSE_ENCRYPT)
        pub const PURPOSE_ENCRYPT : i32 = 1;

        /// public static final [PURPOSE_SIGN](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#PURPOSE_SIGN)
        pub const PURPOSE_SIGN : i32 = 4;

        /// public static final [PURPOSE_VERIFY](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#PURPOSE_VERIFY)
        pub const PURPOSE_VERIFY : i32 = 8;

        /// public static final [PURPOSE_WRAP_KEY](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#PURPOSE_WRAP_KEY)
        pub const PURPOSE_WRAP_KEY : i32 = 32;

        /// public static final [SIGNATURE_PADDING_RSA_PKCS1](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#SIGNATURE_PADDING_RSA_PKCS1)
        pub const SIGNATURE_PADDING_RSA_PKCS1 : &'static str = "PKCS1";

        /// public static final [SIGNATURE_PADDING_RSA_PSS](https://developer.android.com/reference/android/security/keystore/KeyProperties.html#SIGNATURE_PADDING_RSA_PSS)
        pub const SIGNATURE_PADDING_RSA_PSS : &'static str = "PSS";
    }
}
