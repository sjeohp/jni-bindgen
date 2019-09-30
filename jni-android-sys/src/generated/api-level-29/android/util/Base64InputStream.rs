// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-Base64InputStream"))]
__jni_bindgen! {
    /// public class [Base64InputStream](https://developer.android.com/reference/android/util/Base64InputStream.html)
    ///
    /// Required feature: android-util-Base64InputStream
    public class Base64InputStream ("android/util/Base64InputStream") extends crate::java::io::FilterInputStream {

        /// [Base64InputStream](https://developer.android.com/reference/android/util/Base64InputStream.html#Base64InputStream(java.io.InputStream,%20int))
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::Base64InputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "<init>\0", "(Ljava/io/InputStream;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [markSupported](https://developer.android.com/reference/android/util/Base64InputStream.html#markSupported())
        pub fn markSupported<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "markSupported", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "markSupported\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mark](https://developer.android.com/reference/android/util/Base64InputStream.html#mark(int))
        pub fn mark<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "mark", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "mark\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/android/util/Base64InputStream.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/util/Base64InputStream.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [available](https://developer.android.com/reference/android/util/Base64InputStream.html#available())
        pub fn available<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "available", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "available\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [skip](https://developer.android.com/reference/android/util/Base64InputStream.html#skip(long))
        pub fn skip<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "skip", .descriptor == "(J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "skip\0", "(J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/android/util/Base64InputStream.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/android/util/Base64InputStream.html#read(byte%5B%5D,%20int,%20int))
        pub fn read_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Base64InputStream", java.flags == PUBLIC, .name == "read", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Base64InputStream\0", "read\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
