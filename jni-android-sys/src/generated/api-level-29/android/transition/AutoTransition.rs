// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-transition-AutoTransition"))]
__jni_bindgen! {
    /// public class [AutoTransition](https://developer.android.com/reference/android/transition/AutoTransition.html)
    ///
    /// Required feature: android-transition-AutoTransition
    public class AutoTransition ("android/transition/AutoTransition") extends crate::android::transition::TransitionSet {

        /// [AutoTransition](https://developer.android.com/reference/android/transition/AutoTransition.html#AutoTransition())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::AutoTransition>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/AutoTransition", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/AutoTransition\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AutoTransition](https://developer.android.com/reference/android/transition/AutoTransition.html#AutoTransition(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::AutoTransition>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/AutoTransition", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/AutoTransition\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
