// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-gsm-SmsMessage_MessageClass"))]
__jni_bindgen! {
    /// public enum [SmsMessage.MessageClass](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html)
    ///
    /// Required feature: android-telephony-gsm-SmsMessage_MessageClass
    #[deprecated] public enum SmsMessage_MessageClass ("android/telephony/gsm/SmsMessage$MessageClass") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#values())
        ///
        /// Required features: "android-telephony-gsm-SmsMessage_MessageClass"
        #[cfg(any(feature = "all", all(feature = "android-telephony-gsm-SmsMessage_MessageClass")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::telephony::gsm::SmsMessage_MessageClass, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/gsm/SmsMessage$MessageClass", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/telephony/gsm/SmsMessage$MessageClass;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/telephony/gsm/SmsMessage$MessageClass\0", "values\0", "()[Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-telephony-gsm-SmsMessage_MessageClass", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-telephony-gsm-SmsMessage_MessageClass", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/gsm/SmsMessage$MessageClass", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/telephony/gsm/SmsMessage$MessageClass;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/telephony/gsm/SmsMessage$MessageClass\0", "valueOf\0", "(Ljava/lang/String;)Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [MessageClass](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#MessageClass(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/gsm/SmsMessage$MessageClass", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/gsm/SmsMessage$MessageClass\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [UNKNOWN](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#UNKNOWN)
        ///
        /// Required feature: android-telephony-gsm-SmsMessage_MessageClass
        #[cfg(any(feature = "all", feature = "android-telephony-gsm-SmsMessage_MessageClass"))]
        #[deprecated] pub fn UNKNOWN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telephony/gsm/SmsMessage$MessageClass\0", "UNKNOWN\0", "Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CLASS_0](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#CLASS_0)
        ///
        /// Required feature: android-telephony-gsm-SmsMessage_MessageClass
        #[cfg(any(feature = "all", feature = "android-telephony-gsm-SmsMessage_MessageClass"))]
        #[deprecated] pub fn CLASS_0<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telephony/gsm/SmsMessage$MessageClass\0", "CLASS_0\0", "Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CLASS_1](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#CLASS_1)
        ///
        /// Required feature: android-telephony-gsm-SmsMessage_MessageClass
        #[cfg(any(feature = "all", feature = "android-telephony-gsm-SmsMessage_MessageClass"))]
        #[deprecated] pub fn CLASS_1<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telephony/gsm/SmsMessage$MessageClass\0", "CLASS_1\0", "Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CLASS_2](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#CLASS_2)
        ///
        /// Required feature: android-telephony-gsm-SmsMessage_MessageClass
        #[cfg(any(feature = "all", feature = "android-telephony-gsm-SmsMessage_MessageClass"))]
        #[deprecated] pub fn CLASS_2<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telephony/gsm/SmsMessage$MessageClass\0", "CLASS_2\0", "Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CLASS_3](https://developer.android.com/reference/android/telephony/gsm/SmsMessage.MessageClass.html#CLASS_3)
        ///
        /// Required feature: android-telephony-gsm-SmsMessage_MessageClass
        #[cfg(any(feature = "all", feature = "android-telephony-gsm-SmsMessage_MessageClass"))]
        #[deprecated] pub fn CLASS_3<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::gsm::SmsMessage_MessageClass>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telephony/gsm/SmsMessage$MessageClass\0", "CLASS_3\0", "Landroid/telephony/gsm/SmsMessage$MessageClass;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::telephony::gsm::SmsMessage_MessageClass, crate::java::lang::Throwable>>> { ... }
    }
}
