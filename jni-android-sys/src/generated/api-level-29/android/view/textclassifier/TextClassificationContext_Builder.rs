// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassificationContext_Builder"))]
__jni_bindgen! {
    /// public final class [TextClassificationContext.Builder](https://developer.android.com/reference/android/view/textclassifier/TextClassificationContext.Builder.html)
    ///
    /// Required feature: android-view-textclassifier-TextClassificationContext_Builder
    public final class TextClassificationContext_Builder ("android/view/textclassifier/TextClassificationContext$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/view/textclassifier/TextClassificationContext.Builder.html#Builder(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassificationContext_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassificationContext$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassificationContext$Builder\0", "<init>\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWidgetVersion](https://developer.android.com/reference/android/view/textclassifier/TextClassificationContext.Builder.html#setWidgetVersion(java.lang.String))
        ///
        /// Required features: "android-view-textclassifier-TextClassificationContext_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassificationContext_Builder", feature = "java-lang-String")))]
        pub fn setWidgetVersion<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassificationContext_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassificationContext$Builder", java.flags == PUBLIC, .name == "setWidgetVersion", .descriptor == "(Ljava/lang/String;)Landroid/view/textclassifier/TextClassificationContext$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassificationContext$Builder\0", "setWidgetVersion\0", "(Ljava/lang/String;)Landroid/view/textclassifier/TextClassificationContext$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/view/textclassifier/TextClassificationContext.Builder.html#build())
        ///
        /// Required features: "android-view-textclassifier-TextClassificationContext"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassificationContext")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassificationContext>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassificationContext$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/view/textclassifier/TextClassificationContext;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassificationContext$Builder\0", "build\0", "()Landroid/view/textclassifier/TextClassificationContext;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
