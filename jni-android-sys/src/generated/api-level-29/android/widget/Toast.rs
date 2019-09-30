// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-Toast"))]
__jni_bindgen! {
    /// public class [Toast](https://developer.android.com/reference/android/widget/Toast.html)
    ///
    /// Required feature: android-widget-Toast
    public class Toast ("android/widget/Toast") extends crate::java::lang::Object {

        /// [Toast](https://developer.android.com/reference/android/widget/Toast.html#Toast(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::Toast>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/widget/Toast.html#show())
        pub fn show<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "show", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "show\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cancel](https://developer.android.com/reference/android/widget/Toast.html#cancel())
        pub fn cancel<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "cancel", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "cancel\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setView](https://developer.android.com/reference/android/widget/Toast.html#setView(android.view.View))
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        pub fn setView<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "setView", .descriptor == "(Landroid/view/View;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "setView\0", "(Landroid/view/View;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getView](https://developer.android.com/reference/android/widget/Toast.html#getView())
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        pub fn getView<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getView", .descriptor == "()Landroid/view/View;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getView\0", "()Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDuration](https://developer.android.com/reference/android/widget/Toast.html#setDuration(int))
        pub fn setDuration<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "setDuration", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "setDuration\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDuration](https://developer.android.com/reference/android/widget/Toast.html#getDuration())
        pub fn getDuration<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getDuration", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getDuration\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMargin](https://developer.android.com/reference/android/widget/Toast.html#setMargin(float,%20float))
        pub fn setMargin<'env>(&'env self, arg0: f32, arg1: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "setMargin", .descriptor == "(FF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "setMargin\0", "(FF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHorizontalMargin](https://developer.android.com/reference/android/widget/Toast.html#getHorizontalMargin())
        pub fn getHorizontalMargin<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getHorizontalMargin", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getHorizontalMargin\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVerticalMargin](https://developer.android.com/reference/android/widget/Toast.html#getVerticalMargin())
        pub fn getVerticalMargin<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getVerticalMargin", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getVerticalMargin\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setGravity](https://developer.android.com/reference/android/widget/Toast.html#setGravity(int,%20int,%20int))
        pub fn setGravity<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "setGravity", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "setGravity\0", "(III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGravity](https://developer.android.com/reference/android/widget/Toast.html#getGravity())
        pub fn getGravity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getGravity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getGravity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getXOffset](https://developer.android.com/reference/android/widget/Toast.html#getXOffset())
        pub fn getXOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getXOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getXOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getYOffset](https://developer.android.com/reference/android/widget/Toast.html#getYOffset())
        pub fn getYOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "getYOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "getYOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [makeText](https://developer.android.com/reference/android/widget/Toast.html#makeText(android.content.Context,%20java.lang.CharSequence,%20int))
        ///
        /// Required features: "android-content-Context", "android-widget-Toast", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-widget-Toast", feature = "java-lang-CharSequence")))]
        pub fn makeText_Context_CharSequence_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::Toast>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC | STATIC, .name == "makeText", .descriptor == "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/widget/Toast\0", "makeText\0", "(Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [makeText](https://developer.android.com/reference/android/widget/Toast.html#makeText(android.content.Context,%20int,%20int))
        ///
        /// Required features: "android-content-Context", "android-widget-Toast"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-widget-Toast")))]
        pub fn makeText_Context_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::Toast>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC | STATIC, .name == "makeText", .descriptor == "(Landroid/content/Context;II)Landroid/widget/Toast;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/widget/Toast\0", "makeText\0", "(Landroid/content/Context;II)Landroid/widget/Toast;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setText](https://developer.android.com/reference/android/widget/Toast.html#setText(int))
        pub fn setText_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "setText", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "setText\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setText](https://developer.android.com/reference/android/widget/Toast.html#setText(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn setText_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Toast", java.flags == PUBLIC, .name == "setText", .descriptor == "(Ljava/lang/CharSequence;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Toast\0", "setText\0", "(Ljava/lang/CharSequence;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [LENGTH_LONG](https://developer.android.com/reference/android/widget/Toast.html#LENGTH_LONG)
        pub const LENGTH_LONG : i32 = 1;

        /// public static final [LENGTH_SHORT](https://developer.android.com/reference/android/widget/Toast.html#LENGTH_SHORT)
        pub const LENGTH_SHORT : i32 = 0;
    }
}
