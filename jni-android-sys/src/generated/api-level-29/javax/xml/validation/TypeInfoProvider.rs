// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-validation-TypeInfoProvider"))]
__jni_bindgen! {
    /// public class [TypeInfoProvider](https://developer.android.com/reference/javax/xml/validation/TypeInfoProvider.html)
    ///
    /// Required feature: javax-xml-validation-TypeInfoProvider
    public class TypeInfoProvider ("javax/xml/validation/TypeInfoProvider") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [TypeInfoProvider](https://developer.android.com/reference/javax/xml/validation/TypeInfoProvider.html#TypeInfoProvider())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::validation::TypeInfoProvider>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/xml/validation/TypeInfoProvider", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/TypeInfoProvider\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getElementTypeInfo](https://developer.android.com/reference/javax/xml/validation/TypeInfoProvider.html#getElementTypeInfo())
        ///
        /// Required features: "org-w3c-dom-TypeInfo"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-TypeInfo")))]
        pub fn getElementTypeInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::w3c::dom::TypeInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/validation/TypeInfoProvider", java.flags == PUBLIC | ABSTRACT, .name == "getElementTypeInfo", .descriptor == "()Lorg/w3c/dom/TypeInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/TypeInfoProvider\0", "getElementTypeInfo\0", "()Lorg/w3c/dom/TypeInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAttributeTypeInfo](https://developer.android.com/reference/javax/xml/validation/TypeInfoProvider.html#getAttributeTypeInfo(int))
        ///
        /// Required features: "org-w3c-dom-TypeInfo"
        #[cfg(any(feature = "all", all(feature = "org-w3c-dom-TypeInfo")))]
        pub fn getAttributeTypeInfo<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::w3c::dom::TypeInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/validation/TypeInfoProvider", java.flags == PUBLIC | ABSTRACT, .name == "getAttributeTypeInfo", .descriptor == "(I)Lorg/w3c/dom/TypeInfo;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/TypeInfoProvider\0", "getAttributeTypeInfo\0", "(I)Lorg/w3c/dom/TypeInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isIdAttribute](https://developer.android.com/reference/javax/xml/validation/TypeInfoProvider.html#isIdAttribute(int))
        pub fn isIdAttribute<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/validation/TypeInfoProvider", java.flags == PUBLIC | ABSTRACT, .name == "isIdAttribute", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/TypeInfoProvider\0", "isIdAttribute\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSpecified](https://developer.android.com/reference/javax/xml/validation/TypeInfoProvider.html#isSpecified(int))
        pub fn isSpecified<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/validation/TypeInfoProvider", java.flags == PUBLIC | ABSTRACT, .name == "isSpecified", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/TypeInfoProvider\0", "isSpecified\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
