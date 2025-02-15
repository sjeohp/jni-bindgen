// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-camera2-params-RggbChannelVector"))]
__jni_bindgen! {
    /// public final class [RggbChannelVector](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html)
    ///
    /// Required feature: android-hardware-camera2-params-RggbChannelVector
    public final class RggbChannelVector ("android/hardware/camera2/params/RggbChannelVector") extends crate::java::lang::Object {

        /// [RggbChannelVector](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#RggbChannelVector(float,%20float,%20float,%20float))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: f32, arg2: f32, arg3: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::camera2::params::RggbChannelVector>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "<init>", .descriptor == "(FFFF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "<init>\0", "(FFFF)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRed](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#getRed())
        pub fn getRed<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "getRed", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "getRed\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGreenEven](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#getGreenEven())
        pub fn getGreenEven<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "getGreenEven", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "getGreenEven\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGreenOdd](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#getGreenOdd())
        pub fn getGreenOdd<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "getGreenOdd", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "getGreenOdd\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBlue](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#getBlue())
        pub fn getBlue<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "getBlue", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "getBlue\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getComponent](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#getComponent(int))
        pub fn getComponent<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "getComponent", .descriptor == "(I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "getComponent\0", "(I)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [copyTo](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#copyTo(float%5B%5D,%20int))
        pub fn copyTo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "copyTo", .descriptor == "([FI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "copyTo\0", "([FI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/RggbChannelVector", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/RggbChannelVector\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [BLUE](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#BLUE)
        pub const BLUE : i32 = 3;

        /// public static final [COUNT](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#COUNT)
        pub const COUNT : i32 = 4;

        /// public static final [GREEN_EVEN](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#GREEN_EVEN)
        pub const GREEN_EVEN : i32 = 1;

        /// public static final [GREEN_ODD](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#GREEN_ODD)
        pub const GREEN_ODD : i32 = 2;

        /// public static final [RED](https://developer.android.com/reference/android/hardware/camera2/params/RggbChannelVector.html#RED)
        pub const RED : i32 = 0;
    }
}
