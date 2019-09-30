// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-sqlite-SQLiteDatabase_CursorFactory"))]
__jni_bindgen! {
    /// public interface [SQLiteDatabase.CursorFactory](https://developer.android.com/reference/android/database/sqlite/SQLiteDatabase.CursorFactory.html)
    ///
    /// Required feature: android-database-sqlite-SQLiteDatabase_CursorFactory
    public interface SQLiteDatabase_CursorFactory ("android/database/sqlite/SQLiteDatabase$CursorFactory") extends crate::java::lang::Object {

        /// [newCursor](https://developer.android.com/reference/android/database/sqlite/SQLiteDatabase.CursorFactory.html#newCursor(android.database.sqlite.SQLiteDatabase,%20android.database.sqlite.SQLiteCursorDriver,%20java.lang.String,%20android.database.sqlite.SQLiteQuery))
        ///
        /// Required features: "android-database-Cursor", "android-database-sqlite-SQLiteCursorDriver", "android-database-sqlite-SQLiteDatabase", "android-database-sqlite-SQLiteQuery", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-database-Cursor", feature = "android-database-sqlite-SQLiteCursorDriver", feature = "android-database-sqlite-SQLiteDatabase", feature = "android-database-sqlite-SQLiteQuery", feature = "java-lang-String")))]
        pub fn newCursor<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::sqlite::SQLiteDatabase>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::sqlite::SQLiteCursorDriver>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::sqlite::SQLiteQuery>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::database::Cursor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteDatabase$CursorFactory", java.flags == PUBLIC | ABSTRACT, .name == "newCursor", .descriptor == "(Landroid/database/sqlite/SQLiteDatabase;Landroid/database/sqlite/SQLiteCursorDriver;Ljava/lang/String;Landroid/database/sqlite/SQLiteQuery;)Landroid/database/Cursor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteDatabase$CursorFactory\0", "newCursor\0", "(Landroid/database/sqlite/SQLiteDatabase;Landroid/database/sqlite/SQLiteCursorDriver;Ljava/lang/String;Landroid/database/sqlite/SQLiteQuery;)Landroid/database/Cursor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
