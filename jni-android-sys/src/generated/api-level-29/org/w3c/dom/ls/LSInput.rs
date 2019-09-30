// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-w3c-dom-ls-LSInput"))]
__jni_bindgen! {
    /// public interface [LSInput](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html)
    ///
    /// Required feature: org-w3c-dom-ls-LSInput
    public interface LSInput ("org/w3c/dom/ls/LSInput") extends crate::java::lang::Object {

        /// [getCharacterStream](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getCharacterStream())
        ///
        /// Required features: "java-io-Reader"
        #[cfg(any(feature = "all", all(feature = "java-io-Reader")))]
        pub fn getCharacterStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::Reader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getCharacterStream", .descriptor == "()Ljava/io/Reader;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getCharacterStream\0", "()Ljava/io/Reader;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCharacterStream](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setCharacterStream(java.io.Reader))
        ///
        /// Required features: "java-io-Reader"
        #[cfg(any(feature = "all", all(feature = "java-io-Reader")))]
        pub fn setCharacterStream<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::Reader>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setCharacterStream", .descriptor == "(Ljava/io/Reader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setCharacterStream\0", "(Ljava/io/Reader;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getByteStream](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getByteStream())
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn getByteStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::InputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getByteStream", .descriptor == "()Ljava/io/InputStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getByteStream\0", "()Ljava/io/InputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setByteStream](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setByteStream(java.io.InputStream))
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn setByteStream<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setByteStream", .descriptor == "(Ljava/io/InputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setByteStream\0", "(Ljava/io/InputStream;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStringData](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getStringData())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getStringData<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getStringData", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getStringData\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setStringData](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setStringData(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setStringData<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setStringData", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setStringData\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSystemId](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getSystemId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSystemId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getSystemId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getSystemId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSystemId](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setSystemId(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setSystemId<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setSystemId", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setSystemId\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPublicId](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getPublicId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getPublicId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getPublicId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getPublicId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPublicId](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setPublicId(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setPublicId<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setPublicId", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setPublicId\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBaseURI](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getBaseURI())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getBaseURI<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getBaseURI", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getBaseURI\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBaseURI](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setBaseURI(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setBaseURI<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setBaseURI", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setBaseURI\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncoding](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getEncoding())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEncoding<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getEncoding", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getEncoding\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEncoding](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setEncoding(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setEncoding<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setEncoding", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setEncoding\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertifiedText](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#getCertifiedText())
        pub fn getCertifiedText<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "getCertifiedText", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "getCertifiedText\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCertifiedText](https://developer.android.com/reference/org/w3c/dom/ls/LSInput.html#setCertifiedText(boolean))
        pub fn setCertifiedText<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSInput", java.flags == PUBLIC | ABSTRACT, .name == "setCertifiedText", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSInput\0", "setCertifiedText\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
