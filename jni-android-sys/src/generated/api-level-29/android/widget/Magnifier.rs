// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-Magnifier"))]
__jni_bindgen! {
    /// public final class [Magnifier](https://developer.android.com/reference/android/widget/Magnifier.html)
    ///
    /// Required feature: android-widget-Magnifier
    public final class Magnifier ("android/widget/Magnifier") extends crate::java::lang::Object {

        /// [Magnifier](https://developer.android.com/reference/android/widget/Magnifier.html#Magnifier(android.view.View))
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::Magnifier>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/View;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "<init>\0", "(Landroid/view/View;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/widget/Magnifier.html#show(float,%20float))
        pub fn show_float_float<'env>(&'env self, arg0: f32, arg1: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "show", .descriptor == "(FF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "show\0", "(FF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/widget/Magnifier.html#show(float,%20float,%20float,%20float))
        pub fn show_float_float_float_float<'env>(&'env self, arg0: f32, arg1: f32, arg2: f32, arg3: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "show", .descriptor == "(FFFF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "show\0", "(FFFF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dismiss](https://developer.android.com/reference/android/widget/Magnifier.html#dismiss())
        pub fn dismiss<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "dismiss", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "dismiss\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [update](https://developer.android.com/reference/android/widget/Magnifier.html#update())
        pub fn update<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "update", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "update\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWidth](https://developer.android.com/reference/android/widget/Magnifier.html#getWidth())
        pub fn getWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHeight](https://developer.android.com/reference/android/widget/Magnifier.html#getHeight())
        pub fn getHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSourceWidth](https://developer.android.com/reference/android/widget/Magnifier.html#getSourceWidth())
        pub fn getSourceWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getSourceWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getSourceWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSourceHeight](https://developer.android.com/reference/android/widget/Magnifier.html#getSourceHeight())
        pub fn getSourceHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getSourceHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getSourceHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setZoom](https://developer.android.com/reference/android/widget/Magnifier.html#setZoom(float))
        pub fn setZoom<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "setZoom", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "setZoom\0", "(F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getZoom](https://developer.android.com/reference/android/widget/Magnifier.html#getZoom())
        pub fn getZoom<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getZoom", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getZoom\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getElevation](https://developer.android.com/reference/android/widget/Magnifier.html#getElevation())
        pub fn getElevation<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getElevation", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getElevation\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCornerRadius](https://developer.android.com/reference/android/widget/Magnifier.html#getCornerRadius())
        pub fn getCornerRadius<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getCornerRadius", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getCornerRadius\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultHorizontalSourceToMagnifierOffset](https://developer.android.com/reference/android/widget/Magnifier.html#getDefaultHorizontalSourceToMagnifierOffset())
        pub fn getDefaultHorizontalSourceToMagnifierOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getDefaultHorizontalSourceToMagnifierOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getDefaultHorizontalSourceToMagnifierOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultVerticalSourceToMagnifierOffset](https://developer.android.com/reference/android/widget/Magnifier.html#getDefaultVerticalSourceToMagnifierOffset())
        pub fn getDefaultVerticalSourceToMagnifierOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getDefaultVerticalSourceToMagnifierOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getDefaultVerticalSourceToMagnifierOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOverlay](https://developer.android.com/reference/android/widget/Magnifier.html#getOverlay())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getOverlay<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getOverlay", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getOverlay\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isClippingEnabled](https://developer.android.com/reference/android/widget/Magnifier.html#isClippingEnabled())
        pub fn isClippingEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "isClippingEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "isClippingEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPosition](https://developer.android.com/reference/android/widget/Magnifier.html#getPosition())
        ///
        /// Required features: "android-graphics-Point"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Point")))]
        pub fn getPosition<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Point>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getPosition", .descriptor == "()Landroid/graphics/Point;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getPosition\0", "()Landroid/graphics/Point;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSourcePosition](https://developer.android.com/reference/android/widget/Magnifier.html#getSourcePosition())
        ///
        /// Required features: "android-graphics-Point"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Point")))]
        pub fn getSourcePosition<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Point>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Magnifier", java.flags == PUBLIC, .name == "getSourcePosition", .descriptor == "()Landroid/graphics/Point;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Magnifier\0", "getSourcePosition\0", "()Landroid/graphics/Point;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [SOURCE_BOUND_MAX_IN_SURFACE](https://developer.android.com/reference/android/widget/Magnifier.html#SOURCE_BOUND_MAX_IN_SURFACE)
        pub const SOURCE_BOUND_MAX_IN_SURFACE : i32 = 0;

        /// public static final [SOURCE_BOUND_MAX_VISIBLE](https://developer.android.com/reference/android/widget/Magnifier.html#SOURCE_BOUND_MAX_VISIBLE)
        pub const SOURCE_BOUND_MAX_VISIBLE : i32 = 1;
    }
}
