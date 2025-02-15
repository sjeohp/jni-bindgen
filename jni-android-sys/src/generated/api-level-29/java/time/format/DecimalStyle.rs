// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-format-DecimalStyle"))]
__jni_bindgen! {
    /// public final class [DecimalStyle](https://developer.android.com/reference/java/time/format/DecimalStyle.html)
    ///
    /// Required feature: java-time-format-DecimalStyle
    public final class DecimalStyle ("java/time/format/DecimalStyle") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [DecimalStyle](https://developer.android.com/reference/java/time/format/DecimalStyle.html#DecimalStyle(char,%20char,%20char,%20char))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: __jni_bindgen::jchar, arg1: __jni_bindgen::jchar, arg2: __jni_bindgen::jchar, arg3: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/format/DecimalStyle", java.flags == (empty), .name == "<init>", .descriptor == "(CCCC)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "<init>\0", "(CCCC)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getAvailableLocales](https://developer.android.com/reference/java/time/format/DecimalStyle.html#getAvailableLocales())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getAvailableLocales<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC | STATIC, .name == "getAvailableLocales", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/DecimalStyle\0", "getAvailableLocales\0", "()Ljava/util/Set;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofDefaultLocale](https://developer.android.com/reference/java/time/format/DecimalStyle.html#ofDefaultLocale())
        ///
        /// Required features: "java-time-format-DecimalStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-DecimalStyle")))]
        pub fn ofDefaultLocale<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC | STATIC, .name == "ofDefaultLocale", .descriptor == "()Ljava/time/format/DecimalStyle;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/DecimalStyle\0", "ofDefaultLocale\0", "()Ljava/time/format/DecimalStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [of](https://developer.android.com/reference/java/time/format/DecimalStyle.html#of(java.util.Locale))
        ///
        /// Required features: "java-time-format-DecimalStyle", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-time-format-DecimalStyle", feature = "java-util-Locale")))]
        pub fn of<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC | STATIC, .name == "of", .descriptor == "(Ljava/util/Locale;)Ljava/time/format/DecimalStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/DecimalStyle\0", "of\0", "(Ljava/util/Locale;)Ljava/time/format/DecimalStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getZeroDigit](https://developer.android.com/reference/java/time/format/DecimalStyle.html#getZeroDigit())
        pub fn getZeroDigit<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "getZeroDigit", .descriptor == "()C"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "getZeroDigit\0", "()C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [withZeroDigit](https://developer.android.com/reference/java/time/format/DecimalStyle.html#withZeroDigit(char))
        ///
        /// Required features: "java-time-format-DecimalStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-DecimalStyle")))]
        pub fn withZeroDigit<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "withZeroDigit", .descriptor == "(C)Ljava/time/format/DecimalStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "withZeroDigit\0", "(C)Ljava/time/format/DecimalStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPositiveSign](https://developer.android.com/reference/java/time/format/DecimalStyle.html#getPositiveSign())
        pub fn getPositiveSign<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "getPositiveSign", .descriptor == "()C"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "getPositiveSign\0", "()C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [withPositiveSign](https://developer.android.com/reference/java/time/format/DecimalStyle.html#withPositiveSign(char))
        ///
        /// Required features: "java-time-format-DecimalStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-DecimalStyle")))]
        pub fn withPositiveSign<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "withPositiveSign", .descriptor == "(C)Ljava/time/format/DecimalStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "withPositiveSign\0", "(C)Ljava/time/format/DecimalStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNegativeSign](https://developer.android.com/reference/java/time/format/DecimalStyle.html#getNegativeSign())
        pub fn getNegativeSign<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "getNegativeSign", .descriptor == "()C"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "getNegativeSign\0", "()C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [withNegativeSign](https://developer.android.com/reference/java/time/format/DecimalStyle.html#withNegativeSign(char))
        ///
        /// Required features: "java-time-format-DecimalStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-DecimalStyle")))]
        pub fn withNegativeSign<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "withNegativeSign", .descriptor == "(C)Ljava/time/format/DecimalStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "withNegativeSign\0", "(C)Ljava/time/format/DecimalStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDecimalSeparator](https://developer.android.com/reference/java/time/format/DecimalStyle.html#getDecimalSeparator())
        pub fn getDecimalSeparator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "getDecimalSeparator", .descriptor == "()C"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "getDecimalSeparator\0", "()C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [withDecimalSeparator](https://developer.android.com/reference/java/time/format/DecimalStyle.html#withDecimalSeparator(char))
        ///
        /// Required features: "java-time-format-DecimalStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-DecimalStyle")))]
        pub fn withDecimalSeparator<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "withDecimalSeparator", .descriptor == "(C)Ljava/time/format/DecimalStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "withDecimalSeparator\0", "(C)Ljava/time/format/DecimalStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/time/format/DecimalStyle.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/time/format/DecimalStyle.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/time/format/DecimalStyle.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/DecimalStyle", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/DecimalStyle\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [STANDARD](https://developer.android.com/reference/java/time/format/DecimalStyle.html#STANDARD)
        ///
        /// Required feature: java-time-format-DecimalStyle
        #[cfg(any(feature = "all", feature = "java-time-format-DecimalStyle"))]
        pub fn STANDARD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::DecimalStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/DecimalStyle\0", "STANDARD\0", "Ljava/time/format/DecimalStyle;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
