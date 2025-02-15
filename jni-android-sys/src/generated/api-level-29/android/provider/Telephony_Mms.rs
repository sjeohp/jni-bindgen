// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-Telephony_Mms"))]
__jni_bindgen! {
    /// public final class [Telephony.Mms](https://developer.android.com/reference/android/provider/Telephony.Mms.html)
    ///
    /// Required feature: android-provider-Telephony_Mms
    public final class Telephony_Mms ("android/provider/Telephony$Mms") extends crate::java::lang::Object, implements crate::android::provider::Telephony_BaseMmsColumns {

        // // Not emitting: Non-public method
        // /// [Mms](https://developer.android.com/reference/android/provider/Telephony.Mms.html#Mms())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::Telephony_Mms>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/Telephony$Mms", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/Telephony$Mms\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/Telephony.Mms.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/Telephony$Mms\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [DEFAULT_SORT_ORDER](https://developer.android.com/reference/android/provider/Telephony.Mms.html#DEFAULT_SORT_ORDER)
        pub const DEFAULT_SORT_ORDER : &'static str = "date DESC";

        /// **get** public static final [REPORT_REQUEST_URI](https://developer.android.com/reference/android/provider/Telephony.Mms.html#REPORT_REQUEST_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn REPORT_REQUEST_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/Telephony$Mms\0", "REPORT_REQUEST_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REPORT_STATUS_URI](https://developer.android.com/reference/android/provider/Telephony.Mms.html#REPORT_STATUS_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn REPORT_STATUS_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/Telephony$Mms\0", "REPORT_STATUS_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
