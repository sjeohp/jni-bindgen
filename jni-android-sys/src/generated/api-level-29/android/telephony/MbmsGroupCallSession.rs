// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-MbmsGroupCallSession"))]
__jni_bindgen! {
    /// public class [MbmsGroupCallSession](https://developer.android.com/reference/android/telephony/MbmsGroupCallSession.html)
    ///
    /// Required feature: android-telephony-MbmsGroupCallSession
    public class MbmsGroupCallSession ("android/telephony/MbmsGroupCallSession") extends crate::java::lang::Object, implements crate::java::lang::AutoCloseable {

        // // Not emitting: Non-public method
        // /// [MbmsGroupCallSession](https://developer.android.com/reference/android/telephony/MbmsGroupCallSession.html#MbmsGroupCallSession(android.content.Context,%20java.util.concurrent.Executor,%20int,%20android.telephony.mbms.MbmsGroupCallSessionCallback))
        // ///
        // /// Required features: "android-content-Context", "android-telephony-mbms-MbmsGroupCallSessionCallback", "java-util-concurrent-Executor"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-telephony-mbms-MbmsGroupCallSessionCallback", feature = "java-util-concurrent-Executor")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::mbms::MbmsGroupCallSessionCallback>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::MbmsGroupCallSession>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/MbmsGroupCallSession", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;Ljava/util/concurrent/Executor;ILandroid/telephony/mbms/MbmsGroupCallSessionCallback;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/MbmsGroupCallSession\0", "<init>\0", "(Landroid/content/Context;Ljava/util/concurrent/Executor;ILandroid/telephony/mbms/MbmsGroupCallSessionCallback;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [create](https://developer.android.com/reference/android/telephony/MbmsGroupCallSession.html#create(android.content.Context,%20int,%20java.util.concurrent.Executor,%20android.telephony.mbms.MbmsGroupCallSessionCallback))
        ///
        /// Required features: "android-content-Context", "android-telephony-MbmsGroupCallSession", "android-telephony-mbms-MbmsGroupCallSessionCallback", "java-util-concurrent-Executor"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-telephony-MbmsGroupCallSession", feature = "android-telephony-mbms-MbmsGroupCallSessionCallback", feature = "java-util-concurrent-Executor")))]
        pub fn create_Context_int_Executor_MbmsGroupCallSessionCallback<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::mbms::MbmsGroupCallSessionCallback>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::MbmsGroupCallSession>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/MbmsGroupCallSession", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/content/Context;ILjava/util/concurrent/Executor;Landroid/telephony/mbms/MbmsGroupCallSessionCallback;)Landroid/telephony/MbmsGroupCallSession;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/telephony/MbmsGroupCallSession\0", "create\0", "(Landroid/content/Context;ILjava/util/concurrent/Executor;Landroid/telephony/mbms/MbmsGroupCallSessionCallback;)Landroid/telephony/MbmsGroupCallSession;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/telephony/MbmsGroupCallSession.html#create(android.content.Context,%20java.util.concurrent.Executor,%20android.telephony.mbms.MbmsGroupCallSessionCallback))
        ///
        /// Required features: "android-content-Context", "android-telephony-MbmsGroupCallSession", "android-telephony-mbms-MbmsGroupCallSessionCallback", "java-util-concurrent-Executor"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-telephony-MbmsGroupCallSession", feature = "android-telephony-mbms-MbmsGroupCallSessionCallback", feature = "java-util-concurrent-Executor")))]
        pub fn create_Context_Executor_MbmsGroupCallSessionCallback<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::mbms::MbmsGroupCallSessionCallback>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::MbmsGroupCallSession>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/MbmsGroupCallSession", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/content/Context;Ljava/util/concurrent/Executor;Landroid/telephony/mbms/MbmsGroupCallSessionCallback;)Landroid/telephony/MbmsGroupCallSession;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/telephony/MbmsGroupCallSession\0", "create\0", "(Landroid/content/Context;Ljava/util/concurrent/Executor;Landroid/telephony/mbms/MbmsGroupCallSessionCallback;)Landroid/telephony/MbmsGroupCallSession;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/telephony/MbmsGroupCallSession.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/MbmsGroupCallSession", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/MbmsGroupCallSession\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startGroupCall](https://developer.android.com/reference/android/telephony/MbmsGroupCallSession.html#startGroupCall(long,%20java.util.List,%20java.util.List,%20java.util.concurrent.Executor,%20android.telephony.mbms.GroupCallCallback))
        ///
        /// Required features: "android-telephony-mbms-GroupCall", "android-telephony-mbms-GroupCallCallback", "java-util-List", "java-util-concurrent-Executor"
        #[cfg(any(feature = "all", all(feature = "android-telephony-mbms-GroupCall", feature = "android-telephony-mbms-GroupCallCallback", feature = "java-util-List", feature = "java-util-concurrent-Executor")))]
        pub fn startGroupCall<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::mbms::GroupCallCallback>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telephony::mbms::GroupCall>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/MbmsGroupCallSession", java.flags == PUBLIC, .name == "startGroupCall", .descriptor == "(JLjava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;Landroid/telephony/mbms/GroupCallCallback;)Landroid/telephony/mbms/GroupCall;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/MbmsGroupCallSession\0", "startGroupCall\0", "(JLjava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;Landroid/telephony/mbms/GroupCallCallback;)Landroid/telephony/mbms/GroupCall;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
