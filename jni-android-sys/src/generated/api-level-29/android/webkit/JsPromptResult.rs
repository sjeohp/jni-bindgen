// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-JsPromptResult"))]
__jni_bindgen! {
    /// public class [JsPromptResult](https://developer.android.com/reference/android/webkit/JsPromptResult.html)
    ///
    /// Required feature: android-webkit-JsPromptResult
    public class JsPromptResult ("android/webkit/JsPromptResult") extends crate::android::webkit::JsResult {

        // // Not emitting: Non-public method
        // /// [JsPromptResult](https://developer.android.com/reference/android/webkit/JsPromptResult.html#JsPromptResult())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::JsPromptResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/webkit/JsPromptResult", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/JsPromptResult\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [confirm](https://developer.android.com/reference/android/webkit/JsPromptResult.html#confirm(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn confirm<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/JsPromptResult", java.flags == PUBLIC, .name == "confirm", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/JsPromptResult\0", "confirm\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
