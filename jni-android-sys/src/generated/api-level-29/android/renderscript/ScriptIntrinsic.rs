// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-ScriptIntrinsic"))]
__jni_bindgen! {
    /// public class [ScriptIntrinsic](https://developer.android.com/reference/android/renderscript/ScriptIntrinsic.html)
    ///
    /// Required feature: android-renderscript-ScriptIntrinsic
    public class ScriptIntrinsic ("android/renderscript/ScriptIntrinsic") extends crate::android::renderscript::Script {

        // // Not emitting: Non-public method
        // /// [ScriptIntrinsic](https://developer.android.com/reference/android/renderscript/ScriptIntrinsic.html#ScriptIntrinsic(long,%20android.renderscript.RenderScript))
        // ///
        // /// Required features: "android-renderscript-RenderScript"
        // #[cfg(any(feature = "all", all(feature = "android-renderscript-RenderScript")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptIntrinsic>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/renderscript/ScriptIntrinsic", java.flags == (empty), .name == "<init>", .descriptor == "(JLandroid/renderscript/RenderScript;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsic\0", "<init>\0", "(JLandroid/renderscript/RenderScript;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
