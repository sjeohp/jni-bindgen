// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-Watchable"))]
__jni_bindgen! {
    /// public interface [Watchable](https://developer.android.com/reference/java/nio/file/Watchable.html)
    ///
    /// Required feature: java-nio-file-Watchable
    public interface Watchable ("java/nio/file/Watchable") extends crate::java::lang::Object {

        /// [register](https://developer.android.com/reference/java/nio/file/Watchable.html#register(java.nio.file.WatchService,%20java.nio.file.WatchEvent.Kind%5B%5D,%20java.nio.file.WatchEvent.Modifier...))
        ///
        /// Required features: "java-nio-file-WatchEvent_Kind", "java-nio-file-WatchEvent_Modifier", "java-nio-file-WatchKey", "java-nio-file-WatchService"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-WatchEvent_Kind", feature = "java-nio-file-WatchEvent_Modifier", feature = "java-nio-file-WatchKey", feature = "java-nio-file-WatchService")))]
        pub fn register_WatchService_Kind_array_Modifier_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::WatchService>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::WatchEvent_Kind, crate::java::lang::Throwable>>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::WatchEvent_Modifier, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::WatchKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Watchable", java.flags == PUBLIC | VARARGS | ABSTRACT, .name == "register", .descriptor == "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;[Ljava/nio/file/WatchEvent$Modifier;)Ljava/nio/file/WatchKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Watchable\0", "register\0", "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;[Ljava/nio/file/WatchEvent$Modifier;)Ljava/nio/file/WatchKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [register](https://developer.android.com/reference/java/nio/file/Watchable.html#register(java.nio.file.WatchService,%20java.nio.file.WatchEvent.Kind...))
        ///
        /// Required features: "java-nio-file-WatchEvent_Kind", "java-nio-file-WatchKey", "java-nio-file-WatchService"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-WatchEvent_Kind", feature = "java-nio-file-WatchKey", feature = "java-nio-file-WatchService")))]
        pub fn register_WatchService_Kind_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::file::WatchService>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::file::WatchEvent_Kind, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::WatchKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/Watchable", java.flags == PUBLIC | VARARGS | ABSTRACT, .name == "register", .descriptor == "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;)Ljava/nio/file/WatchKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/Watchable\0", "register\0", "(Ljava/nio/file/WatchService;[Ljava/nio/file/WatchEvent$Kind;)Ljava/nio/file/WatchKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
