// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-sql-RowSetWriter"))]
__jni_bindgen! {
    /// public interface [RowSetWriter](https://developer.android.com/reference/javax/sql/RowSetWriter.html)
    ///
    /// Required feature: javax-sql-RowSetWriter
    public interface RowSetWriter ("javax/sql/RowSetWriter") extends crate::java::lang::Object {

        /// [writeData](https://developer.android.com/reference/javax/sql/RowSetWriter.html#writeData(javax.sql.RowSetInternal))
        ///
        /// Required features: "javax-sql-RowSetInternal"
        #[cfg(any(feature = "all", all(feature = "javax-sql-RowSetInternal")))]
        pub fn writeData<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::RowSetInternal>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/RowSetWriter", java.flags == PUBLIC | ABSTRACT, .name == "writeData", .descriptor == "(Ljavax/sql/RowSetInternal;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/RowSetWriter\0", "writeData\0", "(Ljavax/sql/RowSetInternal;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
