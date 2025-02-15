// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-Property"))]
__jni_bindgen! {
    /// public class [Property](https://developer.android.com/reference/android/util/Property.html)
    ///
    /// Required feature: android-util-Property
    public class Property ("android/util/Property") extends crate::java::lang::Object {

        /// [Property](https://developer.android.com/reference/android/util/Property.html#Property(java.lang.Class,%20java.lang.String))
        ///
        /// Required features: "java-lang-Class", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::Property>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Class;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Property\0", "<init>\0", "(Ljava/lang/Class;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [of](https://developer.android.com/reference/android/util/Property.html#of(java.lang.Class,%20java.lang.Class,%20java.lang.String))
        ///
        /// Required features: "android-util-Property", "java-lang-Class", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-util-Property", feature = "java-lang-Class", feature = "java-lang-String")))]
        pub fn of<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::Property>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC | STATIC, .name == "of", .descriptor == "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/String;)Landroid/util/Property;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/Property\0", "of\0", "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/String;)Landroid/util/Property;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isReadOnly](https://developer.android.com/reference/android/util/Property.html#isReadOnly())
        pub fn isReadOnly<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC, .name == "isReadOnly", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Property\0", "isReadOnly\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/android/util/Property.html#set(java.lang.Object,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn set<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC, .name == "set", .descriptor == "(Ljava/lang/Object;Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Property\0", "set\0", "(Ljava/lang/Object;Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/android/util/Property.html#get(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC | ABSTRACT, .name == "get", .descriptor == "(Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Property\0", "get\0", "(Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/android/util/Property.html#getName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC, .name == "getName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Property\0", "getName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getType](https://developer.android.com/reference/android/util/Property.html#getType())
        ///
        /// Required features: "java-lang-Class"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class")))]
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Class>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Property", java.flags == PUBLIC, .name == "getType", .descriptor == "()Ljava/lang/Class;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Property\0", "getType\0", "()Ljava/lang/Class;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
