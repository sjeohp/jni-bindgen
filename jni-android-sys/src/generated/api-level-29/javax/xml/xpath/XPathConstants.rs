// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-xpath-XPathConstants"))]
__jni_bindgen! {
    /// public class [XPathConstants](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html)
    ///
    /// Required feature: javax-xml-xpath-XPathConstants
    public class XPathConstants ("javax/xml/xpath/XPathConstants") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [XPathConstants](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#XPathConstants())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::xpath::XPathConstants>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/xml/xpath/XPathConstants", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/xpath/XPathConstants\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [BOOLEAN](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#BOOLEAN)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn BOOLEAN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/xpath/XPathConstants\0", "BOOLEAN\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [DOM_OBJECT_MODEL](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#DOM_OBJECT_MODEL)
        pub const DOM_OBJECT_MODEL : &'static str = "http://java.sun.com/jaxp/xpath/dom";

        /// **get** public static final [NODE](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#NODE)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn NODE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/xpath/XPathConstants\0", "NODE\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NODESET](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#NODESET)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn NODESET<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/xpath/XPathConstants\0", "NODESET\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NUMBER](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#NUMBER)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn NUMBER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/xpath/XPathConstants\0", "NUMBER\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [STRING](https://developer.android.com/reference/javax/xml/xpath/XPathConstants.html#STRING)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn STRING<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/xpath/XPathConstants\0", "STRING\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
