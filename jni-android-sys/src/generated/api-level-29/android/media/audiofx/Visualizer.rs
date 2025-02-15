// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-Visualizer"))]
__jni_bindgen! {
    /// public class [Visualizer](https://developer.android.com/reference/android/media/audiofx/Visualizer.html)
    ///
    /// Required feature: android-media-audiofx-Visualizer
    public class Visualizer ("android/media/audiofx/Visualizer") extends crate::java::lang::Object {

        /// [Visualizer](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#Visualizer(int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::audiofx::Visualizer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [release](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#release())
        pub fn release<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "release", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "release\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/audiofx/Visualizer", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [setEnabled](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#setEnabled(boolean))
        pub fn setEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "setEnabled", .descriptor == "(Z)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "setEnabled\0", "(Z)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEnabled](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getEnabled())
        pub fn getEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCaptureSizeRange](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getCaptureSizeRange())
        pub fn getCaptureSizeRange<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC | STATIC | NATIVE, .name == "getCaptureSizeRange", .descriptor == "()[I"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/audiofx/Visualizer\0", "getCaptureSizeRange\0", "()[I\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxCaptureRate](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getMaxCaptureRate())
        pub fn getMaxCaptureRate<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC | STATIC | NATIVE, .name == "getMaxCaptureRate", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/audiofx/Visualizer\0", "getMaxCaptureRate\0", "()I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCaptureSize](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#setCaptureSize(int))
        pub fn setCaptureSize<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "setCaptureSize", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "setCaptureSize\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCaptureSize](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getCaptureSize())
        pub fn getCaptureSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getCaptureSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getCaptureSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setScalingMode](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#setScalingMode(int))
        pub fn setScalingMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "setScalingMode", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "setScalingMode\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getScalingMode](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getScalingMode())
        pub fn getScalingMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getScalingMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getScalingMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMeasurementMode](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#setMeasurementMode(int))
        pub fn setMeasurementMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "setMeasurementMode", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "setMeasurementMode\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMeasurementMode](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getMeasurementMode())
        pub fn getMeasurementMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getMeasurementMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getMeasurementMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSamplingRate](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getSamplingRate())
        pub fn getSamplingRate<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getSamplingRate", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getSamplingRate\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWaveForm](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getWaveForm(byte%5B%5D))
        pub fn getWaveForm<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getWaveForm", .descriptor == "([B)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getWaveForm\0", "([B)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFft](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getFft(byte%5B%5D))
        pub fn getFft<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getFft", .descriptor == "([B)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getFft\0", "([B)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMeasurementPeakRms](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#getMeasurementPeakRms(android.media.audiofx.Visualizer.MeasurementPeakRms))
        ///
        /// Required features: "android-media-audiofx-Visualizer_MeasurementPeakRms"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-Visualizer_MeasurementPeakRms")))]
        pub fn getMeasurementPeakRms<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::Visualizer_MeasurementPeakRms>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "getMeasurementPeakRms", .descriptor == "(Landroid/media/audiofx/Visualizer$MeasurementPeakRms;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "getMeasurementPeakRms\0", "(Landroid/media/audiofx/Visualizer$MeasurementPeakRms;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDataCaptureListener](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#setDataCaptureListener(android.media.audiofx.Visualizer.OnDataCaptureListener,%20int,%20boolean,%20boolean))
        ///
        /// Required features: "android-media-audiofx-Visualizer_OnDataCaptureListener"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-Visualizer_OnDataCaptureListener")))]
        pub fn setDataCaptureListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::Visualizer_OnDataCaptureListener>>, arg1: i32, arg2: bool, arg3: bool) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer", java.flags == PUBLIC, .name == "setDataCaptureListener", .descriptor == "(Landroid/media/audiofx/Visualizer$OnDataCaptureListener;IZZ)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer\0", "setDataCaptureListener\0", "(Landroid/media/audiofx/Visualizer$OnDataCaptureListener;IZZ)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ALREADY_EXISTS](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ALREADY_EXISTS)
        pub const ALREADY_EXISTS : i32 = -2;

        /// public static final [ERROR](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ERROR)
        pub const ERROR : i32 = -1;

        /// public static final [ERROR_BAD_VALUE](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ERROR_BAD_VALUE)
        pub const ERROR_BAD_VALUE : i32 = -4;

        /// public static final [ERROR_DEAD_OBJECT](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ERROR_DEAD_OBJECT)
        pub const ERROR_DEAD_OBJECT : i32 = -7;

        /// public static final [ERROR_INVALID_OPERATION](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ERROR_INVALID_OPERATION)
        pub const ERROR_INVALID_OPERATION : i32 = -5;

        /// public static final [ERROR_NO_INIT](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ERROR_NO_INIT)
        pub const ERROR_NO_INIT : i32 = -3;

        /// public static final [ERROR_NO_MEMORY](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#ERROR_NO_MEMORY)
        pub const ERROR_NO_MEMORY : i32 = -6;

        /// public static final [MEASUREMENT_MODE_NONE](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#MEASUREMENT_MODE_NONE)
        pub const MEASUREMENT_MODE_NONE : i32 = 0;

        /// public static final [MEASUREMENT_MODE_PEAK_RMS](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#MEASUREMENT_MODE_PEAK_RMS)
        pub const MEASUREMENT_MODE_PEAK_RMS : i32 = 1;

        /// public static final [SCALING_MODE_AS_PLAYED](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#SCALING_MODE_AS_PLAYED)
        pub const SCALING_MODE_AS_PLAYED : i32 = 1;

        /// public static final [SCALING_MODE_NORMALIZED](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#SCALING_MODE_NORMALIZED)
        pub const SCALING_MODE_NORMALIZED : i32 = 0;

        /// public static final [STATE_ENABLED](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#STATE_ENABLED)
        pub const STATE_ENABLED : i32 = 2;

        /// public static final [STATE_INITIALIZED](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#STATE_INITIALIZED)
        pub const STATE_INITIALIZED : i32 = 1;

        /// public static final [STATE_UNINITIALIZED](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#STATE_UNINITIALIZED)
        pub const STATE_UNINITIALIZED : i32 = 0;

        /// public static final [SUCCESS](https://developer.android.com/reference/android/media/audiofx/Visualizer.html#SUCCESS)
        pub const SUCCESS : i32 = 0;
    }
}
