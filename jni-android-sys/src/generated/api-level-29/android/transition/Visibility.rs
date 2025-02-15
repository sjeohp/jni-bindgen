// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-transition-Visibility"))]
__jni_bindgen! {
    /// public class [Visibility](https://developer.android.com/reference/android/transition/Visibility.html)
    ///
    /// Required feature: android-transition-Visibility
    public class Visibility ("android/transition/Visibility") extends crate::android::transition::Transition {

        /// [Visibility](https://developer.android.com/reference/android/transition/Visibility.html#Visibility())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::Visibility>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Visibility](https://developer.android.com/reference/android/transition/Visibility.html#Visibility(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::Visibility>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMode](https://developer.android.com/reference/android/transition/Visibility.html#setMode(int))
        pub fn setMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "setMode", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "setMode\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMode](https://developer.android.com/reference/android/transition/Visibility.html#getMode())
        pub fn getMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "getMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "getMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTransitionProperties](https://developer.android.com/reference/android/transition/Visibility.html#getTransitionProperties())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getTransitionProperties<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "getTransitionProperties", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "getTransitionProperties\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [captureStartValues](https://developer.android.com/reference/android/transition/Visibility.html#captureStartValues(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn captureStartValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "captureStartValues", .descriptor == "(Landroid/transition/TransitionValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "captureStartValues\0", "(Landroid/transition/TransitionValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [captureEndValues](https://developer.android.com/reference/android/transition/Visibility.html#captureEndValues(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn captureEndValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "captureEndValues", .descriptor == "(Landroid/transition/TransitionValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "captureEndValues\0", "(Landroid/transition/TransitionValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isVisible](https://developer.android.com/reference/android/transition/Visibility.html#isVisible(android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn isVisible<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "isVisible", .descriptor == "(Landroid/transition/TransitionValues;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "isVisible\0", "(Landroid/transition/TransitionValues;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createAnimator](https://developer.android.com/reference/android/transition/Visibility.html#createAnimator(android.view.ViewGroup,%20android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-ViewGroup")))]
        pub fn createAnimator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "createAnimator", .descriptor == "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "createAnimator\0", "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAppear](https://developer.android.com/reference/android/transition/Visibility.html#onAppear(android.view.ViewGroup,%20android.transition.TransitionValues,%20int,%20android.transition.TransitionValues,%20int))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-ViewGroup")))]
        pub fn onAppear_ViewGroup_TransitionValues_int_TransitionValues_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "onAppear", .descriptor == "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;ILandroid/transition/TransitionValues;I)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "onAppear\0", "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;ILandroid/transition/TransitionValues;I)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAppear](https://developer.android.com/reference/android/transition/Visibility.html#onAppear(android.view.ViewGroup,%20android.view.View,%20android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn onAppear_ViewGroup_View_TransitionValues_TransitionValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "onAppear", .descriptor == "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "onAppear\0", "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisappear](https://developer.android.com/reference/android/transition/Visibility.html#onDisappear(android.view.ViewGroup,%20android.transition.TransitionValues,%20int,%20android.transition.TransitionValues,%20int))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-ViewGroup")))]
        pub fn onDisappear_ViewGroup_TransitionValues_int_TransitionValues_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "onDisappear", .descriptor == "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;ILandroid/transition/TransitionValues;I)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "onDisappear\0", "(Landroid/view/ViewGroup;Landroid/transition/TransitionValues;ILandroid/transition/TransitionValues;I)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isTransitionRequired](https://developer.android.com/reference/android/transition/Visibility.html#isTransitionRequired(android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-transition-TransitionValues"
        #[cfg(any(feature = "all", all(feature = "android-transition-TransitionValues")))]
        pub fn isTransitionRequired<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "isTransitionRequired", .descriptor == "(Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "isTransitionRequired\0", "(Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisappear](https://developer.android.com/reference/android/transition/Visibility.html#onDisappear(android.view.ViewGroup,%20android.view.View,%20android.transition.TransitionValues,%20android.transition.TransitionValues))
        ///
        /// Required features: "android-animation-Animator", "android-transition-TransitionValues", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-transition-TransitionValues", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn onDisappear_ViewGroup_View_TransitionValues_TransitionValues<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::transition::TransitionValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/Visibility", java.flags == PUBLIC, .name == "onDisappear", .descriptor == "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/Visibility\0", "onDisappear\0", "(Landroid/view/ViewGroup;Landroid/view/View;Landroid/transition/TransitionValues;Landroid/transition/TransitionValues;)Landroid/animation/Animator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [MODE_IN](https://developer.android.com/reference/android/transition/Visibility.html#MODE_IN)
        pub const MODE_IN : i32 = 1;

        /// public static final [MODE_OUT](https://developer.android.com/reference/android/transition/Visibility.html#MODE_OUT)
        pub const MODE_OUT : i32 = 2;
    }
}
