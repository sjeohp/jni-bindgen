// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-Dictionary"))]
__jni_bindgen! {
    /// public class [Dictionary](https://developer.android.com/reference/java/util/Dictionary.html)
    ///
    /// Required feature: java-util-Dictionary
    public class Dictionary ("java/util/Dictionary") extends crate::java::lang::Object {

        /// [Dictionary](https://developer.android.com/reference/java/util/Dictionary.html#Dictionary())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::Dictionary>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [size](https://developer.android.com/reference/java/util/Dictionary.html#size())
        pub fn size<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "size", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "size\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEmpty](https://developer.android.com/reference/java/util/Dictionary.html#isEmpty())
        pub fn isEmpty<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "isEmpty", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "isEmpty\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [keys](https://developer.android.com/reference/java/util/Dictionary.html#keys())
        ///
        /// Required features: "java-util-Enumeration"
        #[cfg(any(feature = "all", all(feature = "java-util-Enumeration")))]
        pub fn keys<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Enumeration>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "keys", .descriptor == "()Ljava/util/Enumeration;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "keys\0", "()Ljava/util/Enumeration;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [elements](https://developer.android.com/reference/java/util/Dictionary.html#elements())
        ///
        /// Required features: "java-util-Enumeration"
        #[cfg(any(feature = "all", all(feature = "java-util-Enumeration")))]
        pub fn elements<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Enumeration>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "elements", .descriptor == "()Ljava/util/Enumeration;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "elements\0", "()Ljava/util/Enumeration;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/util/Dictionary.html#get(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "get", .descriptor == "(Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "get\0", "(Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/util/Dictionary.html#put(java.lang.Object,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn put<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "put", .descriptor == "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "put\0", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remove](https://developer.android.com/reference/java/util/Dictionary.html#remove(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn remove<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Dictionary", java.flags == PUBLIC | ABSTRACT, .name == "remove", .descriptor == "(Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Dictionary\0", "remove\0", "(Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
