// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-NotDirectoryException"))]
__jni_bindgen! {
    /// public class [NotDirectoryException](https://developer.android.com/reference/java/nio/file/NotDirectoryException.html)
    ///
    /// Required feature: java-nio-file-NotDirectoryException
    public class NotDirectoryException ("java/nio/file/NotDirectoryException") extends crate::java::nio::file::FileSystemException {

        /// [NotDirectoryException](https://developer.android.com/reference/java/nio/file/NotDirectoryException.html#NotDirectoryException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::file::NotDirectoryException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/NotDirectoryException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/NotDirectoryException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
