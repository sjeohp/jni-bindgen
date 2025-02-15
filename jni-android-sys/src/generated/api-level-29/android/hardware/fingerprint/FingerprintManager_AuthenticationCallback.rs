// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-fingerprint-FingerprintManager_AuthenticationCallback"))]
__jni_bindgen! {
    /// public class [FingerprintManager.AuthenticationCallback](https://developer.android.com/reference/android/hardware/fingerprint/FingerprintManager.AuthenticationCallback.html)
    ///
    /// Required feature: android-hardware-fingerprint-FingerprintManager_AuthenticationCallback
    #[deprecated] public class FingerprintManager_AuthenticationCallback ("android/hardware/fingerprint/FingerprintManager$AuthenticationCallback") extends crate::java::lang::Object {

        /// [AuthenticationCallback](https://developer.android.com/reference/android/hardware/fingerprint/FingerprintManager.AuthenticationCallback.html#AuthenticationCallback())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::fingerprint::FingerprintManager_AuthenticationCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/fingerprint/FingerprintManager$AuthenticationCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/fingerprint/FingerprintManager$AuthenticationCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAuthenticationError](https://developer.android.com/reference/android/hardware/fingerprint/FingerprintManager.AuthenticationCallback.html#onAuthenticationError(int,%20java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn onAuthenticationError<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/fingerprint/FingerprintManager$AuthenticationCallback", java.flags == PUBLIC, .name == "onAuthenticationError", .descriptor == "(ILjava/lang/CharSequence;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/fingerprint/FingerprintManager$AuthenticationCallback\0", "onAuthenticationError\0", "(ILjava/lang/CharSequence;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAuthenticationHelp](https://developer.android.com/reference/android/hardware/fingerprint/FingerprintManager.AuthenticationCallback.html#onAuthenticationHelp(int,%20java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn onAuthenticationHelp<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/fingerprint/FingerprintManager$AuthenticationCallback", java.flags == PUBLIC, .name == "onAuthenticationHelp", .descriptor == "(ILjava/lang/CharSequence;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/fingerprint/FingerprintManager$AuthenticationCallback\0", "onAuthenticationHelp\0", "(ILjava/lang/CharSequence;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAuthenticationSucceeded](https://developer.android.com/reference/android/hardware/fingerprint/FingerprintManager.AuthenticationCallback.html#onAuthenticationSucceeded(android.hardware.fingerprint.FingerprintManager.AuthenticationResult))
        ///
        /// Required features: "android-hardware-fingerprint-FingerprintManager_AuthenticationResult"
        #[cfg(any(feature = "all", all(feature = "android-hardware-fingerprint-FingerprintManager_AuthenticationResult")))]
        #[deprecated] pub fn onAuthenticationSucceeded<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::fingerprint::FingerprintManager_AuthenticationResult>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/fingerprint/FingerprintManager$AuthenticationCallback", java.flags == PUBLIC, .name == "onAuthenticationSucceeded", .descriptor == "(Landroid/hardware/fingerprint/FingerprintManager$AuthenticationResult;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/fingerprint/FingerprintManager$AuthenticationCallback\0", "onAuthenticationSucceeded\0", "(Landroid/hardware/fingerprint/FingerprintManager$AuthenticationResult;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAuthenticationFailed](https://developer.android.com/reference/android/hardware/fingerprint/FingerprintManager.AuthenticationCallback.html#onAuthenticationFailed())
        #[deprecated] pub fn onAuthenticationFailed<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/fingerprint/FingerprintManager$AuthenticationCallback", java.flags == PUBLIC, .name == "onAuthenticationFailed", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/fingerprint/FingerprintManager$AuthenticationCallback\0", "onAuthenticationFailed\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
