// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-BluetoothSocket"))]
__jni_bindgen! {
    /// public final class [BluetoothSocket](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html)
    ///
    /// Required feature: android-bluetooth-BluetoothSocket
    public final class BluetoothSocket ("android/bluetooth/BluetoothSocket") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        // // Not emitting: Non-public method
        // /// [BluetoothSocket](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#BluetoothSocket(android.bluetooth.BluetoothSocket))
        // ///
        // /// Required features: "android-bluetooth-BluetoothSocket"
        // #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothSocket")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::bluetooth::BluetoothSocket>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::bluetooth::BluetoothSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/bluetooth/BluetoothSocket", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/bluetooth/BluetoothSocket;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "<init>\0", "(Landroid/bluetooth/BluetoothSocket;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/bluetooth/BluetoothSocket", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getRemoteDevice](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#getRemoteDevice())
        ///
        /// Required features: "android-bluetooth-BluetoothDevice"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothDevice")))]
        pub fn getRemoteDevice<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::bluetooth::BluetoothDevice>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "getRemoteDevice", .descriptor == "()Landroid/bluetooth/BluetoothDevice;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "getRemoteDevice\0", "()Landroid/bluetooth/BluetoothDevice;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInputStream](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#getInputStream())
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn getInputStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::InputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "getInputStream", .descriptor == "()Ljava/io/InputStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "getInputStream\0", "()Ljava/io/InputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOutputStream](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#getOutputStream())
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn getOutputStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::OutputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "getOutputStream", .descriptor == "()Ljava/io/OutputStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "getOutputStream\0", "()Ljava/io/OutputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isConnected](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#isConnected())
        pub fn isConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "isConnected", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "isConnected\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#connect())
        pub fn connect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "connect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "connect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxTransmitPacketSize](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#getMaxTransmitPacketSize())
        pub fn getMaxTransmitPacketSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "getMaxTransmitPacketSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "getMaxTransmitPacketSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxReceivePacketSize](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#getMaxReceivePacketSize())
        pub fn getMaxReceivePacketSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "getMaxReceivePacketSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "getMaxReceivePacketSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConnectionType](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#getConnectionType())
        pub fn getConnectionType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothSocket", java.flags == PUBLIC, .name == "getConnectionType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothSocket\0", "getConnectionType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [TYPE_L2CAP](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#TYPE_L2CAP)
        pub const TYPE_L2CAP : i32 = 3;

        /// public static final [TYPE_RFCOMM](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#TYPE_RFCOMM)
        pub const TYPE_RFCOMM : i32 = 1;

        /// public static final [TYPE_SCO](https://developer.android.com/reference/android/bluetooth/BluetoothSocket.html#TYPE_SCO)
        pub const TYPE_SCO : i32 = 2;
    }
}
