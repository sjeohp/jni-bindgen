// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-FilterQueryProvider"))]
__jni_bindgen! {
    /// public interface [FilterQueryProvider](https://developer.android.com/reference/android/widget/FilterQueryProvider.html)
    ///
    /// Required feature: android-widget-FilterQueryProvider
    public interface FilterQueryProvider ("android/widget/FilterQueryProvider") extends crate::java::lang::Object {

        /// [runQuery](https://developer.android.com/reference/android/widget/FilterQueryProvider.html#runQuery(java.lang.CharSequence))
        ///
        /// Required features: "android-database-Cursor", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-database-Cursor", feature = "java-lang-CharSequence")))]
        pub fn runQuery<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::database::Cursor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FilterQueryProvider", java.flags == PUBLIC | ABSTRACT, .name == "runQuery", .descriptor == "(Ljava/lang/CharSequence;)Landroid/database/Cursor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FilterQueryProvider\0", "runQuery\0", "(Ljava/lang/CharSequence;)Landroid/database/Cursor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
