// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-ResourceBundle_Control"))]
__jni_bindgen! {
    /// public class [ResourceBundle.Control](https://developer.android.com/reference/java/util/ResourceBundle.Control.html)
    ///
    /// Required feature: java-util-ResourceBundle_Control
    public class ResourceBundle_Control ("java/util/ResourceBundle$Control") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Control](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#Control())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::ResourceBundle_Control>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/ResourceBundle$Control", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getControl](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#getControl(java.util.List))
        ///
        /// Required features: "java-util-List", "java-util-ResourceBundle_Control"
        #[cfg(any(feature = "all", all(feature = "java-util-List", feature = "java-util-ResourceBundle_Control")))]
        pub fn getControl<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ResourceBundle_Control>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC | STATIC | FINAL, .name == "getControl", .descriptor == "(Ljava/util/List;)Ljava/util/ResourceBundle$Control;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/ResourceBundle$Control\0", "getControl\0", "(Ljava/util/List;)Ljava/util/ResourceBundle$Control;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNoFallbackControl](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#getNoFallbackControl(java.util.List))
        ///
        /// Required features: "java-util-List", "java-util-ResourceBundle_Control"
        #[cfg(any(feature = "all", all(feature = "java-util-List", feature = "java-util-ResourceBundle_Control")))]
        pub fn getNoFallbackControl<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ResourceBundle_Control>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC | STATIC | FINAL, .name == "getNoFallbackControl", .descriptor == "(Ljava/util/List;)Ljava/util/ResourceBundle$Control;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/ResourceBundle$Control\0", "getNoFallbackControl\0", "(Ljava/util/List;)Ljava/util/ResourceBundle$Control;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFormats](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#getFormats(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-List")))]
        pub fn getFormats<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "getFormats", .descriptor == "(Ljava/lang/String;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "getFormats\0", "(Ljava/lang/String;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCandidateLocales](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#getCandidateLocales(java.lang.String,%20java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-List", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-List", feature = "java-util-Locale")))]
        pub fn getCandidateLocales<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "getCandidateLocales", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "getCandidateLocales\0", "(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFallbackLocale](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#getFallbackLocale(java.lang.String,%20java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getFallbackLocale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Locale>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "getFallbackLocale", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/Locale;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "getFallbackLocale\0", "(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/Locale;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newBundle](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#newBundle(java.lang.String,%20java.util.Locale,%20java.lang.String,%20java.lang.ClassLoader,%20boolean))
        ///
        /// Required features: "java-lang-ClassLoader", "java-lang-String", "java-util-Locale", "java-util-ResourceBundle"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-lang-String", feature = "java-util-Locale", feature = "java-util-ResourceBundle")))]
        pub fn newBundle<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>, arg4: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ResourceBundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "newBundle", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/String;Ljava/lang/ClassLoader;Z)Ljava/util/ResourceBundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "newBundle\0", "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/String;Ljava/lang/ClassLoader;Z)Ljava/util/ResourceBundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeToLive](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#getTimeToLive(java.lang.String,%20java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getTimeToLive<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "getTimeToLive", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "getTimeToLive\0", "(Ljava/lang/String;Ljava/util/Locale;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [needsReload](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#needsReload(java.lang.String,%20java.util.Locale,%20java.lang.String,%20java.lang.ClassLoader,%20java.util.ResourceBundle,%20long))
        ///
        /// Required features: "java-lang-ClassLoader", "java-lang-String", "java-util-Locale", "java-util-ResourceBundle"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-lang-String", feature = "java-util-Locale", feature = "java-util-ResourceBundle")))]
        pub fn needsReload<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::ResourceBundle>>, arg5: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "needsReload", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/String;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle;J)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "needsReload\0", "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/String;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle;J)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toBundleName](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#toBundleName(java.lang.String,%20java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn toBundleName<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC, .name == "toBundleName", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "toBundleName\0", "(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toResourceName](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#toResourceName(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toResourceName<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ResourceBundle$Control", java.flags == PUBLIC | FINAL, .name == "toResourceName", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ResourceBundle$Control\0", "toResourceName\0", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [FORMAT_CLASS](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#FORMAT_CLASS)
        ///
        /// Required feature: java-util-List
        #[cfg(any(feature = "all", feature = "java-util-List"))]
        pub fn FORMAT_CLASS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/ResourceBundle$Control\0", "FORMAT_CLASS\0", "Ljava/util/List;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FORMAT_DEFAULT](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#FORMAT_DEFAULT)
        ///
        /// Required feature: java-util-List
        #[cfg(any(feature = "all", feature = "java-util-List"))]
        pub fn FORMAT_DEFAULT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/ResourceBundle$Control\0", "FORMAT_DEFAULT\0", "Ljava/util/List;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FORMAT_PROPERTIES](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#FORMAT_PROPERTIES)
        ///
        /// Required feature: java-util-List
        #[cfg(any(feature = "all", feature = "java-util-List"))]
        pub fn FORMAT_PROPERTIES<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/ResourceBundle$Control\0", "FORMAT_PROPERTIES\0", "Ljava/util/List;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TTL_DONT_CACHE](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#TTL_DONT_CACHE)
        pub const TTL_DONT_CACHE : i64 = -1i64;

        /// public static final [TTL_NO_EXPIRATION_CONTROL](https://developer.android.com/reference/java/util/ResourceBundle.Control.html#TTL_NO_EXPIRATION_CONTROL)
        pub const TTL_NO_EXPIRATION_CONTROL : i64 = -2i64;
    }
}
