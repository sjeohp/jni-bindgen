// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-p2p-WifiP2pManager_NetworkInfoListener"))]
__jni_bindgen! {
    /// public interface [WifiP2pManager.NetworkInfoListener](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.NetworkInfoListener.html)
    ///
    /// Required feature: android-net-wifi-p2p-WifiP2pManager_NetworkInfoListener
    public interface WifiP2pManager_NetworkInfoListener ("android/net/wifi/p2p/WifiP2pManager$NetworkInfoListener") extends crate::java::lang::Object {

        /// [onNetworkInfoAvailable](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.NetworkInfoListener.html#onNetworkInfoAvailable(android.net.NetworkInfo))
        ///
        /// Required features: "android-net-NetworkInfo"
        #[cfg(any(feature = "all", all(feature = "android-net-NetworkInfo")))]
        pub fn onNetworkInfoAvailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::NetworkInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pManager$NetworkInfoListener", java.flags == PUBLIC | ABSTRACT, .name == "onNetworkInfoAvailable", .descriptor == "(Landroid/net/NetworkInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pManager$NetworkInfoListener\0", "onNetworkInfoAvailable\0", "(Landroid/net/NetworkInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
