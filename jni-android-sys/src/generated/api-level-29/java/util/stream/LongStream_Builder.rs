// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-stream-LongStream_Builder"))]
__jni_bindgen! {
    /// public interface [LongStream.Builder](https://developer.android.com/reference/java/util/stream/LongStream.Builder.html)
    ///
    /// Required feature: java-util-stream-LongStream_Builder
    public interface LongStream_Builder ("java/util/stream/LongStream$Builder") extends crate::java::lang::Object, implements crate::java::util::function::LongConsumer {

        /// [accept](https://developer.android.com/reference/java/util/stream/LongStream.Builder.html#accept(long))
        pub fn accept<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/stream/LongStream$Builder", java.flags == PUBLIC | ABSTRACT, .name == "accept", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/stream/LongStream$Builder\0", "accept\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/stream/LongStream.Builder.html#add(long))
        ///
        /// Required features: "java-util-stream-LongStream_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-stream-LongStream_Builder")))]
        pub fn add<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::stream::LongStream_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/stream/LongStream$Builder", java.flags == PUBLIC, .name == "add", .descriptor == "(J)Ljava/util/stream/LongStream$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/stream/LongStream$Builder\0", "add\0", "(J)Ljava/util/stream/LongStream$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/java/util/stream/LongStream.Builder.html#build())
        ///
        /// Required features: "java-util-stream-LongStream"
        #[cfg(any(feature = "all", all(feature = "java-util-stream-LongStream")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::stream::LongStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/stream/LongStream$Builder", java.flags == PUBLIC | ABSTRACT, .name == "build", .descriptor == "()Ljava/util/stream/LongStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/stream/LongStream$Builder\0", "build\0", "()Ljava/util/stream/LongStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
