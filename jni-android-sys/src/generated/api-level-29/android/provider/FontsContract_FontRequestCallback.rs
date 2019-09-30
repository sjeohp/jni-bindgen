// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-FontsContract_FontRequestCallback"))]
__jni_bindgen! {
    /// public class [FontsContract.FontRequestCallback](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html)
    ///
    /// Required feature: android-provider-FontsContract_FontRequestCallback
    public class FontsContract_FontRequestCallback ("android/provider/FontsContract$FontRequestCallback") extends crate::java::lang::Object {

        /// [FontRequestCallback](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FontRequestCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::FontsContract_FontRequestCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/FontsContract$FontRequestCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/FontsContract$FontRequestCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTypefaceRetrieved](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#onTypefaceRetrieved(android.graphics.Typeface))
        ///
        /// Required features: "android-graphics-Typeface"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface")))]
        pub fn onTypefaceRetrieved<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Typeface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/FontsContract$FontRequestCallback", java.flags == PUBLIC, .name == "onTypefaceRetrieved", .descriptor == "(Landroid/graphics/Typeface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/FontsContract$FontRequestCallback\0", "onTypefaceRetrieved\0", "(Landroid/graphics/Typeface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTypefaceRequestFailed](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#onTypefaceRequestFailed(int))
        pub fn onTypefaceRequestFailed<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/FontsContract$FontRequestCallback", java.flags == PUBLIC, .name == "onTypefaceRequestFailed", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/FontsContract$FontRequestCallback\0", "onTypefaceRequestFailed\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [FAIL_REASON_FONT_LOAD_ERROR](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FAIL_REASON_FONT_LOAD_ERROR)
        pub const FAIL_REASON_FONT_LOAD_ERROR : i32 = -3;

        /// public static final [FAIL_REASON_FONT_NOT_FOUND](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FAIL_REASON_FONT_NOT_FOUND)
        pub const FAIL_REASON_FONT_NOT_FOUND : i32 = 1;

        /// public static final [FAIL_REASON_FONT_UNAVAILABLE](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FAIL_REASON_FONT_UNAVAILABLE)
        pub const FAIL_REASON_FONT_UNAVAILABLE : i32 = 2;

        /// public static final [FAIL_REASON_MALFORMED_QUERY](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FAIL_REASON_MALFORMED_QUERY)
        pub const FAIL_REASON_MALFORMED_QUERY : i32 = 3;

        /// public static final [FAIL_REASON_PROVIDER_NOT_FOUND](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FAIL_REASON_PROVIDER_NOT_FOUND)
        pub const FAIL_REASON_PROVIDER_NOT_FOUND : i32 = -1;

        /// public static final [FAIL_REASON_WRONG_CERTIFICATES](https://developer.android.com/reference/android/provider/FontsContract.FontRequestCallback.html#FAIL_REASON_WRONG_CERTIFICATES)
        pub const FAIL_REASON_WRONG_CERTIFICATES : i32 = -2;
    }
}
