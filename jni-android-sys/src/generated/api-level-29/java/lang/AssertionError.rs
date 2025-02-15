// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-AssertionError"))]
__jni_bindgen! {
    /// public class [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html)
    ///
    /// Required feature: java-lang-AssertionError
    public class AssertionError ("java/lang/AssertionError") extends crate::java::lang::Error {

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn new_Object<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(Ljava/lang/Object;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(boolean))
        pub fn new_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(char))
        pub fn new_char<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(C)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(C)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(long))
        pub fn new_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(J)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(float))
        pub fn new_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(F)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(double))
        pub fn new_double<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(D)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(D)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AssertionError](https://developer.android.com/reference/java/lang/AssertionError.html#AssertionError(java.lang.String,%20java.lang.Throwable))
        ///
        /// Required features: "java-lang-String", "java-lang-Throwable"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-lang-Throwable")))]
        pub fn new_String_Throwable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::AssertionError>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/AssertionError", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;Ljava/lang/Throwable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/AssertionError\0", "<init>\0", "(Ljava/lang/String;Ljava/lang/Throwable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
