// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-strictmode-IntentReceiverLeakedViolation"))]
__jni_bindgen! {
    /// public final class [IntentReceiverLeakedViolation](https://developer.android.com/reference/android/os/strictmode/IntentReceiverLeakedViolation.html)
    ///
    /// Required feature: android-os-strictmode-IntentReceiverLeakedViolation
    public final class IntentReceiverLeakedViolation ("android/os/strictmode/IntentReceiverLeakedViolation") extends crate::android::os::strictmode::Violation {

        // // Not emitting: Non-public method
        // /// [IntentReceiverLeakedViolation](https://developer.android.com/reference/android/os/strictmode/IntentReceiverLeakedViolation.html#IntentReceiverLeakedViolation(java.lang.Throwable))
        // ///
        // /// Required features: "java-lang-Throwable"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Throwable")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::strictmode::IntentReceiverLeakedViolation>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/strictmode/IntentReceiverLeakedViolation", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/lang/Throwable;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/strictmode/IntentReceiverLeakedViolation\0", "<init>\0", "(Ljava/lang/Throwable;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
