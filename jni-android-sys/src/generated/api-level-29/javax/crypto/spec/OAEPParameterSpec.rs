// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-crypto-spec-OAEPParameterSpec"))]
__jni_bindgen! {
    /// public class [OAEPParameterSpec](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html)
    ///
    /// Required feature: javax-crypto-spec-OAEPParameterSpec
    public class OAEPParameterSpec ("javax/crypto/spec/OAEPParameterSpec") extends crate::java::lang::Object, implements crate::java::security::spec::AlgorithmParameterSpec {

        /// [OAEPParameterSpec](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html#OAEPParameterSpec(java.lang.String,%20java.lang.String,%20java.security.spec.AlgorithmParameterSpec,%20javax.crypto.spec.PSource))
        ///
        /// Required features: "java-lang-String", "java-security-spec-AlgorithmParameterSpec", "javax-crypto-spec-PSource"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-spec-AlgorithmParameterSpec", feature = "javax-crypto-spec-PSource")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::crypto::spec::PSource>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::crypto::spec::OAEPParameterSpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/OAEPParameterSpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;Ljava/lang/String;Ljava/security/spec/AlgorithmParameterSpec;Ljavax/crypto/spec/PSource;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/OAEPParameterSpec\0", "<init>\0", "(Ljava/lang/String;Ljava/lang/String;Ljava/security/spec/AlgorithmParameterSpec;Ljavax/crypto/spec/PSource;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDigestAlgorithm](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html#getDigestAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getDigestAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/OAEPParameterSpec", java.flags == PUBLIC, .name == "getDigestAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/OAEPParameterSpec\0", "getDigestAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMGFAlgorithm](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html#getMGFAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMGFAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/OAEPParameterSpec", java.flags == PUBLIC, .name == "getMGFAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/OAEPParameterSpec\0", "getMGFAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMGFParameters](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html#getMGFParameters())
        ///
        /// Required features: "java-security-spec-AlgorithmParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-AlgorithmParameterSpec")))]
        pub fn getMGFParameters<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::AlgorithmParameterSpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/OAEPParameterSpec", java.flags == PUBLIC, .name == "getMGFParameters", .descriptor == "()Ljava/security/spec/AlgorithmParameterSpec;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/OAEPParameterSpec\0", "getMGFParameters\0", "()Ljava/security/spec/AlgorithmParameterSpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPSource](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html#getPSource())
        ///
        /// Required features: "javax-crypto-spec-PSource"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-spec-PSource")))]
        pub fn getPSource<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::spec::PSource>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/OAEPParameterSpec", java.flags == PUBLIC, .name == "getPSource", .descriptor == "()Ljavax/crypto/spec/PSource;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/OAEPParameterSpec\0", "getPSource\0", "()Ljavax/crypto/spec/PSource;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [DEFAULT](https://developer.android.com/reference/javax/crypto/spec/OAEPParameterSpec.html#DEFAULT)
        ///
        /// Required feature: javax-crypto-spec-OAEPParameterSpec
        #[cfg(any(feature = "all", feature = "javax-crypto-spec-OAEPParameterSpec"))]
        pub fn DEFAULT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::spec::OAEPParameterSpec>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/crypto/spec/OAEPParameterSpec\0", "DEFAULT\0", "Ljavax/crypto/spec/OAEPParameterSpec;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
