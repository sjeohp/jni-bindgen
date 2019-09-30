// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-BlockedNumberContract_BlockedNumbers"))]
__jni_bindgen! {
    /// public class [BlockedNumberContract.BlockedNumbers](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html)
    ///
    /// Required feature: android-provider-BlockedNumberContract_BlockedNumbers
    public class BlockedNumberContract_BlockedNumbers ("android/provider/BlockedNumberContract$BlockedNumbers") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [BlockedNumbers](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#BlockedNumbers())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::BlockedNumberContract_BlockedNumbers>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/BlockedNumberContract$BlockedNumbers", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/BlockedNumberContract$BlockedNumbers\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [COLUMN_E164_NUMBER](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#COLUMN_E164_NUMBER)
        pub const COLUMN_E164_NUMBER : &'static str = "e164_number";

        /// public static final [COLUMN_ID](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#COLUMN_ID)
        pub const COLUMN_ID : &'static str = "_id";

        /// public static final [COLUMN_ORIGINAL_NUMBER](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#COLUMN_ORIGINAL_NUMBER)
        pub const COLUMN_ORIGINAL_NUMBER : &'static str = "original_number";

        /// public static final [CONTENT_ITEM_TYPE](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#CONTENT_ITEM_TYPE)
        pub const CONTENT_ITEM_TYPE : &'static str = "vnd.android.cursor.item/blocked_number";

        /// public static final [CONTENT_TYPE](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#CONTENT_TYPE)
        pub const CONTENT_TYPE : &'static str = "vnd.android.cursor.dir/blocked_number";

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/BlockedNumberContract.BlockedNumbers.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/BlockedNumberContract$BlockedNumbers\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
