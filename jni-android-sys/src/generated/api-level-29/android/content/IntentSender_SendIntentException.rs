// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-IntentSender_SendIntentException"))]
__jni_bindgen! {
    /// public class [IntentSender.SendIntentException](https://developer.android.com/reference/android/content/IntentSender.SendIntentException.html)
    ///
    /// Required feature: android-content-IntentSender_SendIntentException
    public class IntentSender_SendIntentException ("android/content/IntentSender$SendIntentException") extends crate::android::util::AndroidException {

        /// [SendIntentException](https://developer.android.com/reference/android/content/IntentSender.SendIntentException.html#SendIntentException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::IntentSender_SendIntentException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/IntentSender$SendIntentException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/IntentSender$SendIntentException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SendIntentException](https://developer.android.com/reference/android/content/IntentSender.SendIntentException.html#SendIntentException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::IntentSender_SendIntentException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/IntentSender$SendIntentException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/IntentSender$SendIntentException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SendIntentException](https://developer.android.com/reference/android/content/IntentSender.SendIntentException.html#SendIntentException(java.lang.Exception))
        ///
        /// Required features: "java-lang-Exception"
        #[cfg(any(feature = "all", all(feature = "java-lang-Exception")))]
        pub fn new_Exception<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Exception>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::IntentSender_SendIntentException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/IntentSender$SendIntentException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Exception;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/IntentSender$SendIntentException\0", "<init>\0", "(Ljava/lang/Exception;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
