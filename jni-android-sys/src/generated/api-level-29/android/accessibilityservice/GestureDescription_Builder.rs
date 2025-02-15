// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accessibilityservice-GestureDescription_Builder"))]
__jni_bindgen! {
    /// public class [GestureDescription.Builder](https://developer.android.com/reference/android/accessibilityservice/GestureDescription.Builder.html)
    ///
    /// Required feature: android-accessibilityservice-GestureDescription_Builder
    public class GestureDescription_Builder ("android/accessibilityservice/GestureDescription$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/accessibilityservice/GestureDescription.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::accessibilityservice::GestureDescription_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/GestureDescription$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/GestureDescription$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addStroke](https://developer.android.com/reference/android/accessibilityservice/GestureDescription.Builder.html#addStroke(android.accessibilityservice.GestureDescription.StrokeDescription))
        ///
        /// Required features: "android-accessibilityservice-GestureDescription_Builder", "android-accessibilityservice-GestureDescription_StrokeDescription"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-GestureDescription_Builder", feature = "android-accessibilityservice-GestureDescription_StrokeDescription")))]
        pub fn addStroke<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::GestureDescription_StrokeDescription>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::accessibilityservice::GestureDescription_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/GestureDescription$Builder", java.flags == PUBLIC, .name == "addStroke", .descriptor == "(Landroid/accessibilityservice/GestureDescription$StrokeDescription;)Landroid/accessibilityservice/GestureDescription$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/GestureDescription$Builder\0", "addStroke\0", "(Landroid/accessibilityservice/GestureDescription$StrokeDescription;)Landroid/accessibilityservice/GestureDescription$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/accessibilityservice/GestureDescription.Builder.html#build())
        ///
        /// Required features: "android-accessibilityservice-GestureDescription"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-GestureDescription")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::accessibilityservice::GestureDescription>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/GestureDescription$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/accessibilityservice/GestureDescription;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/GestureDescription$Builder\0", "build\0", "()Landroid/accessibilityservice/GestureDescription;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
