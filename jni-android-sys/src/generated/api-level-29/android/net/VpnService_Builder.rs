// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-VpnService_Builder"))]
__jni_bindgen! {
    /// public class [VpnService.Builder](https://developer.android.com/reference/android/net/VpnService.Builder.html)
    ///
    /// Required feature: android-net-VpnService_Builder
    public class VpnService_Builder ("android/net/VpnService$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/net/VpnService.Builder.html#Builder(android.net.VpnService))
        ///
        /// Required features: "android-net-VpnService"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::VpnService>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/net/VpnService;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "<init>\0", "(Landroid/net/VpnService;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSession](https://developer.android.com/reference/android/net/VpnService.Builder.html#setSession(java.lang.String))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn setSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setSession", .descriptor == "(Ljava/lang/String;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setSession\0", "(Ljava/lang/String;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setConfigureIntent](https://developer.android.com/reference/android/net/VpnService.Builder.html#setConfigureIntent(android.app.PendingIntent))
        ///
        /// Required features: "android-app-PendingIntent", "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent", feature = "android-net-VpnService_Builder")))]
        pub fn setConfigureIntent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setConfigureIntent", .descriptor == "(Landroid/app/PendingIntent;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setConfigureIntent\0", "(Landroid/app/PendingIntent;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMtu](https://developer.android.com/reference/android/net/VpnService.Builder.html#setMtu(int))
        ///
        /// Required features: "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder")))]
        pub fn setMtu<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setMtu", .descriptor == "(I)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setMtu\0", "(I)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHttpProxy](https://developer.android.com/reference/android/net/VpnService.Builder.html#setHttpProxy(android.net.ProxyInfo))
        ///
        /// Required features: "android-net-ProxyInfo", "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-ProxyInfo", feature = "android-net-VpnService_Builder")))]
        pub fn setHttpProxy<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::ProxyInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setHttpProxy", .descriptor == "(Landroid/net/ProxyInfo;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setHttpProxy\0", "(Landroid/net/ProxyInfo;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAddress](https://developer.android.com/reference/android/net/VpnService.Builder.html#addAddress(java.net.InetAddress,%20int))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-net-InetAddress")))]
        pub fn addAddress_InetAddress_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addAddress", .descriptor == "(Ljava/net/InetAddress;I)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addAddress\0", "(Ljava/net/InetAddress;I)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAddress](https://developer.android.com/reference/android/net/VpnService.Builder.html#addAddress(java.lang.String,%20int))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn addAddress_String_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addAddress", .descriptor == "(Ljava/lang/String;I)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addAddress\0", "(Ljava/lang/String;I)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addRoute](https://developer.android.com/reference/android/net/VpnService.Builder.html#addRoute(java.net.InetAddress,%20int))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-net-InetAddress")))]
        pub fn addRoute_InetAddress_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addRoute", .descriptor == "(Ljava/net/InetAddress;I)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addRoute\0", "(Ljava/net/InetAddress;I)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addRoute](https://developer.android.com/reference/android/net/VpnService.Builder.html#addRoute(java.lang.String,%20int))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn addRoute_String_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addRoute", .descriptor == "(Ljava/lang/String;I)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addRoute\0", "(Ljava/lang/String;I)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addDnsServer](https://developer.android.com/reference/android/net/VpnService.Builder.html#addDnsServer(java.net.InetAddress))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-net-InetAddress")))]
        pub fn addDnsServer_InetAddress<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addDnsServer", .descriptor == "(Ljava/net/InetAddress;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addDnsServer\0", "(Ljava/net/InetAddress;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addDnsServer](https://developer.android.com/reference/android/net/VpnService.Builder.html#addDnsServer(java.lang.String))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn addDnsServer_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addDnsServer", .descriptor == "(Ljava/lang/String;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addDnsServer\0", "(Ljava/lang/String;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addSearchDomain](https://developer.android.com/reference/android/net/VpnService.Builder.html#addSearchDomain(java.lang.String))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn addSearchDomain<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addSearchDomain", .descriptor == "(Ljava/lang/String;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addSearchDomain\0", "(Ljava/lang/String;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [allowFamily](https://developer.android.com/reference/android/net/VpnService.Builder.html#allowFamily(int))
        ///
        /// Required features: "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder")))]
        pub fn allowFamily<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "allowFamily", .descriptor == "(I)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "allowFamily\0", "(I)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAllowedApplication](https://developer.android.com/reference/android/net/VpnService.Builder.html#addAllowedApplication(java.lang.String))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn addAllowedApplication<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addAllowedApplication", .descriptor == "(Ljava/lang/String;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addAllowedApplication\0", "(Ljava/lang/String;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addDisallowedApplication](https://developer.android.com/reference/android/net/VpnService.Builder.html#addDisallowedApplication(java.lang.String))
        ///
        /// Required features: "android-net-VpnService_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder", feature = "java-lang-String")))]
        pub fn addDisallowedApplication<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "addDisallowedApplication", .descriptor == "(Ljava/lang/String;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "addDisallowedApplication\0", "(Ljava/lang/String;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [allowBypass](https://developer.android.com/reference/android/net/VpnService.Builder.html#allowBypass())
        ///
        /// Required features: "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder")))]
        pub fn allowBypass<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "allowBypass", .descriptor == "()Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "allowBypass\0", "()Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBlocking](https://developer.android.com/reference/android/net/VpnService.Builder.html#setBlocking(boolean))
        ///
        /// Required features: "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder")))]
        pub fn setBlocking<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setBlocking", .descriptor == "(Z)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setBlocking\0", "(Z)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setUnderlyingNetworks](https://developer.android.com/reference/android/net/VpnService.Builder.html#setUnderlyingNetworks(android.net.Network%5B%5D))
        ///
        /// Required features: "android-net-Network", "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-Network", feature = "android-net-VpnService_Builder")))]
        pub fn setUnderlyingNetworks<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::net::Network, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setUnderlyingNetworks", .descriptor == "([Landroid/net/Network;)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setUnderlyingNetworks\0", "([Landroid/net/Network;)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMetered](https://developer.android.com/reference/android/net/VpnService.Builder.html#setMetered(boolean))
        ///
        /// Required features: "android-net-VpnService_Builder"
        #[cfg(any(feature = "all", all(feature = "android-net-VpnService_Builder")))]
        pub fn setMetered<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "setMetered", .descriptor == "(Z)Landroid/net/VpnService$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "setMetered\0", "(Z)Landroid/net/VpnService$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [establish](https://developer.android.com/reference/android/net/VpnService.Builder.html#establish())
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn establish<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::ParcelFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/VpnService$Builder", java.flags == PUBLIC, .name == "establish", .descriptor == "()Landroid/os/ParcelFileDescriptor;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/VpnService$Builder\0", "establish\0", "()Landroid/os/ParcelFileDescriptor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: this$N outer class pointer
        // pub fn get_"this$0"<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::VpnService>> { ... }
    }
}
