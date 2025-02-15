// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-w3c-dom-ls-LSOutput"))]
__jni_bindgen! {
    /// public interface [LSOutput](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html)
    ///
    /// Required feature: org-w3c-dom-ls-LSOutput
    public interface LSOutput ("org/w3c/dom/ls/LSOutput") extends crate::java::lang::Object {

        /// [getCharacterStream](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#getCharacterStream())
        ///
        /// Required features: "java-io-Writer"
        #[cfg(any(feature = "all", all(feature = "java-io-Writer")))]
        pub fn getCharacterStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::Writer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "getCharacterStream", .descriptor == "()Ljava/io/Writer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "getCharacterStream\0", "()Ljava/io/Writer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCharacterStream](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#setCharacterStream(java.io.Writer))
        ///
        /// Required features: "java-io-Writer"
        #[cfg(any(feature = "all", all(feature = "java-io-Writer")))]
        pub fn setCharacterStream<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::Writer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "setCharacterStream", .descriptor == "(Ljava/io/Writer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "setCharacterStream\0", "(Ljava/io/Writer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getByteStream](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#getByteStream())
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn getByteStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::OutputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "getByteStream", .descriptor == "()Ljava/io/OutputStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "getByteStream\0", "()Ljava/io/OutputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setByteStream](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#setByteStream(java.io.OutputStream))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn setByteStream<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "setByteStream", .descriptor == "(Ljava/io/OutputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "setByteStream\0", "(Ljava/io/OutputStream;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSystemId](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#getSystemId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSystemId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "getSystemId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "getSystemId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSystemId](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#setSystemId(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setSystemId<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "setSystemId", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "setSystemId\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncoding](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#getEncoding())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEncoding<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "getEncoding", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "getEncoding\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEncoding](https://developer.android.com/reference/org/w3c/dom/ls/LSOutput.html#setEncoding(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setEncoding<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/w3c/dom/ls/LSOutput", java.flags == PUBLIC | ABSTRACT, .name == "setEncoding", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/w3c/dom/ls/LSOutput\0", "setEncoding\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
