// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-BluetoothHidDeviceAppQosSettings"))]
__jni_bindgen! {
    /// public final class [BluetoothHidDeviceAppQosSettings](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html)
    ///
    /// Required feature: android-bluetooth-BluetoothHidDeviceAppQosSettings
    public final class BluetoothHidDeviceAppQosSettings ("android/bluetooth/BluetoothHidDeviceAppQosSettings") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [BluetoothHidDeviceAppQosSettings](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#BluetoothHidDeviceAppQosSettings(int,%20int,%20int,%20int,%20int,%20int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::bluetooth::BluetoothHidDeviceAppQosSettings>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IIIIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "<init>\0", "(IIIIII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getServiceType](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#getServiceType())
        pub fn getServiceType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "getServiceType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "getServiceType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTokenRate](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#getTokenRate())
        pub fn getTokenRate<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "getTokenRate", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "getTokenRate\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTokenBucketSize](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#getTokenBucketSize())
        pub fn getTokenBucketSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "getTokenBucketSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "getTokenBucketSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPeakBandwidth](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#getPeakBandwidth())
        pub fn getPeakBandwidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "getPeakBandwidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "getPeakBandwidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLatency](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#getLatency())
        pub fn getLatency<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "getLatency", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "getLatency\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDelayVariation](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#getDelayVariation())
        pub fn getDelayVariation<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "getDelayVariation", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "getDelayVariation\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothHidDeviceAppQosSettings", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/bluetooth/BluetoothHidDeviceAppQosSettings\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [MAX](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#MAX)
        pub const MAX : i32 = -1;

        /// public static final [SERVICE_BEST_EFFORT](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#SERVICE_BEST_EFFORT)
        pub const SERVICE_BEST_EFFORT : i32 = 1;

        /// public static final [SERVICE_GUARANTEED](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#SERVICE_GUARANTEED)
        pub const SERVICE_GUARANTEED : i32 = 2;

        /// public static final [SERVICE_NO_TRAFFIC](https://developer.android.com/reference/android/bluetooth/BluetoothHidDeviceAppQosSettings.html#SERVICE_NO_TRAFFIC)
        pub const SERVICE_NO_TRAFFIC : i32 = 0;
    }
}
