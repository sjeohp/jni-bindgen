// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-Channel"))]
__jni_bindgen! {
    /// public interface [Channel](https://developer.android.com/reference/java/nio/channels/Channel.html)
    ///
    /// Required feature: java-nio-channels-Channel
    public interface Channel ("java/nio/channels/Channel") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        /// [isOpen](https://developer.android.com/reference/java/nio/channels/Channel.html#isOpen())
        pub fn isOpen<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/Channel", java.flags == PUBLIC | ABSTRACT, .name == "isOpen", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/Channel\0", "isOpen\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/nio/channels/Channel.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/Channel", java.flags == PUBLIC | ABSTRACT, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/Channel\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
