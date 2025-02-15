// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_Groups"))]
__jni_bindgen! {
    /// public final class [ContactsContract.Groups](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html)
    ///
    /// Required feature: android-provider-ContactsContract_Groups
    public final class ContactsContract_Groups ("android/provider/ContactsContract$Groups") extends crate::java::lang::Object, implements crate::android::provider::BaseColumns, crate::android::provider::ContactsContract_GroupsColumns, crate::android::provider::ContactsContract_SyncColumns {

        // // Not emitting: Non-public method
        // /// [Groups](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html#Groups())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_Groups>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/ContactsContract$Groups", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Groups\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [newEntityIterator](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html#newEntityIterator(android.database.Cursor))
        ///
        /// Required features: "android-content-EntityIterator", "android-database-Cursor"
        #[cfg(any(feature = "all", all(feature = "android-content-EntityIterator", feature = "android-database-Cursor")))]
        pub fn newEntityIterator<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::EntityIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Groups", java.flags == PUBLIC | STATIC, .name == "newEntityIterator", .descriptor == "(Landroid/database/Cursor;)Landroid/content/EntityIterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$Groups\0", "newEntityIterator\0", "(Landroid/database/Cursor;)Landroid/content/EntityIterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CONTENT_ITEM_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html#CONTENT_ITEM_TYPE)
        pub const CONTENT_ITEM_TYPE : &'static str = "vnd.android.cursor.item/group";

        /// **get** public static final [CONTENT_SUMMARY_URI](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html#CONTENT_SUMMARY_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_SUMMARY_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/ContactsContract$Groups\0", "CONTENT_SUMMARY_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [CONTENT_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html#CONTENT_TYPE)
        pub const CONTENT_TYPE : &'static str = "vnd.android.cursor.dir/group";

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/ContactsContract.Groups.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/ContactsContract$Groups\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
