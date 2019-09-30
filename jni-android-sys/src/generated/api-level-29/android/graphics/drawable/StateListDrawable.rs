// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-drawable-StateListDrawable"))]
__jni_bindgen! {
    /// public class [StateListDrawable](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html)
    ///
    /// Required feature: android-graphics-drawable-StateListDrawable
    public class StateListDrawable ("android/graphics/drawable/StateListDrawable") extends crate::android::graphics::drawable::DrawableContainer {

        /// [StateListDrawable](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#StateListDrawable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::StateListDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addState](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#addState(int%5B%5D,%20android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn addState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "addState", .descriptor == "([ILandroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "addState\0", "([ILandroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isStateful](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#isStateful())
        pub fn isStateful<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "isStateful", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "isStateful\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onStateChange](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#onStateChange(int%5B%5D))
        // fn onStateChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PROTECTED, .name == "onStateChange", .descriptor == "([I)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "onStateChange\0", "([I)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [inflate](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#inflate(android.content.res.Resources,%20org.xmlpull.v1.XmlPullParser,%20android.util.AttributeSet,%20android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources", "android-content-res-Resources_Theme", "android-util-AttributeSet", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-content-res-Resources_Theme", feature = "android-util-AttributeSet", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "inflate\0", "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStateCount](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#getStateCount())
        pub fn getStateCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "getStateCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "getStateCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStateSet](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#getStateSet(int))
        pub fn getStateSet<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "getStateSet", .descriptor == "(I)[I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "getStateSet\0", "(I)[I\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStateDrawable](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#getStateDrawable(int))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getStateDrawable<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "getStateDrawable", .descriptor == "(I)Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "getStateDrawable\0", "(I)Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [findStateDrawableIndex](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#findStateDrawableIndex(int%5B%5D))
        pub fn findStateDrawableIndex<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "findStateDrawableIndex", .descriptor == "([I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "findStateDrawableIndex\0", "([I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mutate](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#mutate())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn mutate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "mutate", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "mutate\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyTheme](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#applyTheme(android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources_Theme"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources_Theme")))]
        pub fn applyTheme<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PUBLIC, .name == "applyTheme", .descriptor == "(Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "applyTheme\0", "(Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [setConstantState](https://developer.android.com/reference/android/graphics/drawable/StateListDrawable.html#setConstantState(android.graphics.drawable.DrawableContainer.DrawableContainerState))
        // ///
        // /// Required features: "android-graphics-drawable-DrawableContainer_DrawableContainerState"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-DrawableContainer_DrawableContainerState")))]
        // fn setConstantState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::DrawableContainer_DrawableContainerState>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/StateListDrawable", java.flags == PROTECTED, .name == "setConstantState", .descriptor == "(Landroid/graphics/drawable/DrawableContainer$DrawableContainerState;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/StateListDrawable\0", "setConstantState\0", "(Landroid/graphics/drawable/DrawableContainer$DrawableContainerState;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
