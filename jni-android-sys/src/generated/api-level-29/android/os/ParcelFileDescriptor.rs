// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-ParcelFileDescriptor"))]
__jni_bindgen! {
    /// public class [ParcelFileDescriptor](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html)
    ///
    /// Required feature: android-os-ParcelFileDescriptor
    public class ParcelFileDescriptor ("android/os/ParcelFileDescriptor") extends crate::java::lang::Object, implements crate::android::os::Parcelable, crate::java::io::Closeable {

        /// [ParcelFileDescriptor](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#ParcelFileDescriptor(android.os.ParcelFileDescriptor))
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::ParcelFileDescriptor>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/os/ParcelFileDescriptor;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "<init>\0", "(Landroid/os/ParcelFileDescriptor;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [open](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#open(java.io.File,%20int))
        ///
        /// Required features: "android-os-ParcelFileDescriptor", "java-io-File"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor", feature = "java-io-File")))]
        pub fn open_File_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "open", .descriptor == "(Ljava/io/File;I)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "open\0", "(Ljava/io/File;I)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [open](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#open(java.io.File,%20int,%20android.os.Handler,%20android.os.ParcelFileDescriptor.OnCloseListener))
        ///
        /// Required features: "android-os-Handler", "android-os-ParcelFileDescriptor", "android-os-ParcelFileDescriptor_OnCloseListener", "java-io-File"
        #[cfg(any(feature = "all", all(feature = "android-os-Handler", feature = "android-os-ParcelFileDescriptor", feature = "android-os-ParcelFileDescriptor_OnCloseListener", feature = "java-io-File")))]
        pub fn open_File_int_Handler_OnCloseListener<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::ParcelFileDescriptor_OnCloseListener>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "open", .descriptor == "(Ljava/io/File;ILandroid/os/Handler;Landroid/os/ParcelFileDescriptor$OnCloseListener;)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "open\0", "(Ljava/io/File;ILandroid/os/Handler;Landroid/os/ParcelFileDescriptor$OnCloseListener;)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dup](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#dup(java.io.FileDescriptor))
        ///
        /// Required features: "android-os-ParcelFileDescriptor", "java-io-FileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor", feature = "java-io-FileDescriptor")))]
        pub fn dup_FileDescriptor<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "dup", .descriptor == "(Ljava/io/FileDescriptor;)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "dup\0", "(Ljava/io/FileDescriptor;)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dup](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#dup())
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn dup<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "dup", .descriptor == "()Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "dup\0", "()Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fromFd](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#fromFd(int))
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn fromFd<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "fromFd", .descriptor == "(I)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "fromFd\0", "(I)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [adoptFd](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#adoptFd(int))
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn adoptFd<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "adoptFd", .descriptor == "(I)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "adoptFd\0", "(I)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fromSocket](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#fromSocket(java.net.Socket))
        ///
        /// Required features: "android-os-ParcelFileDescriptor", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor", feature = "java-net-Socket")))]
        pub fn fromSocket<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "fromSocket", .descriptor == "(Ljava/net/Socket;)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "fromSocket\0", "(Ljava/net/Socket;)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fromDatagramSocket](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#fromDatagramSocket(java.net.DatagramSocket))
        ///
        /// Required features: "android-os-ParcelFileDescriptor", "java-net-DatagramSocket"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor", feature = "java-net-DatagramSocket")))]
        pub fn fromDatagramSocket<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::DatagramSocket>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "fromDatagramSocket", .descriptor == "(Ljava/net/DatagramSocket;)Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "fromDatagramSocket\0", "(Ljava/net/DatagramSocket;)Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createPipe](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#createPipe())
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn createPipe<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::os::ParcelFileDescriptor, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "createPipe", .descriptor == "()[Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "createPipe\0", "()[Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createReliablePipe](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#createReliablePipe())
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn createReliablePipe<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::os::ParcelFileDescriptor, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "createReliablePipe", .descriptor == "()[Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "createReliablePipe\0", "()[Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocketPair](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#createSocketPair())
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn createSocketPair<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::os::ParcelFileDescriptor, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "createSocketPair", .descriptor == "()[Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "createSocketPair\0", "()[Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createReliableSocketPair](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#createReliableSocketPair())
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn createReliableSocketPair<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::os::ParcelFileDescriptor, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "createReliableSocketPair", .descriptor == "()[Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "createReliableSocketPair\0", "()[Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseMode](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#parseMode(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn parseMode<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC | STATIC, .name == "parseMode", .descriptor == "(Ljava/lang/String;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/ParcelFileDescriptor\0", "parseMode\0", "(Ljava/lang/String;)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFileDescriptor](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#getFileDescriptor())
        ///
        /// Required features: "java-io-FileDescriptor"
        #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor")))]
        pub fn getFileDescriptor<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::FileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "getFileDescriptor", .descriptor == "()Ljava/io/FileDescriptor;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "getFileDescriptor\0", "()Ljava/io/FileDescriptor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStatSize](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#getStatSize())
        pub fn getStatSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "getStatSize", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "getStatSize\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFd](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#getFd())
        pub fn getFd<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "getFd", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "getFd\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [detachFd](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#detachFd())
        pub fn detachFd<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "detachFd", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "detachFd\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [closeWithError](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#closeWithError(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn closeWithError<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "closeWithError", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "closeWithError\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [canDetectErrors](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#canDetectErrors())
        pub fn canDetectErrors<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "canDetectErrors", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "canDetectErrors\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [checkError](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#checkError())
        pub fn checkError<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "checkError", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "checkError\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/ParcelFileDescriptor", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [describeContents](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ParcelFileDescriptor", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ParcelFileDescriptor\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/os/ParcelFileDescriptor\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [MODE_APPEND](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_APPEND)
        pub const MODE_APPEND : i32 = 33554432;

        /// public static final [MODE_CREATE](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_CREATE)
        pub const MODE_CREATE : i32 = 134217728;

        /// public static final [MODE_READ_ONLY](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_READ_ONLY)
        pub const MODE_READ_ONLY : i32 = 268435456;

        /// public static final [MODE_READ_WRITE](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_READ_WRITE)
        pub const MODE_READ_WRITE : i32 = 805306368;

        /// public static final [MODE_TRUNCATE](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_TRUNCATE)
        pub const MODE_TRUNCATE : i32 = 67108864;

        /// public static final [MODE_WORLD_READABLE](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_WORLD_READABLE)
        #[deprecated] pub const MODE_WORLD_READABLE : i32 = 1;

        /// public static final [MODE_WORLD_WRITEABLE](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_WORLD_WRITEABLE)
        #[deprecated] pub const MODE_WORLD_WRITEABLE : i32 = 2;

        /// public static final [MODE_WRITE_ONLY](https://developer.android.com/reference/android/os/ParcelFileDescriptor.html#MODE_WRITE_ONLY)
        pub const MODE_WRITE_ONLY : i32 = 536870912;
    }
}
