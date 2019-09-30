// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-EnumMap"))]
__jni_bindgen! {
    /// public class [EnumMap](https://developer.android.com/reference/java/util/EnumMap.html)
    ///
    /// Required feature: java-util-EnumMap
    public class EnumMap ("java/util/EnumMap") extends crate::java::util::AbstractMap, implements crate::java::io::Serializable, crate::java::lang::Cloneable {

        /// [EnumMap](https://developer.android.com/reference/java/util/EnumMap.html#EnumMap(java.lang.Class))
        ///
        /// Required features: "java-lang-Class"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class")))]
        pub fn new_Class<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::EnumMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Class;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "<init>\0", "(Ljava/lang/Class;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [EnumMap](https://developer.android.com/reference/java/util/EnumMap.html#EnumMap(java.util.EnumMap))
        ///
        /// Required features: "java-util-EnumMap"
        #[cfg(any(feature = "all", all(feature = "java-util-EnumMap")))]
        pub fn new_EnumMap<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::EnumMap>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::EnumMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/EnumMap;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "<init>\0", "(Ljava/util/EnumMap;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [EnumMap](https://developer.android.com/reference/java/util/EnumMap.html#EnumMap(java.util.Map))
        ///
        /// Required features: "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-util-Map")))]
        pub fn new_Map<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::EnumMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Map;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "<init>\0", "(Ljava/util/Map;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [size](https://developer.android.com/reference/java/util/EnumMap.html#size())
        pub fn size<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "size", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "size\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [containsValue](https://developer.android.com/reference/java/util/EnumMap.html#containsValue(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn containsValue<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "containsValue", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "containsValue\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [containsKey](https://developer.android.com/reference/java/util/EnumMap.html#containsKey(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn containsKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "containsKey", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "containsKey\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/util/EnumMap.html#get(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "get", .descriptor == "(Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "get\0", "(Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/util/EnumMap.html#put(java.lang.Enum,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Enum", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Enum", feature = "java-lang-Object")))]
        pub fn put_Enum_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Enum>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "put", .descriptor == "(Ljava/lang/Enum;Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "put\0", "(Ljava/lang/Enum;Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remove](https://developer.android.com/reference/java/util/EnumMap.html#remove(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn remove<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "remove", .descriptor == "(Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "remove\0", "(Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putAll](https://developer.android.com/reference/java/util/EnumMap.html#putAll(java.util.Map))
        ///
        /// Required features: "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-util-Map")))]
        pub fn putAll<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "putAll", .descriptor == "(Ljava/util/Map;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "putAll\0", "(Ljava/util/Map;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/java/util/EnumMap.html#clear())
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "clear", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "clear\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [keySet](https://developer.android.com/reference/java/util/EnumMap.html#keySet())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn keySet<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "keySet", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "keySet\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [values](https://developer.android.com/reference/java/util/EnumMap.html#values())
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn values<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "values", .descriptor == "()Ljava/util/Collection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "values\0", "()Ljava/util/Collection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [entrySet](https://developer.android.com/reference/java/util/EnumMap.html#entrySet())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn entrySet<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "entrySet", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "entrySet\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/util/EnumMap.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/util/EnumMap.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/java/util/EnumMap.html#clone())
        ///
        /// Required features: "java-util-EnumMap"
        #[cfg(any(feature = "all", all(feature = "java-util-EnumMap")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::EnumMap>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/EnumMap", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/util/EnumMap;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "clone\0", "()Ljava/util/EnumMap;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [clone](https://developer.android.com/reference/java/util/EnumMap.html#clone())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/EnumMap", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "clone\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [put](https://developer.android.com/reference/java/util/EnumMap.html#put(java.lang.Object,%20java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn put_Object_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/EnumMap", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "put", .descriptor == "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/EnumMap\0", "put\0", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
