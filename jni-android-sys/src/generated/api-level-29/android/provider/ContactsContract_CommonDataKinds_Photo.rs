// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_CommonDataKinds_Photo"))]
__jni_bindgen! {
    /// public final class [ContactsContract.CommonDataKinds.Photo](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html)
    ///
    /// Required feature: android-provider-ContactsContract_CommonDataKinds_Photo
    public final class ContactsContract_CommonDataKinds_Photo ("android/provider/ContactsContract$CommonDataKinds$Photo") extends crate::java::lang::Object, implements crate::android::provider::ContactsContract_DataColumnsWithJoins {

        // // Not emitting: Non-public method
        // /// [Photo](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#Photo())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_CommonDataKinds_Photo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/ContactsContract$CommonDataKinds$Photo", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$CommonDataKinds$Photo\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [CONTENT_ITEM_TYPE](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#CONTENT_ITEM_TYPE)
        pub const CONTENT_ITEM_TYPE : &'static str = "vnd.android.cursor.item/photo";

        /// public static final [EXTRA_ADDRESS_BOOK_INDEX](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#EXTRA_ADDRESS_BOOK_INDEX)
        pub const EXTRA_ADDRESS_BOOK_INDEX : &'static str = "android.provider.extra.ADDRESS_BOOK_INDEX";

        /// public static final [EXTRA_ADDRESS_BOOK_INDEX_COUNTS](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#EXTRA_ADDRESS_BOOK_INDEX_COUNTS)
        pub const EXTRA_ADDRESS_BOOK_INDEX_COUNTS : &'static str = "android.provider.extra.ADDRESS_BOOK_INDEX_COUNTS";

        /// public static final [EXTRA_ADDRESS_BOOK_INDEX_TITLES](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#EXTRA_ADDRESS_BOOK_INDEX_TITLES)
        pub const EXTRA_ADDRESS_BOOK_INDEX_TITLES : &'static str = "android.provider.extra.ADDRESS_BOOK_INDEX_TITLES";

        /// public static final [PHOTO](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#PHOTO)
        pub const PHOTO : &'static str = "data15";

        /// public static final [PHOTO_FILE_ID](https://developer.android.com/reference/android/provider/ContactsContract.CommonDataKinds.Photo.html#PHOTO_FILE_ID)
        pub const PHOTO_FILE_ID : &'static str = "data14";
    }
}
