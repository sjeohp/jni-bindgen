// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-Pipe_SourceChannel"))]
__jni_bindgen! {
    /// public class [Pipe.SourceChannel](https://developer.android.com/reference/java/nio/channels/Pipe.SourceChannel.html)
    ///
    /// Required feature: java-nio-channels-Pipe_SourceChannel
    public class Pipe_SourceChannel ("java/nio/channels/Pipe$SourceChannel") extends crate::java::nio::channels::spi::AbstractSelectableChannel, implements crate::java::nio::channels::ReadableByteChannel, crate::java::nio::channels::ScatteringByteChannel {

        // // Not emitting: Non-public method
        // /// [SourceChannel](https://developer.android.com/reference/java/nio/channels/Pipe.SourceChannel.html#SourceChannel(java.nio.channels.spi.SelectorProvider))
        // ///
        // /// Required features: "java-nio-channels-spi-SelectorProvider"
        // #[cfg(any(feature = "all", all(feature = "java-nio-channels-spi-SelectorProvider")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::channels::spi::SelectorProvider>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::channels::Pipe_SourceChannel>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/channels/Pipe$SourceChannel", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/nio/channels/spi/SelectorProvider;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/Pipe$SourceChannel\0", "<init>\0", "(Ljava/nio/channels/spi/SelectorProvider;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [validOps](https://developer.android.com/reference/java/nio/channels/Pipe.SourceChannel.html#validOps())
        pub fn validOps<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/Pipe$SourceChannel", java.flags == PUBLIC | FINAL, .name == "validOps", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/Pipe$SourceChannel\0", "validOps\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
