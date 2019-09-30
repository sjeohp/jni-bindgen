// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_CommonDataKinds_Event"))]
__jni_bindgen! {
    /// public final class [ContactsContract.CommonDataKinds.Event](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html)
    ///
    /// Required feature: android-provider-ContactsContract_CommonDataKinds_Event
    public final class ContactsContract_CommonDataKinds_Event ("android/provider/ContactsContract$CommonDataKinds$Event") extends crate::java::lang::Object, implements crate::android::provider::ContactsContract_DataColumnsWithJoins, crate::android::provider::ContactsContract_CommonDataKinds_CommonColumns {

        // // Not emitting: Non-public method
        // /// [Event](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#Event())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_CommonDataKinds_Event>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/ContactsContract$CommonDataKinds$Event", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$CommonDataKinds$Event\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getTypeResource](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#getTypeResource(java.lang.Integer))
        ///
        /// Required features: "java-lang-Integer"
        #[cfg(any(feature = "all", all(feature = "java-lang-Integer")))]
        pub fn getTypeResource<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Integer>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$CommonDataKinds$Event", java.flags == PUBLIC | STATIC, .name == "getTypeResource", .descriptor == "(Ljava/lang/Integer;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$CommonDataKinds$Event\0", "getTypeResource\0", "(Ljava/lang/Integer;)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTypeLabel](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#getTypeLabel(android.content.res.Resources,%20int,%20java.lang.CharSequence))
        ///
        /// Required features: "android-content-res-Resources", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "java-lang-CharSequence")))]
        pub fn getTypeLabel<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$CommonDataKinds$Event", java.flags == PUBLIC | STATIC, .name == "getTypeLabel", .descriptor == "(Landroid/content/res/Resources;ILjava/lang/CharSequence;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$CommonDataKinds$Event\0", "getTypeLabel\0", "(Landroid/content/res/Resources;ILjava/lang/CharSequence;)Ljava/lang/CharSequence;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CONTENT_ITEM_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#CONTENT_ITEM_TYPE)
        pub const CONTENT_ITEM_TYPE : &'static str = "vnd.android.cursor.item/contact_event";

        /// public static final [EXTRA_ADDRESS_BOOK_INDEX](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#EXTRA_ADDRESS_BOOK_INDEX)
        pub const EXTRA_ADDRESS_BOOK_INDEX : &'static str = "android.provider.extra.ADDRESS_BOOK_INDEX";

        /// public static final [EXTRA_ADDRESS_BOOK_INDEX_COUNTS](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#EXTRA_ADDRESS_BOOK_INDEX_COUNTS)
        pub const EXTRA_ADDRESS_BOOK_INDEX_COUNTS : &'static str = "android.provider.extra.ADDRESS_BOOK_INDEX_COUNTS";

        /// public static final [EXTRA_ADDRESS_BOOK_INDEX_TITLES](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#EXTRA_ADDRESS_BOOK_INDEX_TITLES)
        pub const EXTRA_ADDRESS_BOOK_INDEX_TITLES : &'static str = "android.provider.extra.ADDRESS_BOOK_INDEX_TITLES";

        /// public static final [START_DATE](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#START_DATE)
        pub const START_DATE : &'static str = "data1";

        /// public static final [TYPE_ANNIVERSARY](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#TYPE_ANNIVERSARY)
        pub const TYPE_ANNIVERSARY : i32 = 1;

        /// public static final [TYPE_BIRTHDAY](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#TYPE_BIRTHDAY)
        pub const TYPE_BIRTHDAY : i32 = 3;

        /// public static final [TYPE_OTHER](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Event.html#TYPE_OTHER)
        pub const TYPE_OTHER : i32 = 2;
    }
}
