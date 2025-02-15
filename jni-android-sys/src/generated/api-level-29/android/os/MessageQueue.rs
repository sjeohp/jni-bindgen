// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-MessageQueue"))]
__jni_bindgen! {
    /// public final class [MessageQueue](https://developer.android.com/reference/android/os/MessageQueue.html)
    ///
    /// Required feature: android-os-MessageQueue
    public final class MessageQueue ("android/os/MessageQueue") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [MessageQueue](https://developer.android.com/reference/android/os/MessageQueue.html#MessageQueue(boolean))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::MessageQueue>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/MessageQueue", java.flags == (empty), .name == "<init>", .descriptor == "(Z)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "<init>\0", "(Z)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/os/MessageQueue.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/MessageQueue", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isIdle](https://developer.android.com/reference/android/os/MessageQueue.html#isIdle())
        pub fn isIdle<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/MessageQueue", java.flags == PUBLIC, .name == "isIdle", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "isIdle\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addIdleHandler](https://developer.android.com/reference/android/os/MessageQueue.html#addIdleHandler(android.os.MessageQueue.IdleHandler))
        ///
        /// Required features: "android-os-MessageQueue_IdleHandler"
        #[cfg(any(feature = "all", all(feature = "android-os-MessageQueue_IdleHandler")))]
        pub fn addIdleHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::MessageQueue_IdleHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/MessageQueue", java.flags == PUBLIC, .name == "addIdleHandler", .descriptor == "(Landroid/os/MessageQueue$IdleHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "addIdleHandler\0", "(Landroid/os/MessageQueue$IdleHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeIdleHandler](https://developer.android.com/reference/android/os/MessageQueue.html#removeIdleHandler(android.os.MessageQueue.IdleHandler))
        ///
        /// Required features: "android-os-MessageQueue_IdleHandler"
        #[cfg(any(feature = "all", all(feature = "android-os-MessageQueue_IdleHandler")))]
        pub fn removeIdleHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::MessageQueue_IdleHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/MessageQueue", java.flags == PUBLIC, .name == "removeIdleHandler", .descriptor == "(Landroid/os/MessageQueue$IdleHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "removeIdleHandler\0", "(Landroid/os/MessageQueue$IdleHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addOnFileDescriptorEventListener](https://developer.android.com/reference/android/os/MessageQueue.html#addOnFileDescriptorEventListener(java.io.FileDescriptor,%20int,%20android.os.MessageQueue.OnFileDescriptorEventListener))
        ///
        /// Required features: "android-os-MessageQueue_OnFileDescriptorEventListener", "java-io-FileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-MessageQueue_OnFileDescriptorEventListener", feature = "java-io-FileDescriptor")))]
        pub fn addOnFileDescriptorEventListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::MessageQueue_OnFileDescriptorEventListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/MessageQueue", java.flags == PUBLIC, .name == "addOnFileDescriptorEventListener", .descriptor == "(Ljava/io/FileDescriptor;ILandroid/os/MessageQueue$OnFileDescriptorEventListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "addOnFileDescriptorEventListener\0", "(Ljava/io/FileDescriptor;ILandroid/os/MessageQueue$OnFileDescriptorEventListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeOnFileDescriptorEventListener](https://developer.android.com/reference/android/os/MessageQueue.html#removeOnFileDescriptorEventListener(java.io.FileDescriptor))
        ///
        /// Required features: "java-io-FileDescriptor"
        #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor")))]
        pub fn removeOnFileDescriptorEventListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/MessageQueue", java.flags == PUBLIC, .name == "removeOnFileDescriptorEventListener", .descriptor == "(Ljava/io/FileDescriptor;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/MessageQueue\0", "removeOnFileDescriptorEventListener\0", "(Ljava/io/FileDescriptor;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
