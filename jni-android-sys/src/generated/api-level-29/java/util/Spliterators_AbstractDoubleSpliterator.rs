// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-Spliterators_AbstractDoubleSpliterator"))]
__jni_bindgen! {
    /// public class [Spliterators.AbstractDoubleSpliterator](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html)
    ///
    /// Required feature: java-util-Spliterators_AbstractDoubleSpliterator
    public class Spliterators_AbstractDoubleSpliterator ("java/util/Spliterators$AbstractDoubleSpliterator") extends crate::java::lang::Object, implements crate::java::util::Spliterator_OfDouble {

        // // Not emitting: Non-public method
        // /// [AbstractDoubleSpliterator](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html#AbstractDoubleSpliterator(long,%20int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::Spliterators_AbstractDoubleSpliterator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/Spliterators$AbstractDoubleSpliterator", java.flags == PROTECTED, .name == "<init>", .descriptor == "(JI)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators$AbstractDoubleSpliterator\0", "<init>\0", "(JI)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [trySplit](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html#trySplit())
        ///
        /// Required features: "java-util-Spliterator_OfDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfDouble")))]
        pub fn trySplit<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators$AbstractDoubleSpliterator", java.flags == PUBLIC, .name == "trySplit", .descriptor == "()Ljava/util/Spliterator$OfDouble;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators$AbstractDoubleSpliterator\0", "trySplit\0", "()Ljava/util/Spliterator$OfDouble;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [estimateSize](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html#estimateSize())
        pub fn estimateSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators$AbstractDoubleSpliterator", java.flags == PUBLIC, .name == "estimateSize", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators$AbstractDoubleSpliterator\0", "estimateSize\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [characteristics](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html#characteristics())
        pub fn characteristics<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Spliterators$AbstractDoubleSpliterator", java.flags == PUBLIC, .name == "characteristics", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators$AbstractDoubleSpliterator\0", "characteristics\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [trySplit](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html#trySplit())
        // ///
        // /// Required features: "java-util-Spliterator_OfPrimitive"
        // #[cfg(any(feature = "all", all(feature = "java-util-Spliterator_OfPrimitive")))]
        // pub fn trySplit<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator_OfPrimitive>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/Spliterators$AbstractDoubleSpliterator", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "trySplit", .descriptor == "()Ljava/util/Spliterator$OfPrimitive;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators$AbstractDoubleSpliterator\0", "trySplit\0", "()Ljava/util/Spliterator$OfPrimitive;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [trySplit](https://developer.android.com/reference/java/util/Spliterators.AbstractDoubleSpliterator.html#trySplit())
        // ///
        // /// Required features: "java-util-Spliterator"
        // #[cfg(any(feature = "all", all(feature = "java-util-Spliterator")))]
        // pub fn trySplit<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/Spliterators$AbstractDoubleSpliterator", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "trySplit", .descriptor == "()Ljava/util/Spliterator;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Spliterators$AbstractDoubleSpliterator\0", "trySplit\0", "()Ljava/util/Spliterator;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
