// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-location-SettingInjectorService"))]
__jni_bindgen! {
    /// public class [SettingInjectorService](https://developer.android.com/reference/android/location/SettingInjectorService.html)
    ///
    /// Required feature: android-location-SettingInjectorService
    public class SettingInjectorService ("android/location/SettingInjectorService") extends crate::android::app::Service {

        /// [SettingInjectorService](https://developer.android.com/reference/android/location/SettingInjectorService.html#SettingInjectorService(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::location::SettingInjectorService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/SettingInjectorService", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/SettingInjectorService\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/location/SettingInjectorService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/SettingInjectorService", java.flags == PUBLIC | FINAL, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/SettingInjectorService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStart](https://developer.android.com/reference/android/location/SettingInjectorService.html#onStart(android.content.Intent,%20int))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn onStart<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/SettingInjectorService", java.flags == PUBLIC | FINAL, .name == "onStart", .descriptor == "(Landroid/content/Intent;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/SettingInjectorService\0", "onStart\0", "(Landroid/content/Intent;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStartCommand](https://developer.android.com/reference/android/location/SettingInjectorService.html#onStartCommand(android.content.Intent,%20int,%20int))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn onStartCommand<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/SettingInjectorService", java.flags == PUBLIC | FINAL, .name == "onStartCommand", .descriptor == "(Landroid/content/Intent;II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/SettingInjectorService\0", "onStartCommand\0", "(Landroid/content/Intent;II)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onGetSummary](https://developer.android.com/reference/android/location/SettingInjectorService.html#onGetSummary())
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn onGetSummary<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/location/SettingInjectorService", java.flags == PROTECTED | ABSTRACT, .name == "onGetSummary", .descriptor == "()Ljava/lang/String;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/SettingInjectorService\0", "onGetSummary\0", "()Ljava/lang/String;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onGetEnabled](https://developer.android.com/reference/android/location/SettingInjectorService.html#onGetEnabled())
        // fn onGetEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/location/SettingInjectorService", java.flags == PROTECTED | ABSTRACT, .name == "onGetEnabled", .descriptor == "()Z"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/SettingInjectorService\0", "onGetEnabled\0", "()Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [refreshSettings](https://developer.android.com/reference/android/location/SettingInjectorService.html#refreshSettings(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn refreshSettings<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/SettingInjectorService", java.flags == PUBLIC | STATIC | FINAL, .name == "refreshSettings", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/location/SettingInjectorService\0", "refreshSettings\0", "(Landroid/content/Context;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_INJECTED_SETTING_CHANGED](https://developer.android.com/reference/android/location/SettingInjectorService.html#ACTION_INJECTED_SETTING_CHANGED)
        pub const ACTION_INJECTED_SETTING_CHANGED : &'static str = "android.location.InjectedSettingChanged";

        /// public static final [ACTION_SERVICE_INTENT](https://developer.android.com/reference/android/location/SettingInjectorService.html#ACTION_SERVICE_INTENT)
        pub const ACTION_SERVICE_INTENT : &'static str = "android.location.SettingInjectorService";

        /// public static final [ATTRIBUTES_NAME](https://developer.android.com/reference/android/location/SettingInjectorService.html#ATTRIBUTES_NAME)
        pub const ATTRIBUTES_NAME : &'static str = "injected-location-setting";

        /// public static final [META_DATA_NAME](https://developer.android.com/reference/android/location/SettingInjectorService.html#META_DATA_NAME)
        pub const META_DATA_NAME : &'static str = "android.location.SettingInjectorService";
    }
}
