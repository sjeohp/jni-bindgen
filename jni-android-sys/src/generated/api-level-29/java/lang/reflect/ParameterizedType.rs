// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-reflect-ParameterizedType"))]
__jni_bindgen! {
    /// public interface [ParameterizedType](https://developer.android.com/reference/java/lang/reflect/ParameterizedType.html)
    ///
    /// Required feature: java-lang-reflect-ParameterizedType
    public interface ParameterizedType ("java/lang/reflect/ParameterizedType") extends crate::java::lang::Object, implements crate::java::lang::reflect::Type {

        /// [getActualTypeArguments](https://developer.android.com/reference/java/lang/reflect/ParameterizedType.html#getActualTypeArguments())
        ///
        /// Required features: "java-lang-reflect-Type"
        #[cfg(any(feature = "all", all(feature = "java-lang-reflect-Type")))]
        pub fn getActualTypeArguments<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::reflect::Type, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/ParameterizedType", java.flags == PUBLIC | ABSTRACT, .name == "getActualTypeArguments", .descriptor == "()[Ljava/lang/reflect/Type;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/ParameterizedType\0", "getActualTypeArguments\0", "()[Ljava/lang/reflect/Type;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRawType](https://developer.android.com/reference/java/lang/reflect/ParameterizedType.html#getRawType())
        ///
        /// Required features: "java-lang-reflect-Type"
        #[cfg(any(feature = "all", all(feature = "java-lang-reflect-Type")))]
        pub fn getRawType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::reflect::Type>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/ParameterizedType", java.flags == PUBLIC | ABSTRACT, .name == "getRawType", .descriptor == "()Ljava/lang/reflect/Type;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/ParameterizedType\0", "getRawType\0", "()Ljava/lang/reflect/Type;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOwnerType](https://developer.android.com/reference/java/lang/reflect/ParameterizedType.html#getOwnerType())
        ///
        /// Required features: "java-lang-reflect-Type"
        #[cfg(any(feature = "all", all(feature = "java-lang-reflect-Type")))]
        pub fn getOwnerType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::reflect::Type>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/ParameterizedType", java.flags == PUBLIC | ABSTRACT, .name == "getOwnerType", .descriptor == "()Ljava/lang/reflect/Type;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/ParameterizedType\0", "getOwnerType\0", "()Ljava/lang/reflect/Type;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
