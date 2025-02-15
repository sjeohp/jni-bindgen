// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-URLClassLoader"))]
__jni_bindgen! {
    /// public class [URLClassLoader](https://developer.android.com/reference/java/net/URLClassLoader.html)
    ///
    /// Required feature: java-net-URLClassLoader
    public class URLClassLoader ("java/net/URLClassLoader") extends crate::java::security::SecureClassLoader, implements crate::java::io::Closeable {

        /// [URLClassLoader](https://developer.android.com/reference/java/net/URLClassLoader.html#URLClassLoader(java.net.URL%5B%5D,%20java.lang.ClassLoader))
        ///
        /// Required features: "java-lang-ClassLoader", "java-net-URL"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-net-URL")))]
        pub fn new_URL_array_ClassLoader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::net::URL, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::URLClassLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "<init>", .descriptor == "([Ljava/net/URL;Ljava/lang/ClassLoader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "<init>\0", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [URLClassLoader](https://developer.android.com/reference/java/net/URLClassLoader.html#URLClassLoader(java.net.URL%5B%5D))
        ///
        /// Required features: "java-net-URL"
        #[cfg(any(feature = "all", all(feature = "java-net-URL")))]
        pub fn new_URL_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::net::URL, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::URLClassLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "<init>", .descriptor == "([Ljava/net/URL;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "<init>\0", "([Ljava/net/URL;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [URLClassLoader](https://developer.android.com/reference/java/net/URLClassLoader.html#URLClassLoader(java.net.URL%5B%5D,%20java.lang.ClassLoader,%20java.net.URLStreamHandlerFactory))
        ///
        /// Required features: "java-lang-ClassLoader", "java-net-URL", "java-net-URLStreamHandlerFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-net-URL", feature = "java-net-URLStreamHandlerFactory")))]
        pub fn new_URL_array_ClassLoader_URLStreamHandlerFactory<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::net::URL, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URLStreamHandlerFactory>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::URLClassLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "<init>", .descriptor == "([Ljava/net/URL;Ljava/lang/ClassLoader;Ljava/net/URLStreamHandlerFactory;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "<init>\0", "([Ljava/net/URL;Ljava/lang/ClassLoader;Ljava/net/URLStreamHandlerFactory;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResourceAsStream](https://developer.android.com/reference/java/net/URLClassLoader.html#getResourceAsStream(java.lang.String))
        ///
        /// Required features: "java-io-InputStream", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-lang-String")))]
        pub fn getResourceAsStream<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::InputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "getResourceAsStream", .descriptor == "(Ljava/lang/String;)Ljava/io/InputStream;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "getResourceAsStream\0", "(Ljava/lang/String;)Ljava/io/InputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/net/URLClassLoader.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [addURL](https://developer.android.com/reference/java/net/URLClassLoader.html#addURL(java.net.URL))
        // ///
        // /// Required features: "java-net-URL"
        // #[cfg(any(feature = "all", all(feature = "java-net-URL")))]
        // fn addURL<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URL>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/URLClassLoader", java.flags == PROTECTED, .name == "addURL", .descriptor == "(Ljava/net/URL;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "addURL\0", "(Ljava/net/URL;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getURLs](https://developer.android.com/reference/java/net/URLClassLoader.html#getURLs())
        ///
        /// Required features: "java-net-URL"
        #[cfg(any(feature = "all", all(feature = "java-net-URL")))]
        pub fn getURLs<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::net::URL, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "getURLs", .descriptor == "()[Ljava/net/URL;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "getURLs\0", "()[Ljava/net/URL;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [findClass](https://developer.android.com/reference/java/net/URLClassLoader.html#findClass(java.lang.String))
        // ///
        // /// Required features: "java-lang-Class", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-String")))]
        // fn findClass<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Class>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/URLClassLoader", java.flags == PROTECTED, .name == "findClass", .descriptor == "(Ljava/lang/String;)Ljava/lang/Class;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "findClass\0", "(Ljava/lang/String;)Ljava/lang/Class;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [definePackage](https://developer.android.com/reference/java/net/URLClassLoader.html#definePackage(java.lang.String,%20java.util.jar.Manifest,%20java.net.URL))
        // ///
        // /// Required features: "java-lang-Package", "java-lang-String", "java-net-URL", "java-util-jar-Manifest"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Package", feature = "java-lang-String", feature = "java-net-URL", feature = "java-util-jar-Manifest")))]
        // fn definePackage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::jar::Manifest>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URL>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Package>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/URLClassLoader", java.flags == PROTECTED, .name == "definePackage", .descriptor == "(Ljava/lang/String;Ljava/util/jar/Manifest;Ljava/net/URL;)Ljava/lang/Package;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "definePackage\0", "(Ljava/lang/String;Ljava/util/jar/Manifest;Ljava/net/URL;)Ljava/lang/Package;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [findResource](https://developer.android.com/reference/java/net/URLClassLoader.html#findResource(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-net-URL"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-URL")))]
        pub fn findResource<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::URL>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "findResource", .descriptor == "(Ljava/lang/String;)Ljava/net/URL;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "findResource\0", "(Ljava/lang/String;)Ljava/net/URL;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [findResources](https://developer.android.com/reference/java/net/URLClassLoader.html#findResources(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-Enumeration"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Enumeration")))]
        pub fn findResources<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Enumeration>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC, .name == "findResources", .descriptor == "(Ljava/lang/String;)Ljava/util/Enumeration;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "findResources\0", "(Ljava/lang/String;)Ljava/util/Enumeration;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [getPermissions](https://developer.android.com/reference/java/net/URLClassLoader.html#getPermissions(java.security.CodeSource))
        // ///
        // /// Required features: "java-security-CodeSource", "java-security-PermissionCollection"
        // #[cfg(any(feature = "all", all(feature = "java-security-CodeSource", feature = "java-security-PermissionCollection")))]
        // fn getPermissions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::CodeSource>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::PermissionCollection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/URLClassLoader", java.flags == PROTECTED, .name == "getPermissions", .descriptor == "(Ljava/security/CodeSource;)Ljava/security/PermissionCollection;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/URLClassLoader\0", "getPermissions\0", "(Ljava/security/CodeSource;)Ljava/security/PermissionCollection;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [newInstance](https://developer.android.com/reference/java/net/URLClassLoader.html#newInstance(java.net.URL%5B%5D,%20java.lang.ClassLoader))
        ///
        /// Required features: "java-lang-ClassLoader", "java-net-URL", "java-net-URLClassLoader"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-net-URL", feature = "java-net-URLClassLoader")))]
        pub fn newInstance_URL_array_ClassLoader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::net::URL, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::URLClassLoader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC | STATIC, .name == "newInstance", .descriptor == "([Ljava/net/URL;Ljava/lang/ClassLoader;)Ljava/net/URLClassLoader;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/URLClassLoader\0", "newInstance\0", "([Ljava/net/URL;Ljava/lang/ClassLoader;)Ljava/net/URLClassLoader;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newInstance](https://developer.android.com/reference/java/net/URLClassLoader.html#newInstance(java.net.URL%5B%5D))
        ///
        /// Required features: "java-net-URL", "java-net-URLClassLoader"
        #[cfg(any(feature = "all", all(feature = "java-net-URL", feature = "java-net-URLClassLoader")))]
        pub fn newInstance_URL_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::net::URL, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::URLClassLoader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/URLClassLoader", java.flags == PUBLIC | STATIC, .name == "newInstance", .descriptor == "([Ljava/net/URL;)Ljava/net/URLClassLoader;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/URLClassLoader\0", "newInstance\0", "([Ljava/net/URL;)Ljava/net/URLClassLoader;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
