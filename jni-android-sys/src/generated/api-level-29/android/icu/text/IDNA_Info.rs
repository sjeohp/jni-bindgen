// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Info"))]
__jni_bindgen! {
    /// public final class [IDNA.Info](https://developer.android.com/reference/android/icu/text/IDNA.Info.html)
    ///
    /// Required feature: android-icu-text-IDNA_Info
    public final class IDNA_Info ("android/icu/text/IDNA$Info") extends crate::java::lang::Object {

        /// [Info](https://developer.android.com/reference/android/icu/text/IDNA.Info.html#Info())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Info>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/IDNA$Info", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/IDNA$Info\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasErrors](https://developer.android.com/reference/android/icu/text/IDNA.Info.html#hasErrors())
        pub fn hasErrors<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/IDNA$Info", java.flags == PUBLIC, .name == "hasErrors", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/IDNA$Info\0", "hasErrors\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getErrors](https://developer.android.com/reference/android/icu/text/IDNA.Info.html#getErrors())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getErrors<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/IDNA$Info", java.flags == PUBLIC, .name == "getErrors", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/IDNA$Info\0", "getErrors\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isTransitionalDifferent](https://developer.android.com/reference/android/icu/text/IDNA.Info.html#isTransitionalDifferent())
        pub fn isTransitionalDifferent<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/IDNA$Info", java.flags == PUBLIC, .name == "isTransitionalDifferent", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/IDNA$Info\0", "isTransitionalDifferent\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
