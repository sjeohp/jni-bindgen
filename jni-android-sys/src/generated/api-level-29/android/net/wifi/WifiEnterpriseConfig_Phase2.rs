// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-WifiEnterpriseConfig_Phase2"))]
__jni_bindgen! {
    /// public final class [WifiEnterpriseConfig.Phase2](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html)
    ///
    /// Required feature: android-net-wifi-WifiEnterpriseConfig_Phase2
    public final class WifiEnterpriseConfig_Phase2 ("android/net/wifi/WifiEnterpriseConfig$Phase2") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Phase2](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#Phase2())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::wifi::WifiEnterpriseConfig_Phase2>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/wifi/WifiEnterpriseConfig$Phase2", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/WifiEnterpriseConfig$Phase2\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [AKA](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#AKA)
        pub const AKA : i32 = 6;

        /// public static final [AKA_PRIME](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#AKA_PRIME)
        pub const AKA_PRIME : i32 = 7;

        /// public static final [GTC](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#GTC)
        pub const GTC : i32 = 4;

        /// public static final [MSCHAP](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#MSCHAP)
        pub const MSCHAP : i32 = 2;

        /// public static final [MSCHAPV2](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#MSCHAPV2)
        pub const MSCHAPV2 : i32 = 3;

        /// public static final [NONE](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#NONE)
        pub const NONE : i32 = 0;

        /// public static final [PAP](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#PAP)
        pub const PAP : i32 = 1;

        /// public static final [SIM](https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html#SIM)
        pub const SIM : i32 = 5;
    }
}
