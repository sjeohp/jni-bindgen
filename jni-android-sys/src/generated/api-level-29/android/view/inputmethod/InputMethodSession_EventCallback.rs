// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-inputmethod-InputMethodSession_EventCallback"))]
__jni_bindgen! {
    /// public interface [InputMethodSession.EventCallback](https://developer.android.com/reference/android/view/inputmethod/InputMethodSession.EventCallback.html)
    ///
    /// Required feature: android-view-inputmethod-InputMethodSession_EventCallback
    public interface InputMethodSession_EventCallback ("android/view/inputmethod/InputMethodSession$EventCallback") extends crate::java::lang::Object {

        /// [finishedEvent](https://developer.android.com/reference/android/view/inputmethod/InputMethodSession.EventCallback.html#finishedEvent(int,%20boolean))
        pub fn finishedEvent<'env>(&'env self, arg0: i32, arg1: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputMethodSession$EventCallback", java.flags == PUBLIC | ABSTRACT, .name == "finishedEvent", .descriptor == "(IZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputMethodSession$EventCallback\0", "finishedEvent\0", "(IZ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
