// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-LauncherActivity_ListItem"))]
__jni_bindgen! {
    /// public class [LauncherActivity.ListItem](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html)
    ///
    /// Required feature: android-app-LauncherActivity_ListItem
    public class LauncherActivity_ListItem ("android/app/LauncherActivity$ListItem") extends crate::java::lang::Object {

        /// [ListItem](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#ListItem())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::LauncherActivity_ListItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/LauncherActivity$ListItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/LauncherActivity$ListItem\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [className](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#className)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn className<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "className\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [className](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#className)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_className<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "className\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [extras](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#extras)
        ///
        /// Required feature: android-os-Bundle
        #[cfg(any(feature = "all", feature = "android-os-Bundle"))]
        pub fn extras<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "extras\0", "Landroid/os/Bundle;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [extras](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#extras)
        ///
        /// Required feature: android-os-Bundle
        #[cfg(any(feature = "all", feature = "android-os-Bundle"))]
        pub fn set_extras<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::os::Bundle>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "extras\0", "Landroid/os/Bundle;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [icon](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#icon)
        ///
        /// Required feature: android-graphics-drawable-Drawable
        #[cfg(any(feature = "all", feature = "android-graphics-drawable-Drawable"))]
        pub fn icon<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "icon\0", "Landroid/graphics/drawable/Drawable;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [icon](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#icon)
        ///
        /// Required feature: android-graphics-drawable-Drawable
        #[cfg(any(feature = "all", feature = "android-graphics-drawable-Drawable"))]
        pub fn set_icon<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::graphics::drawable::Drawable>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "icon\0", "Landroid/graphics/drawable/Drawable;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [label](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#label)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn label<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "label\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [label](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#label)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn set_label<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "label\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [packageName](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#packageName)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn packageName<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "packageName\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [packageName](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#packageName)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_packageName<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "packageName\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [resolveInfo](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#resolveInfo)
        ///
        /// Required feature: android-content-pm-ResolveInfo
        #[cfg(any(feature = "all", feature = "android-content-pm-ResolveInfo"))]
        pub fn resolveInfo<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::pm::ResolveInfo>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "resolveInfo\0", "Landroid/content/pm/ResolveInfo;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [resolveInfo](https://developer.android.com/reference/android/app/LauncherActivity.ListItem.html#resolveInfo)
        ///
        /// Required feature: android-content-pm-ResolveInfo
        #[cfg(any(feature = "all", feature = "android-content-pm-ResolveInfo"))]
        pub fn set_resolveInfo<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::content::pm::ResolveInfo>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/LauncherActivity$ListItem\0", "resolveInfo\0", "Landroid/content/pm/ResolveInfo;\0");
                env.set_object_field(class, field, value)
            }
        }
    }
}
