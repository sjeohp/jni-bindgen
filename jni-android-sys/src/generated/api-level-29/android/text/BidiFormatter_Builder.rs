// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-BidiFormatter_Builder"))]
__jni_bindgen! {
    /// public final class [BidiFormatter.Builder](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html)
    ///
    /// Required feature: android-text-BidiFormatter_Builder
    public final class BidiFormatter_Builder ("android/text/BidiFormatter$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::BidiFormatter_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/BidiFormatter$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/BidiFormatter$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Builder](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html#Builder(boolean))
        pub fn new_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::BidiFormatter_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/BidiFormatter$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/BidiFormatter$Builder\0", "<init>\0", "(Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Builder](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html#Builder(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn new_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::BidiFormatter_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/BidiFormatter$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/BidiFormatter$Builder\0", "<init>\0", "(Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stereoReset](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html#stereoReset(boolean))
        ///
        /// Required features: "android-text-BidiFormatter_Builder"
        #[cfg(any(feature = "all", all(feature = "android-text-BidiFormatter_Builder")))]
        pub fn stereoReset<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::BidiFormatter_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/BidiFormatter$Builder", java.flags == PUBLIC, .name == "stereoReset", .descriptor == "(Z)Landroid/text/BidiFormatter$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/BidiFormatter$Builder\0", "stereoReset\0", "(Z)Landroid/text/BidiFormatter$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTextDirectionHeuristic](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html#setTextDirectionHeuristic(android.text.TextDirectionHeuristic))
        ///
        /// Required features: "android-text-BidiFormatter_Builder", "android-text-TextDirectionHeuristic"
        #[cfg(any(feature = "all", all(feature = "android-text-BidiFormatter_Builder", feature = "android-text-TextDirectionHeuristic")))]
        pub fn setTextDirectionHeuristic<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::TextDirectionHeuristic>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::BidiFormatter_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/BidiFormatter$Builder", java.flags == PUBLIC, .name == "setTextDirectionHeuristic", .descriptor == "(Landroid/text/TextDirectionHeuristic;)Landroid/text/BidiFormatter$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/BidiFormatter$Builder\0", "setTextDirectionHeuristic\0", "(Landroid/text/TextDirectionHeuristic;)Landroid/text/BidiFormatter$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/text/BidiFormatter.Builder.html#build())
        ///
        /// Required features: "android-text-BidiFormatter"
        #[cfg(any(feature = "all", all(feature = "android-text-BidiFormatter")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::BidiFormatter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/BidiFormatter$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/text/BidiFormatter;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/BidiFormatter$Builder\0", "build\0", "()Landroid/text/BidiFormatter;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
