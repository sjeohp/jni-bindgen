// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-List"))]
__jni_bindgen! {
    /// public interface [List](https://developer.android.com/reference/java/util/List.html)
    ///
    /// Required feature: java-util-List
    public interface List ("java/util/List") extends crate::java::lang::Object, implements crate::java::util::Collection {

        /// [size](https://developer.android.com/reference/java/util/List.html#size())
        pub fn size<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "size", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "size\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEmpty](https://developer.android.com/reference/java/util/List.html#isEmpty())
        pub fn isEmpty<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "isEmpty", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "isEmpty\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [contains](https://developer.android.com/reference/java/util/List.html#contains(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn contains<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "contains", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "contains\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/List.html#iterator())
        ///
        /// Required features: "java-util-Iterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator")))]
        pub fn iterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Iterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "iterator", .descriptor == "()Ljava/util/Iterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "iterator\0", "()Ljava/util/Iterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toArray](https://developer.android.com/reference/java/util/List.html#toArray())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn toArray<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "toArray", .descriptor == "()[Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "toArray\0", "()[Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toArray](https://developer.android.com/reference/java/util/List.html#toArray(java.lang.Object%5B%5D))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn toArray_Object_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "toArray", .descriptor == "([Ljava/lang/Object;)[Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "toArray\0", "([Ljava/lang/Object;)[Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/List.html#add(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn add_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "add", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "add\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remove](https://developer.android.com/reference/java/util/List.html#remove(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn remove_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "remove", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "remove\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [containsAll](https://developer.android.com/reference/java/util/List.html#containsAll(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn containsAll<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "containsAll", .descriptor == "(Ljava/util/Collection;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "containsAll\0", "(Ljava/util/Collection;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAll](https://developer.android.com/reference/java/util/List.html#addAll(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn addAll_Collection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "addAll", .descriptor == "(Ljava/util/Collection;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "addAll\0", "(Ljava/util/Collection;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAll](https://developer.android.com/reference/java/util/List.html#addAll(int,%20java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn addAll_int_Collection<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "addAll", .descriptor == "(ILjava/util/Collection;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "addAll\0", "(ILjava/util/Collection;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeAll](https://developer.android.com/reference/java/util/List.html#removeAll(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn removeAll<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "removeAll", .descriptor == "(Ljava/util/Collection;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "removeAll\0", "(Ljava/util/Collection;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [retainAll](https://developer.android.com/reference/java/util/List.html#retainAll(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn retainAll<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "retainAll", .descriptor == "(Ljava/util/Collection;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "retainAll\0", "(Ljava/util/Collection;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [replaceAll](https://developer.android.com/reference/java/util/List.html#replaceAll(java.util.function.UnaryOperator))
        ///
        /// Required features: "java-util-function-UnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-UnaryOperator")))]
        pub fn replaceAll<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::UnaryOperator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC, .name == "replaceAll", .descriptor == "(Ljava/util/function/UnaryOperator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "replaceAll\0", "(Ljava/util/function/UnaryOperator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sort](https://developer.android.com/reference/java/util/List.html#sort(java.util.Comparator))
        ///
        /// Required features: "java-util-Comparator"
        #[cfg(any(feature = "all", all(feature = "java-util-Comparator")))]
        pub fn sort<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Comparator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC, .name == "sort", .descriptor == "(Ljava/util/Comparator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "sort\0", "(Ljava/util/Comparator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/java/util/List.html#clear())
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "clear", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "clear\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/util/List.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/util/List.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/util/List.html#get(int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "get", .descriptor == "(I)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "get\0", "(I)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/java/util/List.html#set(int,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn set<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "set", .descriptor == "(ILjava/lang/Object;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "set\0", "(ILjava/lang/Object;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/List.html#add(int,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn add_int_Object<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "add", .descriptor == "(ILjava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "add\0", "(ILjava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remove](https://developer.android.com/reference/java/util/List.html#remove(int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn remove_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "remove", .descriptor == "(I)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "remove\0", "(I)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [indexOf](https://developer.android.com/reference/java/util/List.html#indexOf(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn indexOf<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "indexOf", .descriptor == "(Ljava/lang/Object;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "indexOf\0", "(Ljava/lang/Object;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lastIndexOf](https://developer.android.com/reference/java/util/List.html#lastIndexOf(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn lastIndexOf<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "lastIndexOf", .descriptor == "(Ljava/lang/Object;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "lastIndexOf\0", "(Ljava/lang/Object;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [listIterator](https://developer.android.com/reference/java/util/List.html#listIterator())
        ///
        /// Required features: "java-util-ListIterator"
        #[cfg(any(feature = "all", all(feature = "java-util-ListIterator")))]
        pub fn listIterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ListIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "listIterator", .descriptor == "()Ljava/util/ListIterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "listIterator\0", "()Ljava/util/ListIterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [listIterator](https://developer.android.com/reference/java/util/List.html#listIterator(int))
        ///
        /// Required features: "java-util-ListIterator"
        #[cfg(any(feature = "all", all(feature = "java-util-ListIterator")))]
        pub fn listIterator_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ListIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "listIterator", .descriptor == "(I)Ljava/util/ListIterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "listIterator\0", "(I)Ljava/util/ListIterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [subList](https://developer.android.com/reference/java/util/List.html#subList(int,%20int))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn subList<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC | ABSTRACT, .name == "subList", .descriptor == "(II)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "subList\0", "(II)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/List.html#spliterator())
        ///
        /// Required features: "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator")))]
        pub fn spliterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/List", java.flags == PUBLIC, .name == "spliterator", .descriptor == "()Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/List\0", "spliterator\0", "()Ljava/util/Spliterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
