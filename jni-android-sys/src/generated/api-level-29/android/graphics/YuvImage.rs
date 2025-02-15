// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-YuvImage"))]
__jni_bindgen! {
    /// public class [YuvImage](https://developer.android.com/reference/android/graphics/YuvImage.html)
    ///
    /// Required feature: android-graphics-YuvImage
    public class YuvImage ("android/graphics/YuvImage") extends crate::java::lang::Object {

        /// [YuvImage](https://developer.android.com/reference/android/graphics/YuvImage.html#YuvImage(byte%5B%5D,%20int,%20int,%20int,%20int%5B%5D))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::YuvImage>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "<init>", .descriptor == "([BIII[I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "<init>\0", "([BIII[I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compressToJpeg](https://developer.android.com/reference/android/graphics/YuvImage.html#compressToJpeg(android.graphics.Rect,%20int,%20java.io.OutputStream))
        ///
        /// Required features: "android-graphics-Rect", "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect", feature = "java-io-OutputStream")))]
        pub fn compressToJpeg<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "compressToJpeg", .descriptor == "(Landroid/graphics/Rect;ILjava/io/OutputStream;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "compressToJpeg\0", "(Landroid/graphics/Rect;ILjava/io/OutputStream;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getYuvData](https://developer.android.com/reference/android/graphics/YuvImage.html#getYuvData())
        pub fn getYuvData<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "getYuvData", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "getYuvData\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getYuvFormat](https://developer.android.com/reference/android/graphics/YuvImage.html#getYuvFormat())
        pub fn getYuvFormat<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "getYuvFormat", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "getYuvFormat\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStrides](https://developer.android.com/reference/android/graphics/YuvImage.html#getStrides())
        pub fn getStrides<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "getStrides", .descriptor == "()[I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "getStrides\0", "()[I\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWidth](https://developer.android.com/reference/android/graphics/YuvImage.html#getWidth())
        pub fn getWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "getWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "getWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHeight](https://developer.android.com/reference/android/graphics/YuvImage.html#getHeight())
        pub fn getHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/YuvImage", java.flags == PUBLIC, .name == "getHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/YuvImage\0", "getHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
