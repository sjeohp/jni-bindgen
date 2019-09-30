// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-FormatFlagsConversionMismatchException"))]
__jni_bindgen! {
    /// public class [FormatFlagsConversionMismatchException](https://developer.android.com/reference/java/util/FormatFlagsConversionMismatchException.html)
    ///
    /// Required feature: java-util-FormatFlagsConversionMismatchException
    public class FormatFlagsConversionMismatchException ("java/util/FormatFlagsConversionMismatchException") extends crate::java::util::IllegalFormatException {

        /// [FormatFlagsConversionMismatchException](https://developer.android.com/reference/java/util/FormatFlagsConversionMismatchException.html#FormatFlagsConversionMismatchException(java.lang.String,%20char))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::FormatFlagsConversionMismatchException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/FormatFlagsConversionMismatchException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;C)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/FormatFlagsConversionMismatchException\0", "<init>\0", "(Ljava/lang/String;C)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFlags](https://developer.android.com/reference/java/util/FormatFlagsConversionMismatchException.html#getFlags())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getFlags<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/FormatFlagsConversionMismatchException", java.flags == PUBLIC, .name == "getFlags", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/FormatFlagsConversionMismatchException\0", "getFlags\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConversion](https://developer.android.com/reference/java/util/FormatFlagsConversionMismatchException.html#getConversion())
        pub fn getConversion<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/FormatFlagsConversionMismatchException", java.flags == PUBLIC, .name == "getConversion", .descriptor == "()C"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/FormatFlagsConversionMismatchException\0", "getConversion\0", "()C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMessage](https://developer.android.com/reference/java/util/FormatFlagsConversionMismatchException.html#getMessage())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMessage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/FormatFlagsConversionMismatchException", java.flags == PUBLIC, .name == "getMessage", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/FormatFlagsConversionMismatchException\0", "getMessage\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
