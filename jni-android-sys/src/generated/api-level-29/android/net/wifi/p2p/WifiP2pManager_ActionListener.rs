// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-p2p-WifiP2pManager_ActionListener"))]
__jni_bindgen! {
    /// public interface [WifiP2pManager.ActionListener](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.ActionListener.html)
    ///
    /// Required feature: android-net-wifi-p2p-WifiP2pManager_ActionListener
    public interface WifiP2pManager_ActionListener ("android/net/wifi/p2p/WifiP2pManager$ActionListener") extends crate::java::lang::Object {

        /// [onSuccess](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.ActionListener.html#onSuccess())
        pub fn onSuccess<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pManager$ActionListener", java.flags == PUBLIC | ABSTRACT, .name == "onSuccess", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pManager$ActionListener\0", "onSuccess\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFailure](https://developer.android.com/reference/android/net/wifi/p2p/WifiP2pManager.ActionListener.html#onFailure(int))
        pub fn onFailure<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/p2p/WifiP2pManager$ActionListener", java.flags == PUBLIC | ABSTRACT, .name == "onFailure", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/p2p/WifiP2pManager$ActionListener\0", "onFailure\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
