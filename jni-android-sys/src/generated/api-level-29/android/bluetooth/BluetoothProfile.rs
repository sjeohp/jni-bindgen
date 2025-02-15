// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-BluetoothProfile"))]
__jni_bindgen! {
    /// public interface [BluetoothProfile](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html)
    ///
    /// Required feature: android-bluetooth-BluetoothProfile
    public interface BluetoothProfile ("android/bluetooth/BluetoothProfile") extends crate::java::lang::Object {

        /// [getConnectedDevices](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#getConnectedDevices())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getConnectedDevices<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothProfile", java.flags == PUBLIC | ABSTRACT, .name == "getConnectedDevices", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothProfile\0", "getConnectedDevices\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDevicesMatchingConnectionStates](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#getDevicesMatchingConnectionStates(int%5B%5D))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getDevicesMatchingConnectionStates<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothProfile", java.flags == PUBLIC | ABSTRACT, .name == "getDevicesMatchingConnectionStates", .descriptor == "([I)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothProfile\0", "getDevicesMatchingConnectionStates\0", "([I)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConnectionState](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#getConnectionState(android.bluetooth.BluetoothDevice))
        ///
        /// Required features: "android-bluetooth-BluetoothDevice"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothDevice")))]
        pub fn getConnectionState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::bluetooth::BluetoothDevice>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothProfile", java.flags == PUBLIC | ABSTRACT, .name == "getConnectionState", .descriptor == "(Landroid/bluetooth/BluetoothDevice;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothProfile\0", "getConnectionState\0", "(Landroid/bluetooth/BluetoothDevice;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [A2DP](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#A2DP)
        pub const A2DP : i32 = 2;

        /// public static final [EXTRA_PREVIOUS_STATE](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#EXTRA_PREVIOUS_STATE)
        pub const EXTRA_PREVIOUS_STATE : &'static str = "android.bluetooth.profile.extra.PREVIOUS_STATE";

        /// public static final [EXTRA_STATE](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#EXTRA_STATE)
        pub const EXTRA_STATE : &'static str = "android.bluetooth.profile.extra.STATE";

        /// public static final [GATT](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#GATT)
        pub const GATT : i32 = 7;

        /// public static final [GATT_SERVER](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#GATT_SERVER)
        pub const GATT_SERVER : i32 = 8;

        /// public static final [HEADSET](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#HEADSET)
        pub const HEADSET : i32 = 1;

        /// public static final [HEALTH](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#HEALTH)
        #[deprecated] pub const HEALTH : i32 = 3;

        /// public static final [HEARING_AID](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#HEARING_AID)
        pub const HEARING_AID : i32 = 21;

        /// public static final [HID_DEVICE](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#HID_DEVICE)
        pub const HID_DEVICE : i32 = 19;

        /// public static final [SAP](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#SAP)
        pub const SAP : i32 = 10;

        /// public static final [STATE_CONNECTED](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#STATE_CONNECTED)
        pub const STATE_CONNECTED : i32 = 2;

        /// public static final [STATE_CONNECTING](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#STATE_CONNECTING)
        pub const STATE_CONNECTING : i32 = 1;

        /// public static final [STATE_DISCONNECTED](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#STATE_DISCONNECTED)
        pub const STATE_DISCONNECTED : i32 = 0;

        /// public static final [STATE_DISCONNECTING](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.html#STATE_DISCONNECTING)
        pub const STATE_DISCONNECTING : i32 = 3;
    }
}
