// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-R_interpolator"))]
__jni_bindgen! {
    /// public final class [R.interpolator](https://developer.android.com/reference/android/R.interpolator.html)
    ///
    /// Required feature: android-R_interpolator
    public final class R_interpolator ("android/R$interpolator") extends crate::java::lang::Object {

        /// [interpolator](https://developer.android.com/reference/android/R.interpolator.html#interpolator())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::R_interpolator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/R$interpolator", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/R$interpolator\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [accelerate_cubic](https://developer.android.com/reference/android/R.interpolator.html#accelerate_cubic)
        pub const accelerate_cubic : i32 = 17563650;

        /// public static final [accelerate_decelerate](https://developer.android.com/reference/android/R.interpolator.html#accelerate_decelerate)
        pub const accelerate_decelerate : i32 = 17563654;

        /// public static final [accelerate_quad](https://developer.android.com/reference/android/R.interpolator.html#accelerate_quad)
        pub const accelerate_quad : i32 = 17563648;

        /// public static final [accelerate_quint](https://developer.android.com/reference/android/R.interpolator.html#accelerate_quint)
        pub const accelerate_quint : i32 = 17563652;

        /// public static final [anticipate](https://developer.android.com/reference/android/R.interpolator.html#anticipate)
        pub const anticipate : i32 = 17563655;

        /// public static final [anticipate_overshoot](https://developer.android.com/reference/android/R.interpolator.html#anticipate_overshoot)
        pub const anticipate_overshoot : i32 = 17563657;

        /// public static final [bounce](https://developer.android.com/reference/android/R.interpolator.html#bounce)
        pub const bounce : i32 = 17563658;

        /// public static final [cycle](https://developer.android.com/reference/android/R.interpolator.html#cycle)
        pub const cycle : i32 = 17563660;

        /// public static final [decelerate_cubic](https://developer.android.com/reference/android/R.interpolator.html#decelerate_cubic)
        pub const decelerate_cubic : i32 = 17563651;

        /// public static final [decelerate_quad](https://developer.android.com/reference/android/R.interpolator.html#decelerate_quad)
        pub const decelerate_quad : i32 = 17563649;

        /// public static final [decelerate_quint](https://developer.android.com/reference/android/R.interpolator.html#decelerate_quint)
        pub const decelerate_quint : i32 = 17563653;

        /// public static final [fast_out_extra_slow_in](https://developer.android.com/reference/android/R.interpolator.html#fast_out_extra_slow_in)
        pub const fast_out_extra_slow_in : i32 = 17563674;

        /// public static final [fast_out_linear_in](https://developer.android.com/reference/android/R.interpolator.html#fast_out_linear_in)
        pub const fast_out_linear_in : i32 = 17563663;

        /// public static final [fast_out_slow_in](https://developer.android.com/reference/android/R.interpolator.html#fast_out_slow_in)
        pub const fast_out_slow_in : i32 = 17563661;

        /// public static final [linear](https://developer.android.com/reference/android/R.interpolator.html#linear)
        pub const linear : i32 = 17563659;

        /// public static final [linear_out_slow_in](https://developer.android.com/reference/android/R.interpolator.html#linear_out_slow_in)
        pub const linear_out_slow_in : i32 = 17563662;

        /// public static final [overshoot](https://developer.android.com/reference/android/R.interpolator.html#overshoot)
        pub const overshoot : i32 = 17563656;
    }
}
