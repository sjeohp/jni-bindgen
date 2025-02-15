// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-camera2-params-ColorSpaceTransform"))]
__jni_bindgen! {
    /// public final class [ColorSpaceTransform](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html)
    ///
    /// Required feature: android-hardware-camera2-params-ColorSpaceTransform
    public final class ColorSpaceTransform ("android/hardware/camera2/params/ColorSpaceTransform") extends crate::java::lang::Object {

        /// [ColorSpaceTransform](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#ColorSpaceTransform(android.util.Rational%5B%5D))
        ///
        /// Required features: "android-util-Rational"
        #[cfg(any(feature = "all", all(feature = "android-util-Rational")))]
        pub fn new_Rational_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::util::Rational, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::camera2::params::ColorSpaceTransform>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "<init>", .descriptor == "([Landroid/util/Rational;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "<init>\0", "([Landroid/util/Rational;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ColorSpaceTransform](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#ColorSpaceTransform(int%5B%5D))
        pub fn new_int_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::camera2::params::ColorSpaceTransform>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "<init>", .descriptor == "([I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "<init>\0", "([I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getElement](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#getElement(int,%20int))
        ///
        /// Required features: "android-util-Rational"
        #[cfg(any(feature = "all", all(feature = "android-util-Rational")))]
        pub fn getElement<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::Rational>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "getElement", .descriptor == "(II)Landroid/util/Rational;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "getElement\0", "(II)Landroid/util/Rational;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [copyElements](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#copyElements(android.util.Rational%5B%5D,%20int))
        ///
        /// Required features: "android-util-Rational"
        #[cfg(any(feature = "all", all(feature = "android-util-Rational")))]
        pub fn copyElements_Rational_array_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::util::Rational, crate::java::lang::Throwable>>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "copyElements", .descriptor == "([Landroid/util/Rational;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "copyElements\0", "([Landroid/util/Rational;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [copyElements](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#copyElements(int%5B%5D,%20int))
        pub fn copyElements_int_array_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "copyElements", .descriptor == "([II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "copyElements\0", "([II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/hardware/camera2/params/ColorSpaceTransform.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/params/ColorSpaceTransform", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/params/ColorSpaceTransform\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
