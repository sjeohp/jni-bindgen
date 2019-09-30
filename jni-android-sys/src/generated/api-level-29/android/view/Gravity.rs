// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-Gravity"))]
__jni_bindgen! {
    /// public class [Gravity](https://developer.android.com/reference/android/view/Gravity.html)
    ///
    /// Required feature: android-view-Gravity
    public class Gravity ("android/view/Gravity") extends crate::java::lang::Object {

        /// [Gravity](https://developer.android.com/reference/android/view/Gravity.html#Gravity())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::Gravity>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/Gravity\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/view/Gravity.html#apply(int,%20int,%20int,%20android.graphics.Rect,%20android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn apply_int_int_int_Rect_Rect<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "apply", .descriptor == "(IIILandroid/graphics/Rect;Landroid/graphics/Rect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "apply\0", "(IIILandroid/graphics/Rect;Landroid/graphics/Rect;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/view/Gravity.html#apply(int,%20int,%20int,%20android.graphics.Rect,%20android.graphics.Rect,%20int))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn apply_int_int_int_Rect_Rect_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg5: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "apply", .descriptor == "(IIILandroid/graphics/Rect;Landroid/graphics/Rect;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "apply\0", "(IIILandroid/graphics/Rect;Landroid/graphics/Rect;I)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/view/Gravity.html#apply(int,%20int,%20int,%20android.graphics.Rect,%20int,%20int,%20android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn apply_int_int_int_Rect_int_int_Rect<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg4: i32, arg5: i32, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "apply", .descriptor == "(IIILandroid/graphics/Rect;IILandroid/graphics/Rect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "apply\0", "(IIILandroid/graphics/Rect;IILandroid/graphics/Rect;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/view/Gravity.html#apply(int,%20int,%20int,%20android.graphics.Rect,%20int,%20int,%20android.graphics.Rect,%20int))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn apply_int_int_int_Rect_int_int_Rect_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg4: i32, arg5: i32, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg7: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "apply", .descriptor == "(IIILandroid/graphics/Rect;IILandroid/graphics/Rect;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6.into()), __jni_bindgen::AsJValue::as_jvalue(&arg7)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "apply\0", "(IIILandroid/graphics/Rect;IILandroid/graphics/Rect;I)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyDisplay](https://developer.android.com/reference/android/view/Gravity.html#applyDisplay(int,%20android.graphics.Rect,%20android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn applyDisplay_int_Rect_Rect<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "applyDisplay", .descriptor == "(ILandroid/graphics/Rect;Landroid/graphics/Rect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "applyDisplay\0", "(ILandroid/graphics/Rect;Landroid/graphics/Rect;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyDisplay](https://developer.android.com/reference/android/view/Gravity.html#applyDisplay(int,%20android.graphics.Rect,%20android.graphics.Rect,%20int))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn applyDisplay_int_Rect_Rect_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "applyDisplay", .descriptor == "(ILandroid/graphics/Rect;Landroid/graphics/Rect;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "applyDisplay\0", "(ILandroid/graphics/Rect;Landroid/graphics/Rect;I)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isVertical](https://developer.android.com/reference/android/view/Gravity.html#isVertical(int))
        pub fn isVertical<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "isVertical", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "isVertical\0", "(I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isHorizontal](https://developer.android.com/reference/android/view/Gravity.html#isHorizontal(int))
        pub fn isHorizontal<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "isHorizontal", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "isHorizontal\0", "(I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAbsoluteGravity](https://developer.android.com/reference/android/view/Gravity.html#getAbsoluteGravity(int,%20int))
        pub fn getAbsoluteGravity<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Gravity", java.flags == PUBLIC | STATIC, .name == "getAbsoluteGravity", .descriptor == "(II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/Gravity\0", "getAbsoluteGravity\0", "(II)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [AXIS_CLIP](https://developer.android.com/reference/android/view/Gravity.html#AXIS_CLIP)
        pub const AXIS_CLIP : i32 = 8;

        /// public static final [AXIS_PULL_AFTER](https://developer.android.com/reference/android/view/Gravity.html#AXIS_PULL_AFTER)
        pub const AXIS_PULL_AFTER : i32 = 4;

        /// public static final [AXIS_PULL_BEFORE](https://developer.android.com/reference/android/view/Gravity.html#AXIS_PULL_BEFORE)
        pub const AXIS_PULL_BEFORE : i32 = 2;

        /// public static final [AXIS_SPECIFIED](https://developer.android.com/reference/android/view/Gravity.html#AXIS_SPECIFIED)
        pub const AXIS_SPECIFIED : i32 = 1;

        /// public static final [AXIS_X_SHIFT](https://developer.android.com/reference/android/view/Gravity.html#AXIS_X_SHIFT)
        pub const AXIS_X_SHIFT : i32 = 0;

        /// public static final [AXIS_Y_SHIFT](https://developer.android.com/reference/android/view/Gravity.html#AXIS_Y_SHIFT)
        pub const AXIS_Y_SHIFT : i32 = 4;

        /// public static final [BOTTOM](https://developer.android.com/reference/android/view/Gravity.html#BOTTOM)
        pub const BOTTOM : i32 = 80;

        /// public static final [CENTER](https://developer.android.com/reference/android/view/Gravity.html#CENTER)
        pub const CENTER : i32 = 17;

        /// public static final [CENTER_HORIZONTAL](https://developer.android.com/reference/android/view/Gravity.html#CENTER_HORIZONTAL)
        pub const CENTER_HORIZONTAL : i32 = 1;

        /// public static final [CENTER_VERTICAL](https://developer.android.com/reference/android/view/Gravity.html#CENTER_VERTICAL)
        pub const CENTER_VERTICAL : i32 = 16;

        /// public static final [CLIP_HORIZONTAL](https://developer.android.com/reference/android/view/Gravity.html#CLIP_HORIZONTAL)
        pub const CLIP_HORIZONTAL : i32 = 8;

        /// public static final [CLIP_VERTICAL](https://developer.android.com/reference/android/view/Gravity.html#CLIP_VERTICAL)
        pub const CLIP_VERTICAL : i32 = 128;

        /// public static final [DISPLAY_CLIP_HORIZONTAL](https://developer.android.com/reference/android/view/Gravity.html#DISPLAY_CLIP_HORIZONTAL)
        pub const DISPLAY_CLIP_HORIZONTAL : i32 = 16777216;

        /// public static final [DISPLAY_CLIP_VERTICAL](https://developer.android.com/reference/android/view/Gravity.html#DISPLAY_CLIP_VERTICAL)
        pub const DISPLAY_CLIP_VERTICAL : i32 = 268435456;

        /// public static final [END](https://developer.android.com/reference/android/view/Gravity.html#END)
        pub const END : i32 = 8388613;

        /// public static final [FILL](https://developer.android.com/reference/android/view/Gravity.html#FILL)
        pub const FILL : i32 = 119;

        /// public static final [FILL_HORIZONTAL](https://developer.android.com/reference/android/view/Gravity.html#FILL_HORIZONTAL)
        pub const FILL_HORIZONTAL : i32 = 7;

        /// public static final [FILL_VERTICAL](https://developer.android.com/reference/android/view/Gravity.html#FILL_VERTICAL)
        pub const FILL_VERTICAL : i32 = 112;

        /// public static final [HORIZONTAL_GRAVITY_MASK](https://developer.android.com/reference/android/view/Gravity.html#HORIZONTAL_GRAVITY_MASK)
        pub const HORIZONTAL_GRAVITY_MASK : i32 = 7;

        /// public static final [LEFT](https://developer.android.com/reference/android/view/Gravity.html#LEFT)
        pub const LEFT : i32 = 3;

        /// public static final [NO_GRAVITY](https://developer.android.com/reference/android/view/Gravity.html#NO_GRAVITY)
        pub const NO_GRAVITY : i32 = 0;

        /// public static final [RELATIVE_HORIZONTAL_GRAVITY_MASK](https://developer.android.com/reference/android/view/Gravity.html#RELATIVE_HORIZONTAL_GRAVITY_MASK)
        pub const RELATIVE_HORIZONTAL_GRAVITY_MASK : i32 = 8388615;

        /// public static final [RELATIVE_LAYOUT_DIRECTION](https://developer.android.com/reference/android/view/Gravity.html#RELATIVE_LAYOUT_DIRECTION)
        pub const RELATIVE_LAYOUT_DIRECTION : i32 = 8388608;

        /// public static final [RIGHT](https://developer.android.com/reference/android/view/Gravity.html#RIGHT)
        pub const RIGHT : i32 = 5;

        /// public static final [START](https://developer.android.com/reference/android/view/Gravity.html#START)
        pub const START : i32 = 8388611;

        /// public static final [TOP](https://developer.android.com/reference/android/view/Gravity.html#TOP)
        pub const TOP : i32 = 48;

        /// public static final [VERTICAL_GRAVITY_MASK](https://developer.android.com/reference/android/view/Gravity.html#VERTICAL_GRAVITY_MASK)
        pub const VERTICAL_GRAVITY_MASK : i32 = 112;
    }
}
