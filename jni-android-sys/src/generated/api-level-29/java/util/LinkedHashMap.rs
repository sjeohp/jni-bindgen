// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-LinkedHashMap"))]
__jni_bindgen! {
    /// public class [LinkedHashMap](https://developer.android.com/reference/java/util/LinkedHashMap.html)
    ///
    /// Required feature: java-util-LinkedHashMap
    public class LinkedHashMap ("java/util/LinkedHashMap") extends crate::java::util::HashMap, implements crate::java::util::Map {

        /// [LinkedHashMap](https://developer.android.com/reference/java/util/LinkedHashMap.html#LinkedHashMap(int,%20float))
        pub fn new_int_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::LinkedHashMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "<init>\0", "(IF)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LinkedHashMap](https://developer.android.com/reference/java/util/LinkedHashMap.html#LinkedHashMap(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::LinkedHashMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LinkedHashMap](https://developer.android.com/reference/java/util/LinkedHashMap.html#LinkedHashMap())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::LinkedHashMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LinkedHashMap](https://developer.android.com/reference/java/util/LinkedHashMap.html#LinkedHashMap(java.util.Map))
        ///
        /// Required features: "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-util-Map")))]
        pub fn new_Map<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::LinkedHashMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Map;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "<init>\0", "(Ljava/util/Map;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LinkedHashMap](https://developer.android.com/reference/java/util/LinkedHashMap.html#LinkedHashMap(int,%20float,%20boolean))
        pub fn new_int_float_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: f32, arg2: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::LinkedHashMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IFZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "<init>\0", "(IFZ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [containsValue](https://developer.android.com/reference/java/util/LinkedHashMap.html#containsValue(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn containsValue<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "containsValue", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "containsValue\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/util/LinkedHashMap.html#get(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "get", .descriptor == "(Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "get\0", "(Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOrDefault](https://developer.android.com/reference/java/util/LinkedHashMap.html#getOrDefault(java.lang.Object,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getOrDefault<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "getOrDefault", .descriptor == "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "getOrDefault\0", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/java/util/LinkedHashMap.html#clear())
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "clear", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "clear\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [removeEldestEntry](https://developer.android.com/reference/java/util/LinkedHashMap.html#removeEldestEntry(java.util.Map.Entry))
        // ///
        // /// Required features: "java-util-Map_Entry"
        // #[cfg(any(feature = "all", all(feature = "java-util-Map_Entry")))]
        // fn removeEldestEntry<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map_Entry>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/LinkedHashMap", java.flags == PROTECTED, .name == "removeEldestEntry", .descriptor == "(Ljava/util/Map$Entry;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "removeEldestEntry\0", "(Ljava/util/Map$Entry;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [keySet](https://developer.android.com/reference/java/util/LinkedHashMap.html#keySet())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn keySet<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "keySet", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "keySet\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [values](https://developer.android.com/reference/java/util/LinkedHashMap.html#values())
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn values<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "values", .descriptor == "()Ljava/util/Collection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "values\0", "()Ljava/util/Collection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [entrySet](https://developer.android.com/reference/java/util/LinkedHashMap.html#entrySet())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn entrySet<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "entrySet", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "entrySet\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEach](https://developer.android.com/reference/java/util/LinkedHashMap.html#forEach(java.util.function.BiConsumer))
        ///
        /// Required features: "java-util-function-BiConsumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-BiConsumer")))]
        pub fn forEach<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::BiConsumer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "forEach", .descriptor == "(Ljava/util/function/BiConsumer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "forEach\0", "(Ljava/util/function/BiConsumer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [replaceAll](https://developer.android.com/reference/java/util/LinkedHashMap.html#replaceAll(java.util.function.BiFunction))
        ///
        /// Required features: "java-util-function-BiFunction"
        #[cfg(any(feature = "all", all(feature = "java-util-function-BiFunction")))]
        pub fn replaceAll<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::BiFunction>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LinkedHashMap", java.flags == PUBLIC, .name == "replaceAll", .descriptor == "(Ljava/util/function/BiFunction;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LinkedHashMap\0", "replaceAll\0", "(Ljava/util/function/BiFunction;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
