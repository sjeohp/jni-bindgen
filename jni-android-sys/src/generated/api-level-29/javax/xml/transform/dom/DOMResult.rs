// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-transform-dom-DOMResult"))]
__jni_bindgen! {
    /// public class [DOMResult](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html)
    ///
    /// Required feature: javax-xml-transform-dom-DOMResult
    public class DOMResult ("javax/xml/transform/dom/DOMResult") extends crate::java::lang::Object, implements crate::javax::xml::transform::Result {

        /// [DOMResult](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#DOMResult())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::transform::dom::DOMResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DOMResult](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#DOMResult(org.w3c.dom.Node))
        ///
        /// Required features: "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-Node")))]
        pub fn new_Node<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::transform::dom::DOMResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Lorg/w3c/dom/Node;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "<init>\0", "(Lorg/w3c/dom/Node;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DOMResult](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#DOMResult(org.w3c.dom.Node,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-w3c-dom-Node")))]
        pub fn new_Node_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::transform::dom::DOMResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Lorg/w3c/dom/Node;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "<init>\0", "(Lorg/w3c/dom/Node;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DOMResult](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#DOMResult(org.w3c.dom.Node,%20org.w3c.dom.Node))
        ///
        /// Required features: "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-Node")))]
        pub fn new_Node_Node<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::transform::dom::DOMResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Lorg/w3c/dom/Node;Lorg/w3c/dom/Node;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "<init>\0", "(Lorg/w3c/dom/Node;Lorg/w3c/dom/Node;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DOMResult](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#DOMResult(org.w3c.dom.Node,%20org.w3c.dom.Node,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-w3c-dom-Node")))]
        pub fn new_Node_Node_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::transform::dom::DOMResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Lorg/w3c/dom/Node;Lorg/w3c/dom/Node;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "<init>\0", "(Lorg/w3c/dom/Node;Lorg/w3c/dom/Node;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNode](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#setNode(org.w3c.dom.Node))
        ///
        /// Required features: "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-Node")))]
        pub fn setNode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "setNode", .descriptor == "(Lorg/w3c/dom/Node;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "setNode\0", "(Lorg/w3c/dom/Node;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNode](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#getNode())
        ///
        /// Required features: "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-Node")))]
        pub fn getNode<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::w3c::dom::Node>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "getNode", .descriptor == "()Lorg/w3c/dom/Node;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "getNode\0", "()Lorg/w3c/dom/Node;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNextSibling](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#setNextSibling(org.w3c.dom.Node))
        ///
        /// Required features: "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-Node")))]
        pub fn setNextSibling<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::w3c::dom::Node>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "setNextSibling", .descriptor == "(Lorg/w3c/dom/Node;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "setNextSibling\0", "(Lorg/w3c/dom/Node;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNextSibling](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#getNextSibling())
        ///
        /// Required features: "org-w3c-dom-Node"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-Node")))]
        pub fn getNextSibling<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::w3c::dom::Node>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "getNextSibling", .descriptor == "()Lorg/w3c/dom/Node;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "getNextSibling\0", "()Lorg/w3c/dom/Node;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSystemId](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#setSystemId(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setSystemId<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "setSystemId", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "setSystemId\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSystemId](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#getSystemId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSystemId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/dom/DOMResult", java.flags == PUBLIC, .name == "getSystemId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/dom/DOMResult\0", "getSystemId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [FEATURE](https://developer.android.com/reference/javax/xml/transform/dom/DOMResult.html#FEATURE)
        pub const FEATURE : &'static str = "http://javax.xml.transform.dom.DOMResult/feature";
    }
}
