// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-autofill-VisibilitySetterAction_Builder"))]
__jni_bindgen! {
    /// public final class [VisibilitySetterAction.Builder](https://developer.android.com/reference/android/service/autofill/VisibilitySetterAction.Builder.html)
    ///
    /// Required feature: android-service-autofill-VisibilitySetterAction_Builder
    public final class VisibilitySetterAction_Builder ("android/service/autofill/VisibilitySetterAction$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/service/autofill/VisibilitySetterAction.Builder.html#Builder(int,%20int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::autofill::VisibilitySetterAction_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/VisibilitySetterAction$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/VisibilitySetterAction$Builder\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setVisibility](https://developer.android.com/reference/android/service/autofill/VisibilitySetterAction.Builder.html#setVisibility(int,%20int))
        ///
        /// Required features: "android-service-autofill-VisibilitySetterAction_Builder"
        #[cfg(any(feature = "all", all(feature = "android-service-autofill-VisibilitySetterAction_Builder")))]
        pub fn setVisibility<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::service::autofill::VisibilitySetterAction_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/VisibilitySetterAction$Builder", java.flags == PUBLIC, .name == "setVisibility", .descriptor == "(II)Landroid/service/autofill/VisibilitySetterAction$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/VisibilitySetterAction$Builder\0", "setVisibility\0", "(II)Landroid/service/autofill/VisibilitySetterAction$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/service/autofill/VisibilitySetterAction.Builder.html#build())
        ///
        /// Required features: "android-service-autofill-VisibilitySetterAction"
        #[cfg(any(feature = "all", all(feature = "android-service-autofill-VisibilitySetterAction")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::service::autofill::VisibilitySetterAction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/VisibilitySetterAction$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/service/autofill/VisibilitySetterAction;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/VisibilitySetterAction$Builder\0", "build\0", "()Landroid/service/autofill/VisibilitySetterAction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
