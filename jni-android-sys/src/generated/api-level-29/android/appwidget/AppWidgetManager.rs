// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-appwidget-AppWidgetManager"))]
__jni_bindgen! {
    /// public class [AppWidgetManager](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html)
    ///
    /// Required feature: android-appwidget-AppWidgetManager
    public class AppWidgetManager ("android/appwidget/AppWidgetManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [AppWidgetManager](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#AppWidgetManager())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/appwidget/AppWidgetManager", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getInstance(android.content.Context))
        ///
        /// Required features: "android-appwidget-AppWidgetManager", "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-appwidget-AppWidgetManager", feature = "android-content-Context")))]
        pub fn getInstance<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetManager>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Landroid/content/Context;)Landroid/appwidget/AppWidgetManager;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/appwidget/AppWidgetManager\0", "getInstance\0", "(Landroid/content/Context;)Landroid/appwidget/AppWidgetManager;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateAppWidget](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#updateAppWidget(int%5B%5D,%20android.widget.RemoteViews))
        ///
        /// Required features: "android-widget-RemoteViews"
        #[cfg(any(feature = "all", all(feature = "android-widget-RemoteViews")))]
        pub fn updateAppWidget_int_array_RemoteViews<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::RemoteViews>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "updateAppWidget", .descriptor == "([ILandroid/widget/RemoteViews;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "updateAppWidget\0", "([ILandroid/widget/RemoteViews;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateAppWidgetOptions](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#updateAppWidgetOptions(int,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn updateAppWidgetOptions<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "updateAppWidgetOptions", .descriptor == "(ILandroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "updateAppWidgetOptions\0", "(ILandroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAppWidgetOptions](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getAppWidgetOptions(int))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getAppWidgetOptions<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "getAppWidgetOptions", .descriptor == "(I)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "getAppWidgetOptions\0", "(I)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateAppWidget](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#updateAppWidget(int,%20android.widget.RemoteViews))
        ///
        /// Required features: "android-widget-RemoteViews"
        #[cfg(any(feature = "all", all(feature = "android-widget-RemoteViews")))]
        pub fn updateAppWidget_int_RemoteViews<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::RemoteViews>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "updateAppWidget", .descriptor == "(ILandroid/widget/RemoteViews;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "updateAppWidget\0", "(ILandroid/widget/RemoteViews;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [partiallyUpdateAppWidget](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#partiallyUpdateAppWidget(int%5B%5D,%20android.widget.RemoteViews))
        ///
        /// Required features: "android-widget-RemoteViews"
        #[cfg(any(feature = "all", all(feature = "android-widget-RemoteViews")))]
        pub fn partiallyUpdateAppWidget_int_array_RemoteViews<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::RemoteViews>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "partiallyUpdateAppWidget", .descriptor == "([ILandroid/widget/RemoteViews;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "partiallyUpdateAppWidget\0", "([ILandroid/widget/RemoteViews;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [partiallyUpdateAppWidget](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#partiallyUpdateAppWidget(int,%20android.widget.RemoteViews))
        ///
        /// Required features: "android-widget-RemoteViews"
        #[cfg(any(feature = "all", all(feature = "android-widget-RemoteViews")))]
        pub fn partiallyUpdateAppWidget_int_RemoteViews<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::RemoteViews>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "partiallyUpdateAppWidget", .descriptor == "(ILandroid/widget/RemoteViews;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "partiallyUpdateAppWidget\0", "(ILandroid/widget/RemoteViews;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateAppWidget](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#updateAppWidget(android.content.ComponentName,%20android.widget.RemoteViews))
        ///
        /// Required features: "android-content-ComponentName", "android-widget-RemoteViews"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-widget-RemoteViews")))]
        pub fn updateAppWidget_ComponentName_RemoteViews<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::RemoteViews>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "updateAppWidget", .descriptor == "(Landroid/content/ComponentName;Landroid/widget/RemoteViews;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "updateAppWidget\0", "(Landroid/content/ComponentName;Landroid/widget/RemoteViews;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateAppWidgetProviderInfo](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#updateAppWidgetProviderInfo(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String")))]
        pub fn updateAppWidgetProviderInfo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "updateAppWidgetProviderInfo", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "updateAppWidgetProviderInfo\0", "(Landroid/content/ComponentName;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyAppWidgetViewDataChanged](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#notifyAppWidgetViewDataChanged(int%5B%5D,%20int))
        pub fn notifyAppWidgetViewDataChanged_int_array_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "notifyAppWidgetViewDataChanged", .descriptor == "([II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "notifyAppWidgetViewDataChanged\0", "([II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyAppWidgetViewDataChanged](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#notifyAppWidgetViewDataChanged(int,%20int))
        pub fn notifyAppWidgetViewDataChanged_int_int<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "notifyAppWidgetViewDataChanged", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "notifyAppWidgetViewDataChanged\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstalledProvidersForProfile](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getInstalledProvidersForProfile(android.os.UserHandle))
        ///
        /// Required features: "android-os-UserHandle", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-os-UserHandle", feature = "java-util-List")))]
        pub fn getInstalledProvidersForProfile<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::UserHandle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "getInstalledProvidersForProfile", .descriptor == "(Landroid/os/UserHandle;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "getInstalledProvidersForProfile\0", "(Landroid/os/UserHandle;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstalledProvidersForPackage](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getInstalledProvidersForPackage(java.lang.String,%20android.os.UserHandle))
        ///
        /// Required features: "android-os-UserHandle", "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-os-UserHandle", feature = "java-lang-String", feature = "java-util-List")))]
        pub fn getInstalledProvidersForPackage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::UserHandle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "getInstalledProvidersForPackage", .descriptor == "(Ljava/lang/String;Landroid/os/UserHandle;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "getInstalledProvidersForPackage\0", "(Ljava/lang/String;Landroid/os/UserHandle;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstalledProviders](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getInstalledProviders())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getInstalledProviders<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "getInstalledProviders", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "getInstalledProviders\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAppWidgetInfo](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getAppWidgetInfo(int))
        ///
        /// Required features: "android-appwidget-AppWidgetProviderInfo"
        #[cfg(any(feature = "all", all(feature = "android-appwidget-AppWidgetProviderInfo")))]
        pub fn getAppWidgetInfo<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetProviderInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "getAppWidgetInfo", .descriptor == "(I)Landroid/appwidget/AppWidgetProviderInfo;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "getAppWidgetInfo\0", "(I)Landroid/appwidget/AppWidgetProviderInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindAppWidgetIdIfAllowed](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#bindAppWidgetIdIfAllowed(int,%20android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn bindAppWidgetIdIfAllowed_int_ComponentName<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "bindAppWidgetIdIfAllowed", .descriptor == "(ILandroid/content/ComponentName;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "bindAppWidgetIdIfAllowed\0", "(ILandroid/content/ComponentName;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindAppWidgetIdIfAllowed](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#bindAppWidgetIdIfAllowed(int,%20android.content.ComponentName,%20android.os.Bundle))
        ///
        /// Required features: "android-content-ComponentName", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-os-Bundle")))]
        pub fn bindAppWidgetIdIfAllowed_int_ComponentName_Bundle<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "bindAppWidgetIdIfAllowed", .descriptor == "(ILandroid/content/ComponentName;Landroid/os/Bundle;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "bindAppWidgetIdIfAllowed\0", "(ILandroid/content/ComponentName;Landroid/os/Bundle;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindAppWidgetIdIfAllowed](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#bindAppWidgetIdIfAllowed(int,%20android.os.UserHandle,%20android.content.ComponentName,%20android.os.Bundle))
        ///
        /// Required features: "android-content-ComponentName", "android-os-Bundle", "android-os-UserHandle"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-os-Bundle", feature = "android-os-UserHandle")))]
        pub fn bindAppWidgetIdIfAllowed_int_UserHandle_ComponentName_Bundle<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::UserHandle>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "bindAppWidgetIdIfAllowed", .descriptor == "(ILandroid/os/UserHandle;Landroid/content/ComponentName;Landroid/os/Bundle;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "bindAppWidgetIdIfAllowed\0", "(ILandroid/os/UserHandle;Landroid/content/ComponentName;Landroid/os/Bundle;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAppWidgetIds](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#getAppWidgetIds(android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn getAppWidgetIds<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "getAppWidgetIds", .descriptor == "(Landroid/content/ComponentName;)[I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "getAppWidgetIds\0", "(Landroid/content/ComponentName;)[I\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRequestPinAppWidgetSupported](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#isRequestPinAppWidgetSupported())
        pub fn isRequestPinAppWidgetSupported<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "isRequestPinAppWidgetSupported", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "isRequestPinAppWidgetSupported\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestPinAppWidget](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#requestPinAppWidget(android.content.ComponentName,%20android.os.Bundle,%20android.app.PendingIntent))
        ///
        /// Required features: "android-app-PendingIntent", "android-content-ComponentName", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent", feature = "android-content-ComponentName", feature = "android-os-Bundle")))]
        pub fn requestPinAppWidget<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetManager", java.flags == PUBLIC, .name == "requestPinAppWidget", .descriptor == "(Landroid/content/ComponentName;Landroid/os/Bundle;Landroid/app/PendingIntent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetManager\0", "requestPinAppWidget\0", "(Landroid/content/ComponentName;Landroid/os/Bundle;Landroid/app/PendingIntent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_APPWIDGET_BIND](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_BIND)
        pub const ACTION_APPWIDGET_BIND : &'static str = "android.appwidget.action.APPWIDGET_BIND";

        /// public static final [ACTION_APPWIDGET_CONFIGURE](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_CONFIGURE)
        pub const ACTION_APPWIDGET_CONFIGURE : &'static str = "android.appwidget.action.APPWIDGET_CONFIGURE";

        /// public static final [ACTION_APPWIDGET_DELETED](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_DELETED)
        pub const ACTION_APPWIDGET_DELETED : &'static str = "android.appwidget.action.APPWIDGET_DELETED";

        /// public static final [ACTION_APPWIDGET_DISABLED](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_DISABLED)
        pub const ACTION_APPWIDGET_DISABLED : &'static str = "android.appwidget.action.APPWIDGET_DISABLED";

        /// public static final [ACTION_APPWIDGET_ENABLED](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_ENABLED)
        pub const ACTION_APPWIDGET_ENABLED : &'static str = "android.appwidget.action.APPWIDGET_ENABLED";

        /// public static final [ACTION_APPWIDGET_HOST_RESTORED](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_HOST_RESTORED)
        pub const ACTION_APPWIDGET_HOST_RESTORED : &'static str = "android.appwidget.action.APPWIDGET_HOST_RESTORED";

        /// public static final [ACTION_APPWIDGET_OPTIONS_CHANGED](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_OPTIONS_CHANGED)
        pub const ACTION_APPWIDGET_OPTIONS_CHANGED : &'static str = "android.appwidget.action.APPWIDGET_UPDATE_OPTIONS";

        /// public static final [ACTION_APPWIDGET_PICK](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_PICK)
        pub const ACTION_APPWIDGET_PICK : &'static str = "android.appwidget.action.APPWIDGET_PICK";

        /// public static final [ACTION_APPWIDGET_RESTORED](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_RESTORED)
        pub const ACTION_APPWIDGET_RESTORED : &'static str = "android.appwidget.action.APPWIDGET_RESTORED";

        /// public static final [ACTION_APPWIDGET_UPDATE](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#ACTION_APPWIDGET_UPDATE)
        pub const ACTION_APPWIDGET_UPDATE : &'static str = "android.appwidget.action.APPWIDGET_UPDATE";

        /// public static final [EXTRA_APPWIDGET_ID](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_ID)
        pub const EXTRA_APPWIDGET_ID : &'static str = "appWidgetId";

        /// public static final [EXTRA_APPWIDGET_IDS](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_IDS)
        pub const EXTRA_APPWIDGET_IDS : &'static str = "appWidgetIds";

        /// public static final [EXTRA_APPWIDGET_OLD_IDS](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_OLD_IDS)
        pub const EXTRA_APPWIDGET_OLD_IDS : &'static str = "appWidgetOldIds";

        /// public static final [EXTRA_APPWIDGET_OPTIONS](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_OPTIONS)
        pub const EXTRA_APPWIDGET_OPTIONS : &'static str = "appWidgetOptions";

        /// public static final [EXTRA_APPWIDGET_PREVIEW](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_PREVIEW)
        pub const EXTRA_APPWIDGET_PREVIEW : &'static str = "appWidgetPreview";

        /// public static final [EXTRA_APPWIDGET_PROVIDER](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_PROVIDER)
        pub const EXTRA_APPWIDGET_PROVIDER : &'static str = "appWidgetProvider";

        /// public static final [EXTRA_APPWIDGET_PROVIDER_PROFILE](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_APPWIDGET_PROVIDER_PROFILE)
        pub const EXTRA_APPWIDGET_PROVIDER_PROFILE : &'static str = "appWidgetProviderProfile";

        /// public static final [EXTRA_CUSTOM_EXTRAS](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_CUSTOM_EXTRAS)
        pub const EXTRA_CUSTOM_EXTRAS : &'static str = "customExtras";

        /// public static final [EXTRA_CUSTOM_INFO](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_CUSTOM_INFO)
        pub const EXTRA_CUSTOM_INFO : &'static str = "customInfo";

        /// public static final [EXTRA_HOST_ID](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#EXTRA_HOST_ID)
        pub const EXTRA_HOST_ID : &'static str = "hostId";

        /// public static final [INVALID_APPWIDGET_ID](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#INVALID_APPWIDGET_ID)
        pub const INVALID_APPWIDGET_ID : i32 = 0;

        /// public static final [META_DATA_APPWIDGET_PROVIDER](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#META_DATA_APPWIDGET_PROVIDER)
        pub const META_DATA_APPWIDGET_PROVIDER : &'static str = "android.appwidget.provider";

        /// public static final [OPTION_APPWIDGET_HOST_CATEGORY](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#OPTION_APPWIDGET_HOST_CATEGORY)
        pub const OPTION_APPWIDGET_HOST_CATEGORY : &'static str = "appWidgetCategory";

        /// public static final [OPTION_APPWIDGET_MAX_HEIGHT](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#OPTION_APPWIDGET_MAX_HEIGHT)
        pub const OPTION_APPWIDGET_MAX_HEIGHT : &'static str = "appWidgetMaxHeight";

        /// public static final [OPTION_APPWIDGET_MAX_WIDTH](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#OPTION_APPWIDGET_MAX_WIDTH)
        pub const OPTION_APPWIDGET_MAX_WIDTH : &'static str = "appWidgetMaxWidth";

        /// public static final [OPTION_APPWIDGET_MIN_HEIGHT](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#OPTION_APPWIDGET_MIN_HEIGHT)
        pub const OPTION_APPWIDGET_MIN_HEIGHT : &'static str = "appWidgetMinHeight";

        /// public static final [OPTION_APPWIDGET_MIN_WIDTH](https://developer.android.com/reference/android/appwidget/AppWidgetManager.html#OPTION_APPWIDGET_MIN_WIDTH)
        pub const OPTION_APPWIDGET_MIN_WIDTH : &'static str = "appWidgetMinWidth";
    }
}
