// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-animation-AnimationUtils"))]
__jni_bindgen! {
    /// public class [AnimationUtils](https://developer.android.com/reference/android/view/animation/AnimationUtils.html)
    ///
    /// Required feature: android-view-animation-AnimationUtils
    public class AnimationUtils ("android/view/animation/AnimationUtils") extends crate::java::lang::Object {

        /// [AnimationUtils](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#AnimationUtils())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::animation::AnimationUtils>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/animation/AnimationUtils\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [currentAnimationTimeMillis](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#currentAnimationTimeMillis())
        pub fn currentAnimationTimeMillis<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "currentAnimationTimeMillis", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "currentAnimationTimeMillis\0", "()J\0");
                __jni_env.call_static_long_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadAnimation](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#loadAnimation(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-view-animation-Animation"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-animation-Animation")))]
        pub fn loadAnimation<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::animation::Animation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "loadAnimation", .descriptor == "(Landroid/content/Context;I)Landroid/view/animation/Animation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "loadAnimation\0", "(Landroid/content/Context;I)Landroid/view/animation/Animation;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadLayoutAnimation](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#loadLayoutAnimation(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-view-animation-LayoutAnimationController"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-animation-LayoutAnimationController")))]
        pub fn loadLayoutAnimation<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::animation::LayoutAnimationController>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "loadLayoutAnimation", .descriptor == "(Landroid/content/Context;I)Landroid/view/animation/LayoutAnimationController;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "loadLayoutAnimation\0", "(Landroid/content/Context;I)Landroid/view/animation/LayoutAnimationController;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [makeInAnimation](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#makeInAnimation(android.content.Context,%20boolean))
        ///
        /// Required features: "android-content-Context", "android-view-animation-Animation"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-animation-Animation")))]
        pub fn makeInAnimation<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::animation::Animation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "makeInAnimation", .descriptor == "(Landroid/content/Context;Z)Landroid/view/animation/Animation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "makeInAnimation\0", "(Landroid/content/Context;Z)Landroid/view/animation/Animation;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [makeOutAnimation](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#makeOutAnimation(android.content.Context,%20boolean))
        ///
        /// Required features: "android-content-Context", "android-view-animation-Animation"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-animation-Animation")))]
        pub fn makeOutAnimation<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::animation::Animation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "makeOutAnimation", .descriptor == "(Landroid/content/Context;Z)Landroid/view/animation/Animation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "makeOutAnimation\0", "(Landroid/content/Context;Z)Landroid/view/animation/Animation;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [makeInChildBottomAnimation](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#makeInChildBottomAnimation(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-view-animation-Animation"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-animation-Animation")))]
        pub fn makeInChildBottomAnimation<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::animation::Animation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "makeInChildBottomAnimation", .descriptor == "(Landroid/content/Context;)Landroid/view/animation/Animation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "makeInChildBottomAnimation\0", "(Landroid/content/Context;)Landroid/view/animation/Animation;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadInterpolator](https://developer.android.com/reference/android/view/animation/AnimationUtils.html#loadInterpolator(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-view-animation-Interpolator"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-animation-Interpolator")))]
        pub fn loadInterpolator<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::animation::Interpolator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/AnimationUtils", java.flags == PUBLIC | STATIC, .name == "loadInterpolator", .descriptor == "(Landroid/content/Context;I)Landroid/view/animation/Interpolator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/animation/AnimationUtils\0", "loadInterpolator\0", "(Landroid/content/Context;I)Landroid/view/animation/Interpolator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
