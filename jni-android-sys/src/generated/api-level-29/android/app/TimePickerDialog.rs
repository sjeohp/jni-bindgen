// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-TimePickerDialog"))]
__jni_bindgen! {
    /// public class [TimePickerDialog](https://developer.android.com/reference/android/app/TimePickerDialog.html)
    ///
    /// Required feature: android-app-TimePickerDialog
    public class TimePickerDialog ("android/app/TimePickerDialog") extends crate::android::app::AlertDialog, implements crate::android::content::DialogInterface_OnClickListener, crate::android::widget::TimePicker_OnTimeChangedListener {

        /// [TimePickerDialog](https://developer.android.com/reference/android/app/TimePickerDialog.html#TimePickerDialog(android.content.Context,%20android.app.TimePickerDialog.OnTimeSetListener,%20int,%20int,%20boolean))
        ///
        /// Required features: "android-app-TimePickerDialog_OnTimeSetListener", "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-app-TimePickerDialog_OnTimeSetListener", feature = "android-content-Context")))]
        pub fn new_Context_OnTimeSetListener_int_int_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::TimePickerDialog_OnTimeSetListener>>, arg2: i32, arg3: i32, arg4: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::TimePickerDialog>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/app/TimePickerDialog$OnTimeSetListener;IIZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "<init>\0", "(Landroid/content/Context;Landroid/app/TimePickerDialog$OnTimeSetListener;IIZ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [TimePickerDialog](https://developer.android.com/reference/android/app/TimePickerDialog.html#TimePickerDialog(android.content.Context,%20int,%20android.app.TimePickerDialog.OnTimeSetListener,%20int,%20int,%20boolean))
        ///
        /// Required features: "android-app-TimePickerDialog_OnTimeSetListener", "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-app-TimePickerDialog_OnTimeSetListener", feature = "android-content-Context")))]
        pub fn new_Context_int_OnTimeSetListener_int_int_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::TimePickerDialog_OnTimeSetListener>>, arg3: i32, arg4: i32, arg5: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::TimePickerDialog>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;ILandroid/app/TimePickerDialog$OnTimeSetListener;IIZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "<init>\0", "(Landroid/content/Context;ILandroid/app/TimePickerDialog$OnTimeSetListener;IIZ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTimeChanged](https://developer.android.com/reference/android/app/TimePickerDialog.html#onTimeChanged(android.widget.TimePicker,%20int,%20int))
        ///
        /// Required features: "android-widget-TimePicker"
        #[cfg(any(feature = "all", all(feature = "android-widget-TimePicker")))]
        pub fn onTimeChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::TimePicker>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "onTimeChanged", .descriptor == "(Landroid/widget/TimePicker;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "onTimeChanged\0", "(Landroid/widget/TimePicker;II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/TimePickerDialog.html#show())
        pub fn show<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "show", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "show\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onClick](https://developer.android.com/reference/android/app/TimePickerDialog.html#onClick(android.content.DialogInterface,%20int))
        ///
        /// Required features: "android-content-DialogInterface"
        #[cfg(any(feature = "all", all(feature = "android-content-DialogInterface")))]
        pub fn onClick<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::DialogInterface>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "onClick", .descriptor == "(Landroid/content/DialogInterface;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "onClick\0", "(Landroid/content/DialogInterface;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateTime](https://developer.android.com/reference/android/app/TimePickerDialog.html#updateTime(int,%20int))
        pub fn updateTime<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "updateTime", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "updateTime\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSaveInstanceState](https://developer.android.com/reference/android/app/TimePickerDialog.html#onSaveInstanceState())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn onSaveInstanceState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "onSaveInstanceState", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "onSaveInstanceState\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRestoreInstanceState](https://developer.android.com/reference/android/app/TimePickerDialog.html#onRestoreInstanceState(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn onRestoreInstanceState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/TimePickerDialog", java.flags == PUBLIC, .name == "onRestoreInstanceState", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/TimePickerDialog\0", "onRestoreInstanceState\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
