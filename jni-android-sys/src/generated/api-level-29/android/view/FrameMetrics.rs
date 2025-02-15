// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-FrameMetrics"))]
__jni_bindgen! {
    /// public final class [FrameMetrics](https://developer.android.com/reference/android/view/FrameMetrics.html)
    ///
    /// Required feature: android-view-FrameMetrics
    public final class FrameMetrics ("android/view/FrameMetrics") extends crate::java::lang::Object {

        /// [FrameMetrics](https://developer.android.com/reference/android/view/FrameMetrics.html#FrameMetrics(android.view.FrameMetrics))
        ///
        /// Required features: "android-view-FrameMetrics"
        #[cfg(any(feature = "all", all(feature = "android-view-FrameMetrics")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::FrameMetrics>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::FrameMetrics>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/FrameMetrics", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/FrameMetrics;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/FrameMetrics\0", "<init>\0", "(Landroid/view/FrameMetrics;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMetric](https://developer.android.com/reference/android/view/FrameMetrics.html#getMetric(int))
        pub fn getMetric<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/FrameMetrics", java.flags == PUBLIC, .name == "getMetric", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/FrameMetrics\0", "getMetric\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ANIMATION_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#ANIMATION_DURATION)
        pub const ANIMATION_DURATION : i32 = 2;

        /// public static final [COMMAND_ISSUE_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#COMMAND_ISSUE_DURATION)
        pub const COMMAND_ISSUE_DURATION : i32 = 6;

        /// public static final [DRAW_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#DRAW_DURATION)
        pub const DRAW_DURATION : i32 = 4;

        /// public static final [FIRST_DRAW_FRAME](https://developer.android.com/reference/android/view/FrameMetrics.html#FIRST_DRAW_FRAME)
        pub const FIRST_DRAW_FRAME : i32 = 9;

        /// public static final [INPUT_HANDLING_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#INPUT_HANDLING_DURATION)
        pub const INPUT_HANDLING_DURATION : i32 = 1;

        /// public static final [INTENDED_VSYNC_TIMESTAMP](https://developer.android.com/reference/android/view/FrameMetrics.html#INTENDED_VSYNC_TIMESTAMP)
        pub const INTENDED_VSYNC_TIMESTAMP : i32 = 10;

        /// public static final [LAYOUT_MEASURE_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#LAYOUT_MEASURE_DURATION)
        pub const LAYOUT_MEASURE_DURATION : i32 = 3;

        /// public static final [SWAP_BUFFERS_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#SWAP_BUFFERS_DURATION)
        pub const SWAP_BUFFERS_DURATION : i32 = 7;

        /// public static final [SYNC_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#SYNC_DURATION)
        pub const SYNC_DURATION : i32 = 5;

        /// public static final [TOTAL_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#TOTAL_DURATION)
        pub const TOTAL_DURATION : i32 = 8;

        /// public static final [UNKNOWN_DELAY_DURATION](https://developer.android.com/reference/android/view/FrameMetrics.html#UNKNOWN_DELAY_DURATION)
        pub const UNKNOWN_DELAY_DURATION : i32 = 0;

        /// public static final [VSYNC_TIMESTAMP](https://developer.android.com/reference/android/view/FrameMetrics.html#VSYNC_TIMESTAMP)
        pub const VSYNC_TIMESTAMP : i32 = 11;
    }
}
