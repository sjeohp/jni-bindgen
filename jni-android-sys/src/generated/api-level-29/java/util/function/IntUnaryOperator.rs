// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-function-IntUnaryOperator"))]
__jni_bindgen! {
    /// public interface [IntUnaryOperator](https://developer.android.com/reference/java/util/function/IntUnaryOperator.html)
    ///
    /// Required feature: java-util-function-IntUnaryOperator
    public interface IntUnaryOperator ("java/util/function/IntUnaryOperator") extends crate::java::lang::Object {

        /// [applyAsInt](https://developer.android.com/reference/java/util/function/IntUnaryOperator.html#applyAsInt(int))
        pub fn applyAsInt<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/IntUnaryOperator", java.flags == PUBLIC | ABSTRACT, .name == "applyAsInt", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/IntUnaryOperator\0", "applyAsInt\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compose](https://developer.android.com/reference/java/util/function/IntUnaryOperator.html#compose(java.util.function.IntUnaryOperator))
        ///
        /// Required features: "java-util-function-IntUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-IntUnaryOperator")))]
        pub fn compose<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::IntUnaryOperator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::IntUnaryOperator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/IntUnaryOperator", java.flags == PUBLIC, .name == "compose", .descriptor == "(Ljava/util/function/IntUnaryOperator;)Ljava/util/function/IntUnaryOperator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/IntUnaryOperator\0", "compose\0", "(Ljava/util/function/IntUnaryOperator;)Ljava/util/function/IntUnaryOperator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [andThen](https://developer.android.com/reference/java/util/function/IntUnaryOperator.html#andThen(java.util.function.IntUnaryOperator))
        ///
        /// Required features: "java-util-function-IntUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-IntUnaryOperator")))]
        pub fn andThen<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::IntUnaryOperator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::IntUnaryOperator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/IntUnaryOperator", java.flags == PUBLIC, .name == "andThen", .descriptor == "(Ljava/util/function/IntUnaryOperator;)Ljava/util/function/IntUnaryOperator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/IntUnaryOperator\0", "andThen\0", "(Ljava/util/function/IntUnaryOperator;)Ljava/util/function/IntUnaryOperator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [identity](https://developer.android.com/reference/java/util/function/IntUnaryOperator.html#identity())
        ///
        /// Required features: "java-util-function-IntUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-IntUnaryOperator")))]
        pub fn identity<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::IntUnaryOperator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/IntUnaryOperator", java.flags == PUBLIC | STATIC, .name == "identity", .descriptor == "()Ljava/util/function/IntUnaryOperator;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/function/IntUnaryOperator\0", "identity\0", "()Ljava/util/function/IntUnaryOperator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
