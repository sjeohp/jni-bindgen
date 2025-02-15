// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-opengl-Visibility"))]
__jni_bindgen! {
    /// public class [Visibility](https://developer.android.com/reference/android/opengl/Visibility.html)
    ///
    /// Required feature: android-opengl-Visibility
    public class Visibility ("android/opengl/Visibility") extends crate::java::lang::Object {

        /// [Visibility](https://developer.android.com/reference/android/opengl/Visibility.html#Visibility())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::opengl::Visibility>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/Visibility", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/Visibility\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [visibilityTest](https://developer.android.com/reference/android/opengl/Visibility.html#visibilityTest(float%5B%5D,%20int,%20float%5B%5D,%20int,%20char%5B%5D,%20int,%20int))
        pub fn visibilityTest<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg5: i32, arg6: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/Visibility", java.flags == PUBLIC | STATIC | NATIVE, .name == "visibilityTest", .descriptor == "([FI[FI[CII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/Visibility\0", "visibilityTest\0", "([FI[FI[CII)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [frustumCullSpheres](https://developer.android.com/reference/android/opengl/Visibility.html#frustumCullSpheres(float%5B%5D,%20int,%20float%5B%5D,%20int,%20int,%20int%5B%5D,%20int,%20int))
        pub fn frustumCullSpheres<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg3: i32, arg4: i32, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg6: i32, arg7: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/Visibility", java.flags == PUBLIC | STATIC | NATIVE, .name == "frustumCullSpheres", .descriptor == "([FI[FII[III)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6), __jni_bindgen::AsJValue::as_jvalue(&arg7)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/Visibility\0", "frustumCullSpheres\0", "([FI[FII[III)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [computeBoundingSphere](https://developer.android.com/reference/android/opengl/Visibility.html#computeBoundingSphere(float%5B%5D,%20int,%20int,%20float%5B%5D,%20int))
        pub fn computeBoundingSphere<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg4: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/Visibility", java.flags == PUBLIC | STATIC | NATIVE, .name == "computeBoundingSphere", .descriptor == "([FII[FI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/Visibility\0", "computeBoundingSphere\0", "([FII[FI)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
