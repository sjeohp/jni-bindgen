// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-IpSecManager_ResourceUnavailableException"))]
__jni_bindgen! {
    /// public final class [IpSecManager.ResourceUnavailableException](https://developer.android.com/reference/android/net/IpSecManager.ResourceUnavailableException.html)
    ///
    /// Required feature: android-net-IpSecManager_ResourceUnavailableException
    public final class IpSecManager_ResourceUnavailableException ("android/net/IpSecManager$ResourceUnavailableException") extends crate::android::util::AndroidException {

        // // Not emitting: Non-public method
        // /// [ResourceUnavailableException](https://developer.android.com/reference/android/net/IpSecManager.ResourceUnavailableException.html#ResourceUnavailableException(java.lang.String))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::IpSecManager_ResourceUnavailableException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/IpSecManager$ResourceUnavailableException", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/IpSecManager$ResourceUnavailableException\0", "<init>\0", "(Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
