// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-AppOpsManager_OnOpChangedListener"))]
__jni_bindgen! {
    /// public interface [AppOpsManager.OnOpChangedListener](https://developer.android.com/reference/android/app/AppOpsManager.OnOpChangedListener.html)
    ///
    /// Required feature: android-app-AppOpsManager_OnOpChangedListener
    public interface AppOpsManager_OnOpChangedListener ("android/app/AppOpsManager$OnOpChangedListener") extends crate::java::lang::Object {

        /// [onOpChanged](https://developer.android.com/reference/android/app/AppOpsManager.OnOpChangedListener.html#onOpChanged(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onOpChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/AppOpsManager$OnOpChangedListener", java.flags == PUBLIC | ABSTRACT, .name == "onOpChanged", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/AppOpsManager$OnOpChangedListener\0", "onOpChanged\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
