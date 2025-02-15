// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-InterruptedIOException"))]
__jni_bindgen! {
    /// public class [InterruptedIOException](https://developer.android.com/reference/java/io/InterruptedIOException.html)
    ///
    /// Required feature: java-io-InterruptedIOException
    public class InterruptedIOException ("java/io/InterruptedIOException") extends crate::java::io::IOException {

        /// [InterruptedIOException](https://developer.android.com/reference/java/io/InterruptedIOException.html#InterruptedIOException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::InterruptedIOException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InterruptedIOException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InterruptedIOException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InterruptedIOException](https://developer.android.com/reference/java/io/InterruptedIOException.html#InterruptedIOException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::InterruptedIOException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InterruptedIOException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InterruptedIOException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [bytesTransferred](https://developer.android.com/reference/java/io/InterruptedIOException.html#bytesTransferred)
        pub fn bytesTransferred<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("java/io/InterruptedIOException\0", "bytesTransferred\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [bytesTransferred](https://developer.android.com/reference/java/io/InterruptedIOException.html#bytesTransferred)
        pub fn set_bytesTransferred<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("java/io/InterruptedIOException\0", "bytesTransferred\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
