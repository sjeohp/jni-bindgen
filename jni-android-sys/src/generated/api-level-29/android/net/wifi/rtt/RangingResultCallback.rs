// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-wifi-rtt-RangingResultCallback"))]
__jni_bindgen! {
    /// public class [RangingResultCallback](https://developer.android.com/reference/android/net/wifi/rtt/RangingResultCallback.html)
    ///
    /// Required feature: android-net-wifi-rtt-RangingResultCallback
    public class RangingResultCallback ("android/net/wifi/rtt/RangingResultCallback") extends crate::java::lang::Object {

        /// [RangingResultCallback](https://developer.android.com/reference/android/net/wifi/rtt/RangingResultCallback.html#RangingResultCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::wifi::rtt::RangingResultCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResultCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResultCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRangingFailure](https://developer.android.com/reference/android/net/wifi/rtt/RangingResultCallback.html#onRangingFailure(int))
        pub fn onRangingFailure<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResultCallback", java.flags == PUBLIC | ABSTRACT, .name == "onRangingFailure", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResultCallback\0", "onRangingFailure\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRangingResults](https://developer.android.com/reference/android/net/wifi/rtt/RangingResultCallback.html#onRangingResults(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn onRangingResults<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/wifi/rtt/RangingResultCallback", java.flags == PUBLIC | ABSTRACT, .name == "onRangingResults", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/wifi/rtt/RangingResultCallback\0", "onRangingResults\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [STATUS_CODE_FAIL](https://developer.android.com/reference/android/net/wifi/rtt/RangingResultCallback.html#STATUS_CODE_FAIL)
        pub const STATUS_CODE_FAIL : i32 = 1;

        /// public static final [STATUS_CODE_FAIL_RTT_NOT_AVAILABLE](https://developer.android.com/reference/android/net/wifi/rtt/RangingResultCallback.html#STATUS_CODE_FAIL_RTT_NOT_AVAILABLE)
        pub const STATUS_CODE_FAIL_RTT_NOT_AVAILABLE : i32 = 2;
    }
}
