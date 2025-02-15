// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-InputType"))]
__jni_bindgen! {
    /// public interface [InputType](https://developer.android.com/reference/android/text/InputType.html)
    ///
    /// Required feature: android-text-InputType
    public interface InputType ("android/text/InputType") extends crate::java::lang::Object {

        /// public static final [TYPE_CLASS_DATETIME](https://developer.android.com/reference/android/text/InputType.html#TYPE_CLASS_DATETIME)
        pub const TYPE_CLASS_DATETIME : i32 = 4;

        /// public static final [TYPE_CLASS_NUMBER](https://developer.android.com/reference/android/text/InputType.html#TYPE_CLASS_NUMBER)
        pub const TYPE_CLASS_NUMBER : i32 = 2;

        /// public static final [TYPE_CLASS_PHONE](https://developer.android.com/reference/android/text/InputType.html#TYPE_CLASS_PHONE)
        pub const TYPE_CLASS_PHONE : i32 = 3;

        /// public static final [TYPE_CLASS_TEXT](https://developer.android.com/reference/android/text/InputType.html#TYPE_CLASS_TEXT)
        pub const TYPE_CLASS_TEXT : i32 = 1;

        /// public static final [TYPE_DATETIME_VARIATION_DATE](https://developer.android.com/reference/android/text/InputType.html#TYPE_DATETIME_VARIATION_DATE)
        pub const TYPE_DATETIME_VARIATION_DATE : i32 = 16;

        /// public static final [TYPE_DATETIME_VARIATION_NORMAL](https://developer.android.com/reference/android/text/InputType.html#TYPE_DATETIME_VARIATION_NORMAL)
        pub const TYPE_DATETIME_VARIATION_NORMAL : i32 = 0;

        /// public static final [TYPE_DATETIME_VARIATION_TIME](https://developer.android.com/reference/android/text/InputType.html#TYPE_DATETIME_VARIATION_TIME)
        pub const TYPE_DATETIME_VARIATION_TIME : i32 = 32;

        /// public static final [TYPE_MASK_CLASS](https://developer.android.com/reference/android/text/InputType.html#TYPE_MASK_CLASS)
        pub const TYPE_MASK_CLASS : i32 = 15;

        /// public static final [TYPE_MASK_FLAGS](https://developer.android.com/reference/android/text/InputType.html#TYPE_MASK_FLAGS)
        pub const TYPE_MASK_FLAGS : i32 = 16773120;

        /// public static final [TYPE_MASK_VARIATION](https://developer.android.com/reference/android/text/InputType.html#TYPE_MASK_VARIATION)
        pub const TYPE_MASK_VARIATION : i32 = 4080;

        /// public static final [TYPE_NULL](https://developer.android.com/reference/android/text/InputType.html#TYPE_NULL)
        pub const TYPE_NULL : i32 = 0;

        /// public static final [TYPE_NUMBER_FLAG_DECIMAL](https://developer.android.com/reference/android/text/InputType.html#TYPE_NUMBER_FLAG_DECIMAL)
        pub const TYPE_NUMBER_FLAG_DECIMAL : i32 = 8192;

        /// public static final [TYPE_NUMBER_FLAG_SIGNED](https://developer.android.com/reference/android/text/InputType.html#TYPE_NUMBER_FLAG_SIGNED)
        pub const TYPE_NUMBER_FLAG_SIGNED : i32 = 4096;

        /// public static final [TYPE_NUMBER_VARIATION_NORMAL](https://developer.android.com/reference/android/text/InputType.html#TYPE_NUMBER_VARIATION_NORMAL)
        pub const TYPE_NUMBER_VARIATION_NORMAL : i32 = 0;

        /// public static final [TYPE_NUMBER_VARIATION_PASSWORD](https://developer.android.com/reference/android/text/InputType.html#TYPE_NUMBER_VARIATION_PASSWORD)
        pub const TYPE_NUMBER_VARIATION_PASSWORD : i32 = 16;

        /// public static final [TYPE_TEXT_FLAG_AUTO_COMPLETE](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_AUTO_COMPLETE)
        pub const TYPE_TEXT_FLAG_AUTO_COMPLETE : i32 = 65536;

        /// public static final [TYPE_TEXT_FLAG_AUTO_CORRECT](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_AUTO_CORRECT)
        pub const TYPE_TEXT_FLAG_AUTO_CORRECT : i32 = 32768;

        /// public static final [TYPE_TEXT_FLAG_CAP_CHARACTERS](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_CAP_CHARACTERS)
        pub const TYPE_TEXT_FLAG_CAP_CHARACTERS : i32 = 4096;

        /// public static final [TYPE_TEXT_FLAG_CAP_SENTENCES](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_CAP_SENTENCES)
        pub const TYPE_TEXT_FLAG_CAP_SENTENCES : i32 = 16384;

        /// public static final [TYPE_TEXT_FLAG_CAP_WORDS](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_CAP_WORDS)
        pub const TYPE_TEXT_FLAG_CAP_WORDS : i32 = 8192;

        /// public static final [TYPE_TEXT_FLAG_IME_MULTI_LINE](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_IME_MULTI_LINE)
        pub const TYPE_TEXT_FLAG_IME_MULTI_LINE : i32 = 262144;

        /// public static final [TYPE_TEXT_FLAG_MULTI_LINE](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_MULTI_LINE)
        pub const TYPE_TEXT_FLAG_MULTI_LINE : i32 = 131072;

        /// public static final [TYPE_TEXT_FLAG_NO_SUGGESTIONS](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_FLAG_NO_SUGGESTIONS)
        pub const TYPE_TEXT_FLAG_NO_SUGGESTIONS : i32 = 524288;

        /// public static final [TYPE_TEXT_VARIATION_EMAIL_ADDRESS](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_EMAIL_ADDRESS)
        pub const TYPE_TEXT_VARIATION_EMAIL_ADDRESS : i32 = 32;

        /// public static final [TYPE_TEXT_VARIATION_EMAIL_SUBJECT](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_EMAIL_SUBJECT)
        pub const TYPE_TEXT_VARIATION_EMAIL_SUBJECT : i32 = 48;

        /// public static final [TYPE_TEXT_VARIATION_FILTER](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_FILTER)
        pub const TYPE_TEXT_VARIATION_FILTER : i32 = 176;

        /// public static final [TYPE_TEXT_VARIATION_LONG_MESSAGE](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_LONG_MESSAGE)
        pub const TYPE_TEXT_VARIATION_LONG_MESSAGE : i32 = 80;

        /// public static final [TYPE_TEXT_VARIATION_NORMAL](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_NORMAL)
        pub const TYPE_TEXT_VARIATION_NORMAL : i32 = 0;

        /// public static final [TYPE_TEXT_VARIATION_PASSWORD](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_PASSWORD)
        pub const TYPE_TEXT_VARIATION_PASSWORD : i32 = 128;

        /// public static final [TYPE_TEXT_VARIATION_PERSON_NAME](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_PERSON_NAME)
        pub const TYPE_TEXT_VARIATION_PERSON_NAME : i32 = 96;

        /// public static final [TYPE_TEXT_VARIATION_PHONETIC](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_PHONETIC)
        pub const TYPE_TEXT_VARIATION_PHONETIC : i32 = 192;

        /// public static final [TYPE_TEXT_VARIATION_POSTAL_ADDRESS](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_POSTAL_ADDRESS)
        pub const TYPE_TEXT_VARIATION_POSTAL_ADDRESS : i32 = 112;

        /// public static final [TYPE_TEXT_VARIATION_SHORT_MESSAGE](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_SHORT_MESSAGE)
        pub const TYPE_TEXT_VARIATION_SHORT_MESSAGE : i32 = 64;

        /// public static final [TYPE_TEXT_VARIATION_URI](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_URI)
        pub const TYPE_TEXT_VARIATION_URI : i32 = 16;

        /// public static final [TYPE_TEXT_VARIATION_VISIBLE_PASSWORD](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_VISIBLE_PASSWORD)
        pub const TYPE_TEXT_VARIATION_VISIBLE_PASSWORD : i32 = 144;

        /// public static final [TYPE_TEXT_VARIATION_WEB_EDIT_TEXT](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_WEB_EDIT_TEXT)
        pub const TYPE_TEXT_VARIATION_WEB_EDIT_TEXT : i32 = 160;

        /// public static final [TYPE_TEXT_VARIATION_WEB_EMAIL_ADDRESS](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_WEB_EMAIL_ADDRESS)
        pub const TYPE_TEXT_VARIATION_WEB_EMAIL_ADDRESS : i32 = 208;

        /// public static final [TYPE_TEXT_VARIATION_WEB_PASSWORD](https://developer.android.com/reference/android/text/InputType.html#TYPE_TEXT_VARIATION_WEB_PASSWORD)
        pub const TYPE_TEXT_VARIATION_WEB_PASSWORD : i32 = 224;
    }
}
