// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-ContentProvider_CallingIdentity"))]
__jni_bindgen! {
    /// public final class [ContentProvider.CallingIdentity](https://developer.android.com/reference/android/content/ContentProvider.CallingIdentity.html)
    ///
    /// Required feature: android-content-ContentProvider_CallingIdentity
    public final class ContentProvider_CallingIdentity ("android/content/ContentProvider$CallingIdentity") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [CallingIdentity](https://developer.android.com/reference/android/content/ContentProvider.CallingIdentity.html#CallingIdentity(android.content.ContentProvider,%20long,%20java.lang.String))
        // ///
        // /// Required features: "android-content-ContentProvider", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "android-content-ContentProvider", feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentProvider>>, arg1: i64, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::ContentProvider_CallingIdentity>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/ContentProvider$CallingIdentity", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/ContentProvider;JLjava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ContentProvider$CallingIdentity\0", "<init>\0", "(Landroid/content/ContentProvider;JLjava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: this$N outer class pointer
        // pub fn get_"this$0"<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ContentProvider>> { ... }
    }
}
