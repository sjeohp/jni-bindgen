// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-drawable-InsetDrawable"))]
__jni_bindgen! {
    /// public class [InsetDrawable](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html)
    ///
    /// Required feature: android-graphics-drawable-InsetDrawable
    public class InsetDrawable ("android/graphics/drawable/InsetDrawable") extends crate::android::graphics::drawable::DrawableWrapper {

        /// [InsetDrawable](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#InsetDrawable(android.graphics.drawable.Drawable,%20int))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn new_Drawable_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::InsetDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/drawable/Drawable;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "<init>\0", "(Landroid/graphics/drawable/Drawable;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InsetDrawable](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#InsetDrawable(android.graphics.drawable.Drawable,%20float))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn new_Drawable_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>, arg1: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::InsetDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/drawable/Drawable;F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "<init>\0", "(Landroid/graphics/drawable/Drawable;F)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InsetDrawable](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#InsetDrawable(android.graphics.drawable.Drawable,%20int,%20int,%20int,%20int))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn new_Drawable_int_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::InsetDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/drawable/Drawable;IIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "<init>\0", "(Landroid/graphics/drawable/Drawable;IIII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InsetDrawable](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#InsetDrawable(android.graphics.drawable.Drawable,%20float,%20float,%20float,%20float))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn new_Drawable_float_float_float_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>, arg1: f32, arg2: f32, arg3: f32, arg4: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::InsetDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/drawable/Drawable;FFFF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "<init>\0", "(Landroid/graphics/drawable/Drawable;FFFF)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#inflate(android.content.res.Resources,%20org.xmlpull.v1.XmlPullParser,%20android.util.AttributeSet,%20android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources", "android-content-res-Resources_Theme", "android-util-AttributeSet", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-content-res-Resources_Theme", feature = "android-util-AttributeSet", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "inflate\0", "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyTheme](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#applyTheme(android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources_Theme"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources_Theme")))]
        pub fn applyTheme<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "applyTheme", .descriptor == "(Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "applyTheme\0", "(Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPadding](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#getPadding(android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn getPadding<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "getPadding", .descriptor == "(Landroid/graphics/Rect;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "getPadding\0", "(Landroid/graphics/Rect;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOpticalInsets](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#getOpticalInsets())
        ///
        /// Required features: "android-graphics-Insets"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Insets")))]
        pub fn getOpticalInsets<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Insets>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "getOpticalInsets", .descriptor == "()Landroid/graphics/Insets;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "getOpticalInsets\0", "()Landroid/graphics/Insets;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOpacity](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#getOpacity())
        pub fn getOpacity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "getOpacity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "getOpacity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onBoundsChange](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#onBoundsChange(android.graphics.Rect))
        // ///
        // /// Required features: "android-graphics-Rect"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        // fn onBoundsChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PROTECTED, .name == "onBoundsChange", .descriptor == "(Landroid/graphics/Rect;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "onBoundsChange\0", "(Landroid/graphics/Rect;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getIntrinsicWidth](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#getIntrinsicWidth())
        pub fn getIntrinsicWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "getIntrinsicWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "getIntrinsicWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntrinsicHeight](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#getIntrinsicHeight())
        pub fn getIntrinsicHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "getIntrinsicHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "getIntrinsicHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOutline](https://developer.android.com/reference/android/graphics/drawable/InsetDrawable.html#getOutline(android.graphics.Outline))
        ///
        /// Required features: "android-graphics-Outline"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Outline")))]
        pub fn getOutline<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Outline>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/InsetDrawable", java.flags == PUBLIC, .name == "getOutline", .descriptor == "(Landroid/graphics/Outline;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/InsetDrawable\0", "getOutline\0", "(Landroid/graphics/Outline;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
