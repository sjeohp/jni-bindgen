// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-IllformedLocaleException"))]
__jni_bindgen! {
    /// public class [IllformedLocaleException](https://developer.android.com/reference/java/util/IllformedLocaleException.html)
    ///
    /// Required feature: java-util-IllformedLocaleException
    public class IllformedLocaleException ("java/util/IllformedLocaleException") extends crate::java::lang::RuntimeException {

        /// [IllformedLocaleException](https://developer.android.com/reference/java/util/IllformedLocaleException.html#IllformedLocaleException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::IllformedLocaleException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/IllformedLocaleException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/IllformedLocaleException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [IllformedLocaleException](https://developer.android.com/reference/java/util/IllformedLocaleException.html#IllformedLocaleException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::IllformedLocaleException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/IllformedLocaleException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/IllformedLocaleException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [IllformedLocaleException](https://developer.android.com/reference/java/util/IllformedLocaleException.html#IllformedLocaleException(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::IllformedLocaleException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/IllformedLocaleException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/IllformedLocaleException\0", "<init>\0", "(Ljava/lang/String;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getErrorIndex](https://developer.android.com/reference/java/util/IllformedLocaleException.html#getErrorIndex())
        pub fn getErrorIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/IllformedLocaleException", java.flags == PUBLIC, .name == "getErrorIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/IllformedLocaleException\0", "getErrorIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
