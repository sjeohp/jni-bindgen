// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-tv-TvContract_RecordedPrograms"))]
__jni_bindgen! {
    /// public final class [TvContract.RecordedPrograms](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html)
    ///
    /// Required feature: android-media-tv-TvContract_RecordedPrograms
    public final class TvContract_RecordedPrograms ("android/media/tv/TvContract$RecordedPrograms") extends crate::java::lang::Object, implements crate::android::media::tv::TvContract_BaseTvColumns {

        // // Not emitting: Non-public method
        // /// [RecordedPrograms](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#RecordedPrograms())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::tv::TvContract_RecordedPrograms>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/tv/TvContract$RecordedPrograms", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvContract$RecordedPrograms\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [COLUMN_AUDIO_LANGUAGE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_AUDIO_LANGUAGE)
        pub const COLUMN_AUDIO_LANGUAGE : &'static str = "audio_language";

        /// public static final [COLUMN_BROADCAST_GENRE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_BROADCAST_GENRE)
        pub const COLUMN_BROADCAST_GENRE : &'static str = "broadcast_genre";

        /// public static final [COLUMN_CANONICAL_GENRE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_CANONICAL_GENRE)
        pub const COLUMN_CANONICAL_GENRE : &'static str = "canonical_genre";

        /// public static final [COLUMN_CHANNEL_ID](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_CHANNEL_ID)
        pub const COLUMN_CHANNEL_ID : &'static str = "channel_id";

        /// public static final [COLUMN_CONTENT_RATING](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_CONTENT_RATING)
        pub const COLUMN_CONTENT_RATING : &'static str = "content_rating";

        /// public static final [COLUMN_END_TIME_UTC_MILLIS](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_END_TIME_UTC_MILLIS)
        pub const COLUMN_END_TIME_UTC_MILLIS : &'static str = "end_time_utc_millis";

        /// public static final [COLUMN_EPISODE_DISPLAY_NUMBER](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_EPISODE_DISPLAY_NUMBER)
        pub const COLUMN_EPISODE_DISPLAY_NUMBER : &'static str = "episode_display_number";

        /// public static final [COLUMN_EPISODE_TITLE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_EPISODE_TITLE)
        pub const COLUMN_EPISODE_TITLE : &'static str = "episode_title";

        /// public static final [COLUMN_INPUT_ID](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_INPUT_ID)
        pub const COLUMN_INPUT_ID : &'static str = "input_id";

        /// public static final [COLUMN_INTERNAL_PROVIDER_DATA](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_INTERNAL_PROVIDER_DATA)
        pub const COLUMN_INTERNAL_PROVIDER_DATA : &'static str = "internal_provider_data";

        /// public static final [COLUMN_INTERNAL_PROVIDER_FLAG1](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_INTERNAL_PROVIDER_FLAG1)
        pub const COLUMN_INTERNAL_PROVIDER_FLAG1 : &'static str = "internal_provider_flag1";

        /// public static final [COLUMN_INTERNAL_PROVIDER_FLAG2](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_INTERNAL_PROVIDER_FLAG2)
        pub const COLUMN_INTERNAL_PROVIDER_FLAG2 : &'static str = "internal_provider_flag2";

        /// public static final [COLUMN_INTERNAL_PROVIDER_FLAG3](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_INTERNAL_PROVIDER_FLAG3)
        pub const COLUMN_INTERNAL_PROVIDER_FLAG3 : &'static str = "internal_provider_flag3";

        /// public static final [COLUMN_INTERNAL_PROVIDER_FLAG4](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_INTERNAL_PROVIDER_FLAG4)
        pub const COLUMN_INTERNAL_PROVIDER_FLAG4 : &'static str = "internal_provider_flag4";

        /// public static final [COLUMN_LONG_DESCRIPTION](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_LONG_DESCRIPTION)
        pub const COLUMN_LONG_DESCRIPTION : &'static str = "long_description";

        /// public static final [COLUMN_POSTER_ART_URI](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_POSTER_ART_URI)
        pub const COLUMN_POSTER_ART_URI : &'static str = "poster_art_uri";

        /// public static final [COLUMN_RECORDING_DATA_BYTES](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_RECORDING_DATA_BYTES)
        pub const COLUMN_RECORDING_DATA_BYTES : &'static str = "recording_data_bytes";

        /// public static final [COLUMN_RECORDING_DATA_URI](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_RECORDING_DATA_URI)
        pub const COLUMN_RECORDING_DATA_URI : &'static str = "recording_data_uri";

        /// public static final [COLUMN_RECORDING_DURATION_MILLIS](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_RECORDING_DURATION_MILLIS)
        pub const COLUMN_RECORDING_DURATION_MILLIS : &'static str = "recording_duration_millis";

        /// public static final [COLUMN_RECORDING_EXPIRE_TIME_UTC_MILLIS](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_RECORDING_EXPIRE_TIME_UTC_MILLIS)
        pub const COLUMN_RECORDING_EXPIRE_TIME_UTC_MILLIS : &'static str = "recording_expire_time_utc_millis";

        /// public static final [COLUMN_REVIEW_RATING](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_REVIEW_RATING)
        pub const COLUMN_REVIEW_RATING : &'static str = "review_rating";

        /// public static final [COLUMN_REVIEW_RATING_STYLE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_REVIEW_RATING_STYLE)
        pub const COLUMN_REVIEW_RATING_STYLE : &'static str = "review_rating_style";

        /// public static final [COLUMN_SEARCHABLE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_SEARCHABLE)
        pub const COLUMN_SEARCHABLE : &'static str = "searchable";

        /// public static final [COLUMN_SEASON_DISPLAY_NUMBER](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_SEASON_DISPLAY_NUMBER)
        pub const COLUMN_SEASON_DISPLAY_NUMBER : &'static str = "season_display_number";

        /// public static final [COLUMN_SEASON_TITLE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_SEASON_TITLE)
        pub const COLUMN_SEASON_TITLE : &'static str = "season_title";

        /// public static final [COLUMN_SERIES_ID](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_SERIES_ID)
        pub const COLUMN_SERIES_ID : &'static str = "series_id";

        /// public static final [COLUMN_SHORT_DESCRIPTION](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_SHORT_DESCRIPTION)
        pub const COLUMN_SHORT_DESCRIPTION : &'static str = "short_description";

        /// public static final [COLUMN_START_TIME_UTC_MILLIS](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_START_TIME_UTC_MILLIS)
        pub const COLUMN_START_TIME_UTC_MILLIS : &'static str = "start_time_utc_millis";

        /// public static final [COLUMN_THUMBNAIL_URI](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_THUMBNAIL_URI)
        pub const COLUMN_THUMBNAIL_URI : &'static str = "thumbnail_uri";

        /// public static final [COLUMN_TITLE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_TITLE)
        pub const COLUMN_TITLE : &'static str = "title";

        /// public static final [COLUMN_VERSION_NUMBER](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_VERSION_NUMBER)
        pub const COLUMN_VERSION_NUMBER : &'static str = "version_number";

        /// public static final [COLUMN_VIDEO_HEIGHT](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_VIDEO_HEIGHT)
        pub const COLUMN_VIDEO_HEIGHT : &'static str = "video_height";

        /// public static final [COLUMN_VIDEO_WIDTH](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#COLUMN_VIDEO_WIDTH)
        pub const COLUMN_VIDEO_WIDTH : &'static str = "video_width";

        /// public static final [CONTENT_ITEM_TYPE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#CONTENT_ITEM_TYPE)
        pub const CONTENT_ITEM_TYPE : &'static str = "vnd.android.cursor.item/recorded_program";

        /// public static final [CONTENT_TYPE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#CONTENT_TYPE)
        pub const CONTENT_TYPE : &'static str = "vnd.android.cursor.dir/recorded_program";

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/media/tv/TvContract$RecordedPrograms\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [REVIEW_RATING_STYLE_PERCENTAGE](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#REVIEW_RATING_STYLE_PERCENTAGE)
        pub const REVIEW_RATING_STYLE_PERCENTAGE : i32 = 2;

        /// public static final [REVIEW_RATING_STYLE_STARS](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#REVIEW_RATING_STYLE_STARS)
        pub const REVIEW_RATING_STYLE_STARS : i32 = 0;

        /// public static final [REVIEW_RATING_STYLE_THUMBS_UP_DOWN](https://developer.android.com/reference/android/media/tv/TvContract.RecordedPrograms.html#REVIEW_RATING_STYLE_THUMBS_UP_DOWN)
        pub const REVIEW_RATING_STYLE_THUMBS_UP_DOWN : i32 = 1;
    }
}
