// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-format-ResolverStyle"))]
__jni_bindgen! {
    /// public enum [ResolverStyle](https://developer.android.com/reference/java/time/format/ResolverStyle.html)
    ///
    /// Required feature: java-time-format-ResolverStyle
    public enum ResolverStyle ("java/time/format/ResolverStyle") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/time/format/ResolverStyle.html#values())
        ///
        /// Required features: "java-time-format-ResolverStyle"
        #[cfg(any(feature = "all", all(feature = "java-time-format-ResolverStyle")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::time::format::ResolverStyle, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/ResolverStyle", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/time/format/ResolverStyle;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/ResolverStyle\0", "values\0", "()[Ljava/time/format/ResolverStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/time/format/ResolverStyle.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-time-format-ResolverStyle"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-time-format-ResolverStyle")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::ResolverStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/format/ResolverStyle", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/time/format/ResolverStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/format/ResolverStyle\0", "valueOf\0", "(Ljava/lang/String;)Ljava/time/format/ResolverStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [ResolverStyle](https://developer.android.com/reference/java/time/format/ResolverStyle.html#ResolverStyle(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::format::ResolverStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/format/ResolverStyle", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/format/ResolverStyle\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [STRICT](https://developer.android.com/reference/java/time/format/ResolverStyle.html#STRICT)
        ///
        /// Required feature: java-time-format-ResolverStyle
        #[cfg(any(feature = "all", feature = "java-time-format-ResolverStyle"))]
        pub fn STRICT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::ResolverStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/ResolverStyle\0", "STRICT\0", "Ljava/time/format/ResolverStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SMART](https://developer.android.com/reference/java/time/format/ResolverStyle.html#SMART)
        ///
        /// Required feature: java-time-format-ResolverStyle
        #[cfg(any(feature = "all", feature = "java-time-format-ResolverStyle"))]
        pub fn SMART<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::ResolverStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/ResolverStyle\0", "SMART\0", "Ljava/time/format/ResolverStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LENIENT](https://developer.android.com/reference/java/time/format/ResolverStyle.html#LENIENT)
        ///
        /// Required feature: java-time-format-ResolverStyle
        #[cfg(any(feature = "all", feature = "java-time-format-ResolverStyle"))]
        pub fn LENIENT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::format::ResolverStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/format/ResolverStyle\0", "LENIENT\0", "Ljava/time/format/ResolverStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::time::format::ResolverStyle, crate::java::lang::Throwable>>> { ... }
    }
}
