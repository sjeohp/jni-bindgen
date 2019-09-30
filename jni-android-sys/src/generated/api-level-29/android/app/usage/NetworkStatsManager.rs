// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-usage-NetworkStatsManager"))]
__jni_bindgen! {
    /// public class [NetworkStatsManager](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html)
    ///
    /// Required feature: android-app-usage-NetworkStatsManager
    public class NetworkStatsManager ("android/app/usage/NetworkStatsManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [NetworkStatsManager](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#NetworkStatsManager())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStatsManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/usage/NetworkStatsManager", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [querySummaryForDevice](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#querySummaryForDevice(int,%20java.lang.String,%20long,%20long))
        ///
        /// Required features: "android-app-usage-NetworkStats_Bucket", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats_Bucket", feature = "java-lang-String")))]
        pub fn querySummaryForDevice<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats_Bucket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "querySummaryForDevice", .descriptor == "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats$Bucket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "querySummaryForDevice\0", "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats$Bucket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [querySummaryForUser](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#querySummaryForUser(int,%20java.lang.String,%20long,%20long))
        ///
        /// Required features: "android-app-usage-NetworkStats_Bucket", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats_Bucket", feature = "java-lang-String")))]
        pub fn querySummaryForUser<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats_Bucket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "querySummaryForUser", .descriptor == "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats$Bucket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "querySummaryForUser\0", "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats$Bucket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [querySummary](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#querySummary(int,%20java.lang.String,%20long,%20long))
        ///
        /// Required features: "android-app-usage-NetworkStats", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats", feature = "java-lang-String")))]
        pub fn querySummary<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "querySummary", .descriptor == "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "querySummary\0", "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [queryDetailsForUid](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#queryDetailsForUid(int,%20java.lang.String,%20long,%20long,%20int))
        ///
        /// Required features: "android-app-usage-NetworkStats", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats", feature = "java-lang-String")))]
        pub fn queryDetailsForUid<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "queryDetailsForUid", .descriptor == "(ILjava/lang/String;JJI)Landroid/app/usage/NetworkStats;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "queryDetailsForUid\0", "(ILjava/lang/String;JJI)Landroid/app/usage/NetworkStats;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [queryDetailsForUidTag](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#queryDetailsForUidTag(int,%20java.lang.String,%20long,%20long,%20int,%20int))
        ///
        /// Required features: "android-app-usage-NetworkStats", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats", feature = "java-lang-String")))]
        pub fn queryDetailsForUidTag<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "queryDetailsForUidTag", .descriptor == "(ILjava/lang/String;JJII)Landroid/app/usage/NetworkStats;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "queryDetailsForUidTag\0", "(ILjava/lang/String;JJII)Landroid/app/usage/NetworkStats;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [queryDetailsForUidTagState](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#queryDetailsForUidTagState(int,%20java.lang.String,%20long,%20long,%20int,%20int,%20int))
        ///
        /// Required features: "android-app-usage-NetworkStats", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats", feature = "java-lang-String")))]
        pub fn queryDetailsForUidTagState<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64, arg4: i32, arg5: i32, arg6: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "queryDetailsForUidTagState", .descriptor == "(ILjava/lang/String;JJIII)Landroid/app/usage/NetworkStats;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "queryDetailsForUidTagState\0", "(ILjava/lang/String;JJIII)Landroid/app/usage/NetworkStats;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [queryDetails](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#queryDetails(int,%20java.lang.String,%20long,%20long))
        ///
        /// Required features: "android-app-usage-NetworkStats", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStats", feature = "java-lang-String")))]
        pub fn queryDetails<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::usage::NetworkStats>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "queryDetails", .descriptor == "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "queryDetails\0", "(ILjava/lang/String;JJ)Landroid/app/usage/NetworkStats;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerUsageCallback](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#registerUsageCallback(int,%20java.lang.String,%20long,%20android.app.usage.NetworkStatsManager.UsageCallback))
        ///
        /// Required features: "android-app-usage-NetworkStatsManager_UsageCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStatsManager_UsageCallback", feature = "java-lang-String")))]
        pub fn registerUsageCallback_int_String_long_UsageCallback<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::usage::NetworkStatsManager_UsageCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "registerUsageCallback", .descriptor == "(ILjava/lang/String;JLandroid/app/usage/NetworkStatsManager$UsageCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "registerUsageCallback\0", "(ILjava/lang/String;JLandroid/app/usage/NetworkStatsManager$UsageCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerUsageCallback](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#registerUsageCallback(int,%20java.lang.String,%20long,%20android.app.usage.NetworkStatsManager.UsageCallback,%20android.os.Handler))
        ///
        /// Required features: "android-app-usage-NetworkStatsManager_UsageCallback", "android-os-Handler", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStatsManager_UsageCallback", feature = "android-os-Handler", feature = "java-lang-String")))]
        pub fn registerUsageCallback_int_String_long_UsageCallback_Handler<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i64, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::usage::NetworkStatsManager_UsageCallback>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "registerUsageCallback", .descriptor == "(ILjava/lang/String;JLandroid/app/usage/NetworkStatsManager$UsageCallback;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "registerUsageCallback\0", "(ILjava/lang/String;JLandroid/app/usage/NetworkStatsManager$UsageCallback;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterUsageCallback](https://developer.android.com/reference/android/app/usage/NetworkStatsManager.html#unregisterUsageCallback(android.app.usage.NetworkStatsManager.UsageCallback))
        ///
        /// Required features: "android-app-usage-NetworkStatsManager_UsageCallback"
        #[cfg(any(feature = "all", all(feature = "android-app-usage-NetworkStatsManager_UsageCallback")))]
        pub fn unregisterUsageCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::usage::NetworkStatsManager_UsageCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/usage/NetworkStatsManager", java.flags == PUBLIC, .name == "unregisterUsageCallback", .descriptor == "(Landroid/app/usage/NetworkStatsManager$UsageCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/usage/NetworkStatsManager\0", "unregisterUsageCallback\0", "(Landroid/app/usage/NetworkStatsManager$UsageCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
