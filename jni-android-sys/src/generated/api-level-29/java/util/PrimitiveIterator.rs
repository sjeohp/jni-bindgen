// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-PrimitiveIterator"))]
__jni_bindgen! {
    /// public interface [PrimitiveIterator](https://developer.android.com/reference/java/util/PrimitiveIterator.html)
    ///
    /// Required feature: java-util-PrimitiveIterator
    public interface PrimitiveIterator ("java/util/PrimitiveIterator") extends crate::java::lang::Object, implements crate::java::util::Iterator {

        /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.html#forEachRemaining(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn forEachRemaining<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator", java.flags == PUBLIC | ABSTRACT, .name == "forEachRemaining", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator\0", "forEachRemaining\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
