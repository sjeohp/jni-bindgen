// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-Script_InvokeID"))]
__jni_bindgen! {
    /// public final class [Script.InvokeID](https://developer.android.com/reference/android/renderscript/Script.InvokeID.html)
    ///
    /// Required feature: android-renderscript-Script_InvokeID
    public final class Script_InvokeID ("android/renderscript/Script$InvokeID") extends crate::android::renderscript::BaseObj {

        // // Not emitting: Non-public method
        // /// [InvokeID](https://developer.android.com/reference/android/renderscript/Script.InvokeID.html#InvokeID(long,%20android.renderscript.RenderScript,%20android.renderscript.Script,%20int))
        // ///
        // /// Required features: "android-renderscript-RenderScript", "android-renderscript-Script"
        // #[cfg(any(feature = "all", all(feature = "android-renderscript-RenderScript", feature = "android-renderscript-Script")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Script>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Script_InvokeID>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/renderscript/Script$InvokeID", java.flags == (empty), .name == "<init>", .descriptor == "(JLandroid/renderscript/RenderScript;Landroid/renderscript/Script;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Script$InvokeID\0", "<init>\0", "(JLandroid/renderscript/RenderScript;Landroid/renderscript/Script;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
