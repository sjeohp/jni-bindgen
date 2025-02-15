// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-le-ScanSettings"))]
__jni_bindgen! {
    /// public final class [ScanSettings](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html)
    ///
    /// Required feature: android-bluetooth-le-ScanSettings
    public final class ScanSettings ("android/bluetooth/le/ScanSettings") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [ScanSettings](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#ScanSettings(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::bluetooth::le::ScanSettings>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/bluetooth/le/ScanSettings", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getScanMode](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#getScanMode())
        pub fn getScanMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "getScanMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "getScanMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCallbackType](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#getCallbackType())
        pub fn getCallbackType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "getCallbackType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "getCallbackType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getScanResultType](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#getScanResultType())
        pub fn getScanResultType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "getScanResultType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "getScanResultType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLegacy](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#getLegacy())
        pub fn getLegacy<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "getLegacy", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "getLegacy\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPhy](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#getPhy())
        pub fn getPhy<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "getPhy", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "getPhy\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReportDelayMillis](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#getReportDelayMillis())
        pub fn getReportDelayMillis<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "getReportDelayMillis", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "getReportDelayMillis\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/le/ScanSettings", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/le/ScanSettings\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CALLBACK_TYPE_ALL_MATCHES](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#CALLBACK_TYPE_ALL_MATCHES)
        pub const CALLBACK_TYPE_ALL_MATCHES : i32 = 1;

        /// public static final [CALLBACK_TYPE_FIRST_MATCH](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#CALLBACK_TYPE_FIRST_MATCH)
        pub const CALLBACK_TYPE_FIRST_MATCH : i32 = 2;

        /// public static final [CALLBACK_TYPE_MATCH_LOST](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#CALLBACK_TYPE_MATCH_LOST)
        pub const CALLBACK_TYPE_MATCH_LOST : i32 = 4;

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/bluetooth/le/ScanSettings\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [MATCH_MODE_AGGRESSIVE](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#MATCH_MODE_AGGRESSIVE)
        pub const MATCH_MODE_AGGRESSIVE : i32 = 1;

        /// public static final [MATCH_MODE_STICKY](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#MATCH_MODE_STICKY)
        pub const MATCH_MODE_STICKY : i32 = 2;

        /// public static final [MATCH_NUM_FEW_ADVERTISEMENT](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#MATCH_NUM_FEW_ADVERTISEMENT)
        pub const MATCH_NUM_FEW_ADVERTISEMENT : i32 = 2;

        /// public static final [MATCH_NUM_MAX_ADVERTISEMENT](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#MATCH_NUM_MAX_ADVERTISEMENT)
        pub const MATCH_NUM_MAX_ADVERTISEMENT : i32 = 3;

        /// public static final [MATCH_NUM_ONE_ADVERTISEMENT](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#MATCH_NUM_ONE_ADVERTISEMENT)
        pub const MATCH_NUM_ONE_ADVERTISEMENT : i32 = 1;

        /// public static final [PHY_LE_ALL_SUPPORTED](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#PHY_LE_ALL_SUPPORTED)
        pub const PHY_LE_ALL_SUPPORTED : i32 = 255;

        /// public static final [SCAN_MODE_BALANCED](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#SCAN_MODE_BALANCED)
        pub const SCAN_MODE_BALANCED : i32 = 1;

        /// public static final [SCAN_MODE_LOW_LATENCY](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#SCAN_MODE_LOW_LATENCY)
        pub const SCAN_MODE_LOW_LATENCY : i32 = 2;

        /// public static final [SCAN_MODE_LOW_POWER](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#SCAN_MODE_LOW_POWER)
        pub const SCAN_MODE_LOW_POWER : i32 = 0;

        /// public static final [SCAN_MODE_OPPORTUNISTIC](https://developer.android.com/reference/android/bluetooth/le/ScanSettings.html#SCAN_MODE_OPPORTUNISTIC)
        pub const SCAN_MODE_OPPORTUNISTIC : i32 = -1;
    }
}
