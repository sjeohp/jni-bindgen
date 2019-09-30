// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-method-ArrowKeyMovementMethod"))]
__jni_bindgen! {
    /// public class [ArrowKeyMovementMethod](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html)
    ///
    /// Required feature: android-text-method-ArrowKeyMovementMethod
    public class ArrowKeyMovementMethod ("android/text/method/ArrowKeyMovementMethod") extends crate::android::text::method::BaseMovementMethod, implements crate::android::text::method::MovementMethod {

        /// [ArrowKeyMovementMethod](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#ArrowKeyMovementMethod())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::method::ArrowKeyMovementMethod>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [handleMovementKey](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#handleMovementKey(android.widget.TextView,%20android.text.Spannable,%20int,%20int,%20android.view.KeyEvent))
        // ///
        // /// Required features: "android-text-Spannable", "android-view-KeyEvent", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-view-KeyEvent", feature = "android-widget-TextView")))]
        // fn handleMovementKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "handleMovementKey", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;IILandroid/view/KeyEvent;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "handleMovementKey\0", "(Landroid/widget/TextView;Landroid/text/Spannable;IILandroid/view/KeyEvent;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [left](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#left(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn left<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "left", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "left\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [right](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#right(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn right<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "right", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "right\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [up](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#up(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn up<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "up", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "up\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [down](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#down(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn down<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "down", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "down\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [pageUp](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#pageUp(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn pageUp<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "pageUp", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "pageUp\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [pageDown](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#pageDown(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn pageDown<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "pageDown", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "pageDown\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [top](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#top(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn top<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "top", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "top\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [bottom](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#bottom(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn bottom<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "bottom", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "bottom\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [lineStart](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#lineStart(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn lineStart<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "lineStart", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "lineStart\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [lineEnd](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#lineEnd(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn lineEnd<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "lineEnd", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "lineEnd\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [home](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#home(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn home<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "home", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "home\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [end](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#end(android.widget.TextView,%20android.text.Spannable))
        // ///
        // /// Required features: "android-text-Spannable", "android-widget-TextView"
        // #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        // fn end<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PROTECTED, .name == "end", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "end\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [onTouchEvent](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#onTouchEvent(android.widget.TextView,%20android.text.Spannable,%20android.view.MotionEvent))
        ///
        /// Required features: "android-text-Spannable", "android-view-MotionEvent", "android-widget-TextView"
        #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-view-MotionEvent", feature = "android-widget-TextView")))]
        pub fn onTouchEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PUBLIC, .name == "onTouchEvent", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;Landroid/view/MotionEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "onTouchEvent\0", "(Landroid/widget/TextView;Landroid/text/Spannable;Landroid/view/MotionEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [canSelectArbitrarily](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#canSelectArbitrarily())
        pub fn canSelectArbitrarily<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PUBLIC, .name == "canSelectArbitrarily", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "canSelectArbitrarily\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initialize](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#initialize(android.widget.TextView,%20android.text.Spannable))
        ///
        /// Required features: "android-text-Spannable", "android-widget-TextView"
        #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        pub fn initialize<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PUBLIC, .name == "initialize", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "initialize\0", "(Landroid/widget/TextView;Landroid/text/Spannable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTakeFocus](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#onTakeFocus(android.widget.TextView,%20android.text.Spannable,%20int))
        ///
        /// Required features: "android-text-Spannable", "android-widget-TextView"
        #[cfg(any(feature = "all", all(feature = "android-text-Spannable", feature = "android-widget-TextView")))]
        pub fn onTakeFocus<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TextView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spannable>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PUBLIC, .name == "onTakeFocus", .descriptor == "(Landroid/widget/TextView;Landroid/text/Spannable;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/ArrowKeyMovementMethod\0", "onTakeFocus\0", "(Landroid/widget/TextView;Landroid/text/Spannable;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/text/method/ArrowKeyMovementMethod.html#getInstance())
        ///
        /// Required features: "android-text-method-MovementMethod"
        #[cfg(any(feature = "all", all(feature = "android-text-method-MovementMethod")))]
        pub fn getInstance<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::method::MovementMethod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/ArrowKeyMovementMethod", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "()Landroid/text/method/MovementMethod;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/method/ArrowKeyMovementMethod\0", "getInstance\0", "()Landroid/text/method/MovementMethod;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
