// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-validation-SchemaFactoryLoader"))]
__jni_bindgen! {
    /// public class [SchemaFactoryLoader](https://developer.android.com/reference/javax/xml/validation/SchemaFactoryLoader.html)
    ///
    /// Required feature: javax-xml-validation-SchemaFactoryLoader
    public class SchemaFactoryLoader ("javax/xml/validation/SchemaFactoryLoader") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SchemaFactoryLoader](https://developer.android.com/reference/javax/xml/validation/SchemaFactoryLoader.html#SchemaFactoryLoader())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::validation::SchemaFactoryLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/xml/validation/SchemaFactoryLoader", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/SchemaFactoryLoader\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [newFactory](https://developer.android.com/reference/javax/xml/validation/SchemaFactoryLoader.html#newFactory(java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-xml-validation-SchemaFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-xml-validation-SchemaFactory")))]
        pub fn newFactory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::validation::SchemaFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/validation/SchemaFactoryLoader", java.flags == PUBLIC | ABSTRACT, .name == "newFactory", .descriptor == "(Ljava/lang/String;)Ljavax/xml/validation/SchemaFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/validation/SchemaFactoryLoader\0", "newFactory\0", "(Ljava/lang/String;)Ljavax/xml/validation/SchemaFactory;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
