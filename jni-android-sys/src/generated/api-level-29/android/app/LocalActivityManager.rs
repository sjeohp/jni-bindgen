// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-LocalActivityManager"))]
__jni_bindgen! {
    /// public class [LocalActivityManager](https://developer.android.com/reference/android/app/LocalActivityManager.html)
    ///
    /// Required feature: android-app-LocalActivityManager
    #[deprecated] public class LocalActivityManager ("android/app/LocalActivityManager") extends crate::java::lang::Object {

        /// [LocalActivityManager](https://developer.android.com/reference/android/app/LocalActivityManager.html#LocalActivityManager(android.app.Activity,%20boolean))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::LocalActivityManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/app/Activity;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "<init>\0", "(Landroid/app/Activity;Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startActivity](https://developer.android.com/reference/android/app/LocalActivityManager.html#startActivity(java.lang.String,%20android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-view-Window", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-view-Window", feature = "java-lang-String")))]
        #[deprecated] pub fn startActivity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::Window>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "startActivity", .descriptor == "(Ljava/lang/String;Landroid/content/Intent;)Landroid/view/Window;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "startActivity\0", "(Ljava/lang/String;Landroid/content/Intent;)Landroid/view/Window;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [destroyActivity](https://developer.android.com/reference/android/app/LocalActivityManager.html#destroyActivity(java.lang.String,%20boolean))
        ///
        /// Required features: "android-view-Window", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-Window", feature = "java-lang-String")))]
        #[deprecated] pub fn destroyActivity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::Window>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "destroyActivity", .descriptor == "(Ljava/lang/String;Z)Landroid/view/Window;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "destroyActivity\0", "(Ljava/lang/String;Z)Landroid/view/Window;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentActivity](https://developer.android.com/reference/android/app/LocalActivityManager.html#getCurrentActivity())
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        #[deprecated] pub fn getCurrentActivity<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Activity>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "getCurrentActivity", .descriptor == "()Landroid/app/Activity;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "getCurrentActivity\0", "()Landroid/app/Activity;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentId](https://developer.android.com/reference/android/app/LocalActivityManager.html#getCurrentId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn getCurrentId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "getCurrentId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "getCurrentId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActivity](https://developer.android.com/reference/android/app/LocalActivityManager.html#getActivity(java.lang.String))
        ///
        /// Required features: "android-app-Activity", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "java-lang-String")))]
        #[deprecated] pub fn getActivity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Activity>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "getActivity", .descriptor == "(Ljava/lang/String;)Landroid/app/Activity;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "getActivity\0", "(Ljava/lang/String;)Landroid/app/Activity;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dispatchCreate](https://developer.android.com/reference/android/app/LocalActivityManager.html#dispatchCreate(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        #[deprecated] pub fn dispatchCreate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "dispatchCreate", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "dispatchCreate\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [saveInstanceState](https://developer.android.com/reference/android/app/LocalActivityManager.html#saveInstanceState())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        #[deprecated] pub fn saveInstanceState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "saveInstanceState", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "saveInstanceState\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dispatchResume](https://developer.android.com/reference/android/app/LocalActivityManager.html#dispatchResume())
        #[deprecated] pub fn dispatchResume<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "dispatchResume", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "dispatchResume\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dispatchPause](https://developer.android.com/reference/android/app/LocalActivityManager.html#dispatchPause(boolean))
        #[deprecated] pub fn dispatchPause<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "dispatchPause", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "dispatchPause\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dispatchStop](https://developer.android.com/reference/android/app/LocalActivityManager.html#dispatchStop())
        #[deprecated] pub fn dispatchStop<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "dispatchStop", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "dispatchStop\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeAllActivities](https://developer.android.com/reference/android/app/LocalActivityManager.html#removeAllActivities())
        #[deprecated] pub fn removeAllActivities<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "removeAllActivities", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "removeAllActivities\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dispatchDestroy](https://developer.android.com/reference/android/app/LocalActivityManager.html#dispatchDestroy(boolean))
        #[deprecated] pub fn dispatchDestroy<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LocalActivityManager", java.flags == PUBLIC, .name == "dispatchDestroy", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LocalActivityManager\0", "dispatchDestroy\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
