// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ActivityManager_RunningServiceInfo"))]
__jni_bindgen! {
    /// public class [ActivityManager.RunningServiceInfo](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html)
    ///
    /// Required feature: android-app-ActivityManager_RunningServiceInfo
    public class ActivityManager_RunningServiceInfo ("android/app/ActivityManager$RunningServiceInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [RunningServiceInfo](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#RunningServiceInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ActivityManager_RunningServiceInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RunningServiceInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RunningServiceInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RunningServiceInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RunningServiceInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RunningServiceInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RunningServiceInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readFromParcel](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#readFromParcel(android.os.Parcel))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn readFromParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActivityManager$RunningServiceInfo", java.flags == PUBLIC, .name == "readFromParcel", .descriptor == "(Landroid/os/Parcel;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActivityManager$RunningServiceInfo\0", "readFromParcel\0", "(Landroid/os/Parcel;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/ActivityManager$RunningServiceInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [FLAG_FOREGROUND](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#FLAG_FOREGROUND)
        pub const FLAG_FOREGROUND : i32 = 2;

        /// public static final [FLAG_PERSISTENT_PROCESS](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#FLAG_PERSISTENT_PROCESS)
        pub const FLAG_PERSISTENT_PROCESS : i32 = 8;

        /// public static final [FLAG_STARTED](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#FLAG_STARTED)
        pub const FLAG_STARTED : i32 = 1;

        /// public static final [FLAG_SYSTEM_PROCESS](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#FLAG_SYSTEM_PROCESS)
        pub const FLAG_SYSTEM_PROCESS : i32 = 4;

        /// **get** public [activeSince](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#activeSince)
        pub fn activeSince<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "activeSince\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [activeSince](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#activeSince)
        pub fn set_activeSince<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "activeSince\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [clientCount](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#clientCount)
        pub fn clientCount<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "clientCount\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [clientCount](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#clientCount)
        pub fn set_clientCount<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "clientCount\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [clientLabel](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#clientLabel)
        pub fn clientLabel<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "clientLabel\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [clientLabel](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#clientLabel)
        pub fn set_clientLabel<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "clientLabel\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [clientPackage](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#clientPackage)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn clientPackage<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "clientPackage\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [clientPackage](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#clientPackage)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_clientPackage<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "clientPackage\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [crashCount](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#crashCount)
        pub fn crashCount<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "crashCount\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [crashCount](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#crashCount)
        pub fn set_crashCount<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "crashCount\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [flags](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#flags)
        pub fn flags<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "flags\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [flags](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#flags)
        pub fn set_flags<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "flags\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [foreground](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#foreground)
        pub fn foreground<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "foreground\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [foreground](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#foreground)
        pub fn set_foreground<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "foreground\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [lastActivityTime](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#lastActivityTime)
        pub fn lastActivityTime<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "lastActivityTime\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [lastActivityTime](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#lastActivityTime)
        pub fn set_lastActivityTime<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "lastActivityTime\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [pid](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#pid)
        pub fn pid<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "pid\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [pid](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#pid)
        pub fn set_pid<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "pid\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [process](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#process)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn process<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "process\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [process](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#process)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_process<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "process\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [restarting](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#restarting)
        pub fn restarting<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "restarting\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [restarting](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#restarting)
        pub fn set_restarting<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "restarting\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [service](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#service)
        ///
        /// Required feature: android-content-ComponentName
        #[cfg(any(feature = "all", feature = "android-content-ComponentName"))]
        pub fn service<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "service\0", "Landroid/content/ComponentName;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [service](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#service)
        ///
        /// Required feature: android-content-ComponentName
        #[cfg(any(feature = "all", feature = "android-content-ComponentName"))]
        pub fn set_service<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::content::ComponentName>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "service\0", "Landroid/content/ComponentName;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [started](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#started)
        pub fn started<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "started\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [started](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#started)
        pub fn set_started<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "started\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [uid](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#uid)
        pub fn uid<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "uid\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [uid](https://developer.android.com/reference/android/app/ActivityManager.RunningServiceInfo.html#uid)
        pub fn set_uid<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ActivityManager$RunningServiceInfo\0", "uid\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
