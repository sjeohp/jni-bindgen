// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-se-omapi-Reader"))]
__jni_bindgen! {
    /// public final class [Reader](https://developer.android.com/reference/android/se/omapi/Reader.html)
    ///
    /// Required feature: android-se-omapi-Reader
    public final class Reader ("android/se/omapi/Reader") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Reader](https://developer.android.com/reference/android/se/omapi/Reader.html#Reader())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::se::omapi::Reader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/se/omapi/Reader", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/se/omapi/Reader\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getName](https://developer.android.com/reference/android/se/omapi/Reader.html#getName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/se/omapi/Reader", java.flags == PUBLIC, .name == "getName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/se/omapi/Reader\0", "getName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [openSession](https://developer.android.com/reference/android/se/omapi/Reader.html#openSession())
        ///
        /// Required features: "android-se-omapi-Session"
        #[cfg(any(feature = "all", all(feature = "android-se-omapi-Session")))]
        pub fn openSession<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::se::omapi::Session>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/se/omapi/Reader", java.flags == PUBLIC, .name == "openSession", .descriptor == "()Landroid/se/omapi/Session;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/se/omapi/Reader\0", "openSession\0", "()Landroid/se/omapi/Session;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSecureElementPresent](https://developer.android.com/reference/android/se/omapi/Reader.html#isSecureElementPresent())
        pub fn isSecureElementPresent<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/se/omapi/Reader", java.flags == PUBLIC, .name == "isSecureElementPresent", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/se/omapi/Reader\0", "isSecureElementPresent\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSEService](https://developer.android.com/reference/android/se/omapi/Reader.html#getSEService())
        ///
        /// Required features: "android-se-omapi-SEService"
        #[cfg(any(feature = "all", all(feature = "android-se-omapi-SEService")))]
        pub fn getSEService<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::se::omapi::SEService>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/se/omapi/Reader", java.flags == PUBLIC, .name == "getSEService", .descriptor == "()Landroid/se/omapi/SEService;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/se/omapi/Reader\0", "getSEService\0", "()Landroid/se/omapi/SEService;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [closeSessions](https://developer.android.com/reference/android/se/omapi/Reader.html#closeSessions())
        pub fn closeSessions<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/se/omapi/Reader", java.flags == PUBLIC, .name == "closeSessions", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/se/omapi/Reader\0", "closeSessions\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
