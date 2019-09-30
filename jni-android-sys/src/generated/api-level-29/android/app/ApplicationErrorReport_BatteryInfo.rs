// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ApplicationErrorReport_BatteryInfo"))]
__jni_bindgen! {
    /// public class [ApplicationErrorReport.BatteryInfo](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html)
    ///
    /// Required feature: android-app-ApplicationErrorReport_BatteryInfo
    public class ApplicationErrorReport_BatteryInfo ("android/app/ApplicationErrorReport$BatteryInfo") extends crate::java::lang::Object {

        /// [BatteryInfo](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#BatteryInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ApplicationErrorReport_BatteryInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ApplicationErrorReport$BatteryInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ApplicationErrorReport$BatteryInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BatteryInfo](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#BatteryInfo(android.os.Parcel))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn new_Parcel<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ApplicationErrorReport_BatteryInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ApplicationErrorReport$BatteryInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ApplicationErrorReport$BatteryInfo\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ApplicationErrorReport$BatteryInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ApplicationErrorReport$BatteryInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dump](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#dump(android.util.Printer,%20java.lang.String))
        ///
        /// Required features: "android-util-Printer", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-util-Printer", feature = "java-lang-String")))]
        pub fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::Printer>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ApplicationErrorReport$BatteryInfo", java.flags == PUBLIC, .name == "dump", .descriptor == "(Landroid/util/Printer;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ApplicationErrorReport$BatteryInfo\0", "dump\0", "(Landroid/util/Printer;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [checkinDetails](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#checkinDetails)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn checkinDetails<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "checkinDetails\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [checkinDetails](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#checkinDetails)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_checkinDetails<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "checkinDetails\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [durationMicros](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#durationMicros)
        pub fn durationMicros<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "durationMicros\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [durationMicros](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#durationMicros)
        pub fn set_durationMicros<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "durationMicros\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [usageDetails](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#usageDetails)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn usageDetails<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "usageDetails\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [usageDetails](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#usageDetails)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_usageDetails<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "usageDetails\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [usagePercent](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#usagePercent)
        pub fn usagePercent<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "usagePercent\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [usagePercent](https://developer.android.com/reference/android/app/ApplicationErrorReport.BatteryInfo.html#usagePercent)
        pub fn set_usagePercent<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/app/ApplicationErrorReport$BatteryInfo\0", "usagePercent\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
