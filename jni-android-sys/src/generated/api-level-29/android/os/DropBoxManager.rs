// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-DropBoxManager"))]
__jni_bindgen! {
    /// public class [DropBoxManager](https://developer.android.com/reference/android/os/DropBoxManager.html)
    ///
    /// Required feature: android-os-DropBoxManager
    public class DropBoxManager ("android/os/DropBoxManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [DropBoxManager](https://developer.android.com/reference/android/os/DropBoxManager.html#DropBoxManager())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::DropBoxManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/DropBoxManager", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/DropBoxManager\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [addText](https://developer.android.com/reference/android/os/DropBoxManager.html#addText(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn addText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/DropBoxManager", java.flags == PUBLIC, .name == "addText", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/DropBoxManager\0", "addText\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addData](https://developer.android.com/reference/android/os/DropBoxManager.html#addData(java.lang.String,%20byte%5B%5D,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn addData<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/DropBoxManager", java.flags == PUBLIC, .name == "addData", .descriptor == "(Ljava/lang/String;[BI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/DropBoxManager\0", "addData\0", "(Ljava/lang/String;[BI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addFile](https://developer.android.com/reference/android/os/DropBoxManager.html#addFile(java.lang.String,%20java.io.File,%20int))
        ///
        /// Required features: "java-io-File", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-io-File", feature = "java-lang-String")))]
        pub fn addFile<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/DropBoxManager", java.flags == PUBLIC, .name == "addFile", .descriptor == "(Ljava/lang/String;Ljava/io/File;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/DropBoxManager\0", "addFile\0", "(Ljava/lang/String;Ljava/io/File;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isTagEnabled](https://developer.android.com/reference/android/os/DropBoxManager.html#isTagEnabled(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn isTagEnabled<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/DropBoxManager", java.flags == PUBLIC, .name == "isTagEnabled", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/DropBoxManager\0", "isTagEnabled\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNextEntry](https://developer.android.com/reference/android/os/DropBoxManager.html#getNextEntry(java.lang.String,%20long))
        ///
        /// Required features: "android-os-DropBoxManager_Entry", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-DropBoxManager_Entry", feature = "java-lang-String")))]
        pub fn getNextEntry<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::DropBoxManager_Entry>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/DropBoxManager", java.flags == PUBLIC, .name == "getNextEntry", .descriptor == "(Ljava/lang/String;J)Landroid/os/DropBoxManager$Entry;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/DropBoxManager\0", "getNextEntry\0", "(Ljava/lang/String;J)Landroid/os/DropBoxManager$Entry;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_DROPBOX_ENTRY_ADDED](https://developer.android.com/reference/android/os/DropBoxManager.html#ACTION_DROPBOX_ENTRY_ADDED)
        pub const ACTION_DROPBOX_ENTRY_ADDED : &'static str = "android.intent.action.DROPBOX_ENTRY_ADDED";

        /// public static final [EXTRA_DROPPED_COUNT](https://developer.android.com/reference/android/os/DropBoxManager.html#EXTRA_DROPPED_COUNT)
        pub const EXTRA_DROPPED_COUNT : &'static str = "android.os.extra.DROPPED_COUNT";

        /// public static final [EXTRA_TAG](https://developer.android.com/reference/android/os/DropBoxManager.html#EXTRA_TAG)
        pub const EXTRA_TAG : &'static str = "tag";

        /// public static final [EXTRA_TIME](https://developer.android.com/reference/android/os/DropBoxManager.html#EXTRA_TIME)
        pub const EXTRA_TIME : &'static str = "time";

        /// public static final [IS_EMPTY](https://developer.android.com/reference/android/os/DropBoxManager.html#IS_EMPTY)
        pub const IS_EMPTY : i32 = 1;

        /// public static final [IS_GZIPPED](https://developer.android.com/reference/android/os/DropBoxManager.html#IS_GZIPPED)
        pub const IS_GZIPPED : i32 = 4;

        /// public static final [IS_TEXT](https://developer.android.com/reference/android/os/DropBoxManager.html#IS_TEXT)
        pub const IS_TEXT : i32 = 2;
    }
}
