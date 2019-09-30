// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-LinkOption"))]
__jni_bindgen! {
    /// public enum [LinkOption](https://developer.android.com/reference/java/nio/file/LinkOption.html)
    ///
    /// Required feature: java-nio-file-LinkOption
    public enum LinkOption ("java/nio/file/LinkOption") extends crate::java::lang::Enum, implements crate::java::nio::file::OpenOption, crate::java::nio::file::CopyOption {

        /// [values](https://developer.android.com/reference/java/nio/file/LinkOption.html#values())
        ///
        /// Required features: "java-nio-file-LinkOption"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-LinkOption")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::nio::file::LinkOption, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/LinkOption", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/nio/file/LinkOption;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/file/LinkOption\0", "values\0", "()[Ljava/nio/file/LinkOption;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/nio/file/LinkOption.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-file-LinkOption"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-file-LinkOption")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::LinkOption>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/LinkOption", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/nio/file/LinkOption;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/file/LinkOption\0", "valueOf\0", "(Ljava/lang/String;)Ljava/nio/file/LinkOption;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [LinkOption](https://developer.android.com/reference/java/nio/file/LinkOption.html#LinkOption(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::file::LinkOption>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/file/LinkOption", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/LinkOption\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [NOFOLLOW_LINKS](https://developer.android.com/reference/java/nio/file/LinkOption.html#NOFOLLOW_LINKS)
        ///
        /// Required feature: java-nio-file-LinkOption
        #[cfg(any(feature = "all", feature = "java-nio-file-LinkOption"))]
        pub fn NOFOLLOW_LINKS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::LinkOption>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/nio/file/LinkOption\0", "NOFOLLOW_LINKS\0", "Ljava/nio/file/LinkOption;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::nio::file::LinkOption, crate::java::lang::Throwable>>> { ... }
    }
}
