// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-drawable-AnimatedImageDrawable"))]
__jni_bindgen! {
    /// public class [AnimatedImageDrawable](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html)
    ///
    /// Required feature: android-graphics-drawable-AnimatedImageDrawable
    public class AnimatedImageDrawable ("android/graphics/drawable/AnimatedImageDrawable") extends crate::android::graphics::drawable::Drawable, implements crate::android::graphics::drawable::Animatable2 {

        /// [AnimatedImageDrawable](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#AnimatedImageDrawable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::AnimatedImageDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRepeatCount](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#setRepeatCount(int))
        pub fn setRepeatCount<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "setRepeatCount", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "setRepeatCount\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRepeatCount](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#getRepeatCount())
        pub fn getRepeatCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "getRepeatCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "getRepeatCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#inflate(android.content.res.Resources,%20org.xmlpull.v1.XmlPullParser,%20android.util.AttributeSet,%20android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources", "android-content-res-Resources_Theme", "android-util-AttributeSet", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-content-res-Resources_Theme", feature = "android-util-AttributeSet", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "inflate\0", "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntrinsicWidth](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#getIntrinsicWidth())
        pub fn getIntrinsicWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "getIntrinsicWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "getIntrinsicWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntrinsicHeight](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#getIntrinsicHeight())
        pub fn getIntrinsicHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "getIntrinsicHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "getIntrinsicHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [draw](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#draw(android.graphics.Canvas))
        ///
        /// Required features: "android-graphics-Canvas"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Canvas")))]
        pub fn draw<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Canvas>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "draw", .descriptor == "(Landroid/graphics/Canvas;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "draw\0", "(Landroid/graphics/Canvas;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAlpha](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#setAlpha(int))
        pub fn setAlpha<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "setAlpha", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "setAlpha\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlpha](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#getAlpha())
        pub fn getAlpha<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "getAlpha", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "getAlpha\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setColorFilter](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#setColorFilter(android.graphics.ColorFilter))
        ///
        /// Required features: "android-graphics-ColorFilter"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorFilter")))]
        pub fn setColorFilter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::ColorFilter>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "setColorFilter", .descriptor == "(Landroid/graphics/ColorFilter;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "setColorFilter\0", "(Landroid/graphics/ColorFilter;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getColorFilter](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#getColorFilter())
        ///
        /// Required features: "android-graphics-ColorFilter"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorFilter")))]
        pub fn getColorFilter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorFilter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "getColorFilter", .descriptor == "()Landroid/graphics/ColorFilter;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "getColorFilter\0", "()Landroid/graphics/ColorFilter;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOpacity](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#getOpacity())
        pub fn getOpacity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "getOpacity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "getOpacity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAutoMirrored](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#setAutoMirrored(boolean))
        pub fn setAutoMirrored<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "setAutoMirrored", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "setAutoMirrored\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLayoutDirectionChanged](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#onLayoutDirectionChanged(int))
        pub fn onLayoutDirectionChanged<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "onLayoutDirectionChanged", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "onLayoutDirectionChanged\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isAutoMirrored](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#isAutoMirrored())
        pub fn isAutoMirrored<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC | FINAL, .name == "isAutoMirrored", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "isAutoMirrored\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRunning](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#isRunning())
        pub fn isRunning<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "isRunning", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "isRunning\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [start](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#start())
        pub fn start<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "start", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "start\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stop](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#stop())
        pub fn stop<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "stop", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "stop\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerAnimationCallback](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#registerAnimationCallback(android.graphics.drawable.Animatable2.AnimationCallback))
        ///
        /// Required features: "android-graphics-drawable-Animatable2_AnimationCallback"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Animatable2_AnimationCallback")))]
        pub fn registerAnimationCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Animatable2_AnimationCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "registerAnimationCallback", .descriptor == "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "registerAnimationCallback\0", "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterAnimationCallback](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#unregisterAnimationCallback(android.graphics.drawable.Animatable2.AnimationCallback))
        ///
        /// Required features: "android-graphics-drawable-Animatable2_AnimationCallback"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Animatable2_AnimationCallback")))]
        pub fn unregisterAnimationCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Animatable2_AnimationCallback>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "unregisterAnimationCallback", .descriptor == "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "unregisterAnimationCallback\0", "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clearAnimationCallbacks](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#clearAnimationCallbacks())
        pub fn clearAnimationCallbacks<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/AnimatedImageDrawable", java.flags == PUBLIC, .name == "clearAnimationCallbacks", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/AnimatedImageDrawable\0", "clearAnimationCallbacks\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [REPEAT_INFINITE](https://developer.android.com/reference/android/graphics/drawable/AnimatedImageDrawable.html#REPEAT_INFINITE)
        pub const REPEAT_INFINITE : i32 = -1;
    }
}
