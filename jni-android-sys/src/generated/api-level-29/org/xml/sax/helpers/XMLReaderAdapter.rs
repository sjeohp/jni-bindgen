// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-xml-sax-helpers-XMLReaderAdapter"))]
__jni_bindgen! {
    /// public class [XMLReaderAdapter](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html)
    ///
    /// Required feature: org-xml-sax-helpers-XMLReaderAdapter
    public class XMLReaderAdapter ("org/xml/sax/helpers/XMLReaderAdapter") extends crate::java::lang::Object, implements crate::org::xml::sax::Parser, crate::org::xml::sax::ContentHandler {

        /// [XMLReaderAdapter](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#XMLReaderAdapter())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::org::xml::sax::helpers::XMLReaderAdapter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [XMLReaderAdapter](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#XMLReaderAdapter(org.xml.sax.XMLReader))
        ///
        /// Required features: "org-xml-sax-XMLReader"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-XMLReader")))]
        pub fn new_XMLReader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::XMLReader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::org::xml::sax::helpers::XMLReaderAdapter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Lorg/xml/sax/XMLReader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "<init>\0", "(Lorg/xml/sax/XMLReader;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLocale](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#setLocale(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn setLocale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "setLocale", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "setLocale\0", "(Ljava/util/Locale;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEntityResolver](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#setEntityResolver(org.xml.sax.EntityResolver))
        ///
        /// Required features: "org-xml-sax-EntityResolver"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-EntityResolver")))]
        pub fn setEntityResolver<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::EntityResolver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "setEntityResolver", .descriptor == "(Lorg/xml/sax/EntityResolver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "setEntityResolver\0", "(Lorg/xml/sax/EntityResolver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDTDHandler](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#setDTDHandler(org.xml.sax.DTDHandler))
        ///
        /// Required features: "org-xml-sax-DTDHandler"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-DTDHandler")))]
        pub fn setDTDHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::DTDHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "setDTDHandler", .descriptor == "(Lorg/xml/sax/DTDHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "setDTDHandler\0", "(Lorg/xml/sax/DTDHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDocumentHandler](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#setDocumentHandler(org.xml.sax.DocumentHandler))
        ///
        /// Required features: "org-xml-sax-DocumentHandler"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-DocumentHandler")))]
        pub fn setDocumentHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::DocumentHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "setDocumentHandler", .descriptor == "(Lorg/xml/sax/DocumentHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "setDocumentHandler\0", "(Lorg/xml/sax/DocumentHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setErrorHandler](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#setErrorHandler(org.xml.sax.ErrorHandler))
        ///
        /// Required features: "org-xml-sax-ErrorHandler"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-ErrorHandler")))]
        pub fn setErrorHandler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::ErrorHandler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "setErrorHandler", .descriptor == "(Lorg/xml/sax/ErrorHandler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "setErrorHandler\0", "(Lorg/xml/sax/ErrorHandler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#parse(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn parse_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "parse\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#parse(org.xml.sax.InputSource))
        ///
        /// Required features: "org-xml-sax-InputSource"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-InputSource")))]
        pub fn parse_InputSource<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::InputSource>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "parse", .descriptor == "(Lorg/xml/sax/InputSource;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "parse\0", "(Lorg/xml/sax/InputSource;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDocumentLocator](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#setDocumentLocator(org.xml.sax.Locator))
        ///
        /// Required features: "org-xml-sax-Locator"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-Locator")))]
        pub fn setDocumentLocator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::Locator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "setDocumentLocator", .descriptor == "(Lorg/xml/sax/Locator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "setDocumentLocator\0", "(Lorg/xml/sax/Locator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startDocument](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#startDocument())
        pub fn startDocument<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "startDocument", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "startDocument\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endDocument](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#endDocument())
        pub fn endDocument<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "endDocument", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "endDocument\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startPrefixMapping](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#startPrefixMapping(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn startPrefixMapping<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "startPrefixMapping", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "startPrefixMapping\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endPrefixMapping](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#endPrefixMapping(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn endPrefixMapping<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "endPrefixMapping", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "endPrefixMapping\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startElement](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#startElement(java.lang.String,%20java.lang.String,%20java.lang.String,%20org.xml.sax.Attributes))
        ///
        /// Required features: "java-lang-String", "org-xml-sax-Attributes"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-xml-sax-Attributes")))]
        pub fn startElement<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xml::sax::Attributes>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "startElement", .descriptor == "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/xml/sax/Attributes;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "startElement\0", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/xml/sax/Attributes;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endElement](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#endElement(java.lang.String,%20java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn endElement<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "endElement", .descriptor == "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "endElement\0", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [characters](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#characters(char%5B%5D,%20int,%20int))
        pub fn characters<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "characters", .descriptor == "([CII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "characters\0", "([CII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ignorableWhitespace](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#ignorableWhitespace(char%5B%5D,%20int,%20int))
        pub fn ignorableWhitespace<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "ignorableWhitespace", .descriptor == "([CII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "ignorableWhitespace\0", "([CII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [processingInstruction](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#processingInstruction(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn processingInstruction<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "processingInstruction", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "processingInstruction\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [skippedEntity](https://developer.android.com/reference/org/xml/sax/helpers/XMLReaderAdapter.html#skippedEntity(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn skippedEntity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/helpers/XMLReaderAdapter", java.flags == PUBLIC, .name == "skippedEntity", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/helpers/XMLReaderAdapter\0", "skippedEntity\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
