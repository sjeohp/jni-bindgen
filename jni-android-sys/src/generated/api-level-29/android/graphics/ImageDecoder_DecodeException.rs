// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-ImageDecoder_DecodeException"))]
__jni_bindgen! {
    /// public final class [ImageDecoder.DecodeException](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html)
    ///
    /// Required feature: android-graphics-ImageDecoder_DecodeException
    public final class ImageDecoder_DecodeException ("android/graphics/ImageDecoder$DecodeException") extends crate::java::io::IOException {

        // // Not emitting: Non-public method
        // /// [DecodeException](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html#DecodeException(int,%20java.lang.Throwable,%20android.graphics.ImageDecoder.Source))
        // ///
        // /// Required features: "android-graphics-ImageDecoder_Source", "java-lang-Throwable"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-ImageDecoder_Source", feature = "java-lang-Throwable")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::ImageDecoder_Source>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::ImageDecoder_DecodeException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/ImageDecoder$DecodeException", java.flags == (empty), .name == "<init>", .descriptor == "(ILjava/lang/Throwable;Landroid/graphics/ImageDecoder$Source;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ImageDecoder$DecodeException\0", "<init>\0", "(ILjava/lang/Throwable;Landroid/graphics/ImageDecoder$Source;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getError](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html#getError())
        pub fn getError<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ImageDecoder$DecodeException", java.flags == PUBLIC, .name == "getError", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ImageDecoder$DecodeException\0", "getError\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSource](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html#getSource())
        ///
        /// Required features: "android-graphics-ImageDecoder_Source"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ImageDecoder_Source")))]
        pub fn getSource<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ImageDecoder_Source>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ImageDecoder$DecodeException", java.flags == PUBLIC, .name == "getSource", .descriptor == "()Landroid/graphics/ImageDecoder$Source;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ImageDecoder$DecodeException\0", "getSource\0", "()Landroid/graphics/ImageDecoder$Source;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [SOURCE_EXCEPTION](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html#SOURCE_EXCEPTION)
        pub const SOURCE_EXCEPTION : i32 = 1;

        /// public static final [SOURCE_INCOMPLETE](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html#SOURCE_INCOMPLETE)
        pub const SOURCE_INCOMPLETE : i32 = 2;

        /// public static final [SOURCE_MALFORMED_DATA](https://developer.android.com/reference/android/graphics/ImageDecoder.DecodeException.html#SOURCE_MALFORMED_DATA)
        pub const SOURCE_MALFORMED_DATA : i32 = 3;
    }
}
