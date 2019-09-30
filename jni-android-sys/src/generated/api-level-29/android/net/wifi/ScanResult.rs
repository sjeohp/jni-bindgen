// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-ScanResult"))]
__jni_bindgen! {
    /// public class [ScanResult](https://developer.android.com/reference/android/net/wifi/ScanResult.html)
    ///
    /// Required feature: android-net-wifi-ScanResult
    public class ScanResult ("android/net/wifi/ScanResult") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [ScanResult](https://developer.android.com/reference/android/net/wifi/ScanResult.html#ScanResult())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::wifi::ScanResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/wifi/ScanResult", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/ScanResult\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [is80211mcResponder](https://developer.android.com/reference/android/net/wifi/ScanResult.html#is80211mcResponder())
        pub fn is80211mcResponder<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/ScanResult", java.flags == PUBLIC, .name == "is80211mcResponder", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/ScanResult\0", "is80211mcResponder\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isPasspointNetwork](https://developer.android.com/reference/android/net/wifi/ScanResult.html#isPasspointNetwork())
        pub fn isPasspointNetwork<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/ScanResult", java.flags == PUBLIC, .name == "isPasspointNetwork", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/ScanResult\0", "isPasspointNetwork\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/net/wifi/ScanResult.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/ScanResult", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/ScanResult\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/net/wifi/ScanResult.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/ScanResult", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/ScanResult\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/net/wifi/ScanResult.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/ScanResult", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/ScanResult\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [BSSID](https://developer.android.com/reference/android/net/wifi/ScanResult.html#BSSID)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn BSSID<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "BSSID\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [BSSID](https://developer.android.com/reference/android/net/wifi/ScanResult.html#BSSID)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_BSSID<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "BSSID\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// public static final [CHANNEL_WIDTH_160MHZ](https://developer.android.com/reference/android/net/wifi/ScanResult.html#CHANNEL_WIDTH_160MHZ)
        pub const CHANNEL_WIDTH_160MHZ : i32 = 3;

        /// public static final [CHANNEL_WIDTH_20MHZ](https://developer.android.com/reference/android/net/wifi/ScanResult.html#CHANNEL_WIDTH_20MHZ)
        pub const CHANNEL_WIDTH_20MHZ : i32 = 0;

        /// public static final [CHANNEL_WIDTH_40MHZ](https://developer.android.com/reference/android/net/wifi/ScanResult.html#CHANNEL_WIDTH_40MHZ)
        pub const CHANNEL_WIDTH_40MHZ : i32 = 1;

        /// public static final [CHANNEL_WIDTH_80MHZ](https://developer.android.com/reference/android/net/wifi/ScanResult.html#CHANNEL_WIDTH_80MHZ)
        pub const CHANNEL_WIDTH_80MHZ : i32 = 2;

        /// public static final [CHANNEL_WIDTH_80MHZ_PLUS_MHZ](https://developer.android.com/reference/android/net/wifi/ScanResult.html#CHANNEL_WIDTH_80MHZ_PLUS_MHZ)
        pub const CHANNEL_WIDTH_80MHZ_PLUS_MHZ : i32 = 4;

        /// **get** public [SSID](https://developer.android.com/reference/android/net/wifi/ScanResult.html#SSID)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn SSID<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "SSID\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [SSID](https://developer.android.com/reference/android/net/wifi/ScanResult.html#SSID)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_SSID<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "SSID\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [capabilities](https://developer.android.com/reference/android/net/wifi/ScanResult.html#capabilities)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn capabilities<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "capabilities\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [capabilities](https://developer.android.com/reference/android/net/wifi/ScanResult.html#capabilities)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_capabilities<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "capabilities\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [centerFreq0](https://developer.android.com/reference/android/net/wifi/ScanResult.html#centerFreq0)
        pub fn centerFreq0<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "centerFreq0\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [centerFreq0](https://developer.android.com/reference/android/net/wifi/ScanResult.html#centerFreq0)
        pub fn set_centerFreq0<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "centerFreq0\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [centerFreq1](https://developer.android.com/reference/android/net/wifi/ScanResult.html#centerFreq1)
        pub fn centerFreq1<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "centerFreq1\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [centerFreq1](https://developer.android.com/reference/android/net/wifi/ScanResult.html#centerFreq1)
        pub fn set_centerFreq1<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "centerFreq1\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [channelWidth](https://developer.android.com/reference/android/net/wifi/ScanResult.html#channelWidth)
        pub fn channelWidth<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "channelWidth\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [channelWidth](https://developer.android.com/reference/android/net/wifi/ScanResult.html#channelWidth)
        pub fn set_channelWidth<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "channelWidth\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [frequency](https://developer.android.com/reference/android/net/wifi/ScanResult.html#frequency)
        pub fn frequency<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "frequency\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [frequency](https://developer.android.com/reference/android/net/wifi/ScanResult.html#frequency)
        pub fn set_frequency<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "frequency\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [level](https://developer.android.com/reference/android/net/wifi/ScanResult.html#level)
        pub fn level<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "level\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [level](https://developer.android.com/reference/android/net/wifi/ScanResult.html#level)
        pub fn set_level<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "level\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [operatorFriendlyName](https://developer.android.com/reference/android/net/wifi/ScanResult.html#operatorFriendlyName)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn operatorFriendlyName<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "operatorFriendlyName\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [operatorFriendlyName](https://developer.android.com/reference/android/net/wifi/ScanResult.html#operatorFriendlyName)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn set_operatorFriendlyName<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "operatorFriendlyName\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [timestamp](https://developer.android.com/reference/android/net/wifi/ScanResult.html#timestamp)
        pub fn timestamp<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "timestamp\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [timestamp](https://developer.android.com/reference/android/net/wifi/ScanResult.html#timestamp)
        pub fn set_timestamp<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "timestamp\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [venueName](https://developer.android.com/reference/android/net/wifi/ScanResult.html#venueName)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn venueName<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "venueName\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [venueName](https://developer.android.com/reference/android/net/wifi/ScanResult.html#venueName)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn set_venueName<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/ScanResult\0", "venueName\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }
    }
}
