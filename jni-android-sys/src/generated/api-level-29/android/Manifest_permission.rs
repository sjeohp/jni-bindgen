// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-Manifest_permission"))]
__jni_bindgen! {
    /// public final class [Manifest.permission](https://developer.android.com/reference/android/Manifest.permission.html)
    ///
    /// Required feature: android-Manifest_permission
    public final class Manifest_permission ("android/Manifest$permission") extends crate::java::lang::Object {

        /// [permission](https://developer.android.com/reference/android/Manifest.permission.html#permission())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::Manifest_permission>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/Manifest$permission", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/Manifest$permission\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACCEPT_HANDOVER](https://developer.android.com/reference/android/Manifest.permission.html#ACCEPT_HANDOVER)
        pub const ACCEPT_HANDOVER : &'static str = "android.permission.ACCEPT_HANDOVER";

        /// public static final [ACCESS_BACKGROUND_LOCATION](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_BACKGROUND_LOCATION)
        pub const ACCESS_BACKGROUND_LOCATION : &'static str = "android.permission.ACCESS_BACKGROUND_LOCATION";

        /// public static final [ACCESS_CHECKIN_PROPERTIES](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_CHECKIN_PROPERTIES)
        pub const ACCESS_CHECKIN_PROPERTIES : &'static str = "android.permission.ACCESS_CHECKIN_PROPERTIES";

        /// public static final [ACCESS_COARSE_LOCATION](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_COARSE_LOCATION)
        pub const ACCESS_COARSE_LOCATION : &'static str = "android.permission.ACCESS_COARSE_LOCATION";

        /// public static final [ACCESS_FINE_LOCATION](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_FINE_LOCATION)
        pub const ACCESS_FINE_LOCATION : &'static str = "android.permission.ACCESS_FINE_LOCATION";

        /// public static final [ACCESS_LOCATION_EXTRA_COMMANDS](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_LOCATION_EXTRA_COMMANDS)
        pub const ACCESS_LOCATION_EXTRA_COMMANDS : &'static str = "android.permission.ACCESS_LOCATION_EXTRA_COMMANDS";

        /// public static final [ACCESS_MEDIA_LOCATION](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_MEDIA_LOCATION)
        pub const ACCESS_MEDIA_LOCATION : &'static str = "android.permission.ACCESS_MEDIA_LOCATION";

        /// public static final [ACCESS_NETWORK_STATE](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_NETWORK_STATE)
        pub const ACCESS_NETWORK_STATE : &'static str = "android.permission.ACCESS_NETWORK_STATE";

        /// public static final [ACCESS_NOTIFICATION_POLICY](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_NOTIFICATION_POLICY)
        pub const ACCESS_NOTIFICATION_POLICY : &'static str = "android.permission.ACCESS_NOTIFICATION_POLICY";

        /// public static final [ACCESS_WIFI_STATE](https://developer.android.com/reference/android/Manifest.permission.html#ACCESS_WIFI_STATE)
        pub const ACCESS_WIFI_STATE : &'static str = "android.permission.ACCESS_WIFI_STATE";

        /// public static final [ACCOUNT_MANAGER](https://developer.android.com/reference/android/Manifest.permission.html#ACCOUNT_MANAGER)
        pub const ACCOUNT_MANAGER : &'static str = "android.permission.ACCOUNT_MANAGER";

        /// public static final [ACTIVITY_RECOGNITION](https://developer.android.com/reference/android/Manifest.permission.html#ACTIVITY_RECOGNITION)
        pub const ACTIVITY_RECOGNITION : &'static str = "android.permission.ACTIVITY_RECOGNITION";

        /// public static final [ADD_VOICEMAIL](https://developer.android.com/reference/android/Manifest.permission.html#ADD_VOICEMAIL)
        pub const ADD_VOICEMAIL : &'static str = "com.android.voicemail.permission.ADD_VOICEMAIL";

        /// public static final [ANSWER_PHONE_CALLS](https://developer.android.com/reference/android/Manifest.permission.html#ANSWER_PHONE_CALLS)
        pub const ANSWER_PHONE_CALLS : &'static str = "android.permission.ANSWER_PHONE_CALLS";

        /// public static final [BATTERY_STATS](https://developer.android.com/reference/android/Manifest.permission.html#BATTERY_STATS)
        pub const BATTERY_STATS : &'static str = "android.permission.BATTERY_STATS";

        /// public static final [BIND_ACCESSIBILITY_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_ACCESSIBILITY_SERVICE)
        pub const BIND_ACCESSIBILITY_SERVICE : &'static str = "android.permission.BIND_ACCESSIBILITY_SERVICE";

        /// public static final [BIND_APPWIDGET](https://developer.android.com/reference/android/Manifest.permission.html#BIND_APPWIDGET)
        pub const BIND_APPWIDGET : &'static str = "android.permission.BIND_APPWIDGET";

        /// public static final [BIND_AUTOFILL_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_AUTOFILL_SERVICE)
        pub const BIND_AUTOFILL_SERVICE : &'static str = "android.permission.BIND_AUTOFILL_SERVICE";

        /// public static final [BIND_CALL_REDIRECTION_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_CALL_REDIRECTION_SERVICE)
        pub const BIND_CALL_REDIRECTION_SERVICE : &'static str = "android.permission.BIND_CALL_REDIRECTION_SERVICE";

        /// public static final [BIND_CARRIER_MESSAGING_CLIENT_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_CARRIER_MESSAGING_CLIENT_SERVICE)
        pub const BIND_CARRIER_MESSAGING_CLIENT_SERVICE : &'static str = "android.permission.BIND_CARRIER_MESSAGING_CLIENT_SERVICE";

        /// public static final [BIND_CARRIER_MESSAGING_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_CARRIER_MESSAGING_SERVICE)
        #[deprecated] pub const BIND_CARRIER_MESSAGING_SERVICE : &'static str = "android.permission.BIND_CARRIER_MESSAGING_SERVICE";

        /// public static final [BIND_CARRIER_SERVICES](https://developer.android.com/reference/android/Manifest.permission.html#BIND_CARRIER_SERVICES)
        pub const BIND_CARRIER_SERVICES : &'static str = "android.permission.BIND_CARRIER_SERVICES";

        /// public static final [BIND_CHOOSER_TARGET_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_CHOOSER_TARGET_SERVICE)
        pub const BIND_CHOOSER_TARGET_SERVICE : &'static str = "android.permission.BIND_CHOOSER_TARGET_SERVICE";

        /// public static final [BIND_CONDITION_PROVIDER_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_CONDITION_PROVIDER_SERVICE)
        pub const BIND_CONDITION_PROVIDER_SERVICE : &'static str = "android.permission.BIND_CONDITION_PROVIDER_SERVICE";

        /// public static final [BIND_DEVICE_ADMIN](https://developer.android.com/reference/android/Manifest.permission.html#BIND_DEVICE_ADMIN)
        pub const BIND_DEVICE_ADMIN : &'static str = "android.permission.BIND_DEVICE_ADMIN";

        /// public static final [BIND_DREAM_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_DREAM_SERVICE)
        pub const BIND_DREAM_SERVICE : &'static str = "android.permission.BIND_DREAM_SERVICE";

        /// public static final [BIND_INCALL_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_INCALL_SERVICE)
        pub const BIND_INCALL_SERVICE : &'static str = "android.permission.BIND_INCALL_SERVICE";

        /// public static final [BIND_INPUT_METHOD](https://developer.android.com/reference/android/Manifest.permission.html#BIND_INPUT_METHOD)
        pub const BIND_INPUT_METHOD : &'static str = "android.permission.BIND_INPUT_METHOD";

        /// public static final [BIND_MIDI_DEVICE_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_MIDI_DEVICE_SERVICE)
        pub const BIND_MIDI_DEVICE_SERVICE : &'static str = "android.permission.BIND_MIDI_DEVICE_SERVICE";

        /// public static final [BIND_NFC_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_NFC_SERVICE)
        pub const BIND_NFC_SERVICE : &'static str = "android.permission.BIND_NFC_SERVICE";

        /// public static final [BIND_NOTIFICATION_LISTENER_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_NOTIFICATION_LISTENER_SERVICE)
        pub const BIND_NOTIFICATION_LISTENER_SERVICE : &'static str = "android.permission.BIND_NOTIFICATION_LISTENER_SERVICE";

        /// public static final [BIND_PRINT_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_PRINT_SERVICE)
        pub const BIND_PRINT_SERVICE : &'static str = "android.permission.BIND_PRINT_SERVICE";

        /// public static final [BIND_QUICK_SETTINGS_TILE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_QUICK_SETTINGS_TILE)
        pub const BIND_QUICK_SETTINGS_TILE : &'static str = "android.permission.BIND_QUICK_SETTINGS_TILE";

        /// public static final [BIND_REMOTEVIEWS](https://developer.android.com/reference/android/Manifest.permission.html#BIND_REMOTEVIEWS)
        pub const BIND_REMOTEVIEWS : &'static str = "android.permission.BIND_REMOTEVIEWS";

        /// public static final [BIND_SCREENING_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_SCREENING_SERVICE)
        pub const BIND_SCREENING_SERVICE : &'static str = "android.permission.BIND_SCREENING_SERVICE";

        /// public static final [BIND_TELECOM_CONNECTION_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_TELECOM_CONNECTION_SERVICE)
        pub const BIND_TELECOM_CONNECTION_SERVICE : &'static str = "android.permission.BIND_TELECOM_CONNECTION_SERVICE";

        /// public static final [BIND_TEXT_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_TEXT_SERVICE)
        pub const BIND_TEXT_SERVICE : &'static str = "android.permission.BIND_TEXT_SERVICE";

        /// public static final [BIND_TV_INPUT](https://developer.android.com/reference/android/Manifest.permission.html#BIND_TV_INPUT)
        pub const BIND_TV_INPUT : &'static str = "android.permission.BIND_TV_INPUT";

        /// public static final [BIND_VISUAL_VOICEMAIL_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_VISUAL_VOICEMAIL_SERVICE)
        pub const BIND_VISUAL_VOICEMAIL_SERVICE : &'static str = "android.permission.BIND_VISUAL_VOICEMAIL_SERVICE";

        /// public static final [BIND_VOICE_INTERACTION](https://developer.android.com/reference/android/Manifest.permission.html#BIND_VOICE_INTERACTION)
        pub const BIND_VOICE_INTERACTION : &'static str = "android.permission.BIND_VOICE_INTERACTION";

        /// public static final [BIND_VPN_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_VPN_SERVICE)
        pub const BIND_VPN_SERVICE : &'static str = "android.permission.BIND_VPN_SERVICE";

        /// public static final [BIND_VR_LISTENER_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#BIND_VR_LISTENER_SERVICE)
        pub const BIND_VR_LISTENER_SERVICE : &'static str = "android.permission.BIND_VR_LISTENER_SERVICE";

        /// public static final [BIND_WALLPAPER](https://developer.android.com/reference/android/Manifest.permission.html#BIND_WALLPAPER)
        pub const BIND_WALLPAPER : &'static str = "android.permission.BIND_WALLPAPER";

        /// public static final [BLUETOOTH](https://developer.android.com/reference/android/Manifest.permission.html#BLUETOOTH)
        pub const BLUETOOTH : &'static str = "android.permission.BLUETOOTH";

        /// public static final [BLUETOOTH_ADMIN](https://developer.android.com/reference/android/Manifest.permission.html#BLUETOOTH_ADMIN)
        pub const BLUETOOTH_ADMIN : &'static str = "android.permission.BLUETOOTH_ADMIN";

        /// public static final [BLUETOOTH_PRIVILEGED](https://developer.android.com/reference/android/Manifest.permission.html#BLUETOOTH_PRIVILEGED)
        pub const BLUETOOTH_PRIVILEGED : &'static str = "android.permission.BLUETOOTH_PRIVILEGED";

        /// public static final [BODY_SENSORS](https://developer.android.com/reference/android/Manifest.permission.html#BODY_SENSORS)
        pub const BODY_SENSORS : &'static str = "android.permission.BODY_SENSORS";

        /// public static final [BROADCAST_PACKAGE_REMOVED](https://developer.android.com/reference/android/Manifest.permission.html#BROADCAST_PACKAGE_REMOVED)
        pub const BROADCAST_PACKAGE_REMOVED : &'static str = "android.permission.BROADCAST_PACKAGE_REMOVED";

        /// public static final [BROADCAST_SMS](https://developer.android.com/reference/android/Manifest.permission.html#BROADCAST_SMS)
        pub const BROADCAST_SMS : &'static str = "android.permission.BROADCAST_SMS";

        /// public static final [BROADCAST_STICKY](https://developer.android.com/reference/android/Manifest.permission.html#BROADCAST_STICKY)
        pub const BROADCAST_STICKY : &'static str = "android.permission.BROADCAST_STICKY";

        /// public static final [BROADCAST_WAP_PUSH](https://developer.android.com/reference/android/Manifest.permission.html#BROADCAST_WAP_PUSH)
        pub const BROADCAST_WAP_PUSH : &'static str = "android.permission.BROADCAST_WAP_PUSH";

        /// public static final [CALL_COMPANION_APP](https://developer.android.com/reference/android/Manifest.permission.html#CALL_COMPANION_APP)
        pub const CALL_COMPANION_APP : &'static str = "android.permission.CALL_COMPANION_APP";

        /// public static final [CALL_PHONE](https://developer.android.com/reference/android/Manifest.permission.html#CALL_PHONE)
        pub const CALL_PHONE : &'static str = "android.permission.CALL_PHONE";

        /// public static final [CALL_PRIVILEGED](https://developer.android.com/reference/android/Manifest.permission.html#CALL_PRIVILEGED)
        pub const CALL_PRIVILEGED : &'static str = "android.permission.CALL_PRIVILEGED";

        /// public static final [CAMERA](https://developer.android.com/reference/android/Manifest.permission.html#CAMERA)
        pub const CAMERA : &'static str = "android.permission.CAMERA";

        /// public static final [CAPTURE_AUDIO_OUTPUT](https://developer.android.com/reference/android/Manifest.permission.html#CAPTURE_AUDIO_OUTPUT)
        pub const CAPTURE_AUDIO_OUTPUT : &'static str = "android.permission.CAPTURE_AUDIO_OUTPUT";

        /// public static final [CHANGE_COMPONENT_ENABLED_STATE](https://developer.android.com/reference/android/Manifest.permission.html#CHANGE_COMPONENT_ENABLED_STATE)
        pub const CHANGE_COMPONENT_ENABLED_STATE : &'static str = "android.permission.CHANGE_COMPONENT_ENABLED_STATE";

        /// public static final [CHANGE_CONFIGURATION](https://developer.android.com/reference/android/Manifest.permission.html#CHANGE_CONFIGURATION)
        pub const CHANGE_CONFIGURATION : &'static str = "android.permission.CHANGE_CONFIGURATION";

        /// public static final [CHANGE_NETWORK_STATE](https://developer.android.com/reference/android/Manifest.permission.html#CHANGE_NETWORK_STATE)
        pub const CHANGE_NETWORK_STATE : &'static str = "android.permission.CHANGE_NETWORK_STATE";

        /// public static final [CHANGE_WIFI_MULTICAST_STATE](https://developer.android.com/reference/android/Manifest.permission.html#CHANGE_WIFI_MULTICAST_STATE)
        pub const CHANGE_WIFI_MULTICAST_STATE : &'static str = "android.permission.CHANGE_WIFI_MULTICAST_STATE";

        /// public static final [CHANGE_WIFI_STATE](https://developer.android.com/reference/android/Manifest.permission.html#CHANGE_WIFI_STATE)
        pub const CHANGE_WIFI_STATE : &'static str = "android.permission.CHANGE_WIFI_STATE";

        /// public static final [CLEAR_APP_CACHE](https://developer.android.com/reference/android/Manifest.permission.html#CLEAR_APP_CACHE)
        pub const CLEAR_APP_CACHE : &'static str = "android.permission.CLEAR_APP_CACHE";

        /// public static final [CONTROL_LOCATION_UPDATES](https://developer.android.com/reference/android/Manifest.permission.html#CONTROL_LOCATION_UPDATES)
        pub const CONTROL_LOCATION_UPDATES : &'static str = "android.permission.CONTROL_LOCATION_UPDATES";

        /// public static final [DELETE_CACHE_FILES](https://developer.android.com/reference/android/Manifest.permission.html#DELETE_CACHE_FILES)
        pub const DELETE_CACHE_FILES : &'static str = "android.permission.DELETE_CACHE_FILES";

        /// public static final [DELETE_PACKAGES](https://developer.android.com/reference/android/Manifest.permission.html#DELETE_PACKAGES)
        pub const DELETE_PACKAGES : &'static str = "android.permission.DELETE_PACKAGES";

        /// public static final [DIAGNOSTIC](https://developer.android.com/reference/android/Manifest.permission.html#DIAGNOSTIC)
        pub const DIAGNOSTIC : &'static str = "android.permission.DIAGNOSTIC";

        /// public static final [DISABLE_KEYGUARD](https://developer.android.com/reference/android/Manifest.permission.html#DISABLE_KEYGUARD)
        pub const DISABLE_KEYGUARD : &'static str = "android.permission.DISABLE_KEYGUARD";

        /// public static final [DUMP](https://developer.android.com/reference/android/Manifest.permission.html#DUMP)
        pub const DUMP : &'static str = "android.permission.DUMP";

        /// public static final [EXPAND_STATUS_BAR](https://developer.android.com/reference/android/Manifest.permission.html#EXPAND_STATUS_BAR)
        pub const EXPAND_STATUS_BAR : &'static str = "android.permission.EXPAND_STATUS_BAR";

        /// public static final [FACTORY_TEST](https://developer.android.com/reference/android/Manifest.permission.html#FACTORY_TEST)
        pub const FACTORY_TEST : &'static str = "android.permission.FACTORY_TEST";

        /// public static final [FOREGROUND_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#FOREGROUND_SERVICE)
        pub const FOREGROUND_SERVICE : &'static str = "android.permission.FOREGROUND_SERVICE";

        /// public static final [GET_ACCOUNTS](https://developer.android.com/reference/android/Manifest.permission.html#GET_ACCOUNTS)
        pub const GET_ACCOUNTS : &'static str = "android.permission.GET_ACCOUNTS";

        /// public static final [GET_ACCOUNTS_PRIVILEGED](https://developer.android.com/reference/android/Manifest.permission.html#GET_ACCOUNTS_PRIVILEGED)
        pub const GET_ACCOUNTS_PRIVILEGED : &'static str = "android.permission.GET_ACCOUNTS_PRIVILEGED";

        /// public static final [GET_PACKAGE_SIZE](https://developer.android.com/reference/android/Manifest.permission.html#GET_PACKAGE_SIZE)
        pub const GET_PACKAGE_SIZE : &'static str = "android.permission.GET_PACKAGE_SIZE";

        /// public static final [GET_TASKS](https://developer.android.com/reference/android/Manifest.permission.html#GET_TASKS)
        #[deprecated] pub const GET_TASKS : &'static str = "android.permission.GET_TASKS";

        /// public static final [GLOBAL_SEARCH](https://developer.android.com/reference/android/Manifest.permission.html#GLOBAL_SEARCH)
        pub const GLOBAL_SEARCH : &'static str = "android.permission.GLOBAL_SEARCH";

        /// public static final [INSTALL_LOCATION_PROVIDER](https://developer.android.com/reference/android/Manifest.permission.html#INSTALL_LOCATION_PROVIDER)
        pub const INSTALL_LOCATION_PROVIDER : &'static str = "android.permission.INSTALL_LOCATION_PROVIDER";

        /// public static final [INSTALL_PACKAGES](https://developer.android.com/reference/android/Manifest.permission.html#INSTALL_PACKAGES)
        pub const INSTALL_PACKAGES : &'static str = "android.permission.INSTALL_PACKAGES";

        /// public static final [INSTALL_SHORTCUT](https://developer.android.com/reference/android/Manifest.permission.html#INSTALL_SHORTCUT)
        pub const INSTALL_SHORTCUT : &'static str = "com.android.launcher.permission.INSTALL_SHORTCUT";

        /// public static final [INSTANT_APP_FOREGROUND_SERVICE](https://developer.android.com/reference/android/Manifest.permission.html#INSTANT_APP_FOREGROUND_SERVICE)
        pub const INSTANT_APP_FOREGROUND_SERVICE : &'static str = "android.permission.INSTANT_APP_FOREGROUND_SERVICE";

        /// public static final [INTERNET](https://developer.android.com/reference/android/Manifest.permission.html#INTERNET)
        pub const INTERNET : &'static str = "android.permission.INTERNET";

        /// public static final [KILL_BACKGROUND_PROCESSES](https://developer.android.com/reference/android/Manifest.permission.html#KILL_BACKGROUND_PROCESSES)
        pub const KILL_BACKGROUND_PROCESSES : &'static str = "android.permission.KILL_BACKGROUND_PROCESSES";

        /// public static final [LOCATION_HARDWARE](https://developer.android.com/reference/android/Manifest.permission.html#LOCATION_HARDWARE)
        pub const LOCATION_HARDWARE : &'static str = "android.permission.LOCATION_HARDWARE";

        /// public static final [MANAGE_DOCUMENTS](https://developer.android.com/reference/android/Manifest.permission.html#MANAGE_DOCUMENTS)
        pub const MANAGE_DOCUMENTS : &'static str = "android.permission.MANAGE_DOCUMENTS";

        /// public static final [MANAGE_OWN_CALLS](https://developer.android.com/reference/android/Manifest.permission.html#MANAGE_OWN_CALLS)
        pub const MANAGE_OWN_CALLS : &'static str = "android.permission.MANAGE_OWN_CALLS";

        /// public static final [MASTER_CLEAR](https://developer.android.com/reference/android/Manifest.permission.html#MASTER_CLEAR)
        pub const MASTER_CLEAR : &'static str = "android.permission.MASTER_CLEAR";

        /// public static final [MEDIA_CONTENT_CONTROL](https://developer.android.com/reference/android/Manifest.permission.html#MEDIA_CONTENT_CONTROL)
        pub const MEDIA_CONTENT_CONTROL : &'static str = "android.permission.MEDIA_CONTENT_CONTROL";

        /// public static final [MODIFY_AUDIO_SETTINGS](https://developer.android.com/reference/android/Manifest.permission.html#MODIFY_AUDIO_SETTINGS)
        pub const MODIFY_AUDIO_SETTINGS : &'static str = "android.permission.MODIFY_AUDIO_SETTINGS";

        /// public static final [MODIFY_PHONE_STATE](https://developer.android.com/reference/android/Manifest.permission.html#MODIFY_PHONE_STATE)
        pub const MODIFY_PHONE_STATE : &'static str = "android.permission.MODIFY_PHONE_STATE";

        /// public static final [MOUNT_FORMAT_FILESYSTEMS](https://developer.android.com/reference/android/Manifest.permission.html#MOUNT_FORMAT_FILESYSTEMS)
        pub const MOUNT_FORMAT_FILESYSTEMS : &'static str = "android.permission.MOUNT_FORMAT_FILESYSTEMS";

        /// public static final [MOUNT_UNMOUNT_FILESYSTEMS](https://developer.android.com/reference/android/Manifest.permission.html#MOUNT_UNMOUNT_FILESYSTEMS)
        pub const MOUNT_UNMOUNT_FILESYSTEMS : &'static str = "android.permission.MOUNT_UNMOUNT_FILESYSTEMS";

        /// public static final [NFC](https://developer.android.com/reference/android/Manifest.permission.html#NFC)
        pub const NFC : &'static str = "android.permission.NFC";

        /// public static final [NFC_TRANSACTION_EVENT](https://developer.android.com/reference/android/Manifest.permission.html#NFC_TRANSACTION_EVENT)
        pub const NFC_TRANSACTION_EVENT : &'static str = "android.permission.NFC_TRANSACTION_EVENT";

        /// public static final [PACKAGE_USAGE_STATS](https://developer.android.com/reference/android/Manifest.permission.html#PACKAGE_USAGE_STATS)
        pub const PACKAGE_USAGE_STATS : &'static str = "android.permission.PACKAGE_USAGE_STATS";

        /// public static final [PERSISTENT_ACTIVITY](https://developer.android.com/reference/android/Manifest.permission.html#PERSISTENT_ACTIVITY)
        #[deprecated] pub const PERSISTENT_ACTIVITY : &'static str = "android.permission.PERSISTENT_ACTIVITY";

        /// public static final [PROCESS_OUTGOING_CALLS](https://developer.android.com/reference/android/Manifest.permission.html#PROCESS_OUTGOING_CALLS)
        #[deprecated] pub const PROCESS_OUTGOING_CALLS : &'static str = "android.permission.PROCESS_OUTGOING_CALLS";

        /// public static final [READ_CALENDAR](https://developer.android.com/reference/android/Manifest.permission.html#READ_CALENDAR)
        pub const READ_CALENDAR : &'static str = "android.permission.READ_CALENDAR";

        /// public static final [READ_CALL_LOG](https://developer.android.com/reference/android/Manifest.permission.html#READ_CALL_LOG)
        pub const READ_CALL_LOG : &'static str = "android.permission.READ_CALL_LOG";

        /// public static final [READ_CONTACTS](https://developer.android.com/reference/android/Manifest.permission.html#READ_CONTACTS)
        pub const READ_CONTACTS : &'static str = "android.permission.READ_CONTACTS";

        /// public static final [READ_EXTERNAL_STORAGE](https://developer.android.com/reference/android/Manifest.permission.html#READ_EXTERNAL_STORAGE)
        pub const READ_EXTERNAL_STORAGE : &'static str = "android.permission.READ_EXTERNAL_STORAGE";

        /// public static final [READ_INPUT_STATE](https://developer.android.com/reference/android/Manifest.permission.html#READ_INPUT_STATE)
        #[deprecated] pub const READ_INPUT_STATE : &'static str = "android.permission.READ_INPUT_STATE";

        /// public static final [READ_LOGS](https://developer.android.com/reference/android/Manifest.permission.html#READ_LOGS)
        pub const READ_LOGS : &'static str = "android.permission.READ_LOGS";

        /// public static final [READ_PHONE_NUMBERS](https://developer.android.com/reference/android/Manifest.permission.html#READ_PHONE_NUMBERS)
        pub const READ_PHONE_NUMBERS : &'static str = "android.permission.READ_PHONE_NUMBERS";

        /// public static final [READ_PHONE_STATE](https://developer.android.com/reference/android/Manifest.permission.html#READ_PHONE_STATE)
        pub const READ_PHONE_STATE : &'static str = "android.permission.READ_PHONE_STATE";

        /// public static final [READ_SMS](https://developer.android.com/reference/android/Manifest.permission.html#READ_SMS)
        pub const READ_SMS : &'static str = "android.permission.READ_SMS";

        /// public static final [READ_SYNC_SETTINGS](https://developer.android.com/reference/android/Manifest.permission.html#READ_SYNC_SETTINGS)
        pub const READ_SYNC_SETTINGS : &'static str = "android.permission.READ_SYNC_SETTINGS";

        /// public static final [READ_SYNC_STATS](https://developer.android.com/reference/android/Manifest.permission.html#READ_SYNC_STATS)
        pub const READ_SYNC_STATS : &'static str = "android.permission.READ_SYNC_STATS";

        /// public static final [READ_VOICEMAIL](https://developer.android.com/reference/android/Manifest.permission.html#READ_VOICEMAIL)
        pub const READ_VOICEMAIL : &'static str = "com.android.voicemail.permission.READ_VOICEMAIL";

        /// public static final [REBOOT](https://developer.android.com/reference/android/Manifest.permission.html#REBOOT)
        pub const REBOOT : &'static str = "android.permission.REBOOT";

        /// public static final [RECEIVE_BOOT_COMPLETED](https://developer.android.com/reference/android/Manifest.permission.html#RECEIVE_BOOT_COMPLETED)
        pub const RECEIVE_BOOT_COMPLETED : &'static str = "android.permission.RECEIVE_BOOT_COMPLETED";

        /// public static final [RECEIVE_MMS](https://developer.android.com/reference/android/Manifest.permission.html#RECEIVE_MMS)
        pub const RECEIVE_MMS : &'static str = "android.permission.RECEIVE_MMS";

        /// public static final [RECEIVE_SMS](https://developer.android.com/reference/android/Manifest.permission.html#RECEIVE_SMS)
        pub const RECEIVE_SMS : &'static str = "android.permission.RECEIVE_SMS";

        /// public static final [RECEIVE_WAP_PUSH](https://developer.android.com/reference/android/Manifest.permission.html#RECEIVE_WAP_PUSH)
        pub const RECEIVE_WAP_PUSH : &'static str = "android.permission.RECEIVE_WAP_PUSH";

        /// public static final [RECORD_AUDIO](https://developer.android.com/reference/android/Manifest.permission.html#RECORD_AUDIO)
        pub const RECORD_AUDIO : &'static str = "android.permission.RECORD_AUDIO";

        /// public static final [REORDER_TASKS](https://developer.android.com/reference/android/Manifest.permission.html#REORDER_TASKS)
        pub const REORDER_TASKS : &'static str = "android.permission.REORDER_TASKS";

        /// public static final [REQUEST_COMPANION_RUN_IN_BACKGROUND](https://developer.android.com/reference/android/Manifest.permission.html#REQUEST_COMPANION_RUN_IN_BACKGROUND)
        pub const REQUEST_COMPANION_RUN_IN_BACKGROUND : &'static str = "android.permission.REQUEST_COMPANION_RUN_IN_BACKGROUND";

        /// public static final [REQUEST_COMPANION_USE_DATA_IN_BACKGROUND](https://developer.android.com/reference/android/Manifest.permission.html#REQUEST_COMPANION_USE_DATA_IN_BACKGROUND)
        pub const REQUEST_COMPANION_USE_DATA_IN_BACKGROUND : &'static str = "android.permission.REQUEST_COMPANION_USE_DATA_IN_BACKGROUND";

        /// public static final [REQUEST_DELETE_PACKAGES](https://developer.android.com/reference/android/Manifest.permission.html#REQUEST_DELETE_PACKAGES)
        pub const REQUEST_DELETE_PACKAGES : &'static str = "android.permission.REQUEST_DELETE_PACKAGES";

        /// public static final [REQUEST_IGNORE_BATTERY_OPTIMIZATIONS](https://developer.android.com/reference/android/Manifest.permission.html#REQUEST_IGNORE_BATTERY_OPTIMIZATIONS)
        pub const REQUEST_IGNORE_BATTERY_OPTIMIZATIONS : &'static str = "android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS";

        /// public static final [REQUEST_INSTALL_PACKAGES](https://developer.android.com/reference/android/Manifest.permission.html#REQUEST_INSTALL_PACKAGES)
        pub const REQUEST_INSTALL_PACKAGES : &'static str = "android.permission.REQUEST_INSTALL_PACKAGES";

        /// public static final [REQUEST_PASSWORD_COMPLEXITY](https://developer.android.com/reference/android/Manifest.permission.html#REQUEST_PASSWORD_COMPLEXITY)
        pub const REQUEST_PASSWORD_COMPLEXITY : &'static str = "android.permission.REQUEST_PASSWORD_COMPLEXITY";

        /// public static final [RESTART_PACKAGES](https://developer.android.com/reference/android/Manifest.permission.html#RESTART_PACKAGES)
        #[deprecated] pub const RESTART_PACKAGES : &'static str = "android.permission.RESTART_PACKAGES";

        /// public static final [SEND_RESPOND_VIA_MESSAGE](https://developer.android.com/reference/android/Manifest.permission.html#SEND_RESPOND_VIA_MESSAGE)
        pub const SEND_RESPOND_VIA_MESSAGE : &'static str = "android.permission.SEND_RESPOND_VIA_MESSAGE";

        /// public static final [SEND_SMS](https://developer.android.com/reference/android/Manifest.permission.html#SEND_SMS)
        pub const SEND_SMS : &'static str = "android.permission.SEND_SMS";

        /// public static final [SET_ALARM](https://developer.android.com/reference/android/Manifest.permission.html#SET_ALARM)
        pub const SET_ALARM : &'static str = "com.android.alarm.permission.SET_ALARM";

        /// public static final [SET_ALWAYS_FINISH](https://developer.android.com/reference/android/Manifest.permission.html#SET_ALWAYS_FINISH)
        pub const SET_ALWAYS_FINISH : &'static str = "android.permission.SET_ALWAYS_FINISH";

        /// public static final [SET_ANIMATION_SCALE](https://developer.android.com/reference/android/Manifest.permission.html#SET_ANIMATION_SCALE)
        pub const SET_ANIMATION_SCALE : &'static str = "android.permission.SET_ANIMATION_SCALE";

        /// public static final [SET_DEBUG_APP](https://developer.android.com/reference/android/Manifest.permission.html#SET_DEBUG_APP)
        pub const SET_DEBUG_APP : &'static str = "android.permission.SET_DEBUG_APP";

        /// public static final [SET_PREFERRED_APPLICATIONS](https://developer.android.com/reference/android/Manifest.permission.html#SET_PREFERRED_APPLICATIONS)
        #[deprecated] pub const SET_PREFERRED_APPLICATIONS : &'static str = "android.permission.SET_PREFERRED_APPLICATIONS";

        /// public static final [SET_PROCESS_LIMIT](https://developer.android.com/reference/android/Manifest.permission.html#SET_PROCESS_LIMIT)
        pub const SET_PROCESS_LIMIT : &'static str = "android.permission.SET_PROCESS_LIMIT";

        /// public static final [SET_TIME](https://developer.android.com/reference/android/Manifest.permission.html#SET_TIME)
        pub const SET_TIME : &'static str = "android.permission.SET_TIME";

        /// public static final [SET_TIME_ZONE](https://developer.android.com/reference/android/Manifest.permission.html#SET_TIME_ZONE)
        pub const SET_TIME_ZONE : &'static str = "android.permission.SET_TIME_ZONE";

        /// public static final [SET_WALLPAPER](https://developer.android.com/reference/android/Manifest.permission.html#SET_WALLPAPER)
        pub const SET_WALLPAPER : &'static str = "android.permission.SET_WALLPAPER";

        /// public static final [SET_WALLPAPER_HINTS](https://developer.android.com/reference/android/Manifest.permission.html#SET_WALLPAPER_HINTS)
        pub const SET_WALLPAPER_HINTS : &'static str = "android.permission.SET_WALLPAPER_HINTS";

        /// public static final [SIGNAL_PERSISTENT_PROCESSES](https://developer.android.com/reference/android/Manifest.permission.html#SIGNAL_PERSISTENT_PROCESSES)
        pub const SIGNAL_PERSISTENT_PROCESSES : &'static str = "android.permission.SIGNAL_PERSISTENT_PROCESSES";

        /// public static final [SMS_FINANCIAL_TRANSACTIONS](https://developer.android.com/reference/android/Manifest.permission.html#SMS_FINANCIAL_TRANSACTIONS)
        pub const SMS_FINANCIAL_TRANSACTIONS : &'static str = "android.permission.SMS_FINANCIAL_TRANSACTIONS";

        /// public static final [START_VIEW_PERMISSION_USAGE](https://developer.android.com/reference/android/Manifest.permission.html#START_VIEW_PERMISSION_USAGE)
        pub const START_VIEW_PERMISSION_USAGE : &'static str = "android.permission.START_VIEW_PERMISSION_USAGE";

        /// public static final [STATUS_BAR](https://developer.android.com/reference/android/Manifest.permission.html#STATUS_BAR)
        pub const STATUS_BAR : &'static str = "android.permission.STATUS_BAR";

        /// public static final [SYSTEM_ALERT_WINDOW](https://developer.android.com/reference/android/Manifest.permission.html#SYSTEM_ALERT_WINDOW)
        pub const SYSTEM_ALERT_WINDOW : &'static str = "android.permission.SYSTEM_ALERT_WINDOW";

        /// public static final [TRANSMIT_IR](https://developer.android.com/reference/android/Manifest.permission.html#TRANSMIT_IR)
        pub const TRANSMIT_IR : &'static str = "android.permission.TRANSMIT_IR";

        /// public static final [UNINSTALL_SHORTCUT](https://developer.android.com/reference/android/Manifest.permission.html#UNINSTALL_SHORTCUT)
        pub const UNINSTALL_SHORTCUT : &'static str = "com.android.launcher.permission.UNINSTALL_SHORTCUT";

        /// public static final [UPDATE_DEVICE_STATS](https://developer.android.com/reference/android/Manifest.permission.html#UPDATE_DEVICE_STATS)
        pub const UPDATE_DEVICE_STATS : &'static str = "android.permission.UPDATE_DEVICE_STATS";

        /// public static final [USE_BIOMETRIC](https://developer.android.com/reference/android/Manifest.permission.html#USE_BIOMETRIC)
        pub const USE_BIOMETRIC : &'static str = "android.permission.USE_BIOMETRIC";

        /// public static final [USE_FINGERPRINT](https://developer.android.com/reference/android/Manifest.permission.html#USE_FINGERPRINT)
        #[deprecated] pub const USE_FINGERPRINT : &'static str = "android.permission.USE_FINGERPRINT";

        /// public static final [USE_FULL_SCREEN_INTENT](https://developer.android.com/reference/android/Manifest.permission.html#USE_FULL_SCREEN_INTENT)
        pub const USE_FULL_SCREEN_INTENT : &'static str = "android.permission.USE_FULL_SCREEN_INTENT";

        /// public static final [USE_SIP](https://developer.android.com/reference/android/Manifest.permission.html#USE_SIP)
        pub const USE_SIP : &'static str = "android.permission.USE_SIP";

        /// public static final [VIBRATE](https://developer.android.com/reference/android/Manifest.permission.html#VIBRATE)
        pub const VIBRATE : &'static str = "android.permission.VIBRATE";

        /// public static final [WAKE_LOCK](https://developer.android.com/reference/android/Manifest.permission.html#WAKE_LOCK)
        pub const WAKE_LOCK : &'static str = "android.permission.WAKE_LOCK";

        /// public static final [WRITE_APN_SETTINGS](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_APN_SETTINGS)
        pub const WRITE_APN_SETTINGS : &'static str = "android.permission.WRITE_APN_SETTINGS";

        /// public static final [WRITE_CALENDAR](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_CALENDAR)
        pub const WRITE_CALENDAR : &'static str = "android.permission.WRITE_CALENDAR";

        /// public static final [WRITE_CALL_LOG](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_CALL_LOG)
        pub const WRITE_CALL_LOG : &'static str = "android.permission.WRITE_CALL_LOG";

        /// public static final [WRITE_CONTACTS](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_CONTACTS)
        pub const WRITE_CONTACTS : &'static str = "android.permission.WRITE_CONTACTS";

        /// public static final [WRITE_EXTERNAL_STORAGE](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_EXTERNAL_STORAGE)
        pub const WRITE_EXTERNAL_STORAGE : &'static str = "android.permission.WRITE_EXTERNAL_STORAGE";

        /// public static final [WRITE_GSERVICES](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_GSERVICES)
        pub const WRITE_GSERVICES : &'static str = "android.permission.WRITE_GSERVICES";

        /// public static final [WRITE_SECURE_SETTINGS](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_SECURE_SETTINGS)
        pub const WRITE_SECURE_SETTINGS : &'static str = "android.permission.WRITE_SECURE_SETTINGS";

        /// public static final [WRITE_SETTINGS](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_SETTINGS)
        pub const WRITE_SETTINGS : &'static str = "android.permission.WRITE_SETTINGS";

        /// public static final [WRITE_SYNC_SETTINGS](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_SYNC_SETTINGS)
        pub const WRITE_SYNC_SETTINGS : &'static str = "android.permission.WRITE_SYNC_SETTINGS";

        /// public static final [WRITE_VOICEMAIL](https://developer.android.com/reference/android/Manifest.permission.html#WRITE_VOICEMAIL)
        pub const WRITE_VOICEMAIL : &'static str = "com.android.voicemail.permission.WRITE_VOICEMAIL";
    }
}
