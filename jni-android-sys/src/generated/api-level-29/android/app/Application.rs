// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-Application"))]
__jni_bindgen! {
    /// public class [Application](https://developer.android.com/reference/android/app/Application.html)
    ///
    /// Required feature: android-app-Application
    public class Application ("android/app/Application") extends crate::android::content::ContextWrapper, implements crate::android::content::ComponentCallbacks2 {

        /// [Application](https://developer.android.com/reference/android/app/Application.html#Application())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::Application>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreate](https://developer.android.com/reference/android/app/Application.html#onCreate())
        pub fn onCreate<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "onCreate", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "onCreate\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTerminate](https://developer.android.com/reference/android/app/Application.html#onTerminate())
        pub fn onTerminate<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "onTerminate", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "onTerminate\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConfigurationChanged](https://developer.android.com/reference/android/app/Application.html#onConfigurationChanged(android.content.res.Configuration))
        ///
        /// Required features: "android-content-res-Configuration"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Configuration")))]
        pub fn onConfigurationChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Configuration>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "onConfigurationChanged", .descriptor == "(Landroid/content/res/Configuration;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "onConfigurationChanged\0", "(Landroid/content/res/Configuration;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLowMemory](https://developer.android.com/reference/android/app/Application.html#onLowMemory())
        pub fn onLowMemory<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "onLowMemory", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "onLowMemory\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTrimMemory](https://developer.android.com/reference/android/app/Application.html#onTrimMemory(int))
        pub fn onTrimMemory<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "onTrimMemory", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "onTrimMemory\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerComponentCallbacks](https://developer.android.com/reference/android/app/Application.html#registerComponentCallbacks(android.content.ComponentCallbacks))
        ///
        /// Required features: "android-content-ComponentCallbacks"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentCallbacks")))]
        pub fn registerComponentCallbacks<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentCallbacks>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "registerComponentCallbacks", .descriptor == "(Landroid/content/ComponentCallbacks;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "registerComponentCallbacks\0", "(Landroid/content/ComponentCallbacks;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterComponentCallbacks](https://developer.android.com/reference/android/app/Application.html#unregisterComponentCallbacks(android.content.ComponentCallbacks))
        ///
        /// Required features: "android-content-ComponentCallbacks"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentCallbacks")))]
        pub fn unregisterComponentCallbacks<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentCallbacks>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "unregisterComponentCallbacks", .descriptor == "(Landroid/content/ComponentCallbacks;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "unregisterComponentCallbacks\0", "(Landroid/content/ComponentCallbacks;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerActivityLifecycleCallbacks](https://developer.android.com/reference/android/app/Application.html#registerActivityLifecycleCallbacks(android.app.Application.ActivityLifecycleCallbacks))
        ///
        /// Required features: "android-app-Application_ActivityLifecycleCallbacks"
        #[cfg(any(feature = "all", all(feature = "android-app-Application_ActivityLifecycleCallbacks")))]
        pub fn registerActivityLifecycleCallbacks<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Application_ActivityLifecycleCallbacks>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "registerActivityLifecycleCallbacks", .descriptor == "(Landroid/app/Application$ActivityLifecycleCallbacks;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "registerActivityLifecycleCallbacks\0", "(Landroid/app/Application$ActivityLifecycleCallbacks;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterActivityLifecycleCallbacks](https://developer.android.com/reference/android/app/Application.html#unregisterActivityLifecycleCallbacks(android.app.Application.ActivityLifecycleCallbacks))
        ///
        /// Required features: "android-app-Application_ActivityLifecycleCallbacks"
        #[cfg(any(feature = "all", all(feature = "android-app-Application_ActivityLifecycleCallbacks")))]
        pub fn unregisterActivityLifecycleCallbacks<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Application_ActivityLifecycleCallbacks>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "unregisterActivityLifecycleCallbacks", .descriptor == "(Landroid/app/Application$ActivityLifecycleCallbacks;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "unregisterActivityLifecycleCallbacks\0", "(Landroid/app/Application$ActivityLifecycleCallbacks;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerOnProvideAssistDataListener](https://developer.android.com/reference/android/app/Application.html#registerOnProvideAssistDataListener(android.app.Application.OnProvideAssistDataListener))
        ///
        /// Required features: "android-app-Application_OnProvideAssistDataListener"
        #[cfg(any(feature = "all", all(feature = "android-app-Application_OnProvideAssistDataListener")))]
        pub fn registerOnProvideAssistDataListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Application_OnProvideAssistDataListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "registerOnProvideAssistDataListener", .descriptor == "(Landroid/app/Application$OnProvideAssistDataListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "registerOnProvideAssistDataListener\0", "(Landroid/app/Application$OnProvideAssistDataListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterOnProvideAssistDataListener](https://developer.android.com/reference/android/app/Application.html#unregisterOnProvideAssistDataListener(android.app.Application.OnProvideAssistDataListener))
        ///
        /// Required features: "android-app-Application_OnProvideAssistDataListener"
        #[cfg(any(feature = "all", all(feature = "android-app-Application_OnProvideAssistDataListener")))]
        pub fn unregisterOnProvideAssistDataListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Application_OnProvideAssistDataListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC, .name == "unregisterOnProvideAssistDataListener", .descriptor == "(Landroid/app/Application$OnProvideAssistDataListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Application\0", "unregisterOnProvideAssistDataListener\0", "(Landroid/app/Application$OnProvideAssistDataListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProcessName](https://developer.android.com/reference/android/app/Application.html#getProcessName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getProcessName<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Application", java.flags == PUBLIC | STATIC, .name == "getProcessName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/Application\0", "getProcessName\0", "()Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
