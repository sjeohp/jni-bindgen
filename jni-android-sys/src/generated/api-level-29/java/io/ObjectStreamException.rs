// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-ObjectStreamException"))]
__jni_bindgen! {
    /// public class [ObjectStreamException](https://developer.android.com/reference/java/io/ObjectStreamException.html)
    ///
    /// Required feature: java-io-ObjectStreamException
    public class ObjectStreamException ("java/io/ObjectStreamException") extends crate::java::io::IOException {

        // // Not emitting: Non-public method
        // /// [ObjectStreamException](https://developer.android.com/reference/java/io/ObjectStreamException.html#ObjectStreamException(java.lang.String))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::ObjectStreamException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/io/ObjectStreamException", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/ObjectStreamException\0", "<init>\0", "(Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [ObjectStreamException](https://developer.android.com/reference/java/io/ObjectStreamException.html#ObjectStreamException())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::ObjectStreamException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/io/ObjectStreamException", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/ObjectStreamException\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
