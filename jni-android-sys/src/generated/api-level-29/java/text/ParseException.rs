// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-text-ParseException"))]
__jni_bindgen! {
    /// public class [ParseException](https://developer.android.com/reference/java/text/ParseException.html)
    ///
    /// Required feature: java-text-ParseException
    public class ParseException ("java/text/ParseException") extends crate::java::lang::Exception {

        /// [ParseException](https://developer.android.com/reference/java/text/ParseException.html#ParseException(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::text::ParseException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/ParseException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/ParseException\0", "<init>\0", "(Ljava/lang/String;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getErrorOffset](https://developer.android.com/reference/java/text/ParseException.html#getErrorOffset())
        pub fn getErrorOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/ParseException", java.flags == PUBLIC, .name == "getErrorOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/ParseException\0", "getErrorOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
