// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-util-Freezable"))]
__jni_bindgen! {
    /// public interface [Freezable](https://developer.android.com/reference/android/icu/util/Freezable.html)
    ///
    /// Required feature: android-icu-util-Freezable
    public interface Freezable ("android/icu/util/Freezable") extends crate::java::lang::Object, implements crate::java::lang::Cloneable {

        /// [isFrozen](https://developer.android.com/reference/android/icu/util/Freezable.html#isFrozen())
        pub fn isFrozen<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Freezable", java.flags == PUBLIC | ABSTRACT, .name == "isFrozen", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Freezable\0", "isFrozen\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [freeze](https://developer.android.com/reference/android/icu/util/Freezable.html#freeze())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn freeze<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Freezable", java.flags == PUBLIC | ABSTRACT, .name == "freeze", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Freezable\0", "freeze\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cloneAsThawed](https://developer.android.com/reference/android/icu/util/Freezable.html#cloneAsThawed())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn cloneAsThawed<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Freezable", java.flags == PUBLIC | ABSTRACT, .name == "cloneAsThawed", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Freezable\0", "cloneAsThawed\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
