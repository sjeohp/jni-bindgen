// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-Path"))]
__jni_bindgen! {
    /// public interface [Path](https://developer.android.com/reference/java/nio/file/Path.html)
    ///
    /// Required feature: java-nio-file-Path
    public interface Path ("java/nio/file/Path") extends crate::java::lang::Object, implements crate::java::lang::Comparable, crate::java::lang::Iterable, crate::java::nio::file::Watchable {

        /// [getFileSystem](https://developer.android.com/reference/java/nio/file/Path.html#getFileSystem())
        ///
        /// Required features: "java-nio-file-FileSystem"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-FileSystem")))]
        pub fn getFileSystem<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::FileSystem>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "getFileSystem", .descriptor == "()Ljava/nio/file/FileSystem;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "getFileSystem\0", "()Ljava/nio/file/FileSystem;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isAbsolute](https://developer.android.com/reference/java/nio/file/Path.html#isAbsolute())
        pub fn isAbsolute<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "isAbsolute", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "isAbsolute\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRoot](https://developer.android.com/reference/java/nio/file/Path.html#getRoot())
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn getRoot<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "getRoot", .descriptor == "()Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "getRoot\0", "()Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFileName](https://developer.android.com/reference/java/nio/file/Path.html#getFileName())
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn getFileName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "getFileName", .descriptor == "()Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "getFileName\0", "()Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParent](https://developer.android.com/reference/java/nio/file/Path.html#getParent())
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn getParent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "getParent", .descriptor == "()Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "getParent\0", "()Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNameCount](https://developer.android.com/reference/java/nio/file/Path.html#getNameCount())
        pub fn getNameCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "getNameCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "getNameCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/java/nio/file/Path.html#getName(int))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn getName<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "getName", .descriptor == "(I)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "getName\0", "(I)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [subpath](https://developer.android.com/reference/java/nio/file/Path.html#subpath(int,%20int))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn subpath<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "subpath", .descriptor == "(II)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "subpath\0", "(II)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startsWith](https://developer.android.com/reference/java/nio/file/Path.html#startsWith(java.nio.file.Path))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn startsWith_Path<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::Path>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "startsWith", .descriptor == "(Ljava/nio/file/Path;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "startsWith\0", "(Ljava/nio/file/Path;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startsWith](https://developer.android.com/reference/java/nio/file/Path.html#startsWith(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn startsWith_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "startsWith", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "startsWith\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endsWith](https://developer.android.com/reference/java/nio/file/Path.html#endsWith(java.nio.file.Path))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn endsWith_Path<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::Path>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "endsWith", .descriptor == "(Ljava/nio/file/Path;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "endsWith\0", "(Ljava/nio/file/Path;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endsWith](https://developer.android.com/reference/java/nio/file/Path.html#endsWith(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn endsWith_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "endsWith", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "endsWith\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [normalize](https://developer.android.com/reference/java/nio/file/Path.html#normalize())
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn normalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "normalize", .descriptor == "()Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "normalize\0", "()Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [resolve](https://developer.android.com/reference/java/nio/file/Path.html#resolve(java.nio.file.Path))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn resolve_Path<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::Path>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "resolve", .descriptor == "(Ljava/nio/file/Path;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "resolve\0", "(Ljava/nio/file/Path;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [resolve](https://developer.android.com/reference/java/nio/file/Path.html#resolve(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-file-Path")))]
        pub fn resolve_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "resolve", .descriptor == "(Ljava/lang/String;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "resolve\0", "(Ljava/lang/String;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [resolveSibling](https://developer.android.com/reference/java/nio/file/Path.html#resolveSibling(java.nio.file.Path))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn resolveSibling_Path<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::Path>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "resolveSibling", .descriptor == "(Ljava/nio/file/Path;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "resolveSibling\0", "(Ljava/nio/file/Path;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [resolveSibling](https://developer.android.com/reference/java/nio/file/Path.html#resolveSibling(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-file-Path")))]
        pub fn resolveSibling_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "resolveSibling", .descriptor == "(Ljava/lang/String;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "resolveSibling\0", "(Ljava/lang/String;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [relativize](https://developer.android.com/reference/java/nio/file/Path.html#relativize(java.nio.file.Path))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn relativize<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::Path>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "relativize", .descriptor == "(Ljava/nio/file/Path;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "relativize\0", "(Ljava/nio/file/Path;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toUri](https://developer.android.com/reference/java/nio/file/Path.html#toUri())
        ///
        /// Required features: "java-net-URI"
        #[cfg(any(feature = "all", all(feature = "java-net-URI")))]
        pub fn toUri<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::URI>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "toUri", .descriptor == "()Ljava/net/URI;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "toUri\0", "()Ljava/net/URI;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toAbsolutePath](https://developer.android.com/reference/java/nio/file/Path.html#toAbsolutePath())
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn toAbsolutePath<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "toAbsolutePath", .descriptor == "()Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "toAbsolutePath\0", "()Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toRealPath](https://developer.android.com/reference/java/nio/file/Path.html#toRealPath(java.nio.file.LinkOption...))
        ///
        /// Required features: "java-nio-file-LinkOption", "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-LinkOption", feature = "java-nio-file-Path")))]
        pub fn toRealPath<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::LinkOption, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | VARARGS | ABSTRACT, .name == "toRealPath", .descriptor == "([Ljava/nio/file/LinkOption;)Ljava/nio/file/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "toRealPath\0", "([Ljava/nio/file/LinkOption;)Ljava/nio/file/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toFile](https://developer.android.com/reference/java/nio/file/Path.html#toFile())
        ///
        /// Required features: "java-io-File"
        #[cfg(any(feature = "all", all(feature = "java-io-File")))]
        pub fn toFile<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::File>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "toFile", .descriptor == "()Ljava/io/File;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "toFile\0", "()Ljava/io/File;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [register](https://developer.android.com/reference/java/nio/file/Path.html#register(java.nio.file.WatchService,%20java.nio.file.WatchEvent.Kind%5B%5D,%20java.nio.file.WatchEvent.Modifier...))
        ///
        /// Required features: "java-nio-file-WatchEvent_Kind", "java-nio-file-WatchEvent_Modifier", "java-nio-file-WatchKey", "java-nio-file-WatchService"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-WatchEvent_Kind", feature = "java-nio-file-WatchEvent_Modifier", feature = "java-nio-file-WatchKey", feature = "java-nio-file-WatchService")))]
        pub fn register_WatchService_Kind_array_Modifier_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::WatchService>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::WatchEvent_Kind, crate::java::lang::Throwable>>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::WatchEvent_Modifier, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::WatchKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | VARARGS | ABSTRACT, .name == "register", .descriptor == "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;[Ljava/nio/file/WatchEvent$Modifier;)Ljava/nio/file/WatchKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "register\0", "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;[Ljava/nio/file/WatchEvent$Modifier;)Ljava/nio/file/WatchKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [register](https://developer.android.com/reference/java/nio/file/Path.html#register(java.nio.file.WatchService,%20java.nio.file.WatchEvent.Kind...))
        ///
        /// Required features: "java-nio-file-WatchEvent_Kind", "java-nio-file-WatchKey", "java-nio-file-WatchService"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-WatchEvent_Kind", feature = "java-nio-file-WatchKey", feature = "java-nio-file-WatchService")))]
        pub fn register_WatchService_Kind_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::WatchService>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::WatchEvent_Kind, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::WatchKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | VARARGS | ABSTRACT, .name == "register", .descriptor == "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;)Ljava/nio/file/WatchKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "register\0", "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;)Ljava/nio/file/WatchKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/nio/file/Path.html#iterator())
        ///
        /// Required features: "java-util-Iterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator")))]
        pub fn iterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Iterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "iterator", .descriptor == "()Ljava/util/Iterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "iterator\0", "()Ljava/util/Iterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareTo](https://developer.android.com/reference/java/nio/file/Path.html#compareTo(java.nio.file.Path))
        ///
        /// Required features: "java-nio-file-Path"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-Path")))]
        pub fn compareTo_Path<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::Path>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "compareTo", .descriptor == "(Ljava/nio/file/Path;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "compareTo\0", "(Ljava/nio/file/Path;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/nio/file/Path.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/nio/file/Path.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/nio/file/Path.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Path", java.flags == PUBLIC | ABSTRACT, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [compareTo](https://developer.android.com/reference/java/nio/file/Path.html#compareTo(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn compareTo_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/file/Path", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "compareTo", .descriptor == "(Ljava/lang/Object;)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Path\0", "compareTo\0", "(Ljava/lang/Object;)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
