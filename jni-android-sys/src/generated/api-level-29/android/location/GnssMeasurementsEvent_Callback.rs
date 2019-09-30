// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-location-GnssMeasurementsEvent_Callback"))]
__jni_bindgen! {
    /// public class [GnssMeasurementsEvent.Callback](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html)
    ///
    /// Required feature: android-location-GnssMeasurementsEvent_Callback
    public class GnssMeasurementsEvent_Callback ("android/location/GnssMeasurementsEvent$Callback") extends crate::java::lang::Object {

        /// [Callback](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#Callback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::location::GnssMeasurementsEvent_Callback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurementsEvent$Callback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurementsEvent$Callback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGnssMeasurementsReceived](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#onGnssMeasurementsReceived(android.location.GnssMeasurementsEvent))
        ///
        /// Required features: "android-location-GnssMeasurementsEvent"
        #[cfg(any(feature = "all", all(feature = "android-location-GnssMeasurementsEvent")))]
        pub fn onGnssMeasurementsReceived<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::location::GnssMeasurementsEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurementsEvent$Callback", java.flags == PUBLIC, .name == "onGnssMeasurementsReceived", .descriptor == "(Landroid/location/GnssMeasurementsEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurementsEvent$Callback\0", "onGnssMeasurementsReceived\0", "(Landroid/location/GnssMeasurementsEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStatusChanged](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#onStatusChanged(int))
        pub fn onStatusChanged<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurementsEvent$Callback", java.flags == PUBLIC, .name == "onStatusChanged", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurementsEvent$Callback\0", "onStatusChanged\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [STATUS_LOCATION_DISABLED](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#STATUS_LOCATION_DISABLED)
        pub const STATUS_LOCATION_DISABLED : i32 = 2;

        /// public static final [STATUS_NOT_ALLOWED](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#STATUS_NOT_ALLOWED)
        pub const STATUS_NOT_ALLOWED : i32 = 3;

        /// public static final [STATUS_NOT_SUPPORTED](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#STATUS_NOT_SUPPORTED)
        pub const STATUS_NOT_SUPPORTED : i32 = 0;

        /// public static final [STATUS_READY](https://developer.android.com/reference/android/location/GnssMeasurementsEvent.Callback.html#STATUS_READY)
        pub const STATUS_READY : i32 = 1;
    }
}
