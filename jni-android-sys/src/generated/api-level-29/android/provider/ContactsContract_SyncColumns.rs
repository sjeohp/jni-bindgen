// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_SyncColumns"))]
__jni_bindgen! {
    /// public interface [ContactsContract.SyncColumns](https://developer.android.com/reference/android/provider/ContactsContract.SyncColumns.html)
    ///
    /// Required feature: android-provider-ContactsContract_SyncColumns
    public interface ContactsContract_SyncColumns ("android/provider/ContactsContract$SyncColumns") extends crate::java::lang::Object, implements crate::android::provider::ContactsContract_BaseSyncColumns {

        /// public static final [ACCOUNT_NAME](https://developer.android.com/reference/android/provider/ContactsContract.SyncColumns.html#ACCOUNT_NAME)
        pub const ACCOUNT_NAME : &'static str = "account_name";

        /// public static final [ACCOUNT_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.SyncColumns.html#ACCOUNT_TYPE)
        pub const ACCOUNT_TYPE : &'static str = "account_type";

        /// public static final [DIRTY](https://developer.android.com/reference/android/provider/ContactsContract.SyncColumns.html#DIRTY)
        pub const DIRTY : &'static str = "dirty";

        /// public static final [SOURCE_ID](https://developer.android.com/reference/android/provider/ContactsContract.SyncColumns.html#SOURCE_ID)
        pub const SOURCE_ID : &'static str = "sourceid";

        /// public static final [VERSION](https://developer.android.com/reference/android/provider/ContactsContract.SyncColumns.html#VERSION)
        pub const VERSION : &'static str = "version";
    }
}
