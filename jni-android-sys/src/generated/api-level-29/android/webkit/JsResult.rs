// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-JsResult"))]
__jni_bindgen! {
    /// public class [JsResult](https://developer.android.com/reference/android/webkit/JsResult.html)
    ///
    /// Required feature: android-webkit-JsResult
    public class JsResult ("android/webkit/JsResult") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [JsResult](https://developer.android.com/reference/android/webkit/JsResult.html#JsResult())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::JsResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/webkit/JsResult", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/JsResult\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [cancel](https://developer.android.com/reference/android/webkit/JsResult.html#cancel())
        pub fn cancel<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/JsResult", java.flags == PUBLIC | FINAL, .name == "cancel", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/JsResult\0", "cancel\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [confirm](https://developer.android.com/reference/android/webkit/JsResult.html#confirm())
        pub fn confirm<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/JsResult", java.flags == PUBLIC | FINAL, .name == "confirm", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/JsResult\0", "confirm\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
