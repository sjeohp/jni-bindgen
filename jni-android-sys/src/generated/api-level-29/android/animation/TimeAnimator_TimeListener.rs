// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-animation-TimeAnimator_TimeListener"))]
__jni_bindgen! {
    /// public interface [TimeAnimator.TimeListener](https://developer.android.com/reference/android/animation/TimeAnimator.TimeListener.html)
    ///
    /// Required feature: android-animation-TimeAnimator_TimeListener
    public interface TimeAnimator_TimeListener ("android/animation/TimeAnimator$TimeListener") extends crate::java::lang::Object {

        /// [onTimeUpdate](https://developer.android.com/reference/android/animation/TimeAnimator.TimeListener.html#onTimeUpdate(android.animation.TimeAnimator,%20long,%20long))
        ///
        /// Required features: "android-animation-TimeAnimator"
        #[cfg(any(feature = "all", all(feature = "android-animation-TimeAnimator")))]
        pub fn onTimeUpdate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::animation::TimeAnimator>>, arg1: i64, arg2: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/TimeAnimator$TimeListener", java.flags == PUBLIC | ABSTRACT, .name == "onTimeUpdate", .descriptor == "(Landroid/animation/TimeAnimator;JJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/TimeAnimator$TimeListener\0", "onTimeUpdate\0", "(Landroid/animation/TimeAnimator;JJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
