// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-MediaStore"))]
__jni_bindgen! {
    /// public final class [MediaStore](https://developer.android.com/reference/android/provider/MediaStore.html)
    ///
    /// Required feature: android-provider-MediaStore
    public final class MediaStore ("android/provider/MediaStore") extends crate::java::lang::Object {

        /// [MediaStore](https://developer.android.com/reference/android/provider/MediaStore.html#MediaStore())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::MediaStore>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/MediaStore\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIncludePending](https://developer.android.com/reference/android/provider/MediaStore.html#setIncludePending(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn setIncludePending<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "setIncludePending", .descriptor == "(Landroid/net/Uri;)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "setIncludePending\0", "(Landroid/net/Uri;)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRequireOriginal](https://developer.android.com/reference/android/provider/MediaStore.html#setRequireOriginal(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn setRequireOriginal<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "setRequireOriginal", .descriptor == "(Landroid/net/Uri;)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "setRequireOriginal\0", "(Landroid/net/Uri;)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExternalVolumeNames](https://developer.android.com/reference/android/provider/MediaStore.html#getExternalVolumeNames(android.content.Context))
        ///
        /// Required features: "android-content-Context", "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-util-Set")))]
        pub fn getExternalVolumeNames<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getExternalVolumeNames", .descriptor == "(Landroid/content/Context;)Ljava/util/Set;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getExternalVolumeNames\0", "(Landroid/content/Context;)Ljava/util/Set;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVolumeName](https://developer.android.com/reference/android/provider/MediaStore.html#getVolumeName(android.net.Uri))
        ///
        /// Required features: "android-net-Uri", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "java-lang-String")))]
        pub fn getVolumeName<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getVolumeName", .descriptor == "(Landroid/net/Uri;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getVolumeName\0", "(Landroid/net/Uri;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMediaScannerUri](https://developer.android.com/reference/android/provider/MediaStore.html#getMediaScannerUri())
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn getMediaScannerUri<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getMediaScannerUri", .descriptor == "()Landroid/net/Uri;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getMediaScannerUri\0", "()Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVersion](https://developer.android.com/reference/android/provider/MediaStore.html#getVersion(android.content.Context))
        ///
        /// Required features: "android-content-Context", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String")))]
        pub fn getVersion_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getVersion", .descriptor == "(Landroid/content/Context;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getVersion\0", "(Landroid/content/Context;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVersion](https://developer.android.com/reference/android/provider/MediaStore.html#getVersion(android.content.Context,%20java.lang.String))
        ///
        /// Required features: "android-content-Context", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String")))]
        pub fn getVersion_Context_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getVersion", .descriptor == "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getVersion\0", "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDocumentUri](https://developer.android.com/reference/android/provider/MediaStore.html#getDocumentUri(android.content.Context,%20android.net.Uri))
        ///
        /// Required features: "android-content-Context", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri")))]
        pub fn getDocumentUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getDocumentUri", .descriptor == "(Landroid/content/Context;Landroid/net/Uri;)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getDocumentUri\0", "(Landroid/content/Context;Landroid/net/Uri;)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMediaUri](https://developer.android.com/reference/android/provider/MediaStore.html#getMediaUri(android.content.Context,%20android.net.Uri))
        ///
        /// Required features: "android-content-Context", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri")))]
        pub fn getMediaUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/MediaStore", java.flags == PUBLIC | STATIC, .name == "getMediaUri", .descriptor == "(Landroid/content/Context;Landroid/net/Uri;)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/MediaStore\0", "getMediaUri\0", "(Landroid/content/Context;Landroid/net/Uri;)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_IMAGE_CAPTURE](https://developer.android.com/reference/android/provider/MediaStore.html#ACTION_IMAGE_CAPTURE)
        pub const ACTION_IMAGE_CAPTURE : &'static str = "android.media.action.IMAGE_CAPTURE";

        /// public static final [ACTION_IMAGE_CAPTURE_SECURE](https://developer.android.com/reference/android/provider/MediaStore.html#ACTION_IMAGE_CAPTURE_SECURE)
        pub const ACTION_IMAGE_CAPTURE_SECURE : &'static str = "android.media.action.IMAGE_CAPTURE_SECURE";

        /// public static final [ACTION_REVIEW](https://developer.android.com/reference/android/provider/MediaStore.html#ACTION_REVIEW)
        pub const ACTION_REVIEW : &'static str = "android.provider.action.REVIEW";

        /// public static final [ACTION_REVIEW_SECURE](https://developer.android.com/reference/android/provider/MediaStore.html#ACTION_REVIEW_SECURE)
        pub const ACTION_REVIEW_SECURE : &'static str = "android.provider.action.REVIEW_SECURE";

        /// public static final [ACTION_VIDEO_CAPTURE](https://developer.android.com/reference/android/provider/MediaStore.html#ACTION_VIDEO_CAPTURE)
        pub const ACTION_VIDEO_CAPTURE : &'static str = "android.media.action.VIDEO_CAPTURE";

        /// public static final [AUTHORITY](https://developer.android.com/reference/android/provider/MediaStore.html#AUTHORITY)
        pub const AUTHORITY : &'static str = "media";

        /// **get** public static final [AUTHORITY_URI](https://developer.android.com/reference/android/provider/MediaStore.html#AUTHORITY_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn AUTHORITY_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/MediaStore\0", "AUTHORITY_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [EXTRA_BRIGHTNESS](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_BRIGHTNESS)
        pub const EXTRA_BRIGHTNESS : &'static str = "android.provider.extra.BRIGHTNESS";

        /// public static final [EXTRA_DURATION_LIMIT](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_DURATION_LIMIT)
        pub const EXTRA_DURATION_LIMIT : &'static str = "android.intent.extra.durationLimit";

        /// public static final [EXTRA_FINISH_ON_COMPLETION](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_FINISH_ON_COMPLETION)
        pub const EXTRA_FINISH_ON_COMPLETION : &'static str = "android.intent.extra.finishOnCompletion";

        /// public static final [EXTRA_FULL_SCREEN](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_FULL_SCREEN)
        pub const EXTRA_FULL_SCREEN : &'static str = "android.intent.extra.fullScreen";

        /// public static final [EXTRA_MEDIA_ALBUM](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_ALBUM)
        pub const EXTRA_MEDIA_ALBUM : &'static str = "android.intent.extra.album";

        /// public static final [EXTRA_MEDIA_ARTIST](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_ARTIST)
        pub const EXTRA_MEDIA_ARTIST : &'static str = "android.intent.extra.artist";

        /// public static final [EXTRA_MEDIA_FOCUS](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_FOCUS)
        pub const EXTRA_MEDIA_FOCUS : &'static str = "android.intent.extra.focus";

        /// public static final [EXTRA_MEDIA_GENRE](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_GENRE)
        pub const EXTRA_MEDIA_GENRE : &'static str = "android.intent.extra.genre";

        /// public static final [EXTRA_MEDIA_PLAYLIST](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_PLAYLIST)
        pub const EXTRA_MEDIA_PLAYLIST : &'static str = "android.intent.extra.playlist";

        /// public static final [EXTRA_MEDIA_RADIO_CHANNEL](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_RADIO_CHANNEL)
        pub const EXTRA_MEDIA_RADIO_CHANNEL : &'static str = "android.intent.extra.radio_channel";

        /// public static final [EXTRA_MEDIA_TITLE](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_MEDIA_TITLE)
        pub const EXTRA_MEDIA_TITLE : &'static str = "android.intent.extra.title";

        /// public static final [EXTRA_OUTPUT](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_OUTPUT)
        pub const EXTRA_OUTPUT : &'static str = "output";

        /// public static final [EXTRA_SCREEN_ORIENTATION](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_SCREEN_ORIENTATION)
        pub const EXTRA_SCREEN_ORIENTATION : &'static str = "android.intent.extra.screenOrientation";

        /// public static final [EXTRA_SHOW_ACTION_ICONS](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_SHOW_ACTION_ICONS)
        pub const EXTRA_SHOW_ACTION_ICONS : &'static str = "android.intent.extra.showActionIcons";

        /// public static final [EXTRA_SIZE_LIMIT](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_SIZE_LIMIT)
        pub const EXTRA_SIZE_LIMIT : &'static str = "android.intent.extra.sizeLimit";

        /// public static final [EXTRA_VIDEO_QUALITY](https://developer.android.com/reference/android/provider/MediaStore.html#EXTRA_VIDEO_QUALITY)
        pub const EXTRA_VIDEO_QUALITY : &'static str = "android.intent.extra.videoQuality";

        /// public static final [INTENT_ACTION_MEDIA_PLAY_FROM_SEARCH](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_MEDIA_PLAY_FROM_SEARCH)
        pub const INTENT_ACTION_MEDIA_PLAY_FROM_SEARCH : &'static str = "android.media.action.MEDIA_PLAY_FROM_SEARCH";

        /// public static final [INTENT_ACTION_MEDIA_SEARCH](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_MEDIA_SEARCH)
        pub const INTENT_ACTION_MEDIA_SEARCH : &'static str = "android.intent.action.MEDIA_SEARCH";

        /// public static final [INTENT_ACTION_MUSIC_PLAYER](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_MUSIC_PLAYER)
        #[deprecated] pub const INTENT_ACTION_MUSIC_PLAYER : &'static str = "android.intent.action.MUSIC_PLAYER";

        /// public static final [INTENT_ACTION_STILL_IMAGE_CAMERA](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_STILL_IMAGE_CAMERA)
        pub const INTENT_ACTION_STILL_IMAGE_CAMERA : &'static str = "android.media.action.STILL_IMAGE_CAMERA";

        /// public static final [INTENT_ACTION_STILL_IMAGE_CAMERA_SECURE](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_STILL_IMAGE_CAMERA_SECURE)
        pub const INTENT_ACTION_STILL_IMAGE_CAMERA_SECURE : &'static str = "android.media.action.STILL_IMAGE_CAMERA_SECURE";

        /// public static final [INTENT_ACTION_TEXT_OPEN_FROM_SEARCH](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_TEXT_OPEN_FROM_SEARCH)
        pub const INTENT_ACTION_TEXT_OPEN_FROM_SEARCH : &'static str = "android.media.action.TEXT_OPEN_FROM_SEARCH";

        /// public static final [INTENT_ACTION_VIDEO_CAMERA](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_VIDEO_CAMERA)
        pub const INTENT_ACTION_VIDEO_CAMERA : &'static str = "android.media.action.VIDEO_CAMERA";

        /// public static final [INTENT_ACTION_VIDEO_PLAY_FROM_SEARCH](https://developer.android.com/reference/android/provider/MediaStore.html#INTENT_ACTION_VIDEO_PLAY_FROM_SEARCH)
        pub const INTENT_ACTION_VIDEO_PLAY_FROM_SEARCH : &'static str = "android.media.action.VIDEO_PLAY_FROM_SEARCH";

        /// public static final [MEDIA_IGNORE_FILENAME](https://developer.android.com/reference/android/provider/MediaStore.html#MEDIA_IGNORE_FILENAME)
        pub const MEDIA_IGNORE_FILENAME : &'static str = ".nomedia";

        /// public static final [MEDIA_SCANNER_VOLUME](https://developer.android.com/reference/android/provider/MediaStore.html#MEDIA_SCANNER_VOLUME)
        pub const MEDIA_SCANNER_VOLUME : &'static str = "volume";

        /// public static final [META_DATA_STILL_IMAGE_CAMERA_PREWARM_SERVICE](https://developer.android.com/reference/android/provider/MediaStore.html#META_DATA_STILL_IMAGE_CAMERA_PREWARM_SERVICE)
        pub const META_DATA_STILL_IMAGE_CAMERA_PREWARM_SERVICE : &'static str = "android.media.still_image_camera_preview_service";

        /// public static final [UNKNOWN_STRING](https://developer.android.com/reference/android/provider/MediaStore.html#UNKNOWN_STRING)
        pub const UNKNOWN_STRING : &'static str = "<unknown>";

        /// public static final [VOLUME_EXTERNAL](https://developer.android.com/reference/android/provider/MediaStore.html#VOLUME_EXTERNAL)
        pub const VOLUME_EXTERNAL : &'static str = "external";

        /// public static final [VOLUME_EXTERNAL_PRIMARY](https://developer.android.com/reference/android/provider/MediaStore.html#VOLUME_EXTERNAL_PRIMARY)
        pub const VOLUME_EXTERNAL_PRIMARY : &'static str = "external_primary";

        /// public static final [VOLUME_INTERNAL](https://developer.android.com/reference/android/provider/MediaStore.html#VOLUME_INTERNAL)
        pub const VOLUME_INTERNAL : &'static str = "internal";
    }
}
