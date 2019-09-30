// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-transition-Explode"))]
__jni_bindgen! {
    /// public class [Explode](https://developer.android.com/reference/android/transition/Explode.html)
    ///
    /// Required feature: android-transition-Explode
    public class Explode ("android/transition/Explode") extends crate::android::transition::Visibility {

        /// [Explode](https://developer.android.com/reference/android/transition/Explode.html#Explode())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::Explode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Explode", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Explode\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Explode](https://developer.android.com/reference/android/transition/Explode.html#Explode(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::Explode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Explode", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Explode\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [captureStartValues](https://developer.android.com/reference/android/transition/Explode.html#captureStartValues(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn captureStartValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Explode", java.flags == PUBLIC, .name == "captureStartValues", .descriptor == "(Landroid/transition/TransitionValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Explode\0", "captureStartValues\0", "(Landroid/transition/TransitionValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [captureEndValues](https://developer.android.com/reference/android/transition/Explode.html#captureEndValues(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn captureEndValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Explode", java.flags == PUBLIC, .name == "captureEndValues", .descriptor == "(Landroid/transition/TransitionValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Explode\0", "captureEndValues\0", "(Landroid/transition/TransitionValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAppear](https://developer.android.com/reference/android/transition/Explode.html#onAppear(android.view.ViewGroup,%20android.view.View,%20android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn onAppear<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Explode", java.flags == PUBLIC, .name == "onAppear", .descriptor == "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Explode\0", "onAppear\0", "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisappear](https://developer.android.com/reference/android/transition/Explode.html#onDisappear(android.view.ViewGroup,%20android.view.View,%20android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn onDisappear<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Explode", java.flags == PUBLIC, .name == "onDisappear", .descriptor == "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Explode\0", "onDisappear\0", "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
