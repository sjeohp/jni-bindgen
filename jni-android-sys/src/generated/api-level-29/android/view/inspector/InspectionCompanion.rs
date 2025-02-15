// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-inspector-InspectionCompanion"))]
__jni_bindgen! {
    /// public interface [InspectionCompanion](https://developer.android.com/reference/android/view/inspector/InspectionCompanion.html)
    ///
    /// Required feature: android-view-inspector-InspectionCompanion
    public interface InspectionCompanion ("android/view/inspector/InspectionCompanion") extends crate::java::lang::Object {

        /// [mapProperties](https://developer.android.com/reference/android/view/inspector/InspectionCompanion.html#mapProperties(android.view.inspector.PropertyMapper))
        ///
        /// Required features: "android-view-inspector-PropertyMapper"
        #[cfg(any(feature = "all", all(feature = "android-view-inspector-PropertyMapper")))]
        pub fn mapProperties<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inspector::PropertyMapper>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inspector/InspectionCompanion", java.flags == PUBLIC | ABSTRACT, .name == "mapProperties", .descriptor == "(Landroid/view/inspector/PropertyMapper;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inspector/InspectionCompanion\0", "mapProperties\0", "(Landroid/view/inspector/PropertyMapper;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readProperties](https://developer.android.com/reference/android/view/inspector/InspectionCompanion.html#readProperties(java.lang.Object,%20android.view.inspector.PropertyReader))
        ///
        /// Required features: "android-view-inspector-PropertyReader", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-view-inspector-PropertyReader", feature = "java-lang-Object")))]
        pub fn readProperties<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inspector::PropertyReader>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inspector/InspectionCompanion", java.flags == PUBLIC | ABSTRACT, .name == "readProperties", .descriptor == "(Ljava/lang/Object;Landroid/view/inspector/PropertyReader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inspector/InspectionCompanion\0", "readProperties\0", "(Ljava/lang/Object;Landroid/view/inspector/PropertyReader;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
