// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-StatsLog"))]
__jni_bindgen! {
    /// public final class [StatsLog](https://developer.android.com/reference/android/util/StatsLog.html)
    ///
    /// Required feature: android-util-StatsLog
    public final class StatsLog ("android/util/StatsLog") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [StatsLog](https://developer.android.com/reference/android/util/StatsLog.html#StatsLog())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::StatsLog>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/util/StatsLog", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/StatsLog\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [logStart](https://developer.android.com/reference/android/util/StatsLog.html#logStart(int))
        pub fn logStart<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/StatsLog", java.flags == PUBLIC | STATIC, .name == "logStart", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/StatsLog\0", "logStart\0", "(I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [logStop](https://developer.android.com/reference/android/util/StatsLog.html#logStop(int))
        pub fn logStop<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/StatsLog", java.flags == PUBLIC | STATIC, .name == "logStop", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/StatsLog\0", "logStop\0", "(I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [logEvent](https://developer.android.com/reference/android/util/StatsLog.html#logEvent(int))
        pub fn logEvent<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/StatsLog", java.flags == PUBLIC | STATIC, .name == "logEvent", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/StatsLog\0", "logEvent\0", "(I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [logBinaryPushStateChanged](https://developer.android.com/reference/android/util/StatsLog.html#logBinaryPushStateChanged(java.lang.String,%20long,%20int,%20int,%20long%5B%5D))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn logBinaryPushStateChanged<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/StatsLog", java.flags == PUBLIC | STATIC, .name == "logBinaryPushStateChanged", .descriptor == "(Ljava/lang/String;JII[J)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/StatsLog\0", "logBinaryPushStateChanged\0", "(Ljava/lang/String;JII[J)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
