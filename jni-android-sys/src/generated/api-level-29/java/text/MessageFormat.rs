// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-text-MessageFormat"))]
__jni_bindgen! {
    /// public class [MessageFormat](https://developer.android.com/reference/java/text/MessageFormat.html)
    ///
    /// Required feature: java-text-MessageFormat
    public class MessageFormat ("java/text/MessageFormat") extends crate::java::text::Format {

        /// [MessageFormat](https://developer.android.com/reference/java/text/MessageFormat.html#MessageFormat(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::text::MessageFormat>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [MessageFormat](https://developer.android.com/reference/java/text/MessageFormat.html#MessageFormat(java.lang.String,%20java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn new_String_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::text::MessageFormat>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "<init>\0", "(Ljava/lang/String;Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLocale](https://developer.android.com/reference/java/text/MessageFormat.html#setLocale(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn setLocale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "setLocale", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "setLocale\0", "(Ljava/util/Locale;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocale](https://developer.android.com/reference/java/text/MessageFormat.html#getLocale())
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn getLocale<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Locale>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "getLocale", .descriptor == "()Ljava/util/Locale;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "getLocale\0", "()Ljava/util/Locale;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyPattern](https://developer.android.com/reference/java/text/MessageFormat.html#applyPattern(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn applyPattern<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "applyPattern", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "applyPattern\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toPattern](https://developer.android.com/reference/java/text/MessageFormat.html#toPattern())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toPattern<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "toPattern", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "toPattern\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFormatsByArgumentIndex](https://developer.android.com/reference/java/text/MessageFormat.html#setFormatsByArgumentIndex(java.text.Format%5B%5D))
        ///
        /// Required features: "java-text-Format"
        #[cfg(any(feature = "all", all(feature = "java-text-Format")))]
        pub fn setFormatsByArgumentIndex<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::text::Format, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "setFormatsByArgumentIndex", .descriptor == "([Ljava/text/Format;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "setFormatsByArgumentIndex\0", "([Ljava/text/Format;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFormats](https://developer.android.com/reference/java/text/MessageFormat.html#setFormats(java.text.Format%5B%5D))
        ///
        /// Required features: "java-text-Format"
        #[cfg(any(feature = "all", all(feature = "java-text-Format")))]
        pub fn setFormats<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::text::Format, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "setFormats", .descriptor == "([Ljava/text/Format;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "setFormats\0", "([Ljava/text/Format;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFormatByArgumentIndex](https://developer.android.com/reference/java/text/MessageFormat.html#setFormatByArgumentIndex(int,%20java.text.Format))
        ///
        /// Required features: "java-text-Format"
        #[cfg(any(feature = "all", all(feature = "java-text-Format")))]
        pub fn setFormatByArgumentIndex<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::Format>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "setFormatByArgumentIndex", .descriptor == "(ILjava/text/Format;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "setFormatByArgumentIndex\0", "(ILjava/text/Format;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFormat](https://developer.android.com/reference/java/text/MessageFormat.html#setFormat(int,%20java.text.Format))
        ///
        /// Required features: "java-text-Format"
        #[cfg(any(feature = "all", all(feature = "java-text-Format")))]
        pub fn setFormat<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::Format>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "setFormat", .descriptor == "(ILjava/text/Format;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "setFormat\0", "(ILjava/text/Format;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFormatsByArgumentIndex](https://developer.android.com/reference/java/text/MessageFormat.html#getFormatsByArgumentIndex())
        ///
        /// Required features: "java-text-Format"
        #[cfg(any(feature = "all", all(feature = "java-text-Format")))]
        pub fn getFormatsByArgumentIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::text::Format, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "getFormatsByArgumentIndex", .descriptor == "()[Ljava/text/Format;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "getFormatsByArgumentIndex\0", "()[Ljava/text/Format;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFormats](https://developer.android.com/reference/java/text/MessageFormat.html#getFormats())
        ///
        /// Required features: "java-text-Format"
        #[cfg(any(feature = "all", all(feature = "java-text-Format")))]
        pub fn getFormats<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::text::Format, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "getFormats", .descriptor == "()[Ljava/text/Format;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "getFormats\0", "()[Ljava/text/Format;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/java/text/MessageFormat.html#format(java.lang.Object%5B%5D,%20java.lang.StringBuffer,%20java.text.FieldPosition))
        ///
        /// Required features: "java-lang-Object", "java-lang-StringBuffer", "java-text-FieldPosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-StringBuffer", feature = "java-text-FieldPosition")))]
        pub fn format_Object_array_StringBuffer_FieldPosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::StringBuffer>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::FieldPosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::StringBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC | FINAL, .name == "format", .descriptor == "([Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "format\0", "([Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/java/text/MessageFormat.html#format(java.lang.String,%20java.lang.Object...))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn format_String_Object_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC | STATIC | VARARGS, .name == "format", .descriptor == "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/text/MessageFormat\0", "format\0", "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/java/text/MessageFormat.html#format(java.lang.Object,%20java.lang.StringBuffer,%20java.text.FieldPosition))
        ///
        /// Required features: "java-lang-Object", "java-lang-StringBuffer", "java-text-FieldPosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-StringBuffer", feature = "java-text-FieldPosition")))]
        pub fn format_Object_StringBuffer_FieldPosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::StringBuffer>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::FieldPosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::StringBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC | FINAL, .name == "format", .descriptor == "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "format\0", "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [formatToCharacterIterator](https://developer.android.com/reference/java/text/MessageFormat.html#formatToCharacterIterator(java.lang.Object))
        ///
        /// Required features: "java-lang-Object", "java-text-AttributedCharacterIterator"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-text-AttributedCharacterIterator")))]
        pub fn formatToCharacterIterator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::AttributedCharacterIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "formatToCharacterIterator", .descriptor == "(Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "formatToCharacterIterator\0", "(Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/java/text/MessageFormat.html#parse(java.lang.String,%20java.text.ParsePosition))
        ///
        /// Required features: "java-lang-Object", "java-lang-String", "java-text-ParsePosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String", feature = "java-text-ParsePosition")))]
        pub fn parse_String_ParsePosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::ParsePosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/lang/String;Ljava/text/ParsePosition;)[Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "parse\0", "(Ljava/lang/String;Ljava/text/ParsePosition;)[Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/java/text/MessageFormat.html#parse(java.lang.String))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn parse_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/lang/String;)[Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "parse\0", "(Ljava/lang/String;)[Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseObject](https://developer.android.com/reference/java/text/MessageFormat.html#parseObject(java.lang.String,%20java.text.ParsePosition))
        ///
        /// Required features: "java-lang-Object", "java-lang-String", "java-text-ParsePosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String", feature = "java-text-ParsePosition")))]
        pub fn parseObject<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::ParsePosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "parseObject", .descriptor == "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "parseObject\0", "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/java/text/MessageFormat.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/text/MessageFormat.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/text/MessageFormat.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/text/MessageFormat", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/text/MessageFormat\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
