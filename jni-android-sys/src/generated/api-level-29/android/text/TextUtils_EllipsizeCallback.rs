// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-TextUtils_EllipsizeCallback"))]
__jni_bindgen! {
    /// public interface [TextUtils.EllipsizeCallback](https://developer.android.com/reference/android/text/TextUtils.EllipsizeCallback.html)
    ///
    /// Required feature: android-text-TextUtils_EllipsizeCallback
    public interface TextUtils_EllipsizeCallback ("android/text/TextUtils$EllipsizeCallback") extends crate::java::lang::Object {

        /// [ellipsized](https://developer.android.com/reference/android/text/TextUtils.EllipsizeCallback.html#ellipsized(int,%20int))
        pub fn ellipsized<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/TextUtils$EllipsizeCallback", java.flags == PUBLIC | ABSTRACT, .name == "ellipsized", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/TextUtils$EllipsizeCallback\0", "ellipsized\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
