// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-FileObserver"))]
__jni_bindgen! {
    /// public class [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html)
    ///
    /// Required feature: android-os-FileObserver
    public class FileObserver ("android/os/FileObserver") extends crate::java::lang::Object {

        /// [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html#FileObserver(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::FileObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html#FileObserver(java.io.File))
        ///
        /// Required features: "java-io-File"
        #[cfg(any(feature = "all", all(feature = "java-io-File")))]
        pub fn new_File<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::FileObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/File;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "<init>\0", "(Ljava/io/File;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html#FileObserver(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn new_List<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::FileObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "<init>\0", "(Ljava/util/List;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html#FileObserver(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn new_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::FileObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "<init>\0", "(Ljava/lang/String;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html#FileObserver(java.io.File,%20int))
        ///
        /// Required features: "java-io-File"
        #[cfg(any(feature = "all", all(feature = "java-io-File")))]
        pub fn new_File_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::FileObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/File;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "<init>\0", "(Ljava/io/File;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [FileObserver](https://developer.android.com/reference/android/os/FileObserver.html#FileObserver(java.util.List,%20int))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn new_List_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::FileObserver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/List;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "<init>\0", "(Ljava/util/List;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/os/FileObserver.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/FileObserver", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [startWatching](https://developer.android.com/reference/android/os/FileObserver.html#startWatching())
        pub fn startWatching<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "startWatching", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "startWatching\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stopWatching](https://developer.android.com/reference/android/os/FileObserver.html#stopWatching())
        pub fn stopWatching<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC, .name == "stopWatching", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "stopWatching\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onEvent](https://developer.android.com/reference/android/os/FileObserver.html#onEvent(int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onEvent<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/FileObserver", java.flags == PUBLIC | ABSTRACT, .name == "onEvent", .descriptor == "(ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/FileObserver\0", "onEvent\0", "(ILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACCESS](https://developer.android.com/reference/android/os/FileObserver.html#ACCESS)
        pub const ACCESS : i32 = 1;

        /// public static final [ALL_EVENTS](https://developer.android.com/reference/android/os/FileObserver.html#ALL_EVENTS)
        pub const ALL_EVENTS : i32 = 4095;

        /// public static final [ATTRIB](https://developer.android.com/reference/android/os/FileObserver.html#ATTRIB)
        pub const ATTRIB : i32 = 4;

        /// public static final [CLOSE_NOWRITE](https://developer.android.com/reference/android/os/FileObserver.html#CLOSE_NOWRITE)
        pub const CLOSE_NOWRITE : i32 = 16;

        /// public static final [CLOSE_WRITE](https://developer.android.com/reference/android/os/FileObserver.html#CLOSE_WRITE)
        pub const CLOSE_WRITE : i32 = 8;

        /// public static final [CREATE](https://developer.android.com/reference/android/os/FileObserver.html#CREATE)
        pub const CREATE : i32 = 256;

        /// public static final [DELETE](https://developer.android.com/reference/android/os/FileObserver.html#DELETE)
        pub const DELETE : i32 = 512;

        /// public static final [DELETE_SELF](https://developer.android.com/reference/android/os/FileObserver.html#DELETE_SELF)
        pub const DELETE_SELF : i32 = 1024;

        /// public static final [MODIFY](https://developer.android.com/reference/android/os/FileObserver.html#MODIFY)
        pub const MODIFY : i32 = 2;

        /// public static final [MOVED_FROM](https://developer.android.com/reference/android/os/FileObserver.html#MOVED_FROM)
        pub const MOVED_FROM : i32 = 64;

        /// public static final [MOVED_TO](https://developer.android.com/reference/android/os/FileObserver.html#MOVED_TO)
        pub const MOVED_TO : i32 = 128;

        /// public static final [MOVE_SELF](https://developer.android.com/reference/android/os/FileObserver.html#MOVE_SELF)
        pub const MOVE_SELF : i32 = 2048;

        /// public static final [OPEN](https://developer.android.com/reference/android/os/FileObserver.html#OPEN)
        pub const OPEN : i32 = 32;
    }
}
