// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-LoginFilter_UsernameFilterGMail"))]
__jni_bindgen! {
    /// public class [LoginFilter.UsernameFilterGMail](https://developer.android.com/reference/android/text/LoginFilter.UsernameFilterGMail.html)
    ///
    /// Required feature: android-text-LoginFilter_UsernameFilterGMail
    public class LoginFilter_UsernameFilterGMail ("android/text/LoginFilter$UsernameFilterGMail") extends crate::android::text::LoginFilter {

        /// [UsernameFilterGMail](https://developer.android.com/reference/android/text/LoginFilter.UsernameFilterGMail.html#UsernameFilterGMail())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::LoginFilter_UsernameFilterGMail>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter$UsernameFilterGMail", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter$UsernameFilterGMail\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [UsernameFilterGMail](https://developer.android.com/reference/android/text/LoginFilter.UsernameFilterGMail.html#UsernameFilterGMail(boolean))
        pub fn new_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::LoginFilter_UsernameFilterGMail>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter$UsernameFilterGMail", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter$UsernameFilterGMail\0", "<init>\0", "(Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isAllowed](https://developer.android.com/reference/android/text/LoginFilter.UsernameFilterGMail.html#isAllowed(char))
        pub fn isAllowed<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter$UsernameFilterGMail", java.flags == PUBLIC, .name == "isAllowed", .descriptor == "(C)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter$UsernameFilterGMail\0", "isAllowed\0", "(C)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
