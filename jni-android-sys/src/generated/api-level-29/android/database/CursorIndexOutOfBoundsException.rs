// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-CursorIndexOutOfBoundsException"))]
__jni_bindgen! {
    /// public class [CursorIndexOutOfBoundsException](https://developer.android.com/reference/android/database/CursorIndexOutOfBoundsException.html)
    ///
    /// Required feature: android-database-CursorIndexOutOfBoundsException
    public class CursorIndexOutOfBoundsException ("android/database/CursorIndexOutOfBoundsException") extends crate::java::lang::IndexOutOfBoundsException {

        /// [CursorIndexOutOfBoundsException](https://developer.android.com/reference/android/database/CursorIndexOutOfBoundsException.html#CursorIndexOutOfBoundsException(int,%20int))
        pub fn new_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::CursorIndexOutOfBoundsException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorIndexOutOfBoundsException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorIndexOutOfBoundsException\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CursorIndexOutOfBoundsException](https://developer.android.com/reference/android/database/CursorIndexOutOfBoundsException.html#CursorIndexOutOfBoundsException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::CursorIndexOutOfBoundsException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorIndexOutOfBoundsException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorIndexOutOfBoundsException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
