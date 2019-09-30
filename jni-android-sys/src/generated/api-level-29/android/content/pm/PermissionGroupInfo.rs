// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-pm-PermissionGroupInfo"))]
__jni_bindgen! {
    /// public class [PermissionGroupInfo](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html)
    ///
    /// Required feature: android-content-pm-PermissionGroupInfo
    public class PermissionGroupInfo ("android/content/pm/PermissionGroupInfo") extends crate::android::content::pm::PackageItemInfo, implements crate::android::os::Parcelable {

        /// [PermissionGroupInfo](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#PermissionGroupInfo())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::pm::PermissionGroupInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/PermissionGroupInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/PermissionGroupInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PermissionGroupInfo](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#PermissionGroupInfo(android.content.pm.PermissionGroupInfo))
        ///
        /// Required features: "android-content-pm-PermissionGroupInfo"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-PermissionGroupInfo")))]
        #[deprecated] pub fn new_PermissionGroupInfo<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::PermissionGroupInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::pm::PermissionGroupInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/PermissionGroupInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/pm/PermissionGroupInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/PermissionGroupInfo\0", "<init>\0", "(Landroid/content/pm/PermissionGroupInfo;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadDescription](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#loadDescription(android.content.pm.PackageManager))
        ///
        /// Required features: "android-content-pm-PackageManager", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-PackageManager", feature = "java-lang-CharSequence")))]
        pub fn loadDescription<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::PackageManager>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/PermissionGroupInfo", java.flags == PUBLIC, .name == "loadDescription", .descriptor == "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/PermissionGroupInfo\0", "loadDescription\0", "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/PermissionGroupInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/PermissionGroupInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/PermissionGroupInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/PermissionGroupInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/pm/PermissionGroupInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/pm/PermissionGroupInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/content/pm/PermissionGroupInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [FLAG_PERSONAL_INFO](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#FLAG_PERSONAL_INFO)
        pub const FLAG_PERSONAL_INFO : i32 = 1;

        /// **get** public [descriptionRes](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#descriptionRes)
        pub fn descriptionRes<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "descriptionRes\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [descriptionRes](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#descriptionRes)
        pub fn set_descriptionRes<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "descriptionRes\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [flags](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#flags)
        pub fn flags<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "flags\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [flags](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#flags)
        pub fn set_flags<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "flags\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [nonLocalizedDescription](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#nonLocalizedDescription)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn nonLocalizedDescription<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "nonLocalizedDescription\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [nonLocalizedDescription](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#nonLocalizedDescription)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn set_nonLocalizedDescription<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "nonLocalizedDescription\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [priority](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#priority)
        pub fn priority<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "priority\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [priority](https://developer.android.com/reference/android/content/pm/PermissionGroupInfo.html#priority)
        pub fn set_priority<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/pm/PermissionGroupInfo\0", "priority\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
