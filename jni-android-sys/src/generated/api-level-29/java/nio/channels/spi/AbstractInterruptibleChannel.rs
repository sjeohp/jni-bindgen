// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-spi-AbstractInterruptibleChannel"))]
__jni_bindgen! {
    /// public class [AbstractInterruptibleChannel](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html)
    ///
    /// Required feature: java-nio-channels-spi-AbstractInterruptibleChannel
    public class AbstractInterruptibleChannel ("java/nio/channels/spi/AbstractInterruptibleChannel") extends crate::java::lang::Object, implements crate::java::nio::channels::Channel, crate::java::nio::channels::InterruptibleChannel {

        // // Not emitting: Non-public method
        // /// [AbstractInterruptibleChannel](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html#AbstractInterruptibleChannel())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::channels::spi::AbstractInterruptibleChannel>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/channels/spi/AbstractInterruptibleChannel", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/spi/AbstractInterruptibleChannel\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [close](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/spi/AbstractInterruptibleChannel", java.flags == PUBLIC | FINAL, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/spi/AbstractInterruptibleChannel\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [implCloseChannel](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html#implCloseChannel())
        // fn implCloseChannel<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/channels/spi/AbstractInterruptibleChannel", java.flags == PROTECTED | ABSTRACT, .name == "implCloseChannel", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/spi/AbstractInterruptibleChannel\0", "implCloseChannel\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isOpen](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html#isOpen())
        pub fn isOpen<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/spi/AbstractInterruptibleChannel", java.flags == PUBLIC | FINAL, .name == "isOpen", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/spi/AbstractInterruptibleChannel\0", "isOpen\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [begin](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html#begin())
        // fn begin<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/channels/spi/AbstractInterruptibleChannel", java.flags == PROTECTED | FINAL, .name == "begin", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/spi/AbstractInterruptibleChannel\0", "begin\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [end](https://developer.android.com/reference/java/nio/channels/spi/AbstractInterruptibleChannel.html#end(boolean))
        // fn end<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/channels/spi/AbstractInterruptibleChannel", java.flags == PROTECTED | FINAL, .name == "end", .descriptor == "(Z)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/spi/AbstractInterruptibleChannel\0", "end\0", "(Z)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
