// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ProgressDialog"))]
__jni_bindgen! {
    /// public class [ProgressDialog](https://developer.android.com/reference/android/app/ProgressDialog.html)
    ///
    /// Required feature: android-app-ProgressDialog
    #[deprecated] public class ProgressDialog ("android/app/ProgressDialog") extends crate::android::app::AlertDialog {

        /// [ProgressDialog](https://developer.android.com/reference/android/app/ProgressDialog.html#ProgressDialog(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        #[deprecated] pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ProgressDialog>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ProgressDialog](https://developer.android.com/reference/android/app/ProgressDialog.html#ProgressDialog(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        #[deprecated] pub fn new_Context_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ProgressDialog>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "<init>\0", "(Landroid/content/Context;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/ProgressDialog.html#show(android.content.Context,%20java.lang.CharSequence,%20java.lang.CharSequence))
        ///
        /// Required features: "android-app-ProgressDialog", "android-content-Context", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-ProgressDialog", feature = "android-content-Context", feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn show_Context_CharSequence_CharSequence<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ProgressDialog>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC | STATIC, .name == "show", .descriptor == "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/app/ProgressDialog;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/ProgressDialog\0", "show\0", "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/app/ProgressDialog;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/ProgressDialog.html#show(android.content.Context,%20java.lang.CharSequence,%20java.lang.CharSequence,%20boolean))
        ///
        /// Required features: "android-app-ProgressDialog", "android-content-Context", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-ProgressDialog", feature = "android-content-Context", feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn show_Context_CharSequence_CharSequence_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg3: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ProgressDialog>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC | STATIC, .name == "show", .descriptor == "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;Z)Landroid/app/ProgressDialog;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/ProgressDialog\0", "show\0", "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;Z)Landroid/app/ProgressDialog;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/ProgressDialog.html#show(android.content.Context,%20java.lang.CharSequence,%20java.lang.CharSequence,%20boolean,%20boolean))
        ///
        /// Required features: "android-app-ProgressDialog", "android-content-Context", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-ProgressDialog", feature = "android-content-Context", feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn show_Context_CharSequence_CharSequence_boolean_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg3: bool, arg4: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ProgressDialog>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC | STATIC, .name == "show", .descriptor == "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;ZZ)Landroid/app/ProgressDialog;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/ProgressDialog\0", "show\0", "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;ZZ)Landroid/app/ProgressDialog;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/ProgressDialog.html#show(android.content.Context,%20java.lang.CharSequence,%20java.lang.CharSequence,%20boolean,%20boolean,%20android.content.DialogInterface.OnCancelListener))
        ///
        /// Required features: "android-app-ProgressDialog", "android-content-Context", "android-content-DialogInterface_OnCancelListener", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-ProgressDialog", feature = "android-content-Context", feature = "android-content-DialogInterface_OnCancelListener", feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn show_Context_CharSequence_CharSequence_boolean_boolean_OnCancelListener<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg3: bool, arg4: bool, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::DialogInterface_OnCancelListener>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ProgressDialog>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC | STATIC, .name == "show", .descriptor == "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;ZZLandroid/content/DialogInterface$OnCancelListener;)Landroid/app/ProgressDialog;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/ProgressDialog\0", "show\0", "(Landroid/content/Context;Ljava/lang/CharSequence;Ljava/lang/CharSequence;ZZLandroid/content/DialogInterface$OnCancelListener;)Landroid/app/ProgressDialog;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onCreate](https://developer.android.com/reference/android/app/ProgressDialog.html#onCreate(android.os.Bundle))
        // ///
        // /// Required features: "android-os-Bundle"
        // #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        // #[deprecated] fn onCreate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/ProgressDialog", java.flags == PROTECTED, .name == "onCreate", .descriptor == "(Landroid/os/Bundle;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "onCreate\0", "(Landroid/os/Bundle;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [onStart](https://developer.android.com/reference/android/app/ProgressDialog.html#onStart())
        #[deprecated] pub fn onStart<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "onStart", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "onStart\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onStop](https://developer.android.com/reference/android/app/ProgressDialog.html#onStop())
        // #[deprecated] fn onStop<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/ProgressDialog", java.flags == PROTECTED, .name == "onStop", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "onStop\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [setProgress](https://developer.android.com/reference/android/app/ProgressDialog.html#setProgress(int))
        #[deprecated] pub fn setProgress<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setProgress", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setProgress\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSecondaryProgress](https://developer.android.com/reference/android/app/ProgressDialog.html#setSecondaryProgress(int))
        #[deprecated] pub fn setSecondaryProgress<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setSecondaryProgress", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setSecondaryProgress\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProgress](https://developer.android.com/reference/android/app/ProgressDialog.html#getProgress())
        #[deprecated] pub fn getProgress<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "getProgress", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "getProgress\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSecondaryProgress](https://developer.android.com/reference/android/app/ProgressDialog.html#getSecondaryProgress())
        #[deprecated] pub fn getSecondaryProgress<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "getSecondaryProgress", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "getSecondaryProgress\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMax](https://developer.android.com/reference/android/app/ProgressDialog.html#getMax())
        #[deprecated] pub fn getMax<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "getMax", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "getMax\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMax](https://developer.android.com/reference/android/app/ProgressDialog.html#setMax(int))
        #[deprecated] pub fn setMax<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setMax", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setMax\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [incrementProgressBy](https://developer.android.com/reference/android/app/ProgressDialog.html#incrementProgressBy(int))
        #[deprecated] pub fn incrementProgressBy<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "incrementProgressBy", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "incrementProgressBy\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [incrementSecondaryProgressBy](https://developer.android.com/reference/android/app/ProgressDialog.html#incrementSecondaryProgressBy(int))
        #[deprecated] pub fn incrementSecondaryProgressBy<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "incrementSecondaryProgressBy", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "incrementSecondaryProgressBy\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setProgressDrawable](https://developer.android.com/reference/android/app/ProgressDialog.html#setProgressDrawable(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        #[deprecated] pub fn setProgressDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setProgressDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setProgressDrawable\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIndeterminateDrawable](https://developer.android.com/reference/android/app/ProgressDialog.html#setIndeterminateDrawable(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        #[deprecated] pub fn setIndeterminateDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setIndeterminateDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setIndeterminateDrawable\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIndeterminate](https://developer.android.com/reference/android/app/ProgressDialog.html#setIndeterminate(boolean))
        #[deprecated] pub fn setIndeterminate<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setIndeterminate", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setIndeterminate\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isIndeterminate](https://developer.android.com/reference/android/app/ProgressDialog.html#isIndeterminate())
        #[deprecated] pub fn isIndeterminate<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "isIndeterminate", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "isIndeterminate\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMessage](https://developer.android.com/reference/android/app/ProgressDialog.html#setMessage(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn setMessage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setMessage", .descriptor == "(Ljava/lang/CharSequence;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setMessage\0", "(Ljava/lang/CharSequence;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setProgressStyle](https://developer.android.com/reference/android/app/ProgressDialog.html#setProgressStyle(int))
        #[deprecated] pub fn setProgressStyle<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setProgressStyle", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setProgressStyle\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setProgressNumberFormat](https://developer.android.com/reference/android/app/ProgressDialog.html#setProgressNumberFormat(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn setProgressNumberFormat<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setProgressNumberFormat", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setProgressNumberFormat\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setProgressPercentFormat](https://developer.android.com/reference/android/app/ProgressDialog.html#setProgressPercentFormat(java.text.NumberFormat))
        ///
        /// Required features: "java-text-NumberFormat"
        #[cfg(any(feature = "all", all(feature = "java-text-NumberFormat")))]
        #[deprecated] pub fn setProgressPercentFormat<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::NumberFormat>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ProgressDialog", java.flags == PUBLIC, .name == "setProgressPercentFormat", .descriptor == "(Ljava/text/NumberFormat;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ProgressDialog\0", "setProgressPercentFormat\0", "(Ljava/text/NumberFormat;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [STYLE_HORIZONTAL](https://developer.android.com/reference/android/app/ProgressDialog.html#STYLE_HORIZONTAL)
        #[deprecated] pub const STYLE_HORIZONTAL : i32 = 1;

        /// public static final [STYLE_SPINNER](https://developer.android.com/reference/android/app/ProgressDialog.html#STYLE_SPINNER)
        #[deprecated] pub const STYLE_SPINNER : i32 = 0;
    }
}
