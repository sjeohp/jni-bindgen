// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-crypto-interfaces-DHKey"))]
__jni_bindgen! {
    /// public interface [DHKey](https://developer.android.com/reference/javax/crypto/interfaces/DHKey.html)
    ///
    /// Required feature: javax-crypto-interfaces-DHKey
    public interface DHKey ("javax/crypto/interfaces/DHKey") extends crate::java::lang::Object {

        /// [getParams](https://developer.android.com/reference/javax/crypto/interfaces/DHKey.html#getParams())
        ///
        /// Required features: "javax-crypto-spec-DHParameterSpec"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-spec-DHParameterSpec")))]
        pub fn getParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::spec::DHParameterSpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/interfaces/DHKey", java.flags == PUBLIC | ABSTRACT, .name == "getParams", .descriptor == "()Ljavax/crypto/spec/DHParameterSpec;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/interfaces/DHKey\0", "getParams\0", "()Ljavax/crypto/spec/DHParameterSpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
