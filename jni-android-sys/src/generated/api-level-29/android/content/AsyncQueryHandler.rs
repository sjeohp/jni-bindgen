// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-AsyncQueryHandler"))]
__jni_bindgen! {
    /// public class [AsyncQueryHandler](https://developer.android.com/reference/android/content/AsyncQueryHandler.html)
    ///
    /// Required feature: android-content-AsyncQueryHandler
    public class AsyncQueryHandler ("android/content/AsyncQueryHandler") extends crate::android::os::Handler {

        /// [AsyncQueryHandler](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#AsyncQueryHandler(android.content.ContentResolver))
        ///
        /// Required features: "android-content-ContentResolver"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentResolver")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentResolver>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::AsyncQueryHandler>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/ContentResolver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "<init>\0", "(Landroid/content/ContentResolver;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [createHandler](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#createHandler(android.os.Looper))
        // ///
        // /// Required features: "android-os-Handler", "android-os-Looper"
        // #[cfg(any(feature = "all", all(feature = "android-os-Handler", feature = "android-os-Looper")))]
        // fn createHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Looper>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Handler>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/AsyncQueryHandler", java.flags == PROTECTED, .name == "createHandler", .descriptor == "(Landroid/os/Looper;)Landroid/os/Handler;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "createHandler\0", "(Landroid/os/Looper;)Landroid/os/Handler;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [startQuery](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#startQuery(int,%20java.lang.Object,%20android.net.Uri,%20java.lang.String%5B%5D,%20java.lang.String,%20java.lang.String%5B%5D,%20java.lang.String))
        ///
        /// Required features: "android-net-Uri", "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn startQuery<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC, .name == "startQuery", .descriptor == "(ILjava/lang/Object;Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "startQuery\0", "(ILjava/lang/Object;Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cancelOperation](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#cancelOperation(int))
        pub fn cancelOperation<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC | FINAL, .name == "cancelOperation", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "cancelOperation\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startInsert](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#startInsert(int,%20java.lang.Object,%20android.net.Uri,%20android.content.ContentValues))
        ///
        /// Required features: "android-content-ContentValues", "android-net-Uri", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentValues", feature = "android-net-Uri", feature = "java-lang-Object")))]
        pub fn startInsert<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC | FINAL, .name == "startInsert", .descriptor == "(ILjava/lang/Object;Landroid/net/Uri;Landroid/content/ContentValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "startInsert\0", "(ILjava/lang/Object;Landroid/net/Uri;Landroid/content/ContentValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startUpdate](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#startUpdate(int,%20java.lang.Object,%20android.net.Uri,%20android.content.ContentValues,%20java.lang.String,%20java.lang.String%5B%5D))
        ///
        /// Required features: "android-content-ContentValues", "android-net-Uri", "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentValues", feature = "android-net-Uri", feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn startUpdate<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentValues>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC | FINAL, .name == "startUpdate", .descriptor == "(ILjava/lang/Object;Landroid/net/Uri;Landroid/content/ContentValues;Ljava/lang/String;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "startUpdate\0", "(ILjava/lang/Object;Landroid/net/Uri;Landroid/content/ContentValues;Ljava/lang/String;[Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startDelete](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#startDelete(int,%20java.lang.Object,%20android.net.Uri,%20java.lang.String,%20java.lang.String%5B%5D))
        ///
        /// Required features: "android-net-Uri", "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn startDelete<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC | FINAL, .name == "startDelete", .descriptor == "(ILjava/lang/Object;Landroid/net/Uri;Ljava/lang/String;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "startDelete\0", "(ILjava/lang/Object;Landroid/net/Uri;Ljava/lang/String;[Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onQueryComplete](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#onQueryComplete(int,%20java.lang.Object,%20android.database.Cursor))
        // ///
        // /// Required features: "android-database-Cursor", "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "android-database-Cursor", feature = "java-lang-Object")))]
        // fn onQueryComplete<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/AsyncQueryHandler", java.flags == PROTECTED, .name == "onQueryComplete", .descriptor == "(ILjava/lang/Object;Landroid/database/Cursor;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "onQueryComplete\0", "(ILjava/lang/Object;Landroid/database/Cursor;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onInsertComplete](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#onInsertComplete(int,%20java.lang.Object,%20android.net.Uri))
        // ///
        // /// Required features: "android-net-Uri", "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "java-lang-Object")))]
        // fn onInsertComplete<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/AsyncQueryHandler", java.flags == PROTECTED, .name == "onInsertComplete", .descriptor == "(ILjava/lang/Object;Landroid/net/Uri;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "onInsertComplete\0", "(ILjava/lang/Object;Landroid/net/Uri;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onUpdateComplete](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#onUpdateComplete(int,%20java.lang.Object,%20int))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn onUpdateComplete<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/AsyncQueryHandler", java.flags == PROTECTED, .name == "onUpdateComplete", .descriptor == "(ILjava/lang/Object;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "onUpdateComplete\0", "(ILjava/lang/Object;I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onDeleteComplete](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#onDeleteComplete(int,%20java.lang.Object,%20int))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn onDeleteComplete<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/AsyncQueryHandler", java.flags == PROTECTED, .name == "onDeleteComplete", .descriptor == "(ILjava/lang/Object;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "onDeleteComplete\0", "(ILjava/lang/Object;I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [handleMessage](https://developer.android.com/reference/android/content/AsyncQueryHandler.html#handleMessage(android.os.Message))
        ///
        /// Required features: "android-os-Message"
        #[cfg(any(feature = "all", all(feature = "android-os-Message")))]
        pub fn handleMessage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Message>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/AsyncQueryHandler", java.flags == PUBLIC, .name == "handleMessage", .descriptor == "(Landroid/os/Message;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/AsyncQueryHandler\0", "handleMessage\0", "(Landroid/os/Message;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
