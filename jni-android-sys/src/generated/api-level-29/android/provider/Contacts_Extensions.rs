// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-Contacts_Extensions"))]
__jni_bindgen! {
    /// public final class [Contacts.Extensions](https://developer.android.com/reference/android/provider/Contacts.Extensions.html)
    ///
    /// Required feature: android-provider-Contacts_Extensions
    #[deprecated] public final class Contacts_Extensions ("android/provider/Contacts$Extensions") extends crate::java::lang::Object, implements crate::android::provider::BaseColumns, crate::android::provider::Contacts_ExtensionsColumns {

        // // Not emitting: Non-public method
        // /// [Extensions](https://developer.android.com/reference/android/provider/Contacts.Extensions.html#Extensions())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::Contacts_Extensions>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/Contacts$Extensions", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/Contacts$Extensions\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [CONTENT_ITEM_TYPE](https://developer.android.com/reference/android/provider/Contacts.Extensions.html#CONTENT_ITEM_TYPE)
        #[deprecated] pub const CONTENT_ITEM_TYPE : &'static str = "vnd.android.cursor.item/contact_extensions";

        /// public static final [CONTENT_TYPE](https://developer.android.com/reference/android/provider/Contacts.Extensions.html#CONTENT_TYPE)
        #[deprecated] pub const CONTENT_TYPE : &'static str = "vnd.android.cursor.dir/contact_extensions";

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/Contacts.Extensions.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        #[deprecated] pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/Contacts$Extensions\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [DEFAULT_SORT_ORDER](https://developer.android.com/reference/android/provider/Contacts.Extensions.html#DEFAULT_SORT_ORDER)
        #[deprecated] pub const DEFAULT_SORT_ORDER : &'static str = "person, name ASC";

        /// public static final [PERSON_ID](https://developer.android.com/reference/android/provider/Contacts.Extensions.html#PERSON_ID)
        #[deprecated] pub const PERSON_ID : &'static str = "person";
    }
}
