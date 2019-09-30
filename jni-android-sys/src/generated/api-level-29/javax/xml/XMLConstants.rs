// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-XMLConstants"))]
__jni_bindgen! {
    /// public final class [XMLConstants](https://developer.android.com/reference/javax/xml/XMLConstants.html)
    ///
    /// Required feature: javax-xml-XMLConstants
    public final class XMLConstants ("javax/xml/XMLConstants") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [XMLConstants](https://developer.android.com/reference/javax/xml/XMLConstants.html#XMLConstants())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::XMLConstants>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/xml/XMLConstants", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/XMLConstants\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [DEFAULT_NS_PREFIX](https://developer.android.com/reference/javax/xml/XMLConstants.html#DEFAULT_NS_PREFIX)
        pub const DEFAULT_NS_PREFIX : &'static str = "";

        /// public static final [FEATURE_SECURE_PROCESSING](https://developer.android.com/reference/javax/xml/XMLConstants.html#FEATURE_SECURE_PROCESSING)
        pub const FEATURE_SECURE_PROCESSING : &'static str = "http://javax.xml.XMLConstants/feature/secure-processing";

        /// public static final [NULL_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#NULL_NS_URI)
        pub const NULL_NS_URI : &'static str = "";

        /// public static final [RELAXNG_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#RELAXNG_NS_URI)
        pub const RELAXNG_NS_URI : &'static str = "http://relaxng.org/ns/structure/1.0";

        /// public static final [W3C_XML_SCHEMA_INSTANCE_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#W3C_XML_SCHEMA_INSTANCE_NS_URI)
        pub const W3C_XML_SCHEMA_INSTANCE_NS_URI : &'static str = "http://www.w3.org/2001/XMLSchema-instance";

        /// public static final [W3C_XML_SCHEMA_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#W3C_XML_SCHEMA_NS_URI)
        pub const W3C_XML_SCHEMA_NS_URI : &'static str = "http://www.w3.org/2001/XMLSchema";

        /// public static final [W3C_XPATH_DATATYPE_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#W3C_XPATH_DATATYPE_NS_URI)
        pub const W3C_XPATH_DATATYPE_NS_URI : &'static str = "http://www.w3.org/2003/11/xpath-datatypes";

        /// public static final [XMLNS_ATTRIBUTE](https://developer.android.com/reference/javax/xml/XMLConstants.html#XMLNS_ATTRIBUTE)
        pub const XMLNS_ATTRIBUTE : &'static str = "xmlns";

        /// public static final [XMLNS_ATTRIBUTE_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#XMLNS_ATTRIBUTE_NS_URI)
        pub const XMLNS_ATTRIBUTE_NS_URI : &'static str = "http://www.w3.org/2000/xmlns/";

        /// public static final [XML_DTD_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#XML_DTD_NS_URI)
        pub const XML_DTD_NS_URI : &'static str = "http://www.w3.org/TR/REC-xml";

        /// public static final [XML_NS_PREFIX](https://developer.android.com/reference/javax/xml/XMLConstants.html#XML_NS_PREFIX)
        pub const XML_NS_PREFIX : &'static str = "xml";

        /// public static final [XML_NS_URI](https://developer.android.com/reference/javax/xml/XMLConstants.html#XML_NS_URI)
        pub const XML_NS_URI : &'static str = "http://www.w3.org/XML/1998/namespace";
    }
}
