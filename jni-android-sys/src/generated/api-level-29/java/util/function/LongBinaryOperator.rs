// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-function-LongBinaryOperator"))]
__jni_bindgen! {
    /// public interface [LongBinaryOperator](https://developer.android.com/reference/java/util/function/LongBinaryOperator.html)
    ///
    /// Required feature: java-util-function-LongBinaryOperator
    public interface LongBinaryOperator ("java/util/function/LongBinaryOperator") extends crate::java::lang::Object {

        /// [applyAsLong](https://developer.android.com/reference/java/util/function/LongBinaryOperator.html#applyAsLong(long,%20long))
        pub fn applyAsLong<'env>(&'env self, arg0: i64, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/LongBinaryOperator", java.flags == PUBLIC | ABSTRACT, .name == "applyAsLong", .descriptor == "(JJ)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/LongBinaryOperator\0", "applyAsLong\0", "(JJ)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
