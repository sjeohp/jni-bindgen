// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-p2p-WifiP2pManager_P2pStateListener"))]
__jni_bindgen! {
    /// public interface [WifiP2pManager.P2pStateListener](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.P2pStateListener.html)
    ///
    /// Required feature: android-net-wifi-p2p-WifiP2pManager_P2pStateListener
    public interface WifiP2pManager_P2pStateListener ("android/net/wifi/p2p/WifiP2pManager$P2pStateListener") extends crate::java::lang::Object {

        /// [onP2pStateAvailable](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.P2pStateListener.html#onP2pStateAvailable(int))
        pub fn onP2pStateAvailable<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pManager$P2pStateListener", java.flags == PUBLIC | ABSTRACT, .name == "onP2pStateAvailable", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pManager$P2pStateListener\0", "onP2pStateAvailable\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
