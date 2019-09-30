// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-sqlite-SQLiteProgram"))]
__jni_bindgen! {
    /// public class [SQLiteProgram](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html)
    ///
    /// Required feature: android-database-sqlite-SQLiteProgram
    public class SQLiteProgram ("android/database/sqlite/SQLiteProgram") extends crate::android::database::sqlite::SQLiteClosable {

        // // Not emitting: Non-public method
        // /// [SQLiteProgram](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#SQLiteProgram(android.database.sqlite.SQLiteDatabase,%20java.lang.String,%20java.lang.Object%5B%5D,%20android.os.CancellationSignal))
        // ///
        // /// Required features: "android-database-sqlite-SQLiteDatabase", "android-os-CancellationSignal", "java-lang-Object", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "android-database-sqlite-SQLiteDatabase", feature = "android-os-CancellationSignal", feature = "java-lang-Object", feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::sqlite::SQLiteDatabase>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::CancellationSignal>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::sqlite::SQLiteProgram>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/database/sqlite/SQLiteProgram", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/database/sqlite/SQLiteDatabase;Ljava/lang/String;[Ljava/lang/Object;Landroid/os/CancellationSignal;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "<init>\0", "(Landroid/database/sqlite/SQLiteDatabase;Ljava/lang/String;[Ljava/lang/Object;Landroid/os/CancellationSignal;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getUniqueId](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#getUniqueId())
        #[deprecated] pub fn getUniqueId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC | FINAL, .name == "getUniqueId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "getUniqueId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindNull](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#bindNull(int))
        pub fn bindNull<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "bindNull", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "bindNull\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindLong](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#bindLong(int,%20long))
        pub fn bindLong<'env>(&'env self, arg0: i32, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "bindLong", .descriptor == "(IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "bindLong\0", "(IJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindDouble](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#bindDouble(int,%20double))
        pub fn bindDouble<'env>(&'env self, arg0: i32, arg1: f64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "bindDouble", .descriptor == "(ID)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "bindDouble\0", "(ID)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindString](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#bindString(int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn bindString<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "bindString", .descriptor == "(ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "bindString\0", "(ILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindBlob](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#bindBlob(int,%20byte%5B%5D))
        pub fn bindBlob<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "bindBlob", .descriptor == "(I[B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "bindBlob\0", "(I[B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clearBindings](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#clearBindings())
        pub fn clearBindings<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "clearBindings", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "clearBindings\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bindAllArgsAsStrings](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#bindAllArgsAsStrings(java.lang.String%5B%5D))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn bindAllArgsAsStrings<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PUBLIC, .name == "bindAllArgsAsStrings", .descriptor == "([Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "bindAllArgsAsStrings\0", "([Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onAllReferencesReleased](https://developer.android.com/reference/android/database/sqlite/SQLiteProgram.html#onAllReferencesReleased())
        // fn onAllReferencesReleased<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/database/sqlite/SQLiteProgram", java.flags == PROTECTED, .name == "onAllReferencesReleased", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteProgram\0", "onAllReferencesReleased\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
