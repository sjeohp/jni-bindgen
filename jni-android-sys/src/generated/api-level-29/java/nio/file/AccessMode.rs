// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-AccessMode"))]
__jni_bindgen! {
    /// public enum [AccessMode](https://developer.android.com/reference/java/nio/file/AccessMode.html)
    ///
    /// Required feature: java-nio-file-AccessMode
    public enum AccessMode ("java/nio/file/AccessMode") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/nio/file/AccessMode.html#values())
        ///
        /// Required features: "java-nio-file-AccessMode"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-AccessMode")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::nio::file::AccessMode, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/AccessMode", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/nio/file/AccessMode;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/file/AccessMode\0", "values\0", "()[Ljava/nio/file/AccessMode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/nio/file/AccessMode.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-file-AccessMode"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-file-AccessMode")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::AccessMode>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/AccessMode", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/nio/file/AccessMode;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/file/AccessMode\0", "valueOf\0", "(Ljava/lang/String;)Ljava/nio/file/AccessMode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [AccessMode](https://developer.android.com/reference/java/nio/file/AccessMode.html#AccessMode(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::file::AccessMode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/file/AccessMode", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/AccessMode\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [READ](https://developer.android.com/reference/java/nio/file/AccessMode.html#READ)
        ///
        /// Required feature: java-nio-file-AccessMode
        #[cfg(any(feature = "all", feature = "java-nio-file-AccessMode"))]
        pub fn READ<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::AccessMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/nio/file/AccessMode\0", "READ\0", "Ljava/nio/file/AccessMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [WRITE](https://developer.android.com/reference/java/nio/file/AccessMode.html#WRITE)
        ///
        /// Required feature: java-nio-file-AccessMode
        #[cfg(any(feature = "all", feature = "java-nio-file-AccessMode"))]
        pub fn WRITE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::AccessMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/nio/file/AccessMode\0", "WRITE\0", "Ljava/nio/file/AccessMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXECUTE](https://developer.android.com/reference/java/nio/file/AccessMode.html#EXECUTE)
        ///
        /// Required feature: java-nio-file-AccessMode
        #[cfg(any(feature = "all", feature = "java-nio-file-AccessMode"))]
        pub fn EXECUTE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::AccessMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/nio/file/AccessMode\0", "EXECUTE\0", "Ljava/nio/file/AccessMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::nio::file::AccessMode, crate::java::lang::Throwable>>> { ... }
    }
}
