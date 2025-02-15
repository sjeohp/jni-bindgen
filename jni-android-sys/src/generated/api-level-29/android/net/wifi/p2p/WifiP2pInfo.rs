// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-p2p-WifiP2pInfo"))]
__jni_bindgen! {
    /// public class [WifiP2pInfo](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html)
    ///
    /// Required feature: android-net-wifi-p2p-WifiP2pInfo
    public class WifiP2pInfo ("android/net/wifi/p2p/WifiP2pInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [WifiP2pInfo](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#WifiP2pInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::wifi::p2p::WifiP2pInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [WifiP2pInfo](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#WifiP2pInfo(android.net.wifi.p2p.WifiP2pInfo))
        ///
        /// Required features: "android-net-wifi-p2p-WifiP2pInfo"
        #[cfg(any(feature = "all", all(feature = "android-net-wifi-p2p-WifiP2pInfo")))]
        pub fn new_WifiP2pInfo<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::wifi::p2p::WifiP2pInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::wifi::p2p::WifiP2pInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/net/wifi/p2p/WifiP2pInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pInfo\0", "<init>\0", "(Landroid/net/wifi/p2p/WifiP2pInfo;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/net/wifi/p2p/WifiP2pInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public [groupFormed](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#groupFormed)
        pub fn groupFormed<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/p2p/WifiP2pInfo\0", "groupFormed\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [groupFormed](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#groupFormed)
        pub fn set_groupFormed<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/p2p/WifiP2pInfo\0", "groupFormed\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [groupOwnerAddress](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#groupOwnerAddress)
        ///
        /// Required feature: java-net-InetAddress
        #[cfg(any(feature = "all", feature = "java-net-InetAddress"))]
        pub fn groupOwnerAddress<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/p2p/WifiP2pInfo\0", "groupOwnerAddress\0", "Ljava/net/InetAddress;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [groupOwnerAddress](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#groupOwnerAddress)
        ///
        /// Required feature: java-net-InetAddress
        #[cfg(any(feature = "all", feature = "java-net-InetAddress"))]
        pub fn set_groupOwnerAddress<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::net::InetAddress>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/p2p/WifiP2pInfo\0", "groupOwnerAddress\0", "Ljava/net/InetAddress;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [isGroupOwner](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#isGroupOwner)
        pub fn isGroupOwner<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/p2p/WifiP2pInfo\0", "isGroupOwner\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [isGroupOwner](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pInfo.html#isGroupOwner)
        pub fn set_isGroupOwner<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/net/wifi/p2p/WifiP2pInfo\0", "isGroupOwner\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }
    }
}
