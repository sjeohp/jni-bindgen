// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-admin-SystemUpdatePolicy"))]
__jni_bindgen! {
    /// public final class [SystemUpdatePolicy](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html)
    ///
    /// Required feature: android-app-admin-SystemUpdatePolicy
    public final class SystemUpdatePolicy ("android/app/admin/SystemUpdatePolicy") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [SystemUpdatePolicy](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#SystemUpdatePolicy())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::admin::SystemUpdatePolicy>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [createAutomaticInstallPolicy](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#createAutomaticInstallPolicy())
        ///
        /// Required features: "android-app-admin-SystemUpdatePolicy"
        #[cfg(any(feature = "all", all(feature = "android-app-admin-SystemUpdatePolicy")))]
        pub fn createAutomaticInstallPolicy<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::admin::SystemUpdatePolicy>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC | STATIC, .name == "createAutomaticInstallPolicy", .descriptor == "()Landroid/app/admin/SystemUpdatePolicy;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/admin/SystemUpdatePolicy\0", "createAutomaticInstallPolicy\0", "()Landroid/app/admin/SystemUpdatePolicy;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createWindowedInstallPolicy](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#createWindowedInstallPolicy(int,%20int))
        ///
        /// Required features: "android-app-admin-SystemUpdatePolicy"
        #[cfg(any(feature = "all", all(feature = "android-app-admin-SystemUpdatePolicy")))]
        pub fn createWindowedInstallPolicy<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::admin::SystemUpdatePolicy>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC | STATIC, .name == "createWindowedInstallPolicy", .descriptor == "(II)Landroid/app/admin/SystemUpdatePolicy;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/admin/SystemUpdatePolicy\0", "createWindowedInstallPolicy\0", "(II)Landroid/app/admin/SystemUpdatePolicy;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createPostponeInstallPolicy](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#createPostponeInstallPolicy())
        ///
        /// Required features: "android-app-admin-SystemUpdatePolicy"
        #[cfg(any(feature = "all", all(feature = "android-app-admin-SystemUpdatePolicy")))]
        pub fn createPostponeInstallPolicy<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::admin::SystemUpdatePolicy>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC | STATIC, .name == "createPostponeInstallPolicy", .descriptor == "()Landroid/app/admin/SystemUpdatePolicy;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/app/admin/SystemUpdatePolicy\0", "createPostponeInstallPolicy\0", "()Landroid/app/admin/SystemUpdatePolicy;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPolicyType](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#getPolicyType())
        pub fn getPolicyType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "getPolicyType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "getPolicyType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstallWindowStart](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#getInstallWindowStart())
        pub fn getInstallWindowStart<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "getInstallWindowStart", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "getInstallWindowStart\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstallWindowEnd](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#getInstallWindowEnd())
        pub fn getInstallWindowEnd<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "getInstallWindowEnd", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "getInstallWindowEnd\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFreezePeriods](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#setFreezePeriods(java.util.List))
        ///
        /// Required features: "android-app-admin-SystemUpdatePolicy", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-app-admin-SystemUpdatePolicy", feature = "java-util-List")))]
        pub fn setFreezePeriods<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::admin::SystemUpdatePolicy>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "setFreezePeriods", .descriptor == "(Ljava/util/List;)Landroid/app/admin/SystemUpdatePolicy;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "setFreezePeriods\0", "(Ljava/util/List;)Landroid/app/admin/SystemUpdatePolicy;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFreezePeriods](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#getFreezePeriods())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getFreezePeriods<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "getFreezePeriods", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "getFreezePeriods\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/SystemUpdatePolicy", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/SystemUpdatePolicy\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/admin/SystemUpdatePolicy\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TYPE_INSTALL_AUTOMATIC](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#TYPE_INSTALL_AUTOMATIC)
        pub const TYPE_INSTALL_AUTOMATIC : i32 = 1;

        /// public static final [TYPE_INSTALL_WINDOWED](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#TYPE_INSTALL_WINDOWED)
        pub const TYPE_INSTALL_WINDOWED : i32 = 2;

        /// public static final [TYPE_POSTPONE](https://developer.android.com/reference/android/app/admin/SystemUpdatePolicy.html#TYPE_POSTPONE)
        pub const TYPE_POSTPONE : i32 = 3;
    }
}
