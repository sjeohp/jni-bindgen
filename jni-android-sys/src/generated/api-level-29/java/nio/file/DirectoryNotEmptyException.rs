// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-DirectoryNotEmptyException"))]
__jni_bindgen! {
    /// public class [DirectoryNotEmptyException](https://developer.android.com/reference/java/nio/file/DirectoryNotEmptyException.html)
    ///
    /// Required feature: java-nio-file-DirectoryNotEmptyException
    public class DirectoryNotEmptyException ("java/nio/file/DirectoryNotEmptyException") extends crate::java::nio::file::FileSystemException {

        /// [DirectoryNotEmptyException](https://developer.android.com/reference/java/nio/file/DirectoryNotEmptyException.html#DirectoryNotEmptyException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::file::DirectoryNotEmptyException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/DirectoryNotEmptyException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/DirectoryNotEmptyException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
