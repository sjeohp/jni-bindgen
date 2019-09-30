// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-rtt-RangingResult"))]
__jni_bindgen! {
    /// public final class [RangingResult](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html)
    ///
    /// Required feature: android-net-wifi-rtt-RangingResult
    public final class RangingResult ("android/net/wifi/rtt/RangingResult") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [RangingResult](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#RangingResult(int,%20android.net.MacAddress,%20int,%20int,%20int,%20int,%20int,%20byte%5B%5D,%20byte%5B%5D,%20android.net.wifi.rtt.ResponderLocation,%20long))
        // ///
        // /// Required features: "android-net-MacAddress", "android-net-wifi-rtt-ResponderLocation"
        // #[cfg(any(feature = "all", all(feature = "android-net-MacAddress", feature = "android-net-wifi-rtt-ResponderLocation")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::MacAddress>>, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg8: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg9: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::wifi::rtt::ResponderLocation>>, arg10: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::wifi::rtt::RangingResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/wifi/rtt/RangingResult", java.flags == (empty), .name == "<init>", .descriptor == "(ILandroid/net/MacAddress;IIIII[B[BLandroid/net/wifi/rtt/ResponderLocation;J)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6), __jni_bindgen::AsJValue::as_jvalue(&arg7.into()), __jni_bindgen::AsJValue::as_jvalue(&arg8.into()), __jni_bindgen::AsJValue::as_jvalue(&arg9.into()), __jni_bindgen::AsJValue::as_jvalue(&arg10)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "<init>\0", "(ILandroid/net/MacAddress;IIIII[B[BLandroid/net/wifi/rtt/ResponderLocation;J)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getStatus](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getStatus())
        pub fn getStatus<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getStatus", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getStatus\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMacAddress](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getMacAddress())
        ///
        /// Required features: "android-net-MacAddress"
        #[cfg(any(feature = "all", all(feature = "android-net-MacAddress")))]
        pub fn getMacAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::MacAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getMacAddress", .descriptor == "()Landroid/net/MacAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getMacAddress\0", "()Landroid/net/MacAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPeerHandle](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getPeerHandle())
        ///
        /// Required features: "android-net-wifi-aware-PeerHandle"
        #[cfg(any(feature = "all", all(feature = "android-net-wifi-aware-PeerHandle")))]
        pub fn getPeerHandle<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::wifi::aware::PeerHandle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getPeerHandle", .descriptor == "()Landroid/net/wifi/aware/PeerHandle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getPeerHandle\0", "()Landroid/net/wifi/aware/PeerHandle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDistanceMm](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getDistanceMm())
        pub fn getDistanceMm<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getDistanceMm", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getDistanceMm\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDistanceStdDevMm](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getDistanceStdDevMm())
        pub fn getDistanceStdDevMm<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getDistanceStdDevMm", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getDistanceStdDevMm\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRssi](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getRssi())
        pub fn getRssi<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getRssi", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getRssi\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumAttemptedMeasurements](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getNumAttemptedMeasurements())
        pub fn getNumAttemptedMeasurements<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getNumAttemptedMeasurements", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getNumAttemptedMeasurements\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumSuccessfulMeasurements](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getNumSuccessfulMeasurements())
        pub fn getNumSuccessfulMeasurements<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getNumSuccessfulMeasurements", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getNumSuccessfulMeasurements\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUnverifiedResponderLocation](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getUnverifiedResponderLocation())
        ///
        /// Required features: "android-net-wifi-rtt-ResponderLocation"
        #[cfg(any(feature = "all", all(feature = "android-net-wifi-rtt-ResponderLocation")))]
        pub fn getUnverifiedResponderLocation<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::wifi::rtt::ResponderLocation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getUnverifiedResponderLocation", .descriptor == "()Landroid/net/wifi/rtt/ResponderLocation;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getUnverifiedResponderLocation\0", "()Landroid/net/wifi/rtt/ResponderLocation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRangingTimestampMillis](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#getRangingTimestampMillis())
        pub fn getRangingTimestampMillis<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "getRangingTimestampMillis", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "getRangingTimestampMillis\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResult", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResult\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/net/wifi/rtt/RangingResult\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [STATUS_FAIL](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#STATUS_FAIL)
        pub const STATUS_FAIL : i32 = 1;

        /// public static final [STATUS_RESPONDER_DOES_NOT_SUPPORT_IEEE80211MC](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#STATUS_RESPONDER_DOES_NOT_SUPPORT_IEEE80211MC)
        pub const STATUS_RESPONDER_DOES_NOT_SUPPORT_IEEE80211MC : i32 = 2;

        /// public static final [STATUS_SUCCESS](https://developer.android.com/reference/android/net/wifi/rtt/RangingResult.html#STATUS_SUCCESS)
        pub const STATUS_SUCCESS : i32 = 0;
    }
}
