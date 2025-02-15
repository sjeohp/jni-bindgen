// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_Intents_Insert"))]
__jni_bindgen! {
    /// public final class [ContactsContract.Intents.Insert](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html)
    ///
    /// Required feature: android-provider-ContactsContract_Intents_Insert
    public final class ContactsContract_Intents_Insert ("android/provider/ContactsContract$Intents$Insert") extends crate::java::lang::Object {

        /// [Insert](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#Insert())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_Intents_Insert>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Intents$Insert", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Intents$Insert\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#ACTION)
        pub const ACTION : &'static str = "android.intent.action.INSERT";

        /// public static final [COMPANY](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#COMPANY)
        pub const COMPANY : &'static str = "company";

        /// public static final [DATA](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#DATA)
        pub const DATA : &'static str = "data";

        /// public static final [EMAIL](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#EMAIL)
        pub const EMAIL : &'static str = "email";

        /// public static final [EMAIL_ISPRIMARY](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#EMAIL_ISPRIMARY)
        pub const EMAIL_ISPRIMARY : &'static str = "email_isprimary";

        /// public static final [EMAIL_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#EMAIL_TYPE)
        pub const EMAIL_TYPE : &'static str = "email_type";

        /// public static final [EXTRA_ACCOUNT](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#EXTRA_ACCOUNT)
        pub const EXTRA_ACCOUNT : &'static str = "android.provider.extra.ACCOUNT";

        /// public static final [EXTRA_DATA_SET](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#EXTRA_DATA_SET)
        pub const EXTRA_DATA_SET : &'static str = "android.provider.extra.DATA_SET";

        /// public static final [FULL_MODE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#FULL_MODE)
        pub const FULL_MODE : &'static str = "full_mode";

        /// public static final [IM_HANDLE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#IM_HANDLE)
        pub const IM_HANDLE : &'static str = "im_handle";

        /// public static final [IM_ISPRIMARY](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#IM_ISPRIMARY)
        pub const IM_ISPRIMARY : &'static str = "im_isprimary";

        /// public static final [IM_PROTOCOL](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#IM_PROTOCOL)
        pub const IM_PROTOCOL : &'static str = "im_protocol";

        /// public static final [JOB_TITLE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#JOB_TITLE)
        pub const JOB_TITLE : &'static str = "job_title";

        /// public static final [NAME](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#NAME)
        pub const NAME : &'static str = "name";

        /// public static final [NOTES](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#NOTES)
        pub const NOTES : &'static str = "notes";

        /// public static final [PHONE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#PHONE)
        pub const PHONE : &'static str = "phone";

        /// public static final [PHONETIC_NAME](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#PHONETIC_NAME)
        pub const PHONETIC_NAME : &'static str = "phonetic_name";

        /// public static final [PHONE_ISPRIMARY](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#PHONE_ISPRIMARY)
        pub const PHONE_ISPRIMARY : &'static str = "phone_isprimary";

        /// public static final [PHONE_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#PHONE_TYPE)
        pub const PHONE_TYPE : &'static str = "phone_type";

        /// public static final [POSTAL](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#POSTAL)
        pub const POSTAL : &'static str = "postal";

        /// public static final [POSTAL_ISPRIMARY](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#POSTAL_ISPRIMARY)
        pub const POSTAL_ISPRIMARY : &'static str = "postal_isprimary";

        /// public static final [POSTAL_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#POSTAL_TYPE)
        pub const POSTAL_TYPE : &'static str = "postal_type";

        /// public static final [SECONDARY_EMAIL](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#SECONDARY_EMAIL)
        pub const SECONDARY_EMAIL : &'static str = "secondary_email";

        /// public static final [SECONDARY_EMAIL_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#SECONDARY_EMAIL_TYPE)
        pub const SECONDARY_EMAIL_TYPE : &'static str = "secondary_email_type";

        /// public static final [SECONDARY_PHONE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#SECONDARY_PHONE)
        pub const SECONDARY_PHONE : &'static str = "secondary_phone";

        /// public static final [SECONDARY_PHONE_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#SECONDARY_PHONE_TYPE)
        pub const SECONDARY_PHONE_TYPE : &'static str = "secondary_phone_type";

        /// public static final [TERTIARY_EMAIL](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#TERTIARY_EMAIL)
        pub const TERTIARY_EMAIL : &'static str = "tertiary_email";

        /// public static final [TERTIARY_EMAIL_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#TERTIARY_EMAIL_TYPE)
        pub const TERTIARY_EMAIL_TYPE : &'static str = "tertiary_email_type";

        /// public static final [TERTIARY_PHONE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#TERTIARY_PHONE)
        pub const TERTIARY_PHONE : &'static str = "tertiary_phone";

        /// public static final [TERTIARY_PHONE_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Intents.Insert.html#TERTIARY_PHONE_TYPE)
        pub const TERTIARY_PHONE_TYPE : &'static str = "tertiary_phone_type";
    }
}
