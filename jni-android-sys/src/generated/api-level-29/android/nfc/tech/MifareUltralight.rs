// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-nfc-tech-MifareUltralight"))]
__jni_bindgen! {
    /// public final class [MifareUltralight](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html)
    ///
    /// Required feature: android-nfc-tech-MifareUltralight
    public final class MifareUltralight ("android/nfc/tech/MifareUltralight") extends crate::java::lang::Object, implements crate::android::nfc::tech::TagTechnology {

        // // Not emitting: Non-public method
        // /// [MifareUltralight](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#MifareUltralight(android.nfc.Tag))
        // ///
        // /// Required features: "android-nfc-Tag"
        // #[cfg(any(feature = "all", all(feature = "android-nfc-Tag")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::Tag>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::nfc::tech::MifareUltralight>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/nfc/tech/MifareUltralight", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/nfc/Tag;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "<init>\0", "(Landroid/nfc/Tag;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [get](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#get(android.nfc.Tag))
        ///
        /// Required features: "android-nfc-Tag", "android-nfc-tech-MifareUltralight"
        #[cfg(any(feature = "all", all(feature = "android-nfc-Tag", feature = "android-nfc-tech-MifareUltralight")))]
        pub fn get<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::Tag>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::nfc::tech::MifareUltralight>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC | STATIC, .name == "get", .descriptor == "(Landroid/nfc/Tag;)Landroid/nfc/tech/MifareUltralight;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/nfc/tech/MifareUltralight\0", "get\0", "(Landroid/nfc/Tag;)Landroid/nfc/tech/MifareUltralight;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getType](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#getType())
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "getType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "getType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readPages](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#readPages(int))
        pub fn readPages<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "readPages", .descriptor == "(I)[B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "readPages\0", "(I)[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writePage](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#writePage(int,%20byte%5B%5D))
        pub fn writePage<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "writePage", .descriptor == "(I[B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "writePage\0", "(I[B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [transceive](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#transceive(byte%5B%5D))
        pub fn transceive<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "transceive", .descriptor == "([B)[B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "transceive\0", "([B)[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxTransceiveLength](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#getMaxTransceiveLength())
        pub fn getMaxTransceiveLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "getMaxTransceiveLength", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "getMaxTransceiveLength\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeout](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#setTimeout(int))
        pub fn setTimeout<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "setTimeout", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "setTimeout\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeout](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#getTimeout())
        pub fn getTimeout<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "getTimeout", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "getTimeout\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isConnected](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#isConnected())
        pub fn isConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "isConnected", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "isConnected\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTag](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#getTag())
        ///
        /// Required features: "android-nfc-Tag"
        #[cfg(any(feature = "all", all(feature = "android-nfc-Tag")))]
        pub fn getTag<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::nfc::Tag>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "getTag", .descriptor == "()Landroid/nfc/Tag;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "getTag\0", "()Landroid/nfc/Tag;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#connect())
        pub fn connect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/tech/MifareUltralight", java.flags == PUBLIC, .name == "connect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/tech/MifareUltralight\0", "connect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [PAGE_SIZE](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#PAGE_SIZE)
        pub const PAGE_SIZE : i32 = 4;

        /// public static final [TYPE_ULTRALIGHT](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#TYPE_ULTRALIGHT)
        pub const TYPE_ULTRALIGHT : i32 = 1;

        /// public static final [TYPE_ULTRALIGHT_C](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#TYPE_ULTRALIGHT_C)
        pub const TYPE_ULTRALIGHT_C : i32 = 2;

        /// public static final [TYPE_UNKNOWN](https://developer.android.com/reference/android/nfc/tech/MifareUltralight.html#TYPE_UNKNOWN)
        pub const TYPE_UNKNOWN : i32 = -1;
    }
}
