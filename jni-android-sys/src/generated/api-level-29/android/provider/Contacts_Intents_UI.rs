// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-Contacts_Intents_UI"))]
__jni_bindgen! {
    /// public final class [Contacts.Intents.UI](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html)
    ///
    /// Required feature: android-provider-Contacts_Intents_UI
    #[deprecated] public final class Contacts_Intents_UI ("android/provider/Contacts$Intents$UI") extends crate::java::lang::Object {

        /// [UI](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#UI())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::Contacts_Intents_UI>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/Contacts$Intents$UI", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/Contacts$Intents$UI\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [FILTER_CONTACTS_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#FILTER_CONTACTS_ACTION)
        #[deprecated] pub const FILTER_CONTACTS_ACTION : &'static str = "com.android.contacts.action.FILTER_CONTACTS";

        /// public static final [FILTER_TEXT_EXTRA_KEY](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#FILTER_TEXT_EXTRA_KEY)
        #[deprecated] pub const FILTER_TEXT_EXTRA_KEY : &'static str = "com.android.contacts.extra.FILTER_TEXT";

        /// public static final [GROUP_NAME_EXTRA_KEY](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#GROUP_NAME_EXTRA_KEY)
        #[deprecated] pub const GROUP_NAME_EXTRA_KEY : &'static str = "com.android.contacts.extra.GROUP";

        /// public static final [LIST_ALL_CONTACTS_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_ALL_CONTACTS_ACTION)
        #[deprecated] pub const LIST_ALL_CONTACTS_ACTION : &'static str = "com.android.contacts.action.LIST_ALL_CONTACTS";

        /// public static final [LIST_CONTACTS_WITH_PHONES_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_CONTACTS_WITH_PHONES_ACTION)
        #[deprecated] pub const LIST_CONTACTS_WITH_PHONES_ACTION : &'static str = "com.android.contacts.action.LIST_CONTACTS_WITH_PHONES";

        /// public static final [LIST_DEFAULT](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_DEFAULT)
        #[deprecated] pub const LIST_DEFAULT : &'static str = "com.android.contacts.action.LIST_DEFAULT";

        /// public static final [LIST_FREQUENT_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_FREQUENT_ACTION)
        #[deprecated] pub const LIST_FREQUENT_ACTION : &'static str = "com.android.contacts.action.LIST_FREQUENT";

        /// public static final [LIST_GROUP_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_GROUP_ACTION)
        #[deprecated] pub const LIST_GROUP_ACTION : &'static str = "com.android.contacts.action.LIST_GROUP";

        /// public static final [LIST_STARRED_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_STARRED_ACTION)
        #[deprecated] pub const LIST_STARRED_ACTION : &'static str = "com.android.contacts.action.LIST_STARRED";

        /// public static final [LIST_STREQUENT_ACTION](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#LIST_STREQUENT_ACTION)
        #[deprecated] pub const LIST_STREQUENT_ACTION : &'static str = "com.android.contacts.action.LIST_STREQUENT";

        /// public static final [TITLE_EXTRA_KEY](https://developer.android.com/reference/android/provider/Contacts.Intents.UI.html#TITLE_EXTRA_KEY)
        #[deprecated] pub const TITLE_EXTRA_KEY : &'static str = "com.android.contacts.extra.TITLE_EXTRA";
    }
}
