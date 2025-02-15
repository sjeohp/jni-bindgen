// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-pm-LauncherActivityInfo"))]
__jni_bindgen! {
    /// public class [LauncherActivityInfo](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html)
    ///
    /// Required feature: android-content-pm-LauncherActivityInfo
    public class LauncherActivityInfo ("android/content/pm/LauncherActivityInfo") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [LauncherActivityInfo](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#LauncherActivityInfo(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::pm::LauncherActivityInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/pm/LauncherActivityInfo", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getComponentName](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getComponentName())
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn getComponentName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getComponentName", .descriptor == "()Landroid/content/ComponentName;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getComponentName\0", "()Landroid/content/ComponentName;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUser](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getUser())
        ///
        /// Required features: "android-os-UserHandle"
        #[cfg(any(feature = "all", all(feature = "android-os-UserHandle")))]
        pub fn getUser<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::UserHandle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getUser", .descriptor == "()Landroid/os/UserHandle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getUser\0", "()Landroid/os/UserHandle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLabel](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getLabel())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getLabel<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getLabel", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getLabel\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIcon](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getIcon(int))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getIcon<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getIcon", .descriptor == "(I)Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getIcon\0", "(I)Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getApplicationInfo](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getApplicationInfo())
        ///
        /// Required features: "android-content-pm-ApplicationInfo"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-ApplicationInfo")))]
        pub fn getApplicationInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::pm::ApplicationInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getApplicationInfo", .descriptor == "()Landroid/content/pm/ApplicationInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getApplicationInfo\0", "()Landroid/content/pm/ApplicationInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFirstInstallTime](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getFirstInstallTime())
        pub fn getFirstInstallTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getFirstInstallTime", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getFirstInstallTime\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBadgedIcon](https://developer.android.com/reference/android/content/pm/LauncherActivityInfo.html#getBadgedIcon(int))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getBadgedIcon<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/LauncherActivityInfo", java.flags == PUBLIC, .name == "getBadgedIcon", .descriptor == "(I)Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/LauncherActivityInfo\0", "getBadgedIcon\0", "(I)Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
