// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-FileSystem"))]
__jni_bindgen! {
    /// public class [FileSystem](https://developer.android.com/reference/java/nio/file/FileSystem.html)
    ///
    /// Required feature: java-nio-file-FileSystem
    public class FileSystem ("java/nio/file/FileSystem") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        // // Not emitting: Non-public method
        // /// [FileSystem](https://developer.android.com/reference/java/nio/file/FileSystem.html#FileSystem())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::file::FileSystem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/file/FileSystem", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [provider](https://developer.android.com/reference/java/nio/file/FileSystem.html#provider())
        ///
        /// Required features: "java-nio-file-spi-FileSystemProvider"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-spi-FileSystemProvider")))]
        pub fn provider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::spi::FileSystemProvider>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "provider", .descriptor == "()Ljava/nio/file/spi/FileSystemProvider;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "provider\0", "()Ljava/nio/file/spi/FileSystemProvider;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/nio/file/FileSystem.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isOpen](https://developer.android.com/reference/java/nio/file/FileSystem.html#isOpen())
        pub fn isOpen<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "isOpen", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "isOpen\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isReadOnly](https://developer.android.com/reference/java/nio/file/FileSystem.html#isReadOnly())
        pub fn isReadOnly<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "isReadOnly", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "isReadOnly\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSeparator](https://developer.android.com/reference/java/nio/file/FileSystem.html#getSeparator())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSeparator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "getSeparator", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "getSeparator\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRootDirectories](https://developer.android.com/reference/java/nio/file/FileSystem.html#getRootDirectories())
        ///
        /// Required features: "java-lang-Iterable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Iterable")))]
        pub fn getRootDirectories<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Iterable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "getRootDirectories", .descriptor == "()Ljava/lang/Iterable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "getRootDirectories\0", "()Ljava/lang/Iterable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFileStores](https://developer.android.com/reference/java/nio/file/FileSystem.html#getFileStores())
        ///
        /// Required features: "java-lang-Iterable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Iterable")))]
        pub fn getFileStores<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Iterable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "getFileStores", .descriptor == "()Ljava/lang/Iterable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "getFileStores\0", "()Ljava/lang/Iterable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [supportedFileAttributeViews](https://developer.android.com/reference/java/nio/file/FileSystem.html#supportedFileAttributeViews())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn supportedFileAttributeViews<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "supportedFileAttributeViews", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "supportedFileAttributeViews\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPath](https://developer.android.com/reference/java/nio/file/FileSystem.html#getPath(java.lang.String,%20java.lang.String...))
        ///
        /// Required features: "java-lang-String", "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-file-Path")))]
        pub fn getPath<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | VARARGS | ABSTRACT, .name == "getPath", .descriptor == "(Ljava/lang/String;[Ljava/lang/String;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "getPath\0", "(Ljava/lang/String;[Ljava/lang/String;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPathMatcher](https://developer.android.com/reference/java/nio/file/FileSystem.html#getPathMatcher(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-file-PathMatcher"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-file-PathMatcher")))]
        pub fn getPathMatcher<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::PathMatcher>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "getPathMatcher", .descriptor == "(Ljava/lang/String;)Ljava/nio/file/PathMatcher;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "getPathMatcher\0", "(Ljava/lang/String;)Ljava/nio/file/PathMatcher;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUserPrincipalLookupService](https://developer.android.com/reference/java/nio/file/FileSystem.html#getUserPrincipalLookupService())
        ///
        /// Required features: "java-nio-file-attribute-UserPrincipalLookupService"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-UserPrincipalLookupService")))]
        pub fn getUserPrincipalLookupService<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::attribute::UserPrincipalLookupService>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "getUserPrincipalLookupService", .descriptor == "()Ljava/nio/file/attribute/UserPrincipalLookupService;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "getUserPrincipalLookupService\0", "()Ljava/nio/file/attribute/UserPrincipalLookupService;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newWatchService](https://developer.android.com/reference/java/nio/file/FileSystem.html#newWatchService())
        ///
        /// Required features: "java-nio-file-WatchService"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-WatchService")))]
        pub fn newWatchService<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::WatchService>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/FileSystem", java.flags == PUBLIC | ABSTRACT, .name == "newWatchService", .descriptor == "()Ljava/nio/file/WatchService;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/FileSystem\0", "newWatchService\0", "()Ljava/nio/file/WatchService;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
