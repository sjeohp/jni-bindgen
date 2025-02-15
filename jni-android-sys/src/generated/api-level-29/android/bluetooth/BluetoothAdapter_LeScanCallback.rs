// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-BluetoothAdapter_LeScanCallback"))]
__jni_bindgen! {
    /// public interface [BluetoothAdapter.LeScanCallback](https://developer.android.com/reference/android/bluetooth/BluetoothAdapter.LeScanCallback.html)
    ///
    /// Required feature: android-bluetooth-BluetoothAdapter_LeScanCallback
    public interface BluetoothAdapter_LeScanCallback ("android/bluetooth/BluetoothAdapter$LeScanCallback") extends crate::java::lang::Object {

        /// [onLeScan](https://developer.android.com/reference/android/bluetooth/BluetoothAdapter.LeScanCallback.html#onLeScan(android.bluetooth.BluetoothDevice,%20int,%20byte%5B%5D))
        ///
        /// Required features: "android-bluetooth-BluetoothDevice"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothDevice")))]
        pub fn onLeScan<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::bluetooth::BluetoothDevice>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothAdapter$LeScanCallback", java.flags == PUBLIC | ABSTRACT, .name == "onLeScan", .descriptor == "(Landroid/bluetooth/BluetoothDevice;I[B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothAdapter$LeScanCallback\0", "onLeScan\0", "(Landroid/bluetooth/BluetoothDevice;I[B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
