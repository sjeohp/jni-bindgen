// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-transition-ChangeImageTransform"))]
__jni_bindgen! {
    /// public class [ChangeImageTransform](https://developer.android.com/reference/android/transition/ChangeImageTransform.html)
    ///
    /// Required feature: android-transition-ChangeImageTransform
    public class ChangeImageTransform ("android/transition/ChangeImageTransform") extends crate::android::transition::Transition {

        /// [ChangeImageTransform](https://developer.android.com/reference/android/transition/ChangeImageTransform.html#ChangeImageTransform())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::ChangeImageTransform>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/ChangeImageTransform", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/ChangeImageTransform\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ChangeImageTransform](https://developer.android.com/reference/android/transition/ChangeImageTransform.html#ChangeImageTransform(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::ChangeImageTransform>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/ChangeImageTransform", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/ChangeImageTransform\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [captureStartValues](https://developer.android.com/reference/android/transition/ChangeImageTransform.html#captureStartValues(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn captureStartValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/ChangeImageTransform", java.flags == PUBLIC, .name == "captureStartValues", .descriptor == "(Landroid/transition/TransitionValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/ChangeImageTransform\0", "captureStartValues\0", "(Landroid/transition/TransitionValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [captureEndValues](https://developer.android.com/reference/android/transition/ChangeImageTransform.html#captureEndValues(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn captureEndValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/ChangeImageTransform", java.flags == PUBLIC, .name == "captureEndValues", .descriptor == "(Landroid/transition/TransitionValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/ChangeImageTransform\0", "captureEndValues\0", "(Landroid/transition/TransitionValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTransitionProperties](https://developer.android.com/reference/android/transition/ChangeImageTransform.html#getTransitionProperties())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getTransitionProperties<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/ChangeImageTransform", java.flags == PUBLIC, .name == "getTransitionProperties", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/ChangeImageTransform\0", "getTransitionProperties\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createAnimator](https://developer.android.com/reference/android/transition/ChangeImageTransform.html#createAnimator(android.view.ViewGroup,%20android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-ViewGroup")))]
        pub fn createAnimator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/ChangeImageTransform", java.flags == PUBLIC, .name == "createAnimator", .descriptor == "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/ChangeImageTransform\0", "createAnimator\0", "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
