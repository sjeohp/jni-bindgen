// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-charset-CoderMalfunctionError"))]
__jni_bindgen! {
    /// public class [CoderMalfunctionError](https://developer.android.com/reference/java/nio/charset/CoderMalfunctionError.html)
    ///
    /// Required feature: java-nio-charset-CoderMalfunctionError
    public class CoderMalfunctionError ("java/nio/charset/CoderMalfunctionError") extends crate::java::lang::Error {

        /// [CoderMalfunctionError](https://developer.android.com/reference/java/nio/charset/CoderMalfunctionError.html#CoderMalfunctionError(java.lang.Exception))
        ///
        /// Required features: "java-lang-Exception"
        #[cfg(any(feature = "all", all(feature = "java-lang-Exception")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Exception>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::charset::CoderMalfunctionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/CoderMalfunctionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Exception;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/CoderMalfunctionError\0", "<init>\0", "(Ljava/lang/Exception;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
