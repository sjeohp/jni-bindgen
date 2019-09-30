// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-euicc-EuiccManager"))]
__jni_bindgen! {
    /// public class [EuiccManager](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html)
    ///
    /// Required feature: android-telephony-euicc-EuiccManager
    public class EuiccManager ("android/telephony/euicc/EuiccManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [EuiccManager](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#EuiccManager(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::euicc::EuiccManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/euicc/EuiccManager", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [createForCardId](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#createForCardId(int))
        ///
        /// Required features: "android-telephony-euicc-EuiccManager"
        #[cfg(any(feature = "all", all(feature = "android-telephony-euicc-EuiccManager")))]
        pub fn createForCardId<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::euicc::EuiccManager>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "createForCardId", .descriptor == "(I)Landroid/telephony/euicc/EuiccManager;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "createForCardId\0", "(I)Landroid/telephony/euicc/EuiccManager;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEnabled](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#isEnabled())
        pub fn isEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "isEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "isEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEid](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#getEid())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEid<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "getEid", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "getEid\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [downloadSubscription](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#downloadSubscription(android.telephony.euicc.DownloadableSubscription,%20boolean,%20android.app.PendingIntent))
        ///
        /// Required features: "android-app-PendingIntent", "android-telephony-euicc-DownloadableSubscription"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent", feature = "android-telephony-euicc-DownloadableSubscription")))]
        pub fn downloadSubscription<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::euicc::DownloadableSubscription>>, arg1: bool, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "downloadSubscription", .descriptor == "(Landroid/telephony/euicc/DownloadableSubscription;ZLandroid/app/PendingIntent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "downloadSubscription\0", "(Landroid/telephony/euicc/DownloadableSubscription;ZLandroid/app/PendingIntent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startResolutionActivity](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#startResolutionActivity(android.app.Activity,%20int,%20android.content.Intent,%20android.app.PendingIntent))
        ///
        /// Required features: "android-app-Activity", "android-app-PendingIntent", "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-app-PendingIntent", feature = "android-content-Intent")))]
        pub fn startResolutionActivity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "startResolutionActivity", .descriptor == "(Landroid/app/Activity;ILandroid/content/Intent;Landroid/app/PendingIntent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "startResolutionActivity\0", "(Landroid/app/Activity;ILandroid/content/Intent;Landroid/app/PendingIntent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEuiccInfo](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#getEuiccInfo())
        ///
        /// Required features: "android-telephony-euicc-EuiccInfo"
        #[cfg(any(feature = "all", all(feature = "android-telephony-euicc-EuiccInfo")))]
        pub fn getEuiccInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::euicc::EuiccInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "getEuiccInfo", .descriptor == "()Landroid/telephony/euicc/EuiccInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "getEuiccInfo\0", "()Landroid/telephony/euicc/EuiccInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deleteSubscription](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#deleteSubscription(int,%20android.app.PendingIntent))
        ///
        /// Required features: "android-app-PendingIntent"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent")))]
        pub fn deleteSubscription<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "deleteSubscription", .descriptor == "(ILandroid/app/PendingIntent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "deleteSubscription\0", "(ILandroid/app/PendingIntent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [switchToSubscription](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#switchToSubscription(int,%20android.app.PendingIntent))
        ///
        /// Required features: "android-app-PendingIntent"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent")))]
        pub fn switchToSubscription<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "switchToSubscription", .descriptor == "(ILandroid/app/PendingIntent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "switchToSubscription\0", "(ILandroid/app/PendingIntent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateSubscriptionNickname](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#updateSubscriptionNickname(int,%20java.lang.String,%20android.app.PendingIntent))
        ///
        /// Required features: "android-app-PendingIntent", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent", feature = "java-lang-String")))]
        pub fn updateSubscriptionNickname<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/euicc/EuiccManager", java.flags == PUBLIC, .name == "updateSubscriptionNickname", .descriptor == "(ILjava/lang/String;Landroid/app/PendingIntent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/euicc/EuiccManager\0", "updateSubscriptionNickname\0", "(ILjava/lang/String;Landroid/app/PendingIntent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_MANAGE_EMBEDDED_SUBSCRIPTIONS](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#ACTION_MANAGE_EMBEDDED_SUBSCRIPTIONS)
        pub const ACTION_MANAGE_EMBEDDED_SUBSCRIPTIONS : &'static str = "android.telephony.euicc.action.MANAGE_EMBEDDED_SUBSCRIPTIONS";

        /// public static final [ACTION_NOTIFY_CARRIER_SETUP_INCOMPLETE](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#ACTION_NOTIFY_CARRIER_SETUP_INCOMPLETE)
        pub const ACTION_NOTIFY_CARRIER_SETUP_INCOMPLETE : &'static str = "android.telephony.euicc.action.NOTIFY_CARRIER_SETUP_INCOMPLETE";

        /// public static final [EMBEDDED_SUBSCRIPTION_RESULT_ERROR](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#EMBEDDED_SUBSCRIPTION_RESULT_ERROR)
        pub const EMBEDDED_SUBSCRIPTION_RESULT_ERROR : i32 = 2;

        /// public static final [EMBEDDED_SUBSCRIPTION_RESULT_OK](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#EMBEDDED_SUBSCRIPTION_RESULT_OK)
        pub const EMBEDDED_SUBSCRIPTION_RESULT_OK : i32 = 0;

        /// public static final [EMBEDDED_SUBSCRIPTION_RESULT_RESOLVABLE_ERROR](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#EMBEDDED_SUBSCRIPTION_RESULT_RESOLVABLE_ERROR)
        pub const EMBEDDED_SUBSCRIPTION_RESULT_RESOLVABLE_ERROR : i32 = 1;

        /// public static final [EXTRA_EMBEDDED_SUBSCRIPTION_DETAILED_CODE](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#EXTRA_EMBEDDED_SUBSCRIPTION_DETAILED_CODE)
        pub const EXTRA_EMBEDDED_SUBSCRIPTION_DETAILED_CODE : &'static str = "android.telephony.euicc.extra.EMBEDDED_SUBSCRIPTION_DETAILED_CODE";

        /// public static final [EXTRA_EMBEDDED_SUBSCRIPTION_DOWNLOADABLE_SUBSCRIPTION](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#EXTRA_EMBEDDED_SUBSCRIPTION_DOWNLOADABLE_SUBSCRIPTION)
        pub const EXTRA_EMBEDDED_SUBSCRIPTION_DOWNLOADABLE_SUBSCRIPTION : &'static str = "android.telephony.euicc.extra.EMBEDDED_SUBSCRIPTION_DOWNLOADABLE_SUBSCRIPTION";

        /// public static final [META_DATA_CARRIER_ICON](https://developer.android.com/reference/android/telephony/euicc/EuiccManager.html#META_DATA_CARRIER_ICON)
        pub const META_DATA_CARRIER_ICON : &'static str = "android.telephony.euicc.carriericon";
    }
}
