// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-sqlite-SQLiteClosable"))]
__jni_bindgen! {
    /// public class [SQLiteClosable](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html)
    ///
    /// Required feature: android-database-sqlite-SQLiteClosable
    public class SQLiteClosable ("android/database/sqlite/SQLiteClosable") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        /// [SQLiteClosable](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#SQLiteClosable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::sqlite::SQLiteClosable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onAllReferencesReleased](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#onAllReferencesReleased())
        // fn onAllReferencesReleased<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PROTECTED | ABSTRACT, .name == "onAllReferencesReleased", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "onAllReferencesReleased\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onAllReferencesReleasedFromContainer](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#onAllReferencesReleasedFromContainer())
        // #[deprecated] fn onAllReferencesReleasedFromContainer<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PROTECTED, .name == "onAllReferencesReleasedFromContainer", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "onAllReferencesReleasedFromContainer\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [acquireReference](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#acquireReference())
        pub fn acquireReference<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PUBLIC, .name == "acquireReference", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "acquireReference\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [releaseReference](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#releaseReference())
        pub fn releaseReference<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PUBLIC, .name == "releaseReference", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "releaseReference\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [releaseReferenceFromContainer](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#releaseReferenceFromContainer())
        #[deprecated] pub fn releaseReferenceFromContainer<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PUBLIC, .name == "releaseReferenceFromContainer", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "releaseReferenceFromContainer\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/database/sqlite/SQLiteClosable.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteClosable", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteClosable\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
