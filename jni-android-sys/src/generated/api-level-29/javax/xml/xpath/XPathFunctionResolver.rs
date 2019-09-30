// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-xpath-XPathFunctionResolver"))]
__jni_bindgen! {
    /// public interface [XPathFunctionResolver](https://developer.android.com/reference/javax/xml/xpath/XPathFunctionResolver.html)
    ///
    /// Required feature: javax-xml-xpath-XPathFunctionResolver
    public interface XPathFunctionResolver ("javax/xml/xpath/XPathFunctionResolver") extends crate::java::lang::Object {

        /// [resolveFunction](https://developer.android.com/reference/javax/xml/xpath/XPathFunctionResolver.html#resolveFunction(javax.xml.namespace.QName,%20int))
        ///
        /// Required features: "javax-xml-namespace-QName", "javax-xml-xpath-XPathFunction"
        #[cfg(any(feature = "all", all(feature = "javax-xml-namespace-QName", feature = "javax-xml-xpath-XPathFunction")))]
        pub fn resolveFunction<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::xml::namespace::QName>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::xpath::XPathFunction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/xpath/XPathFunctionResolver", java.flags == PUBLIC | ABSTRACT, .name == "resolveFunction", .descriptor == "(Ljavax/xml/namespace/QName;I)Ljavax/xml/xpath/XPathFunction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/xpath/XPathFunctionResolver\0", "resolveFunction\0", "(Ljavax/xml/namespace/QName;I)Ljavax/xml/xpath/XPathFunction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
