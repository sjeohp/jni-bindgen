// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-MotionEvent_PointerCoords"))]
__jni_bindgen! {
    /// public final class [MotionEvent.PointerCoords](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html)
    ///
    /// Required feature: android-view-MotionEvent_PointerCoords
    public final class MotionEvent_PointerCoords ("android/view/MotionEvent$PointerCoords") extends crate::java::lang::Object {

        /// [PointerCoords](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#PointerCoords())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::MotionEvent_PointerCoords>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/MotionEvent$PointerCoords", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/MotionEvent$PointerCoords\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PointerCoords](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#PointerCoords(android.view.MotionEvent.PointerCoords))
        ///
        /// Required features: "android-view-MotionEvent_PointerCoords"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent_PointerCoords")))]
        pub fn new_PointerCoords<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent_PointerCoords>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::MotionEvent_PointerCoords>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/MotionEvent$PointerCoords", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/MotionEvent$PointerCoords;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/MotionEvent$PointerCoords\0", "<init>\0", "(Landroid/view/MotionEvent$PointerCoords;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#clear())
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/MotionEvent$PointerCoords", java.flags == PUBLIC, .name == "clear", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/MotionEvent$PointerCoords\0", "clear\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [copyFrom](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#copyFrom(android.view.MotionEvent.PointerCoords))
        ///
        /// Required features: "android-view-MotionEvent_PointerCoords"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent_PointerCoords")))]
        pub fn copyFrom<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent_PointerCoords>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/MotionEvent$PointerCoords", java.flags == PUBLIC, .name == "copyFrom", .descriptor == "(Landroid/view/MotionEvent$PointerCoords;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/MotionEvent$PointerCoords\0", "copyFrom\0", "(Landroid/view/MotionEvent$PointerCoords;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAxisValue](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#getAxisValue(int))
        pub fn getAxisValue<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/MotionEvent$PointerCoords", java.flags == PUBLIC, .name == "getAxisValue", .descriptor == "(I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/MotionEvent$PointerCoords\0", "getAxisValue\0", "(I)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAxisValue](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#setAxisValue(int,%20float))
        pub fn setAxisValue<'env>(&'env self, arg0: i32, arg1: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/MotionEvent$PointerCoords", java.flags == PUBLIC, .name == "setAxisValue", .descriptor == "(IF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/MotionEvent$PointerCoords\0", "setAxisValue\0", "(IF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [orientation](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#orientation)
        pub fn orientation<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "orientation\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [orientation](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#orientation)
        pub fn set_orientation<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "orientation\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [pressure](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#pressure)
        pub fn pressure<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "pressure\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [pressure](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#pressure)
        pub fn set_pressure<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "pressure\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [size](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#size)
        pub fn size<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "size\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [size](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#size)
        pub fn set_size<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "size\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [toolMajor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#toolMajor)
        pub fn toolMajor<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "toolMajor\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [toolMajor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#toolMajor)
        pub fn set_toolMajor<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "toolMajor\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [toolMinor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#toolMinor)
        pub fn toolMinor<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "toolMinor\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [toolMinor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#toolMinor)
        pub fn set_toolMinor<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "toolMinor\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [touchMajor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#touchMajor)
        pub fn touchMajor<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "touchMajor\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [touchMajor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#touchMajor)
        pub fn set_touchMajor<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "touchMajor\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [touchMinor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#touchMinor)
        pub fn touchMinor<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "touchMinor\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [touchMinor](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#touchMinor)
        pub fn set_touchMinor<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "touchMinor\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [x](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#x)
        pub fn x<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "x\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [x](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#x)
        pub fn set_x<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "x\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }

        /// **get** public [y](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#y)
        pub fn y<'env>(&'env self) -> f32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "y\0", "F\0");
                env.get_float_field(class, field)
            }
        }

        /// **set** public [y](https://developer.android.com/reference/android/view/MotionEvent.PointerCoords.html#y)
        pub fn set_y<'env>(&'env self, value: f32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/MotionEvent$PointerCoords\0", "y\0", "F\0");
                env.set_float_field(class, field, value)
            }
        }
    }
}
