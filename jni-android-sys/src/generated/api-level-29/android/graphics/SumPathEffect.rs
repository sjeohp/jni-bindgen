// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-SumPathEffect"))]
__jni_bindgen! {
    /// public class [SumPathEffect](https://developer.android.com/reference/android/graphics/SumPathEffect.html)
    ///
    /// Required feature: android-graphics-SumPathEffect
    public class SumPathEffect ("android/graphics/SumPathEffect") extends crate::android::graphics::PathEffect {

        /// [SumPathEffect](https://developer.android.com/reference/android/graphics/SumPathEffect.html#SumPathEffect(android.graphics.PathEffect,%20android.graphics.PathEffect))
        ///
        /// Required features: "android-graphics-PathEffect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-PathEffect")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::PathEffect>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::PathEffect>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::SumPathEffect>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/SumPathEffect", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/PathEffect;Landroid/graphics/PathEffect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/SumPathEffect\0", "<init>\0", "(Landroid/graphics/PathEffect;Landroid/graphics/PathEffect;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
