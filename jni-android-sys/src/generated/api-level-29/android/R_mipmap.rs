// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-R_mipmap"))]
__jni_bindgen! {
    /// public final class [R.mipmap](https://developer.android.com/reference/android/R.mipmap.html)
    ///
    /// Required feature: android-R_mipmap
    public final class R_mipmap ("android/R$mipmap") extends crate::java::lang::Object {

        /// [mipmap](https://developer.android.com/reference/android/R.mipmap.html#mipmap())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::R_mipmap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/R$mipmap", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/R$mipmap\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [sym_def_app_icon](https://developer.android.com/reference/android/R.mipmap.html#sym_def_app_icon)
        pub const sym_def_app_icon : i32 = 17629184;
    }
}
