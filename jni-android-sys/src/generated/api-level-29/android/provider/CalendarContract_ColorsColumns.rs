// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-CalendarContract_ColorsColumns"))]
__jni_bindgen! {
    /// public interface [CalendarContract.ColorsColumns](https://developer.android.com/reference/android/provider/CalendarContract.ColorsColumns.html)
    ///
    /// Required feature: android-provider-CalendarContract_ColorsColumns
    public interface CalendarContract_ColorsColumns ("android/provider/CalendarContract$ColorsColumns") extends crate::java::lang::Object, implements crate::android::provider::SyncStateContract_Columns {

        /// public static final [COLOR](https://developer.android.com/reference/android/provider/CalendarContract.ColorsColumns.html#COLOR)
        pub const COLOR : &'static str = "color";

        /// public static final [COLOR_KEY](https://developer.android.com/reference/android/provider/CalendarContract.ColorsColumns.html#COLOR_KEY)
        pub const COLOR_KEY : &'static str = "color_index";

        /// public static final [COLOR_TYPE](https://developer.android.com/reference/android/provider/CalendarContract.ColorsColumns.html#COLOR_TYPE)
        pub const COLOR_TYPE : &'static str = "color_type";

        /// public static final [TYPE_CALENDAR](https://developer.android.com/reference/android/provider/CalendarContract.ColorsColumns.html#TYPE_CALENDAR)
        pub const TYPE_CALENDAR : i32 = 0;

        /// public static final [TYPE_EVENT](https://developer.android.com/reference/android/provider/CalendarContract.ColorsColumns.html#TYPE_EVENT)
        pub const TYPE_EVENT : i32 = 1;
    }
}
