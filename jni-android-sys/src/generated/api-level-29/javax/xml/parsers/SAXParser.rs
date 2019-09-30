// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-parsers-SAXParser"))]
__jni_bindgen! {
    /// public class [SAXParser](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html)
    ///
    /// Required feature: javax-xml-parsers-SAXParser
    public class SAXParser ("javax/xml/parsers/SAXParser") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SAXParser](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#SAXParser())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::parsers::SAXParser>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/xml/parsers/SAXParser", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [reset](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.io.InputStream,%20org.xml.sax.HandlerBase))
        ///
        /// Required features: "java-io-InputStream", "org-xml-sax-HandlerBase"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "org-xml-sax-HandlerBase")))]
        pub fn parse_InputStream_HandlerBase<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::HandlerBase>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/io/InputStream;Lorg/xml/sax/HandlerBase;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/io/InputStream;Lorg/xml/sax/HandlerBase;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.io.InputStream,%20org.xml.sax.HandlerBase,%20java.lang.String))
        ///
        /// Required features: "java-io-InputStream", "java-lang-String", "org-xml-sax-HandlerBase"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-lang-String", feature = "org-xml-sax-HandlerBase")))]
        pub fn parse_InputStream_HandlerBase_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::HandlerBase>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/io/InputStream;Lorg/xml/sax/HandlerBase;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/io/InputStream;Lorg/xml/sax/HandlerBase;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.io.InputStream,%20org.xml.sax.helpers.DefaultHandler))
        ///
        /// Required features: "java-io-InputStream", "org-xml-sax-helpers-DefaultHandler"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "org-xml-sax-helpers-DefaultHandler")))]
        pub fn parse_InputStream_DefaultHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::helpers::DefaultHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.io.InputStream,%20org.xml.sax.helpers.DefaultHandler,%20java.lang.String))
        ///
        /// Required features: "java-io-InputStream", "java-lang-String", "org-xml-sax-helpers-DefaultHandler"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-lang-String", feature = "org-xml-sax-helpers-DefaultHandler")))]
        pub fn parse_InputStream_DefaultHandler_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::helpers::DefaultHandler>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.lang.String,%20org.xml.sax.HandlerBase))
        ///
        /// Required features: "java-lang-String", "org-xml-sax-HandlerBase"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-xml-sax-HandlerBase")))]
        pub fn parse_String_HandlerBase<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::HandlerBase>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/lang/String;Lorg/xml/sax/HandlerBase;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/lang/String;Lorg/xml/sax/HandlerBase;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.lang.String,%20org.xml.sax.helpers.DefaultHandler))
        ///
        /// Required features: "java-lang-String", "org-xml-sax-helpers-DefaultHandler"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-xml-sax-helpers-DefaultHandler")))]
        pub fn parse_String_DefaultHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::helpers::DefaultHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/lang/String;Lorg/xml/sax/helpers/DefaultHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/lang/String;Lorg/xml/sax/helpers/DefaultHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.io.File,%20org.xml.sax.HandlerBase))
        ///
        /// Required features: "java-io-File", "org-xml-sax-HandlerBase"
        #[cfg(any(feature = "all", all(feature = "java-io-File", feature = "org-xml-sax-HandlerBase")))]
        pub fn parse_File_HandlerBase<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::HandlerBase>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/io/File;Lorg/xml/sax/HandlerBase;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/io/File;Lorg/xml/sax/HandlerBase;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(java.io.File,%20org.xml.sax.helpers.DefaultHandler))
        ///
        /// Required features: "java-io-File", "org-xml-sax-helpers-DefaultHandler"
        #[cfg(any(feature = "all", all(feature = "java-io-File", feature = "org-xml-sax-helpers-DefaultHandler")))]
        pub fn parse_File_DefaultHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::helpers::DefaultHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/io/File;Lorg/xml/sax/helpers/DefaultHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Ljava/io/File;Lorg/xml/sax/helpers/DefaultHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(org.xml.sax.InputSource,%20org.xml.sax.HandlerBase))
        ///
        /// Required features: "org-xml-sax-HandlerBase", "org-xml-sax-InputSource"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-HandlerBase", feature = "org-xml-sax-InputSource")))]
        pub fn parse_InputSource_HandlerBase<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::InputSource>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::HandlerBase>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Lorg/xml/sax/InputSource;Lorg/xml/sax/HandlerBase;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Lorg/xml/sax/InputSource;Lorg/xml/sax/HandlerBase;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#parse(org.xml.sax.InputSource,%20org.xml.sax.helpers.DefaultHandler))
        ///
        /// Required features: "org-xml-sax-InputSource", "org-xml-sax-helpers-DefaultHandler"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-InputSource", feature = "org-xml-sax-helpers-DefaultHandler")))]
        pub fn parse_InputSource_DefaultHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::InputSource>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::helpers::DefaultHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "parse", .descriptor == "(Lorg/xml/sax/InputSource;Lorg/xml/sax/helpers/DefaultHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "parse\0", "(Lorg/xml/sax/InputSource;Lorg/xml/sax/helpers/DefaultHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParser](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#getParser())
        ///
        /// Required features: "org-xml-sax-Parser"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-Parser")))]
        pub fn getParser<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::xml::sax::Parser>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC | ABSTRACT, .name == "getParser", .descriptor == "()Lorg/xml/sax/Parser;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "getParser\0", "()Lorg/xml/sax/Parser;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getXMLReader](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#getXMLReader())
        ///
        /// Required features: "org-xml-sax-XMLReader"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-XMLReader")))]
        pub fn getXMLReader<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::xml::sax::XMLReader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC | ABSTRACT, .name == "getXMLReader", .descriptor == "()Lorg/xml/sax/XMLReader;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "getXMLReader\0", "()Lorg/xml/sax/XMLReader;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isNamespaceAware](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#isNamespaceAware())
        pub fn isNamespaceAware<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC | ABSTRACT, .name == "isNamespaceAware", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "isNamespaceAware\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isValidating](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#isValidating())
        pub fn isValidating<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC | ABSTRACT, .name == "isValidating", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "isValidating\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setProperty](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#setProperty(java.lang.String,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn setProperty<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC | ABSTRACT, .name == "setProperty", .descriptor == "(Ljava/lang/String;Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "setProperty\0", "(Ljava/lang/String;Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProperty](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#getProperty(java.lang.String))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn getProperty<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC | ABSTRACT, .name == "getProperty", .descriptor == "(Ljava/lang/String;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "getProperty\0", "(Ljava/lang/String;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSchema](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#getSchema())
        ///
        /// Required features: "javax-xml-validation-Schema"
        #[cfg(any(feature = "all", all(feature = "javax-xml-validation-Schema")))]
        pub fn getSchema<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::validation::Schema>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "getSchema", .descriptor == "()Ljavax/xml/validation/Schema;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "getSchema\0", "()Ljavax/xml/validation/Schema;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isXIncludeAware](https://developer.android.com/reference/javax/xml/parsers/SAXParser.html#isXIncludeAware())
        pub fn isXIncludeAware<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/parsers/SAXParser", java.flags == PUBLIC, .name == "isXIncludeAware", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/parsers/SAXParser\0", "isXIncludeAware\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
