// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-ProcessBuilder_Redirect"))]
__jni_bindgen! {
    /// public class [ProcessBuilder.Redirect](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html)
    ///
    /// Required feature: java-lang-ProcessBuilder_Redirect
    public class ProcessBuilder_Redirect ("java/lang/ProcessBuilder$Redirect") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Redirect](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#Redirect())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ProcessBuilder$Redirect\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [type](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#type())
        ///
        /// Required features: "java-lang-ProcessBuilder_Redirect_Type"
        #[cfg(any(feature = "all", all(feature = "java-lang-ProcessBuilder_Redirect_Type")))]
        pub fn r#type<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect_Type>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC | ABSTRACT, .name == "type", .descriptor == "()Ljava/lang/ProcessBuilder$Redirect$Type;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ProcessBuilder$Redirect\0", "type\0", "()Ljava/lang/ProcessBuilder$Redirect$Type;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [file](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#file())
        ///
        /// Required features: "java-io-File"
        #[cfg(any(feature = "all", all(feature = "java-io-File")))]
        pub fn file<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::File>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC, .name == "file", .descriptor == "()Ljava/io/File;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ProcessBuilder$Redirect\0", "file\0", "()Ljava/io/File;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [from](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#from(java.io.File))
        ///
        /// Required features: "java-io-File", "java-lang-ProcessBuilder_Redirect"
        #[cfg(any(feature = "all", all(feature = "java-io-File", feature = "java-lang-ProcessBuilder_Redirect")))]
        pub fn from<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC | STATIC, .name == "from", .descriptor == "(Ljava/io/File;)Ljava/lang/ProcessBuilder$Redirect;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/ProcessBuilder$Redirect\0", "from\0", "(Ljava/io/File;)Ljava/lang/ProcessBuilder$Redirect;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [to](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#to(java.io.File))
        ///
        /// Required features: "java-io-File", "java-lang-ProcessBuilder_Redirect"
        #[cfg(any(feature = "all", all(feature = "java-io-File", feature = "java-lang-ProcessBuilder_Redirect")))]
        pub fn to<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC | STATIC, .name == "to", .descriptor == "(Ljava/io/File;)Ljava/lang/ProcessBuilder$Redirect;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/ProcessBuilder$Redirect\0", "to\0", "(Ljava/io/File;)Ljava/lang/ProcessBuilder$Redirect;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [appendTo](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#appendTo(java.io.File))
        ///
        /// Required features: "java-io-File", "java-lang-ProcessBuilder_Redirect"
        #[cfg(any(feature = "all", all(feature = "java-io-File", feature = "java-lang-ProcessBuilder_Redirect")))]
        pub fn appendTo<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC | STATIC, .name == "appendTo", .descriptor == "(Ljava/io/File;)Ljava/lang/ProcessBuilder$Redirect;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/ProcessBuilder$Redirect\0", "appendTo\0", "(Ljava/io/File;)Ljava/lang/ProcessBuilder$Redirect;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ProcessBuilder$Redirect\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ProcessBuilder$Redirect", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ProcessBuilder$Redirect\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [INHERIT](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#INHERIT)
        ///
        /// Required feature: java-lang-ProcessBuilder_Redirect
        #[cfg(any(feature = "all", feature = "java-lang-ProcessBuilder_Redirect"))]
        pub fn INHERIT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/lang/ProcessBuilder$Redirect\0", "INHERIT\0", "Ljava/lang/ProcessBuilder$Redirect;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIPE](https://developer.android.com/reference/java/lang/ProcessBuilder.Redirect.html#PIPE)
        ///
        /// Required feature: java-lang-ProcessBuilder_Redirect
        #[cfg(any(feature = "all", feature = "java-lang-ProcessBuilder_Redirect"))]
        pub fn PIPE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::ProcessBuilder_Redirect>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/lang/ProcessBuilder$Redirect\0", "PIPE\0", "Ljava/lang/ProcessBuilder$Redirect;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
