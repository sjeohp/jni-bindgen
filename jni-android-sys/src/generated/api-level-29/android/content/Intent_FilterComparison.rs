// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-Intent_FilterComparison"))]
__jni_bindgen! {
    /// public final class [Intent.FilterComparison](https://developer.android.com/reference/android/content/Intent.FilterComparison.html)
    ///
    /// Required feature: android-content-Intent_FilterComparison
    public final class Intent_FilterComparison ("android/content/Intent$FilterComparison") extends crate::java::lang::Object {

        /// [FilterComparison](https://developer.android.com/reference/android/content/Intent.FilterComparison.html#FilterComparison(android.content.Intent))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::Intent_FilterComparison>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$FilterComparison", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Intent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$FilterComparison\0", "<init>\0", "(Landroid/content/Intent;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntent](https://developer.android.com/reference/android/content/Intent.FilterComparison.html#getIntent())
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn getIntent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Intent>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$FilterComparison", java.flags == PUBLIC, .name == "getIntent", .descriptor == "()Landroid/content/Intent;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$FilterComparison\0", "getIntent\0", "()Landroid/content/Intent;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/content/Intent.FilterComparison.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$FilterComparison", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$FilterComparison\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/content/Intent.FilterComparison.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$FilterComparison", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$FilterComparison\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
