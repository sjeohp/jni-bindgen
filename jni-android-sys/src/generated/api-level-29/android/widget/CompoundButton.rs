// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-CompoundButton"))]
__jni_bindgen! {
    /// public class [CompoundButton](https://developer.android.com/reference/android/widget/CompoundButton.html)
    ///
    /// Required feature: android-widget-CompoundButton
    public class CompoundButton ("android/widget/CompoundButton") extends crate::android::widget::Button, implements crate::android::widget::Checkable {

        /// [CompoundButton](https://developer.android.com/reference/android/widget/CompoundButton.html#CompoundButton(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::CompoundButton>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CompoundButton](https://developer.android.com/reference/android/widget/CompoundButton.html#CompoundButton(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::CompoundButton>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CompoundButton](https://developer.android.com/reference/android/widget/CompoundButton.html#CompoundButton(android.content.Context,%20android.util.AttributeSet,%20int))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::CompoundButton>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CompoundButton](https://developer.android.com/reference/android/widget/CompoundButton.html#CompoundButton(android.content.Context,%20android.util.AttributeSet,%20int,%20int))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::CompoundButton>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toggle](https://developer.android.com/reference/android/widget/CompoundButton.html#toggle())
        pub fn toggle<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "toggle", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "toggle\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [performClick](https://developer.android.com/reference/android/widget/CompoundButton.html#performClick())
        pub fn performClick<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "performClick", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "performClick\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isChecked](https://developer.android.com/reference/android/widget/CompoundButton.html#isChecked())
        pub fn isChecked<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "isChecked", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "isChecked\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setChecked](https://developer.android.com/reference/android/widget/CompoundButton.html#setChecked(boolean))
        pub fn setChecked<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setChecked", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setChecked\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOnCheckedChangeListener](https://developer.android.com/reference/android/widget/CompoundButton.html#setOnCheckedChangeListener(android.widget.CompoundButton.OnCheckedChangeListener))
        ///
        /// Required features: "android-widget-CompoundButton_OnCheckedChangeListener"
        #[cfg(any(feature = "all", all(feature = "android-widget-CompoundButton_OnCheckedChangeListener")))]
        pub fn setOnCheckedChangeListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::CompoundButton_OnCheckedChangeListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setOnCheckedChangeListener", .descriptor == "(Landroid/widget/CompoundButton$OnCheckedChangeListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setOnCheckedChangeListener\0", "(Landroid/widget/CompoundButton$OnCheckedChangeListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setButtonDrawable](https://developer.android.com/reference/android/widget/CompoundButton.html#setButtonDrawable(int))
        pub fn setButtonDrawable_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setButtonDrawable", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setButtonDrawable\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setButtonDrawable](https://developer.android.com/reference/android/widget/CompoundButton.html#setButtonDrawable(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn setButtonDrawable_Drawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setButtonDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setButtonDrawable\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getButtonDrawable](https://developer.android.com/reference/android/widget/CompoundButton.html#getButtonDrawable())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getButtonDrawable<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getButtonDrawable", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getButtonDrawable\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setButtonTintList](https://developer.android.com/reference/android/widget/CompoundButton.html#setButtonTintList(android.content.res.ColorStateList))
        ///
        /// Required features: "android-content-res-ColorStateList"
        #[cfg(any(feature = "all", all(feature = "android-content-res-ColorStateList")))]
        pub fn setButtonTintList<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::ColorStateList>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setButtonTintList", .descriptor == "(Landroid/content/res/ColorStateList;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setButtonTintList\0", "(Landroid/content/res/ColorStateList;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getButtonTintList](https://developer.android.com/reference/android/widget/CompoundButton.html#getButtonTintList())
        ///
        /// Required features: "android-content-res-ColorStateList"
        #[cfg(any(feature = "all", all(feature = "android-content-res-ColorStateList")))]
        pub fn getButtonTintList<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::res::ColorStateList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getButtonTintList", .descriptor == "()Landroid/content/res/ColorStateList;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getButtonTintList\0", "()Landroid/content/res/ColorStateList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setButtonTintMode](https://developer.android.com/reference/android/widget/CompoundButton.html#setButtonTintMode(android.graphics.PorterDuff.Mode))
        ///
        /// Required features: "android-graphics-PorterDuff_Mode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-PorterDuff_Mode")))]
        pub fn setButtonTintMode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::PorterDuff_Mode>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setButtonTintMode", .descriptor == "(Landroid/graphics/PorterDuff$Mode;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setButtonTintMode\0", "(Landroid/graphics/PorterDuff$Mode;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setButtonTintBlendMode](https://developer.android.com/reference/android/widget/CompoundButton.html#setButtonTintBlendMode(android.graphics.BlendMode))
        ///
        /// Required features: "android-graphics-BlendMode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlendMode")))]
        pub fn setButtonTintBlendMode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::BlendMode>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "setButtonTintBlendMode", .descriptor == "(Landroid/graphics/BlendMode;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "setButtonTintBlendMode\0", "(Landroid/graphics/BlendMode;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getButtonTintMode](https://developer.android.com/reference/android/widget/CompoundButton.html#getButtonTintMode())
        ///
        /// Required features: "android-graphics-PorterDuff_Mode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-PorterDuff_Mode")))]
        pub fn getButtonTintMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getButtonTintMode", .descriptor == "()Landroid/graphics/PorterDuff$Mode;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getButtonTintMode\0", "()Landroid/graphics/PorterDuff$Mode;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getButtonTintBlendMode](https://developer.android.com/reference/android/widget/CompoundButton.html#getButtonTintBlendMode())
        ///
        /// Required features: "android-graphics-BlendMode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlendMode")))]
        pub fn getButtonTintBlendMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getButtonTintBlendMode", .descriptor == "()Landroid/graphics/BlendMode;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getButtonTintBlendMode\0", "()Landroid/graphics/BlendMode;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccessibilityClassName](https://developer.android.com/reference/android/widget/CompoundButton.html#getAccessibilityClassName())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getAccessibilityClassName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getAccessibilityClassName", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getAccessibilityClassName\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCompoundPaddingLeft](https://developer.android.com/reference/android/widget/CompoundButton.html#getCompoundPaddingLeft())
        pub fn getCompoundPaddingLeft<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getCompoundPaddingLeft", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getCompoundPaddingLeft\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCompoundPaddingRight](https://developer.android.com/reference/android/widget/CompoundButton.html#getCompoundPaddingRight())
        pub fn getCompoundPaddingRight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getCompoundPaddingRight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getCompoundPaddingRight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onDraw](https://developer.android.com/reference/android/widget/CompoundButton.html#onDraw(android.graphics.Canvas))
        // ///
        // /// Required features: "android-graphics-Canvas"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-Canvas")))]
        // fn onDraw<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Canvas>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/CompoundButton", java.flags == PROTECTED, .name == "onDraw", .descriptor == "(Landroid/graphics/Canvas;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "onDraw\0", "(Landroid/graphics/Canvas;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onCreateDrawableState](https://developer.android.com/reference/android/widget/CompoundButton.html#onCreateDrawableState(int))
        // fn onCreateDrawableState<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/CompoundButton", java.flags == PROTECTED, .name == "onCreateDrawableState", .descriptor == "(I)[I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "onCreateDrawableState\0", "(I)[I\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [drawableStateChanged](https://developer.android.com/reference/android/widget/CompoundButton.html#drawableStateChanged())
        // fn drawableStateChanged<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/CompoundButton", java.flags == PROTECTED, .name == "drawableStateChanged", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "drawableStateChanged\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [drawableHotspotChanged](https://developer.android.com/reference/android/widget/CompoundButton.html#drawableHotspotChanged(float,%20float))
        pub fn drawableHotspotChanged<'env>(&'env self, arg0: f32, arg1: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "drawableHotspotChanged", .descriptor == "(FF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "drawableHotspotChanged\0", "(FF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [verifyDrawable](https://developer.android.com/reference/android/widget/CompoundButton.html#verifyDrawable(android.graphics.drawable.Drawable))
        // ///
        // /// Required features: "android-graphics-drawable-Drawable"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        // fn verifyDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/CompoundButton", java.flags == PROTECTED, .name == "verifyDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "verifyDrawable\0", "(Landroid/graphics/drawable/Drawable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [jumpDrawablesToCurrentState](https://developer.android.com/reference/android/widget/CompoundButton.html#jumpDrawablesToCurrentState())
        pub fn jumpDrawablesToCurrentState<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "jumpDrawablesToCurrentState", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "jumpDrawablesToCurrentState\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSaveInstanceState](https://developer.android.com/reference/android/widget/CompoundButton.html#onSaveInstanceState())
        ///
        /// Required features: "android-os-Parcelable"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcelable")))]
        pub fn onSaveInstanceState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "onSaveInstanceState", .descriptor == "()Landroid/os/Parcelable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "onSaveInstanceState\0", "()Landroid/os/Parcelable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRestoreInstanceState](https://developer.android.com/reference/android/widget/CompoundButton.html#onRestoreInstanceState(android.os.Parcelable))
        ///
        /// Required features: "android-os-Parcelable"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcelable")))]
        pub fn onRestoreInstanceState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcelable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "onRestoreInstanceState", .descriptor == "(Landroid/os/Parcelable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "onRestoreInstanceState\0", "(Landroid/os/Parcelable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [autofill](https://developer.android.com/reference/android/widget/CompoundButton.html#autofill(android.view.autofill.AutofillValue))
        ///
        /// Required features: "android-view-autofill-AutofillValue"
        #[cfg(any(feature = "all", all(feature = "android-view-autofill-AutofillValue")))]
        pub fn autofill<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::autofill::AutofillValue>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "autofill", .descriptor == "(Landroid/view/autofill/AutofillValue;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "autofill\0", "(Landroid/view/autofill/AutofillValue;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAutofillType](https://developer.android.com/reference/android/widget/CompoundButton.html#getAutofillType())
        pub fn getAutofillType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getAutofillType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getAutofillType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAutofillValue](https://developer.android.com/reference/android/widget/CompoundButton.html#getAutofillValue())
        ///
        /// Required features: "android-view-autofill-AutofillValue"
        #[cfg(any(feature = "all", all(feature = "android-view-autofill-AutofillValue")))]
        pub fn getAutofillValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::autofill::AutofillValue>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/CompoundButton", java.flags == PUBLIC, .name == "getAutofillValue", .descriptor == "()Landroid/view/autofill/AutofillValue;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/CompoundButton\0", "getAutofillValue\0", "()Landroid/view/autofill/AutofillValue;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
