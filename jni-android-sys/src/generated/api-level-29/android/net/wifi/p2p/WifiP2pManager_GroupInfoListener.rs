// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-p2p-WifiP2pManager_GroupInfoListener"))]
__jni_bindgen! {
    /// public interface [WifiP2pManager.GroupInfoListener](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.GroupInfoListener.html)
    ///
    /// Required feature: android-net-wifi-p2p-WifiP2pManager_GroupInfoListener
    public interface WifiP2pManager_GroupInfoListener ("android/net/wifi/p2p/WifiP2pManager$GroupInfoListener") extends crate::java::lang::Object {

        /// [onGroupInfoAvailable](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.GroupInfoListener.html#onGroupInfoAvailable(android.net.wifi.p2p.WifiP2pGroup))
        ///
        /// Required features: "android-net-wifi-p2p-WifiP2pGroup"
        #[cfg(any(feature = "all", all(feature = "android-net-wifi-p2p-WifiP2pGroup")))]
        pub fn onGroupInfoAvailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::wifi::p2p::WifiP2pGroup>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pManager$GroupInfoListener", java.flags == PUBLIC | ABSTRACT, .name == "onGroupInfoAvailable", .descriptor == "(Landroid/net/wifi/p2p/WifiP2pGroup;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pManager$GroupInfoListener\0", "onGroupInfoAvailable\0", "(Landroid/net/wifi/p2p/WifiP2pGroup;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
