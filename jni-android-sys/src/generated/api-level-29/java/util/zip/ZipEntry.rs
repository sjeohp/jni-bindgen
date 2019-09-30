// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-zip-ZipEntry"))]
__jni_bindgen! {
    /// public class [ZipEntry](https://developer.android.com/reference/java/util/zip/ZipEntry.html)
    ///
    /// Required feature: java-util-zip-ZipEntry
    public class ZipEntry ("java/util/zip/ZipEntry") extends crate::java::lang::Object, implements crate::java::lang::Cloneable {

        /// [ZipEntry](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ZipEntry(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::ZipEntry>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ZipEntry](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ZipEntry(java.util.zip.ZipEntry))
        ///
        /// Required features: "java-util-zip-ZipEntry"
        #[cfg(any(feature = "all", all(feature = "java-util-zip-ZipEntry")))]
        pub fn new_ZipEntry<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::zip::ZipEntry>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::ZipEntry>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/zip/ZipEntry;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "<init>\0", "(Ljava/util/zip/ZipEntry;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setTime(long))
        pub fn setTime<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setTime", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setTime\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getTime())
        pub fn getTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getTime", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getTime\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLastModifiedTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setLastModifiedTime(java.nio.file.attribute.FileTime))
        ///
        /// Required features: "java-nio-file-attribute-FileTime", "java-util-zip-ZipEntry"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-FileTime", feature = "java-util-zip-ZipEntry")))]
        pub fn setLastModifiedTime<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::attribute::FileTime>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::zip::ZipEntry>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setLastModifiedTime", .descriptor == "(Ljava/nio/file/attribute/FileTime;)Ljava/util/zip/ZipEntry;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setLastModifiedTime\0", "(Ljava/nio/file/attribute/FileTime;)Ljava/util/zip/ZipEntry;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLastModifiedTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getLastModifiedTime())
        ///
        /// Required features: "java-nio-file-attribute-FileTime"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-FileTime")))]
        pub fn getLastModifiedTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::attribute::FileTime>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getLastModifiedTime", .descriptor == "()Ljava/nio/file/attribute/FileTime;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getLastModifiedTime\0", "()Ljava/nio/file/attribute/FileTime;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLastAccessTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setLastAccessTime(java.nio.file.attribute.FileTime))
        ///
        /// Required features: "java-nio-file-attribute-FileTime", "java-util-zip-ZipEntry"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-FileTime", feature = "java-util-zip-ZipEntry")))]
        pub fn setLastAccessTime<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::attribute::FileTime>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::zip::ZipEntry>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setLastAccessTime", .descriptor == "(Ljava/nio/file/attribute/FileTime;)Ljava/util/zip/ZipEntry;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setLastAccessTime\0", "(Ljava/nio/file/attribute/FileTime;)Ljava/util/zip/ZipEntry;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLastAccessTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getLastAccessTime())
        ///
        /// Required features: "java-nio-file-attribute-FileTime"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-FileTime")))]
        pub fn getLastAccessTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::attribute::FileTime>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getLastAccessTime", .descriptor == "()Ljava/nio/file/attribute/FileTime;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getLastAccessTime\0", "()Ljava/nio/file/attribute/FileTime;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCreationTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setCreationTime(java.nio.file.attribute.FileTime))
        ///
        /// Required features: "java-nio-file-attribute-FileTime", "java-util-zip-ZipEntry"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-FileTime", feature = "java-util-zip-ZipEntry")))]
        pub fn setCreationTime<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::attribute::FileTime>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::zip::ZipEntry>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setCreationTime", .descriptor == "(Ljava/nio/file/attribute/FileTime;)Ljava/util/zip/ZipEntry;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setCreationTime\0", "(Ljava/nio/file/attribute/FileTime;)Ljava/util/zip/ZipEntry;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCreationTime](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getCreationTime())
        ///
        /// Required features: "java-nio-file-attribute-FileTime"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-FileTime")))]
        pub fn getCreationTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::attribute::FileTime>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getCreationTime", .descriptor == "()Ljava/nio/file/attribute/FileTime;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getCreationTime\0", "()Ljava/nio/file/attribute/FileTime;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSize](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setSize(long))
        pub fn setSize<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setSize", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setSize\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSize](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getSize())
        pub fn getSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getSize", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getSize\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCompressedSize](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getCompressedSize())
        pub fn getCompressedSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getCompressedSize", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getCompressedSize\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCompressedSize](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setCompressedSize(long))
        pub fn setCompressedSize<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setCompressedSize", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setCompressedSize\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCrc](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setCrc(long))
        pub fn setCrc<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setCrc", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setCrc\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCrc](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getCrc())
        pub fn getCrc<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getCrc", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getCrc\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMethod](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setMethod(int))
        pub fn setMethod<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setMethod", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setMethod\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMethod](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getMethod())
        pub fn getMethod<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getMethod", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getMethod\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setExtra](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setExtra(byte%5B%5D))
        pub fn setExtra<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setExtra", .descriptor == "([B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setExtra\0", "([B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtra](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getExtra())
        pub fn getExtra<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getExtra", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getExtra\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setComment](https://developer.android.com/reference/java/util/zip/ZipEntry.html#setComment(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setComment<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "setComment", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "setComment\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getComment](https://developer.android.com/reference/java/util/zip/ZipEntry.html#getComment())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getComment<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "getComment", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "getComment\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDirectory](https://developer.android.com/reference/java/util/zip/ZipEntry.html#isDirectory())
        pub fn isDirectory<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "isDirectory", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "isDirectory\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/zip/ZipEntry.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/util/zip/ZipEntry.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/java/util/zip/ZipEntry.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipEntry", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipEntry\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CENATT](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENATT)
        pub const CENATT : i32 = 36;

        /// public static final [CENATX](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENATX)
        pub const CENATX : i32 = 38;

        /// public static final [CENCOM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENCOM)
        pub const CENCOM : i32 = 32;

        /// public static final [CENCRC](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENCRC)
        pub const CENCRC : i32 = 16;

        /// public static final [CENDSK](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENDSK)
        pub const CENDSK : i32 = 34;

        /// public static final [CENEXT](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENEXT)
        pub const CENEXT : i32 = 30;

        /// public static final [CENFLG](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENFLG)
        pub const CENFLG : i32 = 8;

        /// public static final [CENHDR](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENHDR)
        pub const CENHDR : i32 = 46;

        /// public static final [CENHOW](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENHOW)
        pub const CENHOW : i32 = 10;

        /// public static final [CENLEN](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENLEN)
        pub const CENLEN : i32 = 24;

        /// public static final [CENNAM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENNAM)
        pub const CENNAM : i32 = 28;

        /// public static final [CENOFF](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENOFF)
        pub const CENOFF : i32 = 42;

        /// public static final [CENSIG](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENSIG)
        pub const CENSIG : i64 = 33639248i64;

        /// public static final [CENSIZ](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENSIZ)
        pub const CENSIZ : i32 = 20;

        /// public static final [CENTIM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENTIM)
        pub const CENTIM : i32 = 12;

        /// public static final [CENVEM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENVEM)
        pub const CENVEM : i32 = 4;

        /// public static final [CENVER](https://developer.android.com/reference/java/util/zip/ZipEntry.html#CENVER)
        pub const CENVER : i32 = 6;

        /// public static final [DEFLATED](https://developer.android.com/reference/java/util/zip/ZipEntry.html#DEFLATED)
        pub const DEFLATED : i32 = 8;

        /// public static final [ENDCOM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDCOM)
        pub const ENDCOM : i32 = 20;

        /// public static final [ENDHDR](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDHDR)
        pub const ENDHDR : i32 = 22;

        /// public static final [ENDOFF](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDOFF)
        pub const ENDOFF : i32 = 16;

        /// public static final [ENDSIG](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDSIG)
        pub const ENDSIG : i64 = 101010256i64;

        /// public static final [ENDSIZ](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDSIZ)
        pub const ENDSIZ : i32 = 12;

        /// public static final [ENDSUB](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDSUB)
        pub const ENDSUB : i32 = 8;

        /// public static final [ENDTOT](https://developer.android.com/reference/java/util/zip/ZipEntry.html#ENDTOT)
        pub const ENDTOT : i32 = 10;

        /// public static final [EXTCRC](https://developer.android.com/reference/java/util/zip/ZipEntry.html#EXTCRC)
        pub const EXTCRC : i32 = 4;

        /// public static final [EXTHDR](https://developer.android.com/reference/java/util/zip/ZipEntry.html#EXTHDR)
        pub const EXTHDR : i32 = 16;

        /// public static final [EXTLEN](https://developer.android.com/reference/java/util/zip/ZipEntry.html#EXTLEN)
        pub const EXTLEN : i32 = 12;

        /// public static final [EXTSIG](https://developer.android.com/reference/java/util/zip/ZipEntry.html#EXTSIG)
        pub const EXTSIG : i64 = 134695760i64;

        /// public static final [EXTSIZ](https://developer.android.com/reference/java/util/zip/ZipEntry.html#EXTSIZ)
        pub const EXTSIZ : i32 = 8;

        /// public static final [LOCCRC](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCCRC)
        pub const LOCCRC : i32 = 14;

        /// public static final [LOCEXT](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCEXT)
        pub const LOCEXT : i32 = 28;

        /// public static final [LOCFLG](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCFLG)
        pub const LOCFLG : i32 = 6;

        /// public static final [LOCHDR](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCHDR)
        pub const LOCHDR : i32 = 30;

        /// public static final [LOCHOW](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCHOW)
        pub const LOCHOW : i32 = 8;

        /// public static final [LOCLEN](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCLEN)
        pub const LOCLEN : i32 = 22;

        /// public static final [LOCNAM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCNAM)
        pub const LOCNAM : i32 = 26;

        /// public static final [LOCSIG](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCSIG)
        pub const LOCSIG : i64 = 67324752i64;

        /// public static final [LOCSIZ](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCSIZ)
        pub const LOCSIZ : i32 = 18;

        /// public static final [LOCTIM](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCTIM)
        pub const LOCTIM : i32 = 10;

        /// public static final [LOCVER](https://developer.android.com/reference/java/util/zip/ZipEntry.html#LOCVER)
        pub const LOCVER : i32 = 4;

        /// public static final [STORED](https://developer.android.com/reference/java/util/zip/ZipEntry.html#STORED)
        pub const STORED : i32 = 0;
    }
}
