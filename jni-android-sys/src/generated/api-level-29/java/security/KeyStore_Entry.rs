// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-KeyStore_Entry"))]
__jni_bindgen! {
    /// public interface [KeyStore.Entry](https://developer.android.com/reference/java/security/KeyStore.Entry.html)
    ///
    /// Required feature: java-security-KeyStore_Entry
    public interface KeyStore_Entry ("java/security/KeyStore$Entry") extends crate::java::lang::Object {

        /// [getAttributes](https://developer.android.com/reference/java/security/KeyStore.Entry.html#getAttributes())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getAttributes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyStore$Entry", java.flags == PUBLIC, .name == "getAttributes", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyStore$Entry\0", "getAttributes\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
