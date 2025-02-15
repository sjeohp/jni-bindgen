// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-PrimitiveIterator_OfInt"))]
__jni_bindgen! {
    /// public interface [PrimitiveIterator.OfInt](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html)
    ///
    /// Required feature: java-util-PrimitiveIterator_OfInt
    public interface PrimitiveIterator_OfInt ("java/util/PrimitiveIterator$OfInt") extends crate::java::lang::Object, implements crate::java::util::PrimitiveIterator {

        /// [nextInt](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html#nextInt())
        pub fn nextInt<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfInt", java.flags == PUBLIC | ABSTRACT, .name == "nextInt", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfInt\0", "nextInt\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html#forEachRemaining(java.util.function.IntConsumer))
        ///
        /// Required features: "java-util-function-IntConsumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-IntConsumer")))]
        pub fn forEachRemaining_IntConsumer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::IntConsumer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfInt", java.flags == PUBLIC, .name == "forEachRemaining", .descriptor == "(Ljava/util/function/IntConsumer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfInt\0", "forEachRemaining\0", "(Ljava/util/function/IntConsumer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [next](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html#next())
        ///
        /// Required features: "java-lang-Integer"
        #[cfg(any(feature = "all", all(feature = "java-lang-Integer")))]
        pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Integer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfInt", java.flags == PUBLIC, .name == "next", .descriptor == "()Ljava/lang/Integer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfInt\0", "next\0", "()Ljava/lang/Integer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html#forEachRemaining(java.util.function.Consumer))
        ///
        /// Required features: "java-util-function-Consumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-Consumer")))]
        pub fn forEachRemaining_Consumer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::Consumer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfInt", java.flags == PUBLIC, .name == "forEachRemaining", .descriptor == "(Ljava/util/function/Consumer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfInt\0", "forEachRemaining\0", "(Ljava/util/function/Consumer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html#forEachRemaining(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn forEachRemaining_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/PrimitiveIterator$OfInt", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "forEachRemaining", .descriptor == "(Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfInt\0", "forEachRemaining\0", "(Ljava/lang/Object;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [next](https://developer.android.com/reference/java/util/PrimitiveIterator.OfInt.html#next())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/PrimitiveIterator$OfInt", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "next", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfInt\0", "next\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
