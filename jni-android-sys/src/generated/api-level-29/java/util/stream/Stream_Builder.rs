// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-stream-Stream_Builder"))]
__jni_bindgen! {
    /// public interface [Stream.Builder](https://developer.android.com/reference/java/util/stream/Stream.Builder.html)
    ///
    /// Required feature: java-util-stream-Stream_Builder
    public interface Stream_Builder ("java/util/stream/Stream$Builder") extends crate::java::lang::Object, implements crate::java::util::function::Consumer {

        /// [accept](https://developer.android.com/reference/java/util/stream/Stream.Builder.html#accept(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn accept<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/stream/Stream$Builder", java.flags == PUBLIC | ABSTRACT, .name == "accept", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/stream/Stream$Builder\0", "accept\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/stream/Stream.Builder.html#add(java.lang.Object))
        ///
        /// Required features: "java-lang-Object", "java-util-stream-Stream_Builder"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-stream-Stream_Builder")))]
        pub fn add<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::stream::Stream_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/stream/Stream$Builder", java.flags == PUBLIC, .name == "add", .descriptor == "(Ljava/lang/Object;)Ljava/util/stream/Stream$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/stream/Stream$Builder\0", "add\0", "(Ljava/lang/Object;)Ljava/util/stream/Stream$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/java/util/stream/Stream.Builder.html#build())
        ///
        /// Required features: "java-util-stream-Stream"
        #[cfg(any(feature = "all", all(feature = "java-util-stream-Stream")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::stream::Stream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/stream/Stream$Builder", java.flags == PUBLIC | ABSTRACT, .name == "build", .descriptor == "()Ljava/util/stream/Stream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/stream/Stream$Builder\0", "build\0", "()Ljava/util/stream/Stream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
