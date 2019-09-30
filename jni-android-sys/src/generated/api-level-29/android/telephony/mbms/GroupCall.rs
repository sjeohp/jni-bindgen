// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-mbms-GroupCall"))]
__jni_bindgen! {
    /// public class [GroupCall](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html)
    ///
    /// Required feature: android-telephony-mbms-GroupCall
    public class GroupCall ("android/telephony/mbms/GroupCall") extends crate::java::lang::Object, implements crate::java::lang::AutoCloseable {

        // // Not emitting: Non-public method
        // /// [GroupCall](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#GroupCall())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::mbms::GroupCall>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/mbms/GroupCall", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/GroupCall\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getTmgi](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#getTmgi())
        pub fn getTmgi<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/GroupCall", java.flags == PUBLIC, .name == "getTmgi", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/GroupCall\0", "getTmgi\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateGroupCall](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#updateGroupCall(java.util.List,%20java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn updateGroupCall<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/GroupCall", java.flags == PUBLIC, .name == "updateGroupCall", .descriptor == "(Ljava/util/List;Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/GroupCall\0", "updateGroupCall\0", "(Ljava/util/List;Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/mbms/GroupCall", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/GroupCall\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [REASON_BY_USER_REQUEST](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#REASON_BY_USER_REQUEST)
        pub const REASON_BY_USER_REQUEST : i32 = 1;

        /// public static final [REASON_FREQUENCY_CONFLICT](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#REASON_FREQUENCY_CONFLICT)
        pub const REASON_FREQUENCY_CONFLICT : i32 = 3;

        /// public static final [REASON_LEFT_MBMS_BROADCAST_AREA](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#REASON_LEFT_MBMS_BROADCAST_AREA)
        pub const REASON_LEFT_MBMS_BROADCAST_AREA : i32 = 6;

        /// public static final [REASON_NONE](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#REASON_NONE)
        pub const REASON_NONE : i32 = 0;

        /// public static final [REASON_NOT_CONNECTED_TO_HOMECARRIER_LTE](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#REASON_NOT_CONNECTED_TO_HOMECARRIER_LTE)
        pub const REASON_NOT_CONNECTED_TO_HOMECARRIER_LTE : i32 = 5;

        /// public static final [REASON_OUT_OF_MEMORY](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#REASON_OUT_OF_MEMORY)
        pub const REASON_OUT_OF_MEMORY : i32 = 4;

        /// public static final [STATE_STALLED](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#STATE_STALLED)
        pub const STATE_STALLED : i32 = 3;

        /// public static final [STATE_STARTED](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#STATE_STARTED)
        pub const STATE_STARTED : i32 = 2;

        /// public static final [STATE_STOPPED](https://developer.android.com/reference/android/telephony/mbms/GroupCall.html#STATE_STOPPED)
        pub const STATE_STOPPED : i32 = 1;
    }
}
