// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-SweepGradient"))]
__jni_bindgen! {
    /// public class [SweepGradient](https://developer.android.com/reference/android/graphics/SweepGradient.html)
    ///
    /// Required feature: android-graphics-SweepGradient
    public class SweepGradient ("android/graphics/SweepGradient") extends crate::android::graphics::Shader {

        /// [SweepGradient](https://developer.android.com/reference/android/graphics/SweepGradient.html#SweepGradient(float,%20float,%20int%5B%5D,%20float%5B%5D))
        pub fn new_float_float_int_array_float_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: f32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::SweepGradient>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/SweepGradient", java.flags == PUBLIC, .name == "<init>", .descriptor == "(FF[I[F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/SweepGradient\0", "<init>\0", "(FF[I[F)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SweepGradient](https://developer.android.com/reference/android/graphics/SweepGradient.html#SweepGradient(float,%20float,%20long%5B%5D,%20float%5B%5D))
        pub fn new_float_float_long_array_float_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: f32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::SweepGradient>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/SweepGradient", java.flags == PUBLIC, .name == "<init>", .descriptor == "(FF[J[F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/SweepGradient\0", "<init>\0", "(FF[J[F)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SweepGradient](https://developer.android.com/reference/android/graphics/SweepGradient.html#SweepGradient(float,%20float,%20int,%20int))
        pub fn new_float_float_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: f32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::SweepGradient>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/SweepGradient", java.flags == PUBLIC, .name == "<init>", .descriptor == "(FFII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/SweepGradient\0", "<init>\0", "(FFII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SweepGradient](https://developer.android.com/reference/android/graphics/SweepGradient.html#SweepGradient(float,%20float,%20long,%20long))
        pub fn new_float_float_long_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: f32, arg2: i64, arg3: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::SweepGradient>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/SweepGradient", java.flags == PUBLIC, .name == "<init>", .descriptor == "(FFJJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/SweepGradient\0", "<init>\0", "(FFJJ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
