// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-animation-BounceInterpolator"))]
__jni_bindgen! {
    /// public class [BounceInterpolator](https://developer.android.com/reference/android/view/animation/BounceInterpolator.html)
    ///
    /// Required feature: android-view-animation-BounceInterpolator
    public class BounceInterpolator ("android/view/animation/BounceInterpolator") extends crate::android::view::animation::BaseInterpolator {

        /// [BounceInterpolator](https://developer.android.com/reference/android/view/animation/BounceInterpolator.html#BounceInterpolator())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::animation::BounceInterpolator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/BounceInterpolator", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/animation/BounceInterpolator\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BounceInterpolator](https://developer.android.com/reference/android/view/animation/BounceInterpolator.html#BounceInterpolator(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::animation::BounceInterpolator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/BounceInterpolator", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/animation/BounceInterpolator\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInterpolation](https://developer.android.com/reference/android/view/animation/BounceInterpolator.html#getInterpolation(float))
        pub fn getInterpolation<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/animation/BounceInterpolator", java.flags == PUBLIC, .name == "getInterpolation", .descriptor == "(F)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/animation/BounceInterpolator\0", "getInterpolation\0", "(F)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
