// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-nfc-tech-NfcB"))]
__jni_bindgen! {
    /// public final class [NfcB](https://developer.android.com/reference/android/nfc/tech/NfcB.html)
    ///
    /// Required feature: android-nfc-tech-NfcB
    public final class NfcB ("android/nfc/tech/NfcB") extends crate::java::lang::Object, implements crate::android::nfc::tech::TagTechnology {

        // // Not emitting: Non-public method
        // /// [NfcB](https://developer.android.com/reference/android/nfc/tech/NfcB.html#NfcB(android.nfc.Tag))
        // ///
        // /// Required features: "android-nfc-Tag"
        // #[cfg(any(feature = "all", all(feature = "android-nfc-Tag")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::Tag>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::nfc::tech::NfcB>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/nfc/tech/NfcB", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/nfc/Tag;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "<init>\0", "(Landroid/nfc/Tag;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [get](https://developer.android.com/reference/android/nfc/tech/NfcB.html#get(android.nfc.Tag))
        ///
        /// Required features: "android-nfc-Tag", "android-nfc-tech-NfcB"
        #[cfg(any(feature = "all", all(feature = "android-nfc-Tag", feature = "android-nfc-tech-NfcB")))]
        pub fn get<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::Tag>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::nfc::tech::NfcB>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC | STATIC, .name == "get", .descriptor == "(Landroid/nfc/Tag;)Landroid/nfc/tech/NfcB;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/nfc/tech/NfcB\0", "get\0", "(Landroid/nfc/Tag;)Landroid/nfc/tech/NfcB;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getApplicationData](https://developer.android.com/reference/android/nfc/tech/NfcB.html#getApplicationData())
        pub fn getApplicationData<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "getApplicationData", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "getApplicationData\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProtocolInfo](https://developer.android.com/reference/android/nfc/tech/NfcB.html#getProtocolInfo())
        pub fn getProtocolInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "getProtocolInfo", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "getProtocolInfo\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [transceive](https://developer.android.com/reference/android/nfc/tech/NfcB.html#transceive(byte%5B%5D))
        pub fn transceive<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "transceive", .descriptor == "([B)[B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "transceive\0", "([B)[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxTransceiveLength](https://developer.android.com/reference/android/nfc/tech/NfcB.html#getMaxTransceiveLength())
        pub fn getMaxTransceiveLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "getMaxTransceiveLength", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "getMaxTransceiveLength\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isConnected](https://developer.android.com/reference/android/nfc/tech/NfcB.html#isConnected())
        pub fn isConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "isConnected", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "isConnected\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTag](https://developer.android.com/reference/android/nfc/tech/NfcB.html#getTag())
        ///
        /// Required features: "android-nfc-Tag"
        #[cfg(any(feature = "all", all(feature = "android-nfc-Tag")))]
        pub fn getTag<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::nfc::Tag>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "getTag", .descriptor == "()Landroid/nfc/Tag;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "getTag\0", "()Landroid/nfc/Tag;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/nfc/tech/NfcB.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/android/nfc/tech/NfcB.html#connect())
        pub fn connect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/NfcB", java.flags == PUBLIC, .name == "connect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/NfcB\0", "connect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
