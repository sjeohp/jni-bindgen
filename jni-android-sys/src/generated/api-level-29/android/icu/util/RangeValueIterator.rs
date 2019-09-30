// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-util-RangeValueIterator"))]
__jni_bindgen! {
    /// public interface [RangeValueIterator](https://developer.android.com/reference/android/icu/util/RangeValueIterator.html)
    ///
    /// Required feature: android-icu-util-RangeValueIterator
    public interface RangeValueIterator ("android/icu/util/RangeValueIterator") extends crate::java::lang::Object {

        /// [next](https://developer.android.com/reference/android/icu/util/RangeValueIterator.html#next(android.icu.util.RangeValueIterator.Element))
        ///
        /// Required features: "android-icu-util-RangeValueIterator_Element"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-RangeValueIterator_Element")))]
        pub fn next<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::RangeValueIterator_Element>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/RangeValueIterator", java.flags == PUBLIC | ABSTRACT, .name == "next", .descriptor == "(Landroid/icu/util/RangeValueIterator$Element;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/RangeValueIterator\0", "next\0", "(Landroid/icu/util/RangeValueIterator$Element;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/android/icu/util/RangeValueIterator.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/RangeValueIterator", java.flags == PUBLIC | ABSTRACT, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/RangeValueIterator\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
