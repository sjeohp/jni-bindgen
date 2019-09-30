// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-HardwarePropertiesManager"))]
__jni_bindgen! {
    /// public class [HardwarePropertiesManager](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html)
    ///
    /// Required feature: android-os-HardwarePropertiesManager
    public class HardwarePropertiesManager ("android/os/HardwarePropertiesManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [HardwarePropertiesManager](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#HardwarePropertiesManager())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::HardwarePropertiesManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/HardwarePropertiesManager", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HardwarePropertiesManager\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDeviceTemperatures](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#getDeviceTemperatures(int,%20int))
        pub fn getDeviceTemperatures<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::FloatArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HardwarePropertiesManager", java.flags == PUBLIC, .name == "getDeviceTemperatures", .descriptor == "(II)[F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HardwarePropertiesManager\0", "getDeviceTemperatures\0", "(II)[F\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCpuUsages](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#getCpuUsages())
        ///
        /// Required features: "android-os-CpuUsageInfo"
        #[cfg(any(feature = "all", all(feature = "android-os-CpuUsageInfo")))]
        pub fn getCpuUsages<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::os::CpuUsageInfo, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HardwarePropertiesManager", java.flags == PUBLIC, .name == "getCpuUsages", .descriptor == "()[Landroid/os/CpuUsageInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HardwarePropertiesManager\0", "getCpuUsages\0", "()[Landroid/os/CpuUsageInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFanSpeeds](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#getFanSpeeds())
        pub fn getFanSpeeds<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::FloatArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HardwarePropertiesManager", java.flags == PUBLIC, .name == "getFanSpeeds", .descriptor == "()[F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HardwarePropertiesManager\0", "getFanSpeeds\0", "()[F\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DEVICE_TEMPERATURE_BATTERY](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#DEVICE_TEMPERATURE_BATTERY)
        pub const DEVICE_TEMPERATURE_BATTERY : i32 = 2;

        /// public static final [DEVICE_TEMPERATURE_CPU](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#DEVICE_TEMPERATURE_CPU)
        pub const DEVICE_TEMPERATURE_CPU : i32 = 0;

        /// public static final [DEVICE_TEMPERATURE_GPU](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#DEVICE_TEMPERATURE_GPU)
        pub const DEVICE_TEMPERATURE_GPU : i32 = 1;

        /// public static final [DEVICE_TEMPERATURE_SKIN](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#DEVICE_TEMPERATURE_SKIN)
        pub const DEVICE_TEMPERATURE_SKIN : i32 = 3;

        /// public static final [TEMPERATURE_CURRENT](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#TEMPERATURE_CURRENT)
        pub const TEMPERATURE_CURRENT : i32 = 0;

        /// public static final [TEMPERATURE_SHUTDOWN](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#TEMPERATURE_SHUTDOWN)
        pub const TEMPERATURE_SHUTDOWN : i32 = 2;

        /// public static final [TEMPERATURE_THROTTLING](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#TEMPERATURE_THROTTLING)
        pub const TEMPERATURE_THROTTLING : i32 = 1;

        /// public static final [TEMPERATURE_THROTTLING_BELOW_VR_MIN](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#TEMPERATURE_THROTTLING_BELOW_VR_MIN)
        pub const TEMPERATURE_THROTTLING_BELOW_VR_MIN : i32 = 3;

        /// public static final [UNDEFINED_TEMPERATURE](https://developer.android.com/reference/android/os/HardwarePropertiesManager.html#UNDEFINED_TEMPERATURE)
        pub const UNDEFINED_TEMPERATURE : f32 = -340282350000000000000000000000000000000f32;
    }
}
