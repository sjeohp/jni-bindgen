// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-opengl-ETC1Util_ETC1Texture"))]
__jni_bindgen! {
    /// public class [ETC1Util.ETC1Texture](https://developer.android.com/reference/android/opengl/ETC1Util.ETC1Texture.html)
    ///
    /// Required feature: android-opengl-ETC1Util_ETC1Texture
    public class ETC1Util_ETC1Texture ("android/opengl/ETC1Util$ETC1Texture") extends crate::java::lang::Object {

        /// [ETC1Texture](https://developer.android.com/reference/android/opengl/ETC1Util.ETC1Texture.html#ETC1Texture(int,%20int,%20java.nio.ByteBuffer))
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::opengl::ETC1Util_ETC1Texture>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/ETC1Util$ETC1Texture", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IILjava/nio/ByteBuffer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/ETC1Util$ETC1Texture\0", "<init>\0", "(IILjava/nio/ByteBuffer;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWidth](https://developer.android.com/reference/android/opengl/ETC1Util.ETC1Texture.html#getWidth())
        pub fn getWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/ETC1Util$ETC1Texture", java.flags == PUBLIC, .name == "getWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/ETC1Util$ETC1Texture\0", "getWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHeight](https://developer.android.com/reference/android/opengl/ETC1Util.ETC1Texture.html#getHeight())
        pub fn getHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/ETC1Util$ETC1Texture", java.flags == PUBLIC, .name == "getHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/ETC1Util$ETC1Texture\0", "getHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getData](https://developer.android.com/reference/android/opengl/ETC1Util.ETC1Texture.html#getData())
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn getData<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ByteBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/ETC1Util$ETC1Texture", java.flags == PUBLIC, .name == "getData", .descriptor == "()Ljava/nio/ByteBuffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/ETC1Util$ETC1Texture\0", "getData\0", "()Ljava/nio/ByteBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
