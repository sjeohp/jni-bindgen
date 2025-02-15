// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-PrimitiveIterator_OfLong"))]
__jni_bindgen! {
    /// public interface [PrimitiveIterator.OfLong](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html)
    ///
    /// Required feature: java-util-PrimitiveIterator_OfLong
    public interface PrimitiveIterator_OfLong ("java/util/PrimitiveIterator$OfLong") extends crate::java::lang::Object, implements crate::java::util::PrimitiveIterator {

        /// [nextLong](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html#nextLong())
        pub fn nextLong<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfLong", java.flags == PUBLIC | ABSTRACT, .name == "nextLong", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfLong\0", "nextLong\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html#forEachRemaining(java.util.function.LongConsumer))
        ///
        /// Required features: "java-util-function-LongConsumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-LongConsumer")))]
        pub fn forEachRemaining_LongConsumer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::LongConsumer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfLong", java.flags == PUBLIC, .name == "forEachRemaining", .descriptor == "(Ljava/util/function/LongConsumer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfLong\0", "forEachRemaining\0", "(Ljava/util/function/LongConsumer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [next](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html#next())
        ///
        /// Required features: "java-lang-Long"
        #[cfg(any(feature = "all", all(feature = "java-lang-Long")))]
        pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Long>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfLong", java.flags == PUBLIC, .name == "next", .descriptor == "()Ljava/lang/Long;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfLong\0", "next\0", "()Ljava/lang/Long;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html#forEachRemaining(java.util.function.Consumer))
        ///
        /// Required features: "java-util-function-Consumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-Consumer")))]
        pub fn forEachRemaining_Consumer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::Consumer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/PrimitiveIterator$OfLong", java.flags == PUBLIC, .name == "forEachRemaining", .descriptor == "(Ljava/util/function/Consumer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfLong\0", "forEachRemaining\0", "(Ljava/util/function/Consumer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [forEachRemaining](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html#forEachRemaining(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn forEachRemaining_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/PrimitiveIterator$OfLong", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "forEachRemaining", .descriptor == "(Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfLong\0", "forEachRemaining\0", "(Ljava/lang/Object;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [next](https://developer.android.com/reference/java/util/PrimitiveIterator.OfLong.html#next())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/PrimitiveIterator$OfLong", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "next", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/PrimitiveIterator$OfLong\0", "next\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
