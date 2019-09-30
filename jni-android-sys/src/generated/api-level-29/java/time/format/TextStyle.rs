// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
__jni_bindgen! {
    /// public enum [TextStyle](https://developer.android.com/reference/java/time/format/TextStyle.html)
    ///
    /// Required feature: java-time-format-TextStyle
    public enum TextStyle ("java/time/format/TextStyle") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/time/format/TextStyle.html#values())
        ///
        /// Required features: "java-time-format-TextStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-TextStyle")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::time::format::TextStyle, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/TextStyle", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/time/format/TextStyle;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/TextStyle\0", "values\0", "()[Ljava/time/format/TextStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/time/format/TextStyle.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-time-format-TextStyle"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-time-format-TextStyle")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/TextStyle", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/time/format/TextStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/TextStyle\0", "valueOf\0", "(Ljava/lang/String;)Ljava/time/format/TextStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [TextStyle](https://developer.android.com/reference/java/time/format/TextStyle.html#TextStyle(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/format/TextStyle", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/TextStyle\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isStandalone](https://developer.android.com/reference/java/time/format/TextStyle.html#isStandalone())
        pub fn isStandalone<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/TextStyle", java.flags == PUBLIC, .name == "isStandalone", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/TextStyle\0", "isStandalone\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [asStandalone](https://developer.android.com/reference/java/time/format/TextStyle.html#asStandalone())
        ///
        /// Required features: "java-time-format-TextStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-TextStyle")))]
        pub fn asStandalone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/TextStyle", java.flags == PUBLIC, .name == "asStandalone", .descriptor == "()Ljava/time/format/TextStyle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/TextStyle\0", "asStandalone\0", "()Ljava/time/format/TextStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [asNormal](https://developer.android.com/reference/java/time/format/TextStyle.html#asNormal())
        ///
        /// Required features: "java-time-format-TextStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-TextStyle")))]
        pub fn asNormal<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/TextStyle", java.flags == PUBLIC, .name == "asNormal", .descriptor == "()Ljava/time/format/TextStyle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/TextStyle\0", "asNormal\0", "()Ljava/time/format/TextStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [FULL](https://developer.android.com/reference/java/time/format/TextStyle.html#FULL)
        ///
        /// Required feature: java-time-format-TextStyle
        #[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
        pub fn FULL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/TextStyle\0", "FULL\0", "Ljava/time/format/TextStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FULL_STANDALONE](https://developer.android.com/reference/java/time/format/TextStyle.html#FULL_STANDALONE)
        ///
        /// Required feature: java-time-format-TextStyle
        #[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
        pub fn FULL_STANDALONE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/TextStyle\0", "FULL_STANDALONE\0", "Ljava/time/format/TextStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SHORT](https://developer.android.com/reference/java/time/format/TextStyle.html#SHORT)
        ///
        /// Required feature: java-time-format-TextStyle
        #[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
        pub fn SHORT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/TextStyle\0", "SHORT\0", "Ljava/time/format/TextStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SHORT_STANDALONE](https://developer.android.com/reference/java/time/format/TextStyle.html#SHORT_STANDALONE)
        ///
        /// Required feature: java-time-format-TextStyle
        #[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
        pub fn SHORT_STANDALONE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/TextStyle\0", "SHORT_STANDALONE\0", "Ljava/time/format/TextStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NARROW](https://developer.android.com/reference/java/time/format/TextStyle.html#NARROW)
        ///
        /// Required feature: java-time-format-TextStyle
        #[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
        pub fn NARROW<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/TextStyle\0", "NARROW\0", "Ljava/time/format/TextStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NARROW_STANDALONE](https://developer.android.com/reference/java/time/format/TextStyle.html#NARROW_STANDALONE)
        ///
        /// Required feature: java-time-format-TextStyle
        #[cfg(any(feature = "all", feature = "java-time-format-TextStyle"))]
        pub fn NARROW_STANDALONE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::TextStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/TextStyle\0", "NARROW_STANDALONE\0", "Ljava/time/format/TextStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::time::format::TextStyle, crate::java::lang::Throwable>>> { ... }
    }
}
