// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-TestLooperManager"))]
__jni_bindgen! {
    /// public class [TestLooperManager](https://developer.android.com/reference/android/os/TestLooperManager.html)
    ///
    /// Required feature: android-os-TestLooperManager
    public class TestLooperManager ("android/os/TestLooperManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [TestLooperManager](https://developer.android.com/reference/android/os/TestLooperManager.html#TestLooperManager(android.os.Looper))
        // ///
        // /// Required features: "android-os-Looper"
        // #[cfg(any(feature = "all", all(feature = "android-os-Looper")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Looper>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::TestLooperManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/TestLooperManager", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Looper;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "<init>\0", "(Landroid/os/Looper;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getMessageQueue](https://developer.android.com/reference/android/os/TestLooperManager.html#getMessageQueue())
        ///
        /// Required features: "android-os-MessageQueue"
        #[cfg(any(feature = "all", all(feature = "android-os-MessageQueue")))]
        pub fn getMessageQueue<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::MessageQueue>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "getMessageQueue", .descriptor == "()Landroid/os/MessageQueue;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "getMessageQueue\0", "()Landroid/os/MessageQueue;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [next](https://developer.android.com/reference/android/os/TestLooperManager.html#next())
        ///
        /// Required features: "android-os-Message"
        #[cfg(any(feature = "all", all(feature = "android-os-Message")))]
        pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Message>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "next", .descriptor == "()Landroid/os/Message;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "next\0", "()Landroid/os/Message;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [release](https://developer.android.com/reference/android/os/TestLooperManager.html#release())
        pub fn release<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "release", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "release\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [execute](https://developer.android.com/reference/android/os/TestLooperManager.html#execute(android.os.Message))
        ///
        /// Required features: "android-os-Message"
        #[cfg(any(feature = "all", all(feature = "android-os-Message")))]
        pub fn execute<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Message>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "execute", .descriptor == "(Landroid/os/Message;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "execute\0", "(Landroid/os/Message;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [recycle](https://developer.android.com/reference/android/os/TestLooperManager.html#recycle(android.os.Message))
        ///
        /// Required features: "android-os-Message"
        #[cfg(any(feature = "all", all(feature = "android-os-Message")))]
        pub fn recycle<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Message>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "recycle", .descriptor == "(Landroid/os/Message;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "recycle\0", "(Landroid/os/Message;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasMessages](https://developer.android.com/reference/android/os/TestLooperManager.html#hasMessages(android.os.Handler,%20java.lang.Object,%20int))
        ///
        /// Required features: "android-os-Handler", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-os-Handler", feature = "java-lang-Object")))]
        pub fn hasMessages_Handler_Object_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "hasMessages", .descriptor == "(Landroid/os/Handler;Ljava/lang/Object;I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "hasMessages\0", "(Landroid/os/Handler;Ljava/lang/Object;I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasMessages](https://developer.android.com/reference/android/os/TestLooperManager.html#hasMessages(android.os.Handler,%20java.lang.Object,%20java.lang.Runnable))
        ///
        /// Required features: "android-os-Handler", "java-lang-Object", "java-lang-Runnable"
        #[cfg(any(feature = "all", all(feature = "android-os-Handler", feature = "java-lang-Object", feature = "java-lang-Runnable")))]
        pub fn hasMessages_Handler_Object_Runnable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/TestLooperManager", java.flags == PUBLIC, .name == "hasMessages", .descriptor == "(Landroid/os/Handler;Ljava/lang/Object;Ljava/lang/Runnable;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/TestLooperManager\0", "hasMessages\0", "(Landroid/os/Handler;Ljava/lang/Object;Ljava/lang/Runnable;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
