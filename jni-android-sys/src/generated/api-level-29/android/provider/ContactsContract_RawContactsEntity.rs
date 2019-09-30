// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_RawContactsEntity"))]
__jni_bindgen! {
    /// public final class [ContactsContract.RawContactsEntity](https://developer.android.com/reference/android/provider/ContactsContract.RawContactsEntity.html)
    ///
    /// Required feature: android-provider-ContactsContract_RawContactsEntity
    public final class ContactsContract_RawContactsEntity ("android/provider/ContactsContract$RawContactsEntity") extends crate::java::lang::Object, implements crate::android::provider::BaseColumns, crate::android::provider::ContactsContract_DataColumns, crate::android::provider::ContactsContract_RawContactsColumns {

        // // Not emitting: Non-public method
        // /// [RawContactsEntity](https://developer.android.com/reference/android/provider/ContactsContract.RawContactsEntity.html#RawContactsEntity())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_RawContactsEntity>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/ContactsContract$RawContactsEntity", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$RawContactsEntity\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [CONTENT_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.RawContactsEntity.html#CONTENT_TYPE)
        pub const CONTENT_TYPE : &'static str = "vnd.android.cursor.dir/raw_contact_entity";

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/ContactsContract.RawContactsEntity.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/ContactsContract$RawContactsEntity\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [DATA_ID](https://developer.android.com/reference/android/provider/ContactsContract.RawContactsEntity.html#DATA_ID)
        pub const DATA_ID : &'static str = "data_id";

        /// **get** public static final [PROFILE_CONTENT_URI](https://developer.android.com/reference/android/provider/ContactsContract.RawContactsEntity.html#PROFILE_CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn PROFILE_CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/ContactsContract$RawContactsEntity\0", "PROFILE_CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
