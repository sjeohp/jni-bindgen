// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-UnresolvedAddressException"))]
__jni_bindgen! {
    /// public class [UnresolvedAddressException](https://developer.android.com/reference/java/nio/channels/UnresolvedAddressException.html)
    ///
    /// Required feature: java-nio-channels-UnresolvedAddressException
    public class UnresolvedAddressException ("java/nio/channels/UnresolvedAddressException") extends crate::java::lang::IllegalArgumentException {

        /// [UnresolvedAddressException](https://developer.android.com/reference/java/nio/channels/UnresolvedAddressException.html#UnresolvedAddressException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::channels::UnresolvedAddressException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/UnresolvedAddressException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/UnresolvedAddressException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
