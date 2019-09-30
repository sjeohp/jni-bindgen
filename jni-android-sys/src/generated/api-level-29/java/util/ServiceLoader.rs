// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-ServiceLoader"))]
__jni_bindgen! {
    /// public final class [ServiceLoader](https://developer.android.com/reference/java/util/ServiceLoader.html)
    ///
    /// Required feature: java-util-ServiceLoader
    public final class ServiceLoader ("java/util/ServiceLoader") extends crate::java::lang::Object, implements crate::java::lang::Iterable {

        // // Not emitting: Non-public method
        // /// [ServiceLoader](https://developer.android.com/reference/java/util/ServiceLoader.html#ServiceLoader(java.lang.Class,%20java.lang.ClassLoader))
        // ///
        // /// Required features: "java-lang-Class", "java-lang-ClassLoader"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-ClassLoader")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::ServiceLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/ServiceLoader", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/lang/Class;Ljava/lang/ClassLoader;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ServiceLoader\0", "<init>\0", "(Ljava/lang/Class;Ljava/lang/ClassLoader;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [reload](https://developer.android.com/reference/java/util/ServiceLoader.html#reload())
        pub fn reload<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ServiceLoader", java.flags == PUBLIC, .name == "reload", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ServiceLoader\0", "reload\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/ServiceLoader.html#iterator())
        ///
        /// Required features: "java-util-Iterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator")))]
        pub fn iterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Iterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ServiceLoader", java.flags == PUBLIC, .name == "iterator", .descriptor == "()Ljava/util/Iterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ServiceLoader\0", "iterator\0", "()Ljava/util/Iterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [load](https://developer.android.com/reference/java/util/ServiceLoader.html#load(java.lang.Class,%20java.lang.ClassLoader))
        ///
        /// Required features: "java-lang-Class", "java-lang-ClassLoader", "java-util-ServiceLoader"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-ClassLoader", feature = "java-util-ServiceLoader")))]
        pub fn load_Class_ClassLoader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ServiceLoader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ServiceLoader", java.flags == PUBLIC | STATIC, .name == "load", .descriptor == "(Ljava/lang/Class;Ljava/lang/ClassLoader;)Ljava/util/ServiceLoader;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/ServiceLoader\0", "load\0", "(Ljava/lang/Class;Ljava/lang/ClassLoader;)Ljava/util/ServiceLoader;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [load](https://developer.android.com/reference/java/util/ServiceLoader.html#load(java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-util-ServiceLoader"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-util-ServiceLoader")))]
        pub fn load_Class<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ServiceLoader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ServiceLoader", java.flags == PUBLIC | STATIC, .name == "load", .descriptor == "(Ljava/lang/Class;)Ljava/util/ServiceLoader;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/ServiceLoader\0", "load\0", "(Ljava/lang/Class;)Ljava/util/ServiceLoader;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadInstalled](https://developer.android.com/reference/java/util/ServiceLoader.html#loadInstalled(java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-util-ServiceLoader"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-util-ServiceLoader")))]
        pub fn loadInstalled<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ServiceLoader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ServiceLoader", java.flags == PUBLIC | STATIC, .name == "loadInstalled", .descriptor == "(Ljava/lang/Class;)Ljava/util/ServiceLoader;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/ServiceLoader\0", "loadInstalled\0", "(Ljava/lang/Class;)Ljava/util/ServiceLoader;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/ServiceLoader.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/ServiceLoader", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/ServiceLoader\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
