// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-CalendarContract_SyncColumns"))]
__jni_bindgen! {
    /// public interface [CalendarContract.SyncColumns](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html)
    ///
    /// Required feature: android-provider-CalendarContract_SyncColumns
    public interface CalendarContract_SyncColumns ("android/provider/CalendarContract$SyncColumns") extends crate::java::lang::Object, implements crate::android::provider::CalendarContract_CalendarSyncColumns {

        /// public static final [ACCOUNT_NAME](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#ACCOUNT_NAME)
        pub const ACCOUNT_NAME : &'static str = "account_name";

        /// public static final [ACCOUNT_TYPE](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#ACCOUNT_TYPE)
        pub const ACCOUNT_TYPE : &'static str = "account_type";

        /// public static final [CAN_PARTIALLY_UPDATE](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#CAN_PARTIALLY_UPDATE)
        pub const CAN_PARTIALLY_UPDATE : &'static str = "canPartiallyUpdate";

        /// public static final [DELETED](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#DELETED)
        pub const DELETED : &'static str = "deleted";

        /// public static final [DIRTY](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#DIRTY)
        pub const DIRTY : &'static str = "dirty";

        /// public static final [MUTATORS](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#MUTATORS)
        pub const MUTATORS : &'static str = "mutators";

        /// public static final [_SYNC_ID](https://developer.android.com/reference/android/provider/CalendarContract.SyncColumns.html#_SYNC_ID)
        pub const _SYNC_ID : &'static str = "_sync_id";
    }
}
