// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-sql-RowSetReader"))]
__jni_bindgen! {
    /// public interface [RowSetReader](https://developer.android.com/reference/javax/sql/RowSetReader.html)
    ///
    /// Required feature: javax-sql-RowSetReader
    public interface RowSetReader ("javax/sql/RowSetReader") extends crate::java::lang::Object {

        /// [readData](https://developer.android.com/reference/javax/sql/RowSetReader.html#readData(javax.sql.RowSetInternal))
        ///
        /// Required features: "javax-sql-RowSetInternal"
        #[cfg(any(feature = "all", all(feature = "javax-sql-RowSetInternal")))]
        pub fn readData<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::RowSetInternal>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/RowSetReader", java.flags == PUBLIC | ABSTRACT, .name == "readData", .descriptor == "(Ljavax/sql/RowSetInternal;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/RowSetReader\0", "readData\0", "(Ljavax/sql/RowSetInternal;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
