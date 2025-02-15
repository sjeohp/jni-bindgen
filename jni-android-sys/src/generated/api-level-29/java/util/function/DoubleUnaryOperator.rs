// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-function-DoubleUnaryOperator"))]
__jni_bindgen! {
    /// public interface [DoubleUnaryOperator](https://developer.android.com/reference/java/util/function/DoubleUnaryOperator.html)
    ///
    /// Required feature: java-util-function-DoubleUnaryOperator
    public interface DoubleUnaryOperator ("java/util/function/DoubleUnaryOperator") extends crate::java::lang::Object {

        /// [applyAsDouble](https://developer.android.com/reference/java/util/function/DoubleUnaryOperator.html#applyAsDouble(double))
        pub fn applyAsDouble<'env>(&'env self, arg0: f64) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/DoubleUnaryOperator", java.flags == PUBLIC | ABSTRACT, .name == "applyAsDouble", .descriptor == "(D)D"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/DoubleUnaryOperator\0", "applyAsDouble\0", "(D)D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compose](https://developer.android.com/reference/java/util/function/DoubleUnaryOperator.html#compose(java.util.function.DoubleUnaryOperator))
        ///
        /// Required features: "java-util-function-DoubleUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-DoubleUnaryOperator")))]
        pub fn compose<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::DoubleUnaryOperator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::DoubleUnaryOperator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/DoubleUnaryOperator", java.flags == PUBLIC, .name == "compose", .descriptor == "(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/function/DoubleUnaryOperator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/DoubleUnaryOperator\0", "compose\0", "(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/function/DoubleUnaryOperator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [andThen](https://developer.android.com/reference/java/util/function/DoubleUnaryOperator.html#andThen(java.util.function.DoubleUnaryOperator))
        ///
        /// Required features: "java-util-function-DoubleUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-DoubleUnaryOperator")))]
        pub fn andThen<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::DoubleUnaryOperator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::DoubleUnaryOperator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/DoubleUnaryOperator", java.flags == PUBLIC, .name == "andThen", .descriptor == "(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/function/DoubleUnaryOperator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/DoubleUnaryOperator\0", "andThen\0", "(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/function/DoubleUnaryOperator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [identity](https://developer.android.com/reference/java/util/function/DoubleUnaryOperator.html#identity())
        ///
        /// Required features: "java-util-function-DoubleUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-DoubleUnaryOperator")))]
        pub fn identity<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::DoubleUnaryOperator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/DoubleUnaryOperator", java.flags == PUBLIC | STATIC, .name == "identity", .descriptor == "()Ljava/util/function/DoubleUnaryOperator;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/function/DoubleUnaryOperator\0", "identity\0", "()Ljava/util/function/DoubleUnaryOperator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
