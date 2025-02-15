// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassifier_EntityConfig_Builder"))]
__jni_bindgen! {
    /// public final class [TextClassifier.EntityConfig.Builder](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html)
    ///
    /// Required feature: android-view-textclassifier-TextClassifier_EntityConfig_Builder
    public final class TextClassifier_EntityConfig_Builder ("android/view/textclassifier/TextClassifier$EntityConfig$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier_EntityConfig_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier$EntityConfig$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier$EntityConfig$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIncludedTypes](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html#setIncludedTypes(java.util.Collection))
        ///
        /// Required features: "android-view-textclassifier-TextClassifier_EntityConfig_Builder", "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifier_EntityConfig_Builder", feature = "java-util-Collection")))]
        pub fn setIncludedTypes<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier_EntityConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier$EntityConfig$Builder", java.flags == PUBLIC, .name == "setIncludedTypes", .descriptor == "(Ljava/util/Collection;)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier$EntityConfig$Builder\0", "setIncludedTypes\0", "(Ljava/util/Collection;)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setExcludedTypes](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html#setExcludedTypes(java.util.Collection))
        ///
        /// Required features: "android-view-textclassifier-TextClassifier_EntityConfig_Builder", "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifier_EntityConfig_Builder", feature = "java-util-Collection")))]
        pub fn setExcludedTypes<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier_EntityConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier$EntityConfig$Builder", java.flags == PUBLIC, .name == "setExcludedTypes", .descriptor == "(Ljava/util/Collection;)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier$EntityConfig$Builder\0", "setExcludedTypes\0", "(Ljava/util/Collection;)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [includeTypesFromTextClassifier](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html#includeTypesFromTextClassifier(boolean))
        ///
        /// Required features: "android-view-textclassifier-TextClassifier_EntityConfig_Builder"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifier_EntityConfig_Builder")))]
        pub fn includeTypesFromTextClassifier<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier_EntityConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier$EntityConfig$Builder", java.flags == PUBLIC, .name == "includeTypesFromTextClassifier", .descriptor == "(Z)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier$EntityConfig$Builder\0", "includeTypesFromTextClassifier\0", "(Z)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHints](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html#setHints(java.util.Collection))
        ///
        /// Required features: "android-view-textclassifier-TextClassifier_EntityConfig_Builder", "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifier_EntityConfig_Builder", feature = "java-util-Collection")))]
        pub fn setHints<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier_EntityConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier$EntityConfig$Builder", java.flags == PUBLIC, .name == "setHints", .descriptor == "(Ljava/util/Collection;)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier$EntityConfig$Builder\0", "setHints\0", "(Ljava/util/Collection;)Landroid/view/textclassifier/TextClassifier$EntityConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.EntityConfig.Builder.html#build())
        ///
        /// Required features: "android-view-textclassifier-TextClassifier_EntityConfig"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifier_EntityConfig")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier_EntityConfig>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier$EntityConfig$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/view/textclassifier/TextClassifier$EntityConfig;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier$EntityConfig$Builder\0", "build\0", "()Landroid/view/textclassifier/TextClassifier$EntityConfig;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
