// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-CalendarContract_EventsEntity"))]
__jni_bindgen! {
    /// public final class [CalendarContract.EventsEntity](https://developer.android.com/reference/android/provider/CalendarContract.EventsEntity.html)
    ///
    /// Required feature: android-provider-CalendarContract_EventsEntity
    public final class CalendarContract_EventsEntity ("android/provider/CalendarContract$EventsEntity") extends crate::java::lang::Object, implements crate::android::provider::BaseColumns, crate::android::provider::CalendarContract_SyncColumns, crate::android::provider::CalendarContract_EventsColumns {

        // // Not emitting: Non-public method
        // /// [EventsEntity](https://developer.android.com/reference/android/provider/CalendarContract.EventsEntity.html#EventsEntity())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::CalendarContract_EventsEntity>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/CalendarContract$EventsEntity", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/CalendarContract$EventsEntity\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [newEntityIterator](https://developer.android.com/reference/android/provider/CalendarContract.EventsEntity.html#newEntityIterator(android.database.Cursor,%20android.content.ContentResolver))
        ///
        /// Required features: "android-content-ContentResolver", "android-content-EntityIterator", "android-database-Cursor"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentResolver", feature = "android-content-EntityIterator", feature = "android-database-Cursor")))]
        pub fn newEntityIterator_Cursor_ContentResolver<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentResolver>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::EntityIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/CalendarContract$EventsEntity", java.flags == PUBLIC | STATIC, .name == "newEntityIterator", .descriptor == "(Landroid/database/Cursor;Landroid/content/ContentResolver;)Landroid/content/EntityIterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/CalendarContract$EventsEntity\0", "newEntityIterator\0", "(Landroid/database/Cursor;Landroid/content/ContentResolver;)Landroid/content/EntityIterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newEntityIterator](https://developer.android.com/reference/android/provider/CalendarContract.EventsEntity.html#newEntityIterator(android.database.Cursor,%20android.content.ContentProviderClient))
        ///
        /// Required features: "android-content-ContentProviderClient", "android-content-EntityIterator", "android-database-Cursor"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentProviderClient", feature = "android-content-EntityIterator", feature = "android-database-Cursor")))]
        pub fn newEntityIterator_Cursor_ContentProviderClient<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentProviderClient>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::EntityIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/CalendarContract$EventsEntity", java.flags == PUBLIC | STATIC, .name == "newEntityIterator", .descriptor == "(Landroid/database/Cursor;Landroid/content/ContentProviderClient;)Landroid/content/EntityIterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/CalendarContract$EventsEntity\0", "newEntityIterator\0", "(Landroid/database/Cursor;Landroid/content/ContentProviderClient;)Landroid/content/EntityIterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/CalendarContract.EventsEntity.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/CalendarContract$EventsEntity\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
