// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-drawable-DrawableWrapper"))]
__jni_bindgen! {
    /// public class [DrawableWrapper](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html)
    ///
    /// Required feature: android-graphics-drawable-DrawableWrapper
    public class DrawableWrapper ("android/graphics/drawable/DrawableWrapper") extends crate::android::graphics::drawable::Drawable, implements crate::android::graphics::drawable::Drawable_Callback {

        /// [DrawableWrapper](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#DrawableWrapper(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::DrawableWrapper>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "<init>\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDrawable](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setDrawable(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn setDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setDrawable\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDrawable](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getDrawable())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getDrawable<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getDrawable", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getDrawable\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#inflate(android.content.res.Resources,%20org.xmlpull.v1.XmlPullParser,%20android.util.AttributeSet,%20android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources", "android-content-res-Resources_Theme", "android-util-AttributeSet", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-content-res-Resources_Theme", feature = "android-util-AttributeSet", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "inflate\0", "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyTheme](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#applyTheme(android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources_Theme"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources_Theme")))]
        pub fn applyTheme<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "applyTheme", .descriptor == "(Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "applyTheme\0", "(Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [canApplyTheme](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#canApplyTheme())
        pub fn canApplyTheme<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "canApplyTheme", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "canApplyTheme\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [invalidateDrawable](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#invalidateDrawable(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn invalidateDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "invalidateDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "invalidateDrawable\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [scheduleDrawable](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#scheduleDrawable(android.graphics.drawable.Drawable,%20java.lang.Runnable,%20long))
        ///
        /// Required features: "android-graphics-drawable-Drawable", "java-lang-Runnable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable", feature = "java-lang-Runnable")))]
        pub fn scheduleDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>, arg2: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "scheduleDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;Ljava/lang/Runnable;J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "scheduleDrawable\0", "(Landroid/graphics/drawable/Drawable;Ljava/lang/Runnable;J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unscheduleDrawable](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#unscheduleDrawable(android.graphics.drawable.Drawable,%20java.lang.Runnable))
        ///
        /// Required features: "android-graphics-drawable-Drawable", "java-lang-Runnable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable", feature = "java-lang-Runnable")))]
        pub fn unscheduleDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "unscheduleDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;Ljava/lang/Runnable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "unscheduleDrawable\0", "(Landroid/graphics/drawable/Drawable;Ljava/lang/Runnable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [draw](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#draw(android.graphics.Canvas))
        ///
        /// Required features: "android-graphics-Canvas"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Canvas")))]
        pub fn draw<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Canvas>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "draw", .descriptor == "(Landroid/graphics/Canvas;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "draw\0", "(Landroid/graphics/Canvas;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChangingConfigurations](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getChangingConfigurations())
        pub fn getChangingConfigurations<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getChangingConfigurations", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getChangingConfigurations\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPadding](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getPadding(android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn getPadding<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getPadding", .descriptor == "(Landroid/graphics/Rect;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getPadding\0", "(Landroid/graphics/Rect;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOpticalInsets](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getOpticalInsets())
        ///
        /// Required features: "android-graphics-Insets"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Insets")))]
        pub fn getOpticalInsets<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Insets>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getOpticalInsets", .descriptor == "()Landroid/graphics/Insets;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getOpticalInsets\0", "()Landroid/graphics/Insets;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHotspot](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setHotspot(float,%20float))
        pub fn setHotspot<'env>(&'env self, arg0: f32, arg1: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setHotspot", .descriptor == "(FF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setHotspot\0", "(FF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHotspotBounds](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setHotspotBounds(int,%20int,%20int,%20int))
        pub fn setHotspotBounds<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setHotspotBounds", .descriptor == "(IIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setHotspotBounds\0", "(IIII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHotspotBounds](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getHotspotBounds(android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn getHotspotBounds<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getHotspotBounds", .descriptor == "(Landroid/graphics/Rect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getHotspotBounds\0", "(Landroid/graphics/Rect;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setVisible](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setVisible(boolean,%20boolean))
        pub fn setVisible<'env>(&'env self, arg0: bool, arg1: bool) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setVisible", .descriptor == "(ZZ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setVisible\0", "(ZZ)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAlpha](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setAlpha(int))
        pub fn setAlpha<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setAlpha", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setAlpha\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlpha](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getAlpha())
        pub fn getAlpha<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getAlpha", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getAlpha\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setColorFilter](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setColorFilter(android.graphics.ColorFilter))
        ///
        /// Required features: "android-graphics-ColorFilter"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorFilter")))]
        pub fn setColorFilter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::ColorFilter>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setColorFilter", .descriptor == "(Landroid/graphics/ColorFilter;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setColorFilter\0", "(Landroid/graphics/ColorFilter;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getColorFilter](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getColorFilter())
        ///
        /// Required features: "android-graphics-ColorFilter"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorFilter")))]
        pub fn getColorFilter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorFilter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getColorFilter", .descriptor == "()Landroid/graphics/ColorFilter;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getColorFilter\0", "()Landroid/graphics/ColorFilter;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTintList](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setTintList(android.content.res.ColorStateList))
        ///
        /// Required features: "android-content-res-ColorStateList"
        #[cfg(any(feature = "all", all(feature = "android-content-res-ColorStateList")))]
        pub fn setTintList<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::ColorStateList>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setTintList", .descriptor == "(Landroid/content/res/ColorStateList;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setTintList\0", "(Landroid/content/res/ColorStateList;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTintBlendMode](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#setTintBlendMode(android.graphics.BlendMode))
        ///
        /// Required features: "android-graphics-BlendMode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlendMode")))]
        pub fn setTintBlendMode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::BlendMode>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "setTintBlendMode", .descriptor == "(Landroid/graphics/BlendMode;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "setTintBlendMode\0", "(Landroid/graphics/BlendMode;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLayoutDirectionChanged](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#onLayoutDirectionChanged(int))
        pub fn onLayoutDirectionChanged<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "onLayoutDirectionChanged", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "onLayoutDirectionChanged\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOpacity](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getOpacity())
        pub fn getOpacity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getOpacity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getOpacity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isStateful](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#isStateful())
        pub fn isStateful<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "isStateful", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "isStateful\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onStateChange](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#onStateChange(int%5B%5D))
        // fn onStateChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PROTECTED, .name == "onStateChange", .descriptor == "([I)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "onStateChange\0", "([I)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onLevelChange](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#onLevelChange(int))
        // fn onLevelChange<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PROTECTED, .name == "onLevelChange", .descriptor == "(I)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "onLevelChange\0", "(I)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onBoundsChange](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#onBoundsChange(android.graphics.Rect))
        // ///
        // /// Required features: "android-graphics-Rect"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        // fn onBoundsChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PROTECTED, .name == "onBoundsChange", .descriptor == "(Landroid/graphics/Rect;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "onBoundsChange\0", "(Landroid/graphics/Rect;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getIntrinsicWidth](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getIntrinsicWidth())
        pub fn getIntrinsicWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getIntrinsicWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getIntrinsicWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntrinsicHeight](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getIntrinsicHeight())
        pub fn getIntrinsicHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getIntrinsicHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getIntrinsicHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOutline](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getOutline(android.graphics.Outline))
        ///
        /// Required features: "android-graphics-Outline"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Outline")))]
        pub fn getOutline<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Outline>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getOutline", .descriptor == "(Landroid/graphics/Outline;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getOutline\0", "(Landroid/graphics/Outline;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConstantState](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#getConstantState())
        ///
        /// Required features: "android-graphics-drawable-Drawable_ConstantState"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable_ConstantState")))]
        pub fn getConstantState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable_ConstantState>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "getConstantState", .descriptor == "()Landroid/graphics/drawable/Drawable$ConstantState;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "getConstantState\0", "()Landroid/graphics/drawable/Drawable$ConstantState;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mutate](https://developer.android.com/reference/android/graphics/drawable/DrawableWrapper.html#mutate())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn mutate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/DrawableWrapper", java.flags == PUBLIC, .name == "mutate", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/DrawableWrapper\0", "mutate\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
