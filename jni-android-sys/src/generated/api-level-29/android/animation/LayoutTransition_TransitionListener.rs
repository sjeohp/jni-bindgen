// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-animation-LayoutTransition_TransitionListener"))]
__jni_bindgen! {
    /// public interface [LayoutTransition.TransitionListener](https://developer.android.com/reference/android/animation/LayoutTransition.TransitionListener.html)
    ///
    /// Required feature: android-animation-LayoutTransition_TransitionListener
    public interface LayoutTransition_TransitionListener ("android/animation/LayoutTransition$TransitionListener") extends crate::java::lang::Object {

        /// [startTransition](https://developer.android.com/reference/android/animation/LayoutTransition.TransitionListener.html#startTransition(android.animation.LayoutTransition,%20android.view.ViewGroup,%20android.view.View,%20int))
        ///
        /// Required features: "android-animation-LayoutTransition", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-LayoutTransition", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn startTransition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::animation::LayoutTransition>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/LayoutTransition$TransitionListener", java.flags == PUBLIC | ABSTRACT, .name == "startTransition", .descriptor == "(Landroid/animation/LayoutTransition;Landroid/view/ViewGroup;Landroid/view/View;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/LayoutTransition$TransitionListener\0", "startTransition\0", "(Landroid/animation/LayoutTransition;Landroid/view/ViewGroup;Landroid/view/View;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endTransition](https://developer.android.com/reference/android/animation/LayoutTransition.TransitionListener.html#endTransition(android.animation.LayoutTransition,%20android.view.ViewGroup,%20android.view.View,%20int))
        ///
        /// Required features: "android-animation-LayoutTransition", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-LayoutTransition", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn endTransition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::animation::LayoutTransition>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/LayoutTransition$TransitionListener", java.flags == PUBLIC | ABSTRACT, .name == "endTransition", .descriptor == "(Landroid/animation/LayoutTransition;Landroid/view/ViewGroup;Landroid/view/View;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/LayoutTransition$TransitionListener\0", "endTransition\0", "(Landroid/animation/LayoutTransition;Landroid/view/ViewGroup;Landroid/view/View;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
