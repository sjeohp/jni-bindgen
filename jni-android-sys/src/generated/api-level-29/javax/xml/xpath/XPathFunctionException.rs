// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-xpath-XPathFunctionException"))]
__jni_bindgen! {
    /// public class [XPathFunctionException](https://developer.android.com/reference/javax/xml/xpath/XPathFunctionException.html)
    ///
    /// Required feature: javax-xml-xpath-XPathFunctionException
    public class XPathFunctionException ("javax/xml/xpath/XPathFunctionException") extends crate::javax::xml::xpath::XPathExpressionException {

        /// [XPathFunctionException](https://developer.android.com/reference/javax/xml/xpath/XPathFunctionException.html#XPathFunctionException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::xpath::XPathFunctionException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/xpath/XPathFunctionException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/xpath/XPathFunctionException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [XPathFunctionException](https://developer.android.com/reference/javax/xml/xpath/XPathFunctionException.html#XPathFunctionException(java.lang.Throwable))
        ///
        /// Required features: "java-lang-Throwable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Throwable")))]
        pub fn new_Throwable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::xpath::XPathFunctionException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/xpath/XPathFunctionException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Throwable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/xpath/XPathFunctionException\0", "<init>\0", "(Ljava/lang/Throwable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
