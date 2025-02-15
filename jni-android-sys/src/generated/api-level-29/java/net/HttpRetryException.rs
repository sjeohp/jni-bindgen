// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-HttpRetryException"))]
__jni_bindgen! {
    /// public class [HttpRetryException](https://developer.android.com/reference/java/net/HttpRetryException.html)
    ///
    /// Required feature: java-net-HttpRetryException
    public class HttpRetryException ("java/net/HttpRetryException") extends crate::java::io::IOException {

        /// [HttpRetryException](https://developer.android.com/reference/java/net/HttpRetryException.html#HttpRetryException(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::HttpRetryException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpRetryException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpRetryException\0", "<init>\0", "(Ljava/lang/String;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [HttpRetryException](https://developer.android.com/reference/java/net/HttpRetryException.html#HttpRetryException(java.lang.String,%20int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String_int_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::HttpRetryException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpRetryException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpRetryException\0", "<init>\0", "(Ljava/lang/String;ILjava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [responseCode](https://developer.android.com/reference/java/net/HttpRetryException.html#responseCode())
        pub fn responseCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpRetryException", java.flags == PUBLIC, .name == "responseCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpRetryException\0", "responseCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReason](https://developer.android.com/reference/java/net/HttpRetryException.html#getReason())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getReason<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpRetryException", java.flags == PUBLIC, .name == "getReason", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpRetryException\0", "getReason\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocation](https://developer.android.com/reference/java/net/HttpRetryException.html#getLocation())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getLocation<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpRetryException", java.flags == PUBLIC, .name == "getLocation", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpRetryException\0", "getLocation\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
