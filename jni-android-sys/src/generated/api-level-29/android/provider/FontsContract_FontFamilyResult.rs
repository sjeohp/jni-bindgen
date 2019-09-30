// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-FontsContract_FontFamilyResult"))]
__jni_bindgen! {
    /// public class [FontsContract.FontFamilyResult](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html)
    ///
    /// Required feature: android-provider-FontsContract_FontFamilyResult
    public class FontsContract_FontFamilyResult ("android/provider/FontsContract$FontFamilyResult") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [FontFamilyResult](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#FontFamilyResult(int,%20android.provider.FontsContract.FontInfo%5B%5D))
        // ///
        // /// Required features: "android-provider-FontsContract_FontInfo"
        // #[cfg(any(feature = "all", all(feature = "android-provider-FontsContract_FontInfo")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::provider::FontsContract_FontInfo, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::FontsContract_FontFamilyResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/FontsContract$FontFamilyResult", java.flags == (empty), .name == "<init>", .descriptor == "(I[Landroid/provider/FontsContract$FontInfo;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/FontsContract$FontFamilyResult\0", "<init>\0", "(I[Landroid/provider/FontsContract$FontInfo;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getStatusCode](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#getStatusCode())
        pub fn getStatusCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/FontsContract$FontFamilyResult", java.flags == PUBLIC, .name == "getStatusCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/FontsContract$FontFamilyResult\0", "getStatusCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFonts](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#getFonts())
        ///
        /// Required features: "android-provider-FontsContract_FontInfo"
        #[cfg(any(feature = "all", all(feature = "android-provider-FontsContract_FontInfo")))]
        pub fn getFonts<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::provider::FontsContract_FontInfo, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/FontsContract$FontFamilyResult", java.flags == PUBLIC, .name == "getFonts", .descriptor == "()[Landroid/provider/FontsContract$FontInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/FontsContract$FontFamilyResult\0", "getFonts\0", "()[Landroid/provider/FontsContract$FontInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [STATUS_OK](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#STATUS_OK)
        pub const STATUS_OK : i32 = 0;

        /// public static final [STATUS_REJECTED](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#STATUS_REJECTED)
        pub const STATUS_REJECTED : i32 = 3;

        /// public static final [STATUS_UNEXPECTED_DATA_PROVIDED](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#STATUS_UNEXPECTED_DATA_PROVIDED)
        pub const STATUS_UNEXPECTED_DATA_PROVIDED : i32 = 2;

        /// public static final [STATUS_WRONG_CERTIFICATES](https://developer.android.com/reference/android/provider/FontsContract.FontFamilyResult.html#STATUS_WRONG_CERTIFICATES)
        pub const STATUS_WRONG_CERTIFICATES : i32 = 1;
    }
}
