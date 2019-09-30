// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ActivityManager_RecentTaskInfo"))]
__jni_bindgen! {
    /// public class [ActivityManager.RecentTaskInfo](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html)
    ///
    /// Required feature: android-app-ActivityManager_RecentTaskInfo
    public class ActivityManager_RecentTaskInfo ("android/app/ActivityManager$RecentTaskInfo") extends crate::android::app::TaskInfo, implements crate::android::os::Parcelable {

        /// [RecentTaskInfo](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#RecentTaskInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ActivityManager_RecentTaskInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RecentTaskInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RecentTaskInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RecentTaskInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RecentTaskInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readFromParcel](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#readFromParcel(android.os.Parcel))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn readFromParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RecentTaskInfo", java.flags == PUBLIC, .name == "readFromParcel", .descriptor == "(Landroid/os/Parcel;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RecentTaskInfo\0", "readFromParcel\0", "(Landroid/os/Parcel;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RecentTaskInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RecentTaskInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/ActivityManager$RecentTaskInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public [affiliatedTaskId](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#affiliatedTaskId)
        #[deprecated] pub fn affiliatedTaskId<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "affiliatedTaskId\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [affiliatedTaskId](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#affiliatedTaskId)
        #[deprecated] pub fn set_affiliatedTaskId<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "affiliatedTaskId\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [description](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#description)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn description<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "description\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [description](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#description)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn set_description<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "description\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [id](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#id)
        #[deprecated] pub fn id<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "id\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [id](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#id)
        #[deprecated] pub fn set_id<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "id\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [persistentId](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#persistentId)
        #[deprecated] pub fn persistentId<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "persistentId\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [persistentId](https://developer.android.com/reference/android/app/ActivityManager.RecentTaskInfo.html#persistentId)
        #[deprecated] pub fn set_persistentId<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RecentTaskInfo\0", "persistentId\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
