// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-style-TabStopSpan_Standard"))]
__jni_bindgen! {
    /// public class [TabStopSpan.Standard](https://developer.android.com/reference/android/text/style/TabStopSpan.Standard.html)
    ///
    /// Required feature: android-text-style-TabStopSpan_Standard
    public class TabStopSpan_Standard ("android/text/style/TabStopSpan$Standard") extends crate::java::lang::Object, implements crate::android::text::style::TabStopSpan {

        /// [Standard](https://developer.android.com/reference/android/text/style/TabStopSpan.Standard.html#Standard(int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::TabStopSpan_Standard>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TabStopSpan$Standard", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TabStopSpan$Standard\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTabStop](https://developer.android.com/reference/android/text/style/TabStopSpan.Standard.html#getTabStop())
        pub fn getTabStop<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TabStopSpan$Standard", java.flags == PUBLIC, .name == "getTabStop", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TabStopSpan$Standard\0", "getTabStop\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
