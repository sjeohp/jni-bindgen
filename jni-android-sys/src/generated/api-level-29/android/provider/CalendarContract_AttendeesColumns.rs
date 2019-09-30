// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-CalendarContract_AttendeesColumns"))]
__jni_bindgen! {
    /// public interface [CalendarContract.AttendeesColumns](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html)
    ///
    /// Required feature: android-provider-CalendarContract_AttendeesColumns
    public interface CalendarContract_AttendeesColumns ("android/provider/CalendarContract$AttendeesColumns") extends crate::java::lang::Object {

        /// public static final [ATTENDEE_EMAIL](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_EMAIL)
        pub const ATTENDEE_EMAIL : &'static str = "attendeeEmail";

        /// public static final [ATTENDEE_IDENTITY](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_IDENTITY)
        pub const ATTENDEE_IDENTITY : &'static str = "attendeeIdentity";

        /// public static final [ATTENDEE_ID_NAMESPACE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_ID_NAMESPACE)
        pub const ATTENDEE_ID_NAMESPACE : &'static str = "attendeeIdNamespace";

        /// public static final [ATTENDEE_NAME](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_NAME)
        pub const ATTENDEE_NAME : &'static str = "attendeeName";

        /// public static final [ATTENDEE_RELATIONSHIP](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_RELATIONSHIP)
        pub const ATTENDEE_RELATIONSHIP : &'static str = "attendeeRelationship";

        /// public static final [ATTENDEE_STATUS](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_STATUS)
        pub const ATTENDEE_STATUS : &'static str = "attendeeStatus";

        /// public static final [ATTENDEE_STATUS_ACCEPTED](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_STATUS_ACCEPTED)
        pub const ATTENDEE_STATUS_ACCEPTED : i32 = 1;

        /// public static final [ATTENDEE_STATUS_DECLINED](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_STATUS_DECLINED)
        pub const ATTENDEE_STATUS_DECLINED : i32 = 2;

        /// public static final [ATTENDEE_STATUS_INVITED](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_STATUS_INVITED)
        pub const ATTENDEE_STATUS_INVITED : i32 = 3;

        /// public static final [ATTENDEE_STATUS_NONE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_STATUS_NONE)
        pub const ATTENDEE_STATUS_NONE : i32 = 0;

        /// public static final [ATTENDEE_STATUS_TENTATIVE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_STATUS_TENTATIVE)
        pub const ATTENDEE_STATUS_TENTATIVE : i32 = 4;

        /// public static final [ATTENDEE_TYPE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#ATTENDEE_TYPE)
        pub const ATTENDEE_TYPE : &'static str = "attendeeType";

        /// public static final [EVENT_ID](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#EVENT_ID)
        pub const EVENT_ID : &'static str = "event_id";

        /// public static final [RELATIONSHIP_ATTENDEE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#RELATIONSHIP_ATTENDEE)
        pub const RELATIONSHIP_ATTENDEE : i32 = 1;

        /// public static final [RELATIONSHIP_NONE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#RELATIONSHIP_NONE)
        pub const RELATIONSHIP_NONE : i32 = 0;

        /// public static final [RELATIONSHIP_ORGANIZER](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#RELATIONSHIP_ORGANIZER)
        pub const RELATIONSHIP_ORGANIZER : i32 = 2;

        /// public static final [RELATIONSHIP_PERFORMER](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#RELATIONSHIP_PERFORMER)
        pub const RELATIONSHIP_PERFORMER : i32 = 3;

        /// public static final [RELATIONSHIP_SPEAKER](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#RELATIONSHIP_SPEAKER)
        pub const RELATIONSHIP_SPEAKER : i32 = 4;

        /// public static final [TYPE_NONE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#TYPE_NONE)
        pub const TYPE_NONE : i32 = 0;

        /// public static final [TYPE_OPTIONAL](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#TYPE_OPTIONAL)
        pub const TYPE_OPTIONAL : i32 = 2;

        /// public static final [TYPE_REQUIRED](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#TYPE_REQUIRED)
        pub const TYPE_REQUIRED : i32 = 1;

        /// public static final [TYPE_RESOURCE](https://developer.android.com/reference/android/provider/CalendarContract.AttendeesColumns.html#TYPE_RESOURCE)
        pub const TYPE_RESOURCE : i32 = 3;
    }
}
