// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-SensorAdditionalInfo"))]
__jni_bindgen! {
    /// public class [SensorAdditionalInfo](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html)
    ///
    /// Required feature: android-hardware-SensorAdditionalInfo
    public class SensorAdditionalInfo ("android/hardware/SensorAdditionalInfo") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SensorAdditionalInfo](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#SensorAdditionalInfo(android.hardware.Sensor,%20int,%20int,%20int%5B%5D,%20float%5B%5D))
        // ///
        // /// Required features: "android-hardware-Sensor"
        // #[cfg(any(feature = "all", all(feature = "android-hardware-Sensor")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::Sensor>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::SensorAdditionalInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/hardware/SensorAdditionalInfo", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/hardware/Sensor;II[I[F)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/SensorAdditionalInfo\0", "<init>\0", "(Landroid/hardware/Sensor;II[I[F)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [TYPE_FRAME_BEGIN](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_FRAME_BEGIN)
        pub const TYPE_FRAME_BEGIN : i32 = 0;

        /// public static final [TYPE_FRAME_END](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_FRAME_END)
        pub const TYPE_FRAME_END : i32 = 1;

        /// public static final [TYPE_INTERNAL_TEMPERATURE](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_INTERNAL_TEMPERATURE)
        pub const TYPE_INTERNAL_TEMPERATURE : i32 = 65537;

        /// public static final [TYPE_SAMPLING](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_SAMPLING)
        pub const TYPE_SAMPLING : i32 = 65540;

        /// public static final [TYPE_SENSOR_PLACEMENT](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_SENSOR_PLACEMENT)
        pub const TYPE_SENSOR_PLACEMENT : i32 = 65539;

        /// public static final [TYPE_UNTRACKED_DELAY](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_UNTRACKED_DELAY)
        pub const TYPE_UNTRACKED_DELAY : i32 = 65536;

        /// public static final [TYPE_VEC3_CALIBRATION](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#TYPE_VEC3_CALIBRATION)
        pub const TYPE_VEC3_CALIBRATION : i32 = 65538;

        /// **get** public final [floatValues](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#floatValues)
        pub fn floatValues<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::FloatArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/hardware/SensorAdditionalInfo\0", "floatValues\0", "[F\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [intValues](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#intValues)
        pub fn intValues<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/hardware/SensorAdditionalInfo\0", "intValues\0", "[I\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [sensor](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#sensor)
        ///
        /// Required feature: android-hardware-Sensor
        #[cfg(any(feature = "all", feature = "android-hardware-Sensor"))]
        pub fn sensor<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::hardware::Sensor>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/hardware/SensorAdditionalInfo\0", "sensor\0", "Landroid/hardware/Sensor;\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [serial](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#serial)
        pub fn serial<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/hardware/SensorAdditionalInfo\0", "serial\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **get** public final [type](https://developer.android.com/reference/android/hardware/SensorAdditionalInfo.html#type)
        pub fn r#type<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/hardware/SensorAdditionalInfo\0", "type\0", "I\0");
                env.get_int_field(class, field)
            }
        }
    }
}
