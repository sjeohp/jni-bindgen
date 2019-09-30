// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-Spliterators"))]
__jni_bindgen! {
    /// public final class [Spliterators](https://developer.android.com/reference/java/util/Spliterators.html)
    ///
    /// Required feature: java-util-Spliterators
    public final class Spliterators ("java/util/Spliterators") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Spliterators](https://developer.android.com/reference/java/util/Spliterators.html#Spliterators())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::Spliterators>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/Spliterators", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [emptySpliterator](https://developer.android.com/reference/java/util/Spliterators.html#emptySpliterator())
        ///
        /// Required features: "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator")))]
        pub fn emptySpliterator<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "emptySpliterator", .descriptor == "()Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "emptySpliterator\0", "()Ljava/util/Spliterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [emptyIntSpliterator](https://developer.android.com/reference/java/util/Spliterators.html#emptyIntSpliterator())
        ///
        /// Required features: "java-util-Spliterator_OfInt"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfInt")))]
        pub fn emptyIntSpliterator<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfInt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "emptyIntSpliterator", .descriptor == "()Ljava/util/Spliterator$OfInt;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "emptyIntSpliterator\0", "()Ljava/util/Spliterator$OfInt;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [emptyLongSpliterator](https://developer.android.com/reference/java/util/Spliterators.html#emptyLongSpliterator())
        ///
        /// Required features: "java-util-Spliterator_OfLong"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfLong")))]
        pub fn emptyLongSpliterator<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfLong>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "emptyLongSpliterator", .descriptor == "()Ljava/util/Spliterator$OfLong;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "emptyLongSpliterator\0", "()Ljava/util/Spliterator$OfLong;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [emptyDoubleSpliterator](https://developer.android.com/reference/java/util/Spliterators.html#emptyDoubleSpliterator())
        ///
        /// Required features: "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfDouble")))]
        pub fn emptyDoubleSpliterator<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "emptyDoubleSpliterator", .descriptor == "()Ljava/util/Spliterator$OfDouble;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "emptyDoubleSpliterator\0", "()Ljava/util/Spliterator$OfDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.lang.Object%5B%5D,%20int))
        ///
        /// Required features: "java-lang-Object", "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-Spliterator")))]
        pub fn spliterator_Object_array_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([Ljava/lang/Object;I)Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([Ljava/lang/Object;I)Ljava/util/Spliterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.lang.Object%5B%5D,%20int,%20int,%20int))
        ///
        /// Required features: "java-lang-Object", "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-Spliterator")))]
        pub fn spliterator_Object_array_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([Ljava/lang/Object;III)Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([Ljava/lang/Object;III)Ljava/util/Spliterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(int%5B%5D,%20int))
        ///
        /// Required features: "java-util-Spliterator_OfInt"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfInt")))]
        pub fn spliterator_int_array_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfInt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([II)Ljava/util/Spliterator$OfInt;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([II)Ljava/util/Spliterator$OfInt;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(int%5B%5D,%20int,%20int,%20int))
        ///
        /// Required features: "java-util-Spliterator_OfInt"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfInt")))]
        pub fn spliterator_int_array_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfInt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([IIII)Ljava/util/Spliterator$OfInt;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([IIII)Ljava/util/Spliterator$OfInt;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(long%5B%5D,%20int))
        ///
        /// Required features: "java-util-Spliterator_OfLong"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfLong")))]
        pub fn spliterator_long_array_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfLong>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([JI)Ljava/util/Spliterator$OfLong;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([JI)Ljava/util/Spliterator$OfLong;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(long%5B%5D,%20int,%20int,%20int))
        ///
        /// Required features: "java-util-Spliterator_OfLong"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfLong")))]
        pub fn spliterator_long_array_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfLong>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([JIII)Ljava/util/Spliterator$OfLong;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([JIII)Ljava/util/Spliterator$OfLong;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(double%5B%5D,%20int))
        ///
        /// Required features: "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfDouble")))]
        pub fn spliterator_double_array_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::DoubleArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([DI)Ljava/util/Spliterator$OfDouble;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([DI)Ljava/util/Spliterator$OfDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(double%5B%5D,%20int,%20int,%20int))
        ///
        /// Required features: "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfDouble")))]
        pub fn spliterator_double_array_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::DoubleArray>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "([DIII)Ljava/util/Spliterator$OfDouble;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "([DIII)Ljava/util/Spliterator$OfDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.util.Collection,%20int))
        ///
        /// Required features: "java-util-Collection", "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection", feature = "java-util-Spliterator")))]
        pub fn spliterator_Collection_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "(Ljava/util/Collection;I)Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "(Ljava/util/Collection;I)Ljava/util/Spliterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.util.Iterator,%20long,%20int))
        ///
        /// Required features: "java-util-Iterator", "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator", feature = "java-util-Spliterator")))]
        pub fn spliterator_Iterator_long_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Iterator>>, arg1: i64, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "(Ljava/util/Iterator;JI)Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "(Ljava/util/Iterator;JI)Ljava/util/Spliterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliteratorUnknownSize](https://developer.android.com/reference/java/util/Spliterators.html#spliteratorUnknownSize(java.util.Iterator,%20int))
        ///
        /// Required features: "java-util-Iterator", "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator", feature = "java-util-Spliterator")))]
        pub fn spliteratorUnknownSize_Iterator_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Iterator>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliteratorUnknownSize", .descriptor == "(Ljava/util/Iterator;I)Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliteratorUnknownSize\0", "(Ljava/util/Iterator;I)Ljava/util/Spliterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.util.PrimitiveIterator.OfInt,%20long,%20int))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfInt", "java-util-Spliterator_OfInt"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfInt", feature = "java-util-Spliterator_OfInt")))]
        pub fn spliterator_OfInt_long_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::PrimitiveIterator_OfInt>>, arg1: i64, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfInt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliteratorUnknownSize](https://developer.android.com/reference/java/util/Spliterators.html#spliteratorUnknownSize(java.util.PrimitiveIterator.OfInt,%20int))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfInt", "java-util-Spliterator_OfInt"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfInt", feature = "java-util-Spliterator_OfInt")))]
        pub fn spliteratorUnknownSize_OfInt_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::PrimitiveIterator_OfInt>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfInt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliteratorUnknownSize", .descriptor == "(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliteratorUnknownSize\0", "(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.util.PrimitiveIterator.OfLong,%20long,%20int))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfLong", "java-util-Spliterator_OfLong"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfLong", feature = "java-util-Spliterator_OfLong")))]
        pub fn spliterator_OfLong_long_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::PrimitiveIterator_OfLong>>, arg1: i64, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfLong>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliteratorUnknownSize](https://developer.android.com/reference/java/util/Spliterators.html#spliteratorUnknownSize(java.util.PrimitiveIterator.OfLong,%20int))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfLong", "java-util-Spliterator_OfLong"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfLong", feature = "java-util-Spliterator_OfLong")))]
        pub fn spliteratorUnknownSize_OfLong_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::PrimitiveIterator_OfLong>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfLong>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliteratorUnknownSize", .descriptor == "(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliteratorUnknownSize\0", "(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/Spliterators.html#spliterator(java.util.PrimitiveIterator.OfDouble,%20long,%20int))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfDouble", "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfDouble", feature = "java-util-Spliterator_OfDouble")))]
        pub fn spliterator_OfDouble_long_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::PrimitiveIterator_OfDouble>>, arg1: i64, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliterator", .descriptor == "(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliterator\0", "(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliteratorUnknownSize](https://developer.android.com/reference/java/util/Spliterators.html#spliteratorUnknownSize(java.util.PrimitiveIterator.OfDouble,%20int))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfDouble", "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfDouble", feature = "java-util-Spliterator_OfDouble")))]
        pub fn spliteratorUnknownSize_OfDouble_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::PrimitiveIterator_OfDouble>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "spliteratorUnknownSize", .descriptor == "(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "spliteratorUnknownSize\0", "(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/Spliterators.html#iterator(java.util.Spliterator))
        ///
        /// Required features: "java-util-Iterator", "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator", feature = "java-util-Spliterator")))]
        pub fn iterator_Spliterator<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Spliterator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Iterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "iterator", .descriptor == "(Ljava/util/Spliterator;)Ljava/util/Iterator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "iterator\0", "(Ljava/util/Spliterator;)Ljava/util/Iterator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/Spliterators.html#iterator(java.util.Spliterator.OfInt))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfInt", "java-util-Spliterator_OfInt"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfInt", feature = "java-util-Spliterator_OfInt")))]
        pub fn iterator_OfInt<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Spliterator_OfInt>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::PrimitiveIterator_OfInt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "iterator", .descriptor == "(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "iterator\0", "(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/Spliterators.html#iterator(java.util.Spliterator.OfLong))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfLong", "java-util-Spliterator_OfLong"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfLong", feature = "java-util-Spliterator_OfLong")))]
        pub fn iterator_OfLong<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Spliterator_OfLong>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::PrimitiveIterator_OfLong>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "iterator", .descriptor == "(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "iterator\0", "(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/Spliterators.html#iterator(java.util.Spliterator.OfDouble))
        ///
        /// Required features: "java-util-PrimitiveIterator_OfDouble", "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-PrimitiveIterator_OfDouble", feature = "java-util-Spliterator_OfDouble")))]
        pub fn iterator_OfDouble<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Spliterator_OfDouble>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::PrimitiveIterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators", java.flags == PUBLIC | STATIC, .name == "iterator", .descriptor == "(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/Spliterators\0", "iterator\0", "(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
