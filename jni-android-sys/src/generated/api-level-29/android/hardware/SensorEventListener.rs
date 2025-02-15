// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-SensorEventListener"))]
__jni_bindgen! {
    /// public interface [SensorEventListener](https://developer.android.com/reference/android/hardware/SensorEventListener.html)
    ///
    /// Required feature: android-hardware-SensorEventListener
    public interface SensorEventListener ("android/hardware/SensorEventListener") extends crate::java::lang::Object {

        /// [onSensorChanged](https://developer.android.com/reference/android/hardware/SensorEventListener.html#onSensorChanged(android.hardware.SensorEvent))
        ///
        /// Required features: "android-hardware-SensorEvent"
        #[cfg(any(feature = "all", all(feature = "android-hardware-SensorEvent")))]
        pub fn onSensorChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::SensorEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/SensorEventListener", java.flags == PUBLIC | ABSTRACT, .name == "onSensorChanged", .descriptor == "(Landroid/hardware/SensorEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/SensorEventListener\0", "onSensorChanged\0", "(Landroid/hardware/SensorEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAccuracyChanged](https://developer.android.com/reference/android/hardware/SensorEventListener.html#onAccuracyChanged(android.hardware.Sensor,%20int))
        ///
        /// Required features: "android-hardware-Sensor"
        #[cfg(any(feature = "all", all(feature = "android-hardware-Sensor")))]
        pub fn onAccuracyChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::Sensor>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/SensorEventListener", java.flags == PUBLIC | ABSTRACT, .name == "onAccuracyChanged", .descriptor == "(Landroid/hardware/Sensor;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/SensorEventListener\0", "onAccuracyChanged\0", "(Landroid/hardware/Sensor;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
