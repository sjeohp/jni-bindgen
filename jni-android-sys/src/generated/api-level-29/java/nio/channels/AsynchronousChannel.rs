// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-AsynchronousChannel"))]
__jni_bindgen! {
    /// public interface [AsynchronousChannel](https://developer.android.com/reference/java/nio/channels/AsynchronousChannel.html)
    ///
    /// Required feature: java-nio-channels-AsynchronousChannel
    public interface AsynchronousChannel ("java/nio/channels/AsynchronousChannel") extends crate::java::lang::Object, implements crate::java::nio::channels::Channel {

        /// [close](https://developer.android.com/reference/java/nio/channels/AsynchronousChannel.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/AsynchronousChannel", java.flags == PUBLIC | ABSTRACT, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/AsynchronousChannel\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
