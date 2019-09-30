// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaCodec_CryptoInfo"))]
__jni_bindgen! {
    /// public final class [MediaCodec.CryptoInfo](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html)
    ///
    /// Required feature: android-media-MediaCodec_CryptoInfo
    public final class MediaCodec_CryptoInfo ("android/media/MediaCodec$CryptoInfo") extends crate::java::lang::Object {

        /// [CryptoInfo](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#CryptoInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaCodec_CryptoInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#set(int,%20int%5B%5D,%20int%5B%5D,%20byte%5B%5D,%20byte%5B%5D,%20int))
        pub fn set<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg5: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo", java.flags == PUBLIC, .name == "set", .descriptor == "(I[I[I[B[BI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo\0", "set\0", "(I[I[I[B[BI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPattern](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#setPattern(android.media.MediaCodec.CryptoInfo.Pattern))
        ///
        /// Required features: "android-media-MediaCodec_CryptoInfo_Pattern"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaCodec_CryptoInfo_Pattern")))]
        pub fn setPattern<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaCodec_CryptoInfo_Pattern>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo", java.flags == PUBLIC, .name == "setPattern", .descriptor == "(Landroid/media/MediaCodec$CryptoInfo$Pattern;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo\0", "setPattern\0", "(Landroid/media/MediaCodec$CryptoInfo$Pattern;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$CryptoInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$CryptoInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [iv](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#iv)
        pub fn iv<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "iv\0", "[B\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [iv](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#iv)
        pub fn set_iv<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::ByteArray>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "iv\0", "[B\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [key](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#key)
        pub fn key<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "key\0", "[B\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [key](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#key)
        pub fn set_key<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::ByteArray>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "key\0", "[B\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [mode](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#mode)
        pub fn mode<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "mode\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [mode](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#mode)
        pub fn set_mode<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "mode\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [numBytesOfClearData](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#numBytesOfClearData)
        pub fn numBytesOfClearData<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "numBytesOfClearData\0", "[I\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [numBytesOfClearData](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#numBytesOfClearData)
        pub fn set_numBytesOfClearData<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::IntArray>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "numBytesOfClearData\0", "[I\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [numBytesOfEncryptedData](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#numBytesOfEncryptedData)
        pub fn numBytesOfEncryptedData<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "numBytesOfEncryptedData\0", "[I\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [numBytesOfEncryptedData](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#numBytesOfEncryptedData)
        pub fn set_numBytesOfEncryptedData<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::IntArray>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "numBytesOfEncryptedData\0", "[I\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [numSubSamples](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#numSubSamples)
        pub fn numSubSamples<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "numSubSamples\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [numSubSamples](https://developer.android.com/reference/android/media/MediaCodec.CryptoInfo.html#numSubSamples)
        pub fn set_numSubSamples<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$CryptoInfo\0", "numSubSamples\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
