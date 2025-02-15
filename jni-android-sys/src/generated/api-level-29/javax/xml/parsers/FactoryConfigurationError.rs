// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-parsers-FactoryConfigurationError"))]
__jni_bindgen! {
    /// public class [FactoryConfigurationError](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html)
    ///
    /// Required feature: javax-xml-parsers-FactoryConfigurationError
    public class FactoryConfigurationError ("javax/xml/parsers/FactoryConfigurationError") extends crate::java::lang::Error {

        /// [FactoryConfigurationError](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html#FactoryConfigurationError())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::parsers::FactoryConfigurationError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/FactoryConfigurationError", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/FactoryConfigurationError\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FactoryConfigurationError](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html#FactoryConfigurationError(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::parsers::FactoryConfigurationError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/FactoryConfigurationError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/FactoryConfigurationError\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FactoryConfigurationError](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html#FactoryConfigurationError(java.lang.Exception))
        ///
        /// Required features: "java-lang-Exception"
        #[cfg(any(feature = "all", all(feature = "java-lang-Exception")))]
        pub fn new_Exception<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Exception>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::parsers::FactoryConfigurationError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/FactoryConfigurationError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Exception;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/FactoryConfigurationError\0", "<init>\0", "(Ljava/lang/Exception;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FactoryConfigurationError](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html#FactoryConfigurationError(java.lang.Exception,%20java.lang.String))
        ///
        /// Required features: "java-lang-Exception", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Exception", feature = "java-lang-String")))]
        pub fn new_Exception_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Exception>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::parsers::FactoryConfigurationError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/FactoryConfigurationError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Exception;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/FactoryConfigurationError\0", "<init>\0", "(Ljava/lang/Exception;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMessage](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html#getMessage())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMessage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/FactoryConfigurationError", java.flags == PUBLIC, .name == "getMessage", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/FactoryConfigurationError\0", "getMessage\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getException](https://developer.android.com/reference/javax/xml/parsers/FactoryConfigurationError.html#getException())
        ///
        /// Required features: "java-lang-Exception"
        #[cfg(any(feature = "all", all(feature = "java-lang-Exception")))]
        pub fn getException<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Exception>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/FactoryConfigurationError", java.flags == PUBLIC, .name == "getException", .descriptor == "()Ljava/lang/Exception;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/FactoryConfigurationError\0", "getException\0", "()Ljava/lang/Exception;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
