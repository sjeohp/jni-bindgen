// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-HttpURLConnection"))]
__jni_bindgen! {
    /// public class [HttpURLConnection](https://developer.android.com/reference/java/net/HttpURLConnection.html)
    ///
    /// Required feature: java-net-HttpURLConnection
    public class HttpURLConnection ("java/net/HttpURLConnection") extends crate::java::net::URLConnection {

        // // Not emitting: Non-public method
        // /// [HttpURLConnection](https://developer.android.com/reference/java/net/HttpURLConnection.html#HttpURLConnection(java.net.URL))
        // ///
        // /// Required features: "java-net-URL"
        // #[cfg(any(feature = "all", all(feature = "java-net-URL")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URL>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::HttpURLConnection>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/HttpURLConnection", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/net/URL;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "<init>\0", "(Ljava/net/URL;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getHeaderFieldKey](https://developer.android.com/reference/java/net/HttpURLConnection.html#getHeaderFieldKey(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getHeaderFieldKey<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getHeaderFieldKey", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getHeaderFieldKey\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFixedLengthStreamingMode](https://developer.android.com/reference/java/net/HttpURLConnection.html#setFixedLengthStreamingMode(int))
        pub fn setFixedLengthStreamingMode_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "setFixedLengthStreamingMode", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "setFixedLengthStreamingMode\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFixedLengthStreamingMode](https://developer.android.com/reference/java/net/HttpURLConnection.html#setFixedLengthStreamingMode(long))
        pub fn setFixedLengthStreamingMode_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "setFixedLengthStreamingMode", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "setFixedLengthStreamingMode\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setChunkedStreamingMode](https://developer.android.com/reference/java/net/HttpURLConnection.html#setChunkedStreamingMode(int))
        pub fn setChunkedStreamingMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "setChunkedStreamingMode", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "setChunkedStreamingMode\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHeaderField](https://developer.android.com/reference/java/net/HttpURLConnection.html#getHeaderField(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getHeaderField<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getHeaderField", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getHeaderField\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFollowRedirects](https://developer.android.com/reference/java/net/HttpURLConnection.html#setFollowRedirects(boolean))
        pub fn setFollowRedirects<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC | STATIC, .name == "setFollowRedirects", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/HttpURLConnection\0", "setFollowRedirects\0", "(Z)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFollowRedirects](https://developer.android.com/reference/java/net/HttpURLConnection.html#getFollowRedirects())
        pub fn getFollowRedirects<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC | STATIC, .name == "getFollowRedirects", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/HttpURLConnection\0", "getFollowRedirects\0", "()Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInstanceFollowRedirects](https://developer.android.com/reference/java/net/HttpURLConnection.html#setInstanceFollowRedirects(boolean))
        pub fn setInstanceFollowRedirects<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "setInstanceFollowRedirects", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "setInstanceFollowRedirects\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstanceFollowRedirects](https://developer.android.com/reference/java/net/HttpURLConnection.html#getInstanceFollowRedirects())
        pub fn getInstanceFollowRedirects<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getInstanceFollowRedirects", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getInstanceFollowRedirects\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRequestMethod](https://developer.android.com/reference/java/net/HttpURLConnection.html#setRequestMethod(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setRequestMethod<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "setRequestMethod", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "setRequestMethod\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRequestMethod](https://developer.android.com/reference/java/net/HttpURLConnection.html#getRequestMethod())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getRequestMethod<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getRequestMethod", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getRequestMethod\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResponseCode](https://developer.android.com/reference/java/net/HttpURLConnection.html#getResponseCode())
        pub fn getResponseCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getResponseCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getResponseCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResponseMessage](https://developer.android.com/reference/java/net/HttpURLConnection.html#getResponseMessage())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getResponseMessage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getResponseMessage", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getResponseMessage\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHeaderFieldDate](https://developer.android.com/reference/java/net/HttpURLConnection.html#getHeaderFieldDate(java.lang.String,%20long))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getHeaderFieldDate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getHeaderFieldDate", .descriptor == "(Ljava/lang/String;J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getHeaderFieldDate\0", "(Ljava/lang/String;J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disconnect](https://developer.android.com/reference/java/net/HttpURLConnection.html#disconnect())
        pub fn disconnect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC | ABSTRACT, .name == "disconnect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "disconnect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [usingProxy](https://developer.android.com/reference/java/net/HttpURLConnection.html#usingProxy())
        pub fn usingProxy<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC | ABSTRACT, .name == "usingProxy", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "usingProxy\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPermission](https://developer.android.com/reference/java/net/HttpURLConnection.html#getPermission())
        ///
        /// Required features: "java-security-Permission"
        #[cfg(any(feature = "all", all(feature = "java-security-Permission")))]
        pub fn getPermission<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Permission>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getPermission", .descriptor == "()Ljava/security/Permission;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getPermission\0", "()Ljava/security/Permission;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getErrorStream](https://developer.android.com/reference/java/net/HttpURLConnection.html#getErrorStream())
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn getErrorStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::InputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/HttpURLConnection", java.flags == PUBLIC, .name == "getErrorStream", .descriptor == "()Ljava/io/InputStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/HttpURLConnection\0", "getErrorStream\0", "()Ljava/io/InputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [HTTP_ACCEPTED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_ACCEPTED)
        pub const HTTP_ACCEPTED : i32 = 202;

        /// public static final [HTTP_BAD_GATEWAY](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_BAD_GATEWAY)
        pub const HTTP_BAD_GATEWAY : i32 = 502;

        /// public static final [HTTP_BAD_METHOD](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_BAD_METHOD)
        pub const HTTP_BAD_METHOD : i32 = 405;

        /// public static final [HTTP_BAD_REQUEST](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_BAD_REQUEST)
        pub const HTTP_BAD_REQUEST : i32 = 400;

        /// public static final [HTTP_CLIENT_TIMEOUT](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_CLIENT_TIMEOUT)
        pub const HTTP_CLIENT_TIMEOUT : i32 = 408;

        /// public static final [HTTP_CONFLICT](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_CONFLICT)
        pub const HTTP_CONFLICT : i32 = 409;

        /// public static final [HTTP_CREATED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_CREATED)
        pub const HTTP_CREATED : i32 = 201;

        /// public static final [HTTP_ENTITY_TOO_LARGE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_ENTITY_TOO_LARGE)
        pub const HTTP_ENTITY_TOO_LARGE : i32 = 413;

        /// public static final [HTTP_FORBIDDEN](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_FORBIDDEN)
        pub const HTTP_FORBIDDEN : i32 = 403;

        /// public static final [HTTP_GATEWAY_TIMEOUT](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_GATEWAY_TIMEOUT)
        pub const HTTP_GATEWAY_TIMEOUT : i32 = 504;

        /// public static final [HTTP_GONE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_GONE)
        pub const HTTP_GONE : i32 = 410;

        /// public static final [HTTP_INTERNAL_ERROR](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_INTERNAL_ERROR)
        pub const HTTP_INTERNAL_ERROR : i32 = 500;

        /// public static final [HTTP_LENGTH_REQUIRED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_LENGTH_REQUIRED)
        pub const HTTP_LENGTH_REQUIRED : i32 = 411;

        /// public static final [HTTP_MOVED_PERM](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_MOVED_PERM)
        pub const HTTP_MOVED_PERM : i32 = 301;

        /// public static final [HTTP_MOVED_TEMP](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_MOVED_TEMP)
        pub const HTTP_MOVED_TEMP : i32 = 302;

        /// public static final [HTTP_MULT_CHOICE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_MULT_CHOICE)
        pub const HTTP_MULT_CHOICE : i32 = 300;

        /// public static final [HTTP_NOT_ACCEPTABLE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_NOT_ACCEPTABLE)
        pub const HTTP_NOT_ACCEPTABLE : i32 = 406;

        /// public static final [HTTP_NOT_AUTHORITATIVE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_NOT_AUTHORITATIVE)
        pub const HTTP_NOT_AUTHORITATIVE : i32 = 203;

        /// public static final [HTTP_NOT_FOUND](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_NOT_FOUND)
        pub const HTTP_NOT_FOUND : i32 = 404;

        /// public static final [HTTP_NOT_IMPLEMENTED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_NOT_IMPLEMENTED)
        pub const HTTP_NOT_IMPLEMENTED : i32 = 501;

        /// public static final [HTTP_NOT_MODIFIED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_NOT_MODIFIED)
        pub const HTTP_NOT_MODIFIED : i32 = 304;

        /// public static final [HTTP_NO_CONTENT](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_NO_CONTENT)
        pub const HTTP_NO_CONTENT : i32 = 204;

        /// public static final [HTTP_OK](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_OK)
        pub const HTTP_OK : i32 = 200;

        /// public static final [HTTP_PARTIAL](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_PARTIAL)
        pub const HTTP_PARTIAL : i32 = 206;

        /// public static final [HTTP_PAYMENT_REQUIRED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_PAYMENT_REQUIRED)
        pub const HTTP_PAYMENT_REQUIRED : i32 = 402;

        /// public static final [HTTP_PRECON_FAILED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_PRECON_FAILED)
        pub const HTTP_PRECON_FAILED : i32 = 412;

        /// public static final [HTTP_PROXY_AUTH](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_PROXY_AUTH)
        pub const HTTP_PROXY_AUTH : i32 = 407;

        /// public static final [HTTP_REQ_TOO_LONG](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_REQ_TOO_LONG)
        pub const HTTP_REQ_TOO_LONG : i32 = 414;

        /// public static final [HTTP_RESET](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_RESET)
        pub const HTTP_RESET : i32 = 205;

        /// public static final [HTTP_SEE_OTHER](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_SEE_OTHER)
        pub const HTTP_SEE_OTHER : i32 = 303;

        /// public static final [HTTP_SERVER_ERROR](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_SERVER_ERROR)
        #[deprecated] pub const HTTP_SERVER_ERROR : i32 = 500;

        /// public static final [HTTP_UNAUTHORIZED](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_UNAUTHORIZED)
        pub const HTTP_UNAUTHORIZED : i32 = 401;

        /// public static final [HTTP_UNAVAILABLE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_UNAVAILABLE)
        pub const HTTP_UNAVAILABLE : i32 = 503;

        /// public static final [HTTP_UNSUPPORTED_TYPE](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_UNSUPPORTED_TYPE)
        pub const HTTP_UNSUPPORTED_TYPE : i32 = 415;

        /// public static final [HTTP_USE_PROXY](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_USE_PROXY)
        pub const HTTP_USE_PROXY : i32 = 305;

        /// public static final [HTTP_VERSION](https://developer.android.com/reference/java/net/HttpURLConnection.html#HTTP_VERSION)
        pub const HTTP_VERSION : i32 = 505;

        // // Not emitting: Non-public field
        // /// **get** protected [chunkLength](https://developer.android.com/reference/java/net/HttpURLConnection.html#chunkLength)
        // pub fn chunkLength<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "chunkLength\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [chunkLength](https://developer.android.com/reference/java/net/HttpURLConnection.html#chunkLength)
        // pub fn set_chunkLength<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "chunkLength\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [fixedContentLength](https://developer.android.com/reference/java/net/HttpURLConnection.html#fixedContentLength)
        // pub fn fixedContentLength<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "fixedContentLength\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [fixedContentLength](https://developer.android.com/reference/java/net/HttpURLConnection.html#fixedContentLength)
        // pub fn set_fixedContentLength<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "fixedContentLength\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [fixedContentLengthLong](https://developer.android.com/reference/java/net/HttpURLConnection.html#fixedContentLengthLong)
        // pub fn fixedContentLengthLong<'env>(&'env self) -> i64 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "fixedContentLengthLong\0", "J\0");
        //         env.get_long_field(class, field)
        //     }
        // }

        // /// **set** protected [fixedContentLengthLong](https://developer.android.com/reference/java/net/HttpURLConnection.html#fixedContentLengthLong)
        // pub fn set_fixedContentLengthLong<'env>(&'env self, value: i64) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "fixedContentLengthLong\0", "J\0");
        //         env.set_long_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [instanceFollowRedirects](https://developer.android.com/reference/java/net/HttpURLConnection.html#instanceFollowRedirects)
        // pub fn instanceFollowRedirects<'env>(&'env self) -> bool {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "instanceFollowRedirects\0", "Z\0");
        //         env.get_boolean_field(class, field)
        //     }
        // }

        // /// **set** protected [instanceFollowRedirects](https://developer.android.com/reference/java/net/HttpURLConnection.html#instanceFollowRedirects)
        // pub fn set_instanceFollowRedirects<'env>(&'env self, value: bool) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "instanceFollowRedirects\0", "Z\0");
        //         env.set_boolean_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [method](https://developer.android.com/reference/java/net/HttpURLConnection.html#method)
        // ///
        // /// Required feature: java-lang-String
        // #[cfg(any(feature = "all", feature = "java-lang-String"))]
        // pub fn method<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "method\0", "Ljava/lang/String;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [method](https://developer.android.com/reference/java/net/HttpURLConnection.html#method)
        // ///
        // /// Required feature: java-lang-String
        // #[cfg(any(feature = "all", feature = "java-lang-String"))]
        // pub fn set_method<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "method\0", "Ljava/lang/String;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [responseCode](https://developer.android.com/reference/java/net/HttpURLConnection.html#responseCode)
        // pub fn responseCode<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "responseCode\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [responseCode](https://developer.android.com/reference/java/net/HttpURLConnection.html#responseCode)
        // pub fn set_responseCode<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "responseCode\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [responseMessage](https://developer.android.com/reference/java/net/HttpURLConnection.html#responseMessage)
        // ///
        // /// Required feature: java-lang-String
        // #[cfg(any(feature = "all", feature = "java-lang-String"))]
        // pub fn responseMessage<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "responseMessage\0", "Ljava/lang/String;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [responseMessage](https://developer.android.com/reference/java/net/HttpURLConnection.html#responseMessage)
        // ///
        // /// Required feature: java-lang-String
        // #[cfg(any(feature = "all", feature = "java-lang-String"))]
        // pub fn set_responseMessage<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/net/HttpURLConnection\0", "responseMessage\0", "Ljava/lang/String;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }
    }
}
