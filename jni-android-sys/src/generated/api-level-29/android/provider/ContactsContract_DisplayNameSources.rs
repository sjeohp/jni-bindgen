// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_DisplayNameSources"))]
__jni_bindgen! {
    /// public interface [ContactsContract.DisplayNameSources](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html)
    ///
    /// Required feature: android-provider-ContactsContract_DisplayNameSources
    public interface ContactsContract_DisplayNameSources ("android/provider/ContactsContract$DisplayNameSources") extends crate::java::lang::Object {

        /// public static final [EMAIL](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#EMAIL)
        pub const EMAIL : i32 = 10;

        /// public static final [NICKNAME](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#NICKNAME)
        pub const NICKNAME : i32 = 35;

        /// public static final [ORGANIZATION](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#ORGANIZATION)
        pub const ORGANIZATION : i32 = 30;

        /// public static final [PHONE](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#PHONE)
        pub const PHONE : i32 = 20;

        /// public static final [STRUCTURED_NAME](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#STRUCTURED_NAME)
        pub const STRUCTURED_NAME : i32 = 40;

        /// public static final [STRUCTURED_PHONETIC_NAME](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#STRUCTURED_PHONETIC_NAME)
        pub const STRUCTURED_PHONETIC_NAME : i32 = 37;

        /// public static final [UNDEFINED](https://developer.android.com/reference/android/provider/ContactsContract.DisplayNameSources.html#UNDEFINED)
        pub const UNDEFINED : i32 = 0;
    }
}
