// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-BidiClassifier"))]
__jni_bindgen! {
    /// public class [BidiClassifier](https://developer.android.com/reference/android/icu/text/BidiClassifier.html)
    ///
    /// Required feature: android-icu-text-BidiClassifier
    public class BidiClassifier ("android/icu/text/BidiClassifier") extends crate::java::lang::Object {

        /// [BidiClassifier](https://developer.android.com/reference/android/icu/text/BidiClassifier.html#BidiClassifier(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::BidiClassifier>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/BidiClassifier", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/BidiClassifier\0", "<init>\0", "(Ljava/lang/Object;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setContext](https://developer.android.com/reference/android/icu/text/BidiClassifier.html#setContext(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setContext<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/BidiClassifier", java.flags == PUBLIC, .name == "setContext", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/BidiClassifier\0", "setContext\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getContext](https://developer.android.com/reference/android/icu/text/BidiClassifier.html#getContext())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getContext<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/BidiClassifier", java.flags == PUBLIC, .name == "getContext", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/BidiClassifier\0", "getContext\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [classify](https://developer.android.com/reference/android/icu/text/BidiClassifier.html#classify(int))
        pub fn classify<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/BidiClassifier", java.flags == PUBLIC, .name == "classify", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/BidiClassifier\0", "classify\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
