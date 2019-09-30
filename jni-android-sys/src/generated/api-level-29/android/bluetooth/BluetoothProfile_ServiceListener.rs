// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-BluetoothProfile_ServiceListener"))]
__jni_bindgen! {
    /// public interface [BluetoothProfile.ServiceListener](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.ServiceListener.html)
    ///
    /// Required feature: android-bluetooth-BluetoothProfile_ServiceListener
    public interface BluetoothProfile_ServiceListener ("android/bluetooth/BluetoothProfile$ServiceListener") extends crate::java::lang::Object {

        /// [onServiceConnected](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.ServiceListener.html#onServiceConnected(int,%20android.bluetooth.BluetoothProfile))
        ///
        /// Required features: "android-bluetooth-BluetoothProfile"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothProfile")))]
        pub fn onServiceConnected<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::bluetooth::BluetoothProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothProfile$ServiceListener", java.flags == PUBLIC | ABSTRACT, .name == "onServiceConnected", .descriptor == "(ILandroid/bluetooth/BluetoothProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothProfile$ServiceListener\0", "onServiceConnected\0", "(ILandroid/bluetooth/BluetoothProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onServiceDisconnected](https://developer.android.com/reference/android/bluetooth/BluetoothProfile.ServiceListener.html#onServiceDisconnected(int))
        pub fn onServiceDisconnected<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothProfile$ServiceListener", java.flags == PUBLIC | ABSTRACT, .name == "onServiceDisconnected", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothProfile$ServiceListener\0", "onServiceDisconnected\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
