// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-InputDevice_MotionRange"))]
__jni_bindgen! {
    /// public final class [InputDevice.MotionRange](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html)
    ///
    /// Required feature: android-view-InputDevice_MotionRange
    public final class InputDevice_MotionRange ("android/view/InputDevice$MotionRange") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [MotionRange](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#MotionRange(int,%20int,%20float,%20float,%20float,%20float,%20float))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: f32, arg3: f32, arg4: f32, arg5: f32, arg6: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::InputDevice_MotionRange>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/InputDevice$MotionRange", java.flags == (empty), .name == "<init>", .descriptor == "(IIFFFFF)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "<init>\0", "(IIFFFFF)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getAxis](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getAxis())
        pub fn getAxis<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getAxis", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getAxis\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSource](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getSource())
        pub fn getSource<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getSource", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getSource\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isFromSource](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#isFromSource(int))
        pub fn isFromSource<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "isFromSource", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "isFromSource\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMin](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getMin())
        pub fn getMin<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getMin", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getMin\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMax](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getMax())
        pub fn getMax<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getMax", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getMax\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRange](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getRange())
        pub fn getRange<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getRange", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getRange\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFlat](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getFlat())
        pub fn getFlat<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getFlat", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getFlat\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFuzz](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getFuzz())
        pub fn getFuzz<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getFuzz", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getFuzz\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResolution](https://developer.android.com/reference/android/view/InputDevice.MotionRange.html#getResolution())
        pub fn getResolution<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputDevice$MotionRange", java.flags == PUBLIC, .name == "getResolution", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputDevice$MotionRange\0", "getResolution\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
