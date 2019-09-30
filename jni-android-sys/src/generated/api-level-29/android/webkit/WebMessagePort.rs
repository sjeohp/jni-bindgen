// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-WebMessagePort"))]
__jni_bindgen! {
    /// public class [WebMessagePort](https://developer.android.com/reference/android/webkit/WebMessagePort.html)
    ///
    /// Required feature: android-webkit-WebMessagePort
    public class WebMessagePort ("android/webkit/WebMessagePort") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [WebMessagePort](https://developer.android.com/reference/android/webkit/WebMessagePort.html#WebMessagePort())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::WebMessagePort>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/webkit/WebMessagePort", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebMessagePort\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [postMessage](https://developer.android.com/reference/android/webkit/WebMessagePort.html#postMessage(android.webkit.WebMessage))
        ///
        /// Required features: "android-webkit-WebMessage"
        #[cfg(any(feature = "all", all(feature = "android-webkit-WebMessage")))]
        pub fn postMessage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::webkit::WebMessage>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebMessagePort", java.flags == PUBLIC | ABSTRACT, .name == "postMessage", .descriptor == "(Landroid/webkit/WebMessage;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebMessagePort\0", "postMessage\0", "(Landroid/webkit/WebMessage;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/webkit/WebMessagePort.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebMessagePort", java.flags == PUBLIC | ABSTRACT, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebMessagePort\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWebMessageCallback](https://developer.android.com/reference/android/webkit/WebMessagePort.html#setWebMessageCallback(android.webkit.WebMessagePort.WebMessageCallback))
        ///
        /// Required features: "android-webkit-WebMessagePort_WebMessageCallback"
        #[cfg(any(feature = "all", all(feature = "android-webkit-WebMessagePort_WebMessageCallback")))]
        pub fn setWebMessageCallback_WebMessageCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::webkit::WebMessagePort_WebMessageCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebMessagePort", java.flags == PUBLIC | ABSTRACT, .name == "setWebMessageCallback", .descriptor == "(Landroid/webkit/WebMessagePort$WebMessageCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebMessagePort\0", "setWebMessageCallback\0", "(Landroid/webkit/WebMessagePort$WebMessageCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWebMessageCallback](https://developer.android.com/reference/android/webkit/WebMessagePort.html#setWebMessageCallback(android.webkit.WebMessagePort.WebMessageCallback,%20android.os.Handler))
        ///
        /// Required features: "android-os-Handler", "android-webkit-WebMessagePort_WebMessageCallback"
        #[cfg(any(feature = "all", all(feature = "android-os-Handler", feature = "android-webkit-WebMessagePort_WebMessageCallback")))]
        pub fn setWebMessageCallback_WebMessageCallback_Handler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::webkit::WebMessagePort_WebMessageCallback>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebMessagePort", java.flags == PUBLIC | ABSTRACT, .name == "setWebMessageCallback", .descriptor == "(Landroid/webkit/WebMessagePort$WebMessageCallback;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebMessagePort\0", "setWebMessageCallback\0", "(Landroid/webkit/WebMessagePort$WebMessageCallback;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
