// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-appwidget-AppWidgetProvider"))]
__jni_bindgen! {
    /// public class [AppWidgetProvider](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html)
    ///
    /// Required feature: android-appwidget-AppWidgetProvider
    public class AppWidgetProvider ("android/appwidget/AppWidgetProvider") extends crate::android::content::BroadcastReceiver {

        /// [AppWidgetProvider](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#AppWidgetProvider())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetProvider>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onReceive](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onReceive(android.content.Context,%20android.content.Intent))
        ///
        /// Required features: "android-content-Context", "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-Intent")))]
        pub fn onReceive<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onReceive", .descriptor == "(Landroid/content/Context;Landroid/content/Intent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onReceive\0", "(Landroid/content/Context;Landroid/content/Intent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onUpdate](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onUpdate(android.content.Context,%20android.appwidget.AppWidgetManager,%20int%5B%5D))
        ///
        /// Required features: "android-appwidget-AppWidgetManager", "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-appwidget-AppWidgetManager", feature = "android-content-Context")))]
        pub fn onUpdate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::appwidget::AppWidgetManager>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onUpdate", .descriptor == "(Landroid/content/Context;Landroid/appwidget/AppWidgetManager;[I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onUpdate\0", "(Landroid/content/Context;Landroid/appwidget/AppWidgetManager;[I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAppWidgetOptionsChanged](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onAppWidgetOptionsChanged(android.content.Context,%20android.appwidget.AppWidgetManager,%20int,%20android.os.Bundle))
        ///
        /// Required features: "android-appwidget-AppWidgetManager", "android-content-Context", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-appwidget-AppWidgetManager", feature = "android-content-Context", feature = "android-os-Bundle")))]
        pub fn onAppWidgetOptionsChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::appwidget::AppWidgetManager>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onAppWidgetOptionsChanged", .descriptor == "(Landroid/content/Context;Landroid/appwidget/AppWidgetManager;ILandroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onAppWidgetOptionsChanged\0", "(Landroid/content/Context;Landroid/appwidget/AppWidgetManager;ILandroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDeleted](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onDeleted(android.content.Context,%20int%5B%5D))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn onDeleted<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onDeleted", .descriptor == "(Landroid/content/Context;[I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onDeleted\0", "(Landroid/content/Context;[I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onEnabled](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onEnabled(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn onEnabled<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onEnabled", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onEnabled\0", "(Landroid/content/Context;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisabled](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onDisabled(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn onDisabled<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onDisabled", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onDisabled\0", "(Landroid/content/Context;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRestored](https://developer.android.com/reference/android/appwidget/AppWidgetProvider.html#onRestored(android.content.Context,%20int%5B%5D,%20int%5B%5D))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn onRestored<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProvider", java.flags == PUBLIC, .name == "onRestored", .descriptor == "(Landroid/content/Context;[I[I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProvider\0", "onRestored\0", "(Landroid/content/Context;[I[I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
