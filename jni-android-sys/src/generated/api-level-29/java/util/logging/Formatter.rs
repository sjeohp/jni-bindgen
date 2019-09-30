// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-logging-Formatter"))]
__jni_bindgen! {
    /// public class [Formatter](https://developer.android.com/reference/java/util/logging/Formatter.html)
    ///
    /// Required feature: java-util-logging-Formatter
    public class Formatter ("java/util/logging/Formatter") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Formatter](https://developer.android.com/reference/java/util/logging/Formatter.html#Formatter())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::logging::Formatter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/logging/Formatter", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/logging/Formatter\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [format](https://developer.android.com/reference/java/util/logging/Formatter.html#format(java.util.logging.LogRecord))
        ///
        /// Required features: "java-lang-String", "java-util-logging-LogRecord"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-logging-LogRecord")))]
        pub fn format<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::logging::LogRecord>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/logging/Formatter", java.flags == PUBLIC | ABSTRACT, .name == "format", .descriptor == "(Ljava/util/logging/LogRecord;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/logging/Formatter\0", "format\0", "(Ljava/util/logging/LogRecord;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHead](https://developer.android.com/reference/java/util/logging/Formatter.html#getHead(java.util.logging.Handler))
        ///
        /// Required features: "java-lang-String", "java-util-logging-Handler"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-logging-Handler")))]
        pub fn getHead<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::logging::Handler>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/logging/Formatter", java.flags == PUBLIC, .name == "getHead", .descriptor == "(Ljava/util/logging/Handler;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/logging/Formatter\0", "getHead\0", "(Ljava/util/logging/Handler;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTail](https://developer.android.com/reference/java/util/logging/Formatter.html#getTail(java.util.logging.Handler))
        ///
        /// Required features: "java-lang-String", "java-util-logging-Handler"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-logging-Handler")))]
        pub fn getTail<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::logging::Handler>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/logging/Formatter", java.flags == PUBLIC, .name == "getTail", .descriptor == "(Ljava/util/logging/Handler;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/logging/Formatter\0", "getTail\0", "(Ljava/util/logging/Handler;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [formatMessage](https://developer.android.com/reference/java/util/logging/Formatter.html#formatMessage(java.util.logging.LogRecord))
        ///
        /// Required features: "java-lang-String", "java-util-logging-LogRecord"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-logging-LogRecord")))]
        pub fn formatMessage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::logging::LogRecord>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/logging/Formatter", java.flags == PUBLIC | SYNCRONIZED, .name == "formatMessage", .descriptor == "(Ljava/util/logging/LogRecord;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/logging/Formatter\0", "formatMessage\0", "(Ljava/util/logging/LogRecord;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
