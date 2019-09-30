// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassificationSessionFactory"))]
__jni_bindgen! {
    /// public interface [TextClassificationSessionFactory](https://developer.android.com/reference/android/view/textclassifier/TextClassificationSessionFactory.html)
    ///
    /// Required feature: android-view-textclassifier-TextClassificationSessionFactory
    public interface TextClassificationSessionFactory ("android/view/textclassifier/TextClassificationSessionFactory") extends crate::java::lang::Object {

        /// [createTextClassificationSession](https://developer.android.com/reference/android/view/textclassifier/TextClassificationSessionFactory.html#createTextClassificationSession(android.view.textclassifier.TextClassificationContext))
        ///
        /// Required features: "android-view-textclassifier-TextClassificationContext", "android-view-textclassifier-TextClassifier"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassificationContext", feature = "android-view-textclassifier-TextClassifier")))]
        pub fn createTextClassificationSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextClassificationContext>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassificationSessionFactory", java.flags == PUBLIC | ABSTRACT, .name == "createTextClassificationSession", .descriptor == "(Landroid/view/textclassifier/TextClassificationContext;)Landroid/view/textclassifier/TextClassifier;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassificationSessionFactory\0", "createTextClassificationSession\0", "(Landroid/view/textclassifier/TextClassificationContext;)Landroid/view/textclassifier/TextClassifier;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
