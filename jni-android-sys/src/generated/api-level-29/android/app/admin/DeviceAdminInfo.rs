// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-admin-DeviceAdminInfo"))]
__jni_bindgen! {
    /// public final class [DeviceAdminInfo](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html)
    ///
    /// Required feature: android-app-admin-DeviceAdminInfo
    public final class DeviceAdminInfo ("android/app/admin/DeviceAdminInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [DeviceAdminInfo](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#DeviceAdminInfo(android.content.Context,%20android.content.pm.ResolveInfo))
        ///
        /// Required features: "android-content-Context", "android-content-pm-ResolveInfo"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-pm-ResolveInfo")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::ResolveInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::admin::DeviceAdminInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/content/pm/ResolveInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "<init>\0", "(Landroid/content/Context;Landroid/content/pm/ResolveInfo;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPackageName](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#getPackageName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getPackageName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "getPackageName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "getPackageName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReceiverName](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#getReceiverName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getReceiverName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "getReceiverName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "getReceiverName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActivityInfo](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#getActivityInfo())
        ///
        /// Required features: "android-content-pm-ActivityInfo"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-ActivityInfo")))]
        pub fn getActivityInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::pm::ActivityInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "getActivityInfo", .descriptor == "()Landroid/content/pm/ActivityInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "getActivityInfo\0", "()Landroid/content/pm/ActivityInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getComponent](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#getComponent())
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn getComponent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "getComponent", .descriptor == "()Landroid/content/ComponentName;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "getComponent\0", "()Landroid/content/ComponentName;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadLabel](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#loadLabel(android.content.pm.PackageManager))
        ///
        /// Required features: "android-content-pm-PackageManager", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-PackageManager", feature = "java-lang-CharSequence")))]
        pub fn loadLabel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::PackageManager>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "loadLabel", .descriptor == "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "loadLabel\0", "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadDescription](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#loadDescription(android.content.pm.PackageManager))
        ///
        /// Required features: "android-content-pm-PackageManager", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-PackageManager", feature = "java-lang-CharSequence")))]
        pub fn loadDescription<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::PackageManager>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "loadDescription", .descriptor == "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "loadDescription\0", "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadIcon](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#loadIcon(android.content.pm.PackageManager))
        ///
        /// Required features: "android-content-pm-PackageManager", "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-PackageManager", feature = "android-graphics-drawable-Drawable")))]
        pub fn loadIcon<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::PackageManager>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "loadIcon", .descriptor == "(Landroid/content/pm/PackageManager;)Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "loadIcon\0", "(Landroid/content/pm/PackageManager;)Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isVisible](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#isVisible())
        pub fn isVisible<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "isVisible", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "isVisible\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [usesPolicy](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#usesPolicy(int))
        pub fn usesPolicy<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "usesPolicy", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "usesPolicy\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTagForPolicy](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#getTagForPolicy(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getTagForPolicy<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "getTagForPolicy", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "getTagForPolicy\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [supportsTransferOwnership](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#supportsTransferOwnership())
        pub fn supportsTransferOwnership<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "supportsTransferOwnership", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "supportsTransferOwnership\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dump](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#dump(android.util.Printer,%20java.lang.String))
        ///
        /// Required features: "android-util-Printer", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-util-Printer", feature = "java-lang-String")))]
        pub fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::Printer>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "dump", .descriptor == "(Landroid/util/Printer;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "dump\0", "(Landroid/util/Printer;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DeviceAdminInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DeviceAdminInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/admin/DeviceAdminInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [USES_ENCRYPTED_STORAGE](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_ENCRYPTED_STORAGE)
        pub const USES_ENCRYPTED_STORAGE : i32 = 7;

        /// public static final [USES_POLICY_DISABLE_CAMERA](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_DISABLE_CAMERA)
        pub const USES_POLICY_DISABLE_CAMERA : i32 = 8;

        /// public static final [USES_POLICY_DISABLE_KEYGUARD_FEATURES](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_DISABLE_KEYGUARD_FEATURES)
        pub const USES_POLICY_DISABLE_KEYGUARD_FEATURES : i32 = 9;

        /// public static final [USES_POLICY_EXPIRE_PASSWORD](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_EXPIRE_PASSWORD)
        pub const USES_POLICY_EXPIRE_PASSWORD : i32 = 6;

        /// public static final [USES_POLICY_FORCE_LOCK](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_FORCE_LOCK)
        pub const USES_POLICY_FORCE_LOCK : i32 = 3;

        /// public static final [USES_POLICY_LIMIT_PASSWORD](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_LIMIT_PASSWORD)
        pub const USES_POLICY_LIMIT_PASSWORD : i32 = 0;

        /// public static final [USES_POLICY_RESET_PASSWORD](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_RESET_PASSWORD)
        pub const USES_POLICY_RESET_PASSWORD : i32 = 2;

        /// public static final [USES_POLICY_WATCH_LOGIN](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_WATCH_LOGIN)
        pub const USES_POLICY_WATCH_LOGIN : i32 = 1;

        /// public static final [USES_POLICY_WIPE_DATA](https://developer.android.com/reference/android/app/admin/DeviceAdminInfo.html#USES_POLICY_WIPE_DATA)
        pub const USES_POLICY_WIPE_DATA : i32 = 4;
    }
}
