// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "dalvik-system-InMemoryDexClassLoader"))]
__jni_bindgen! {
    /// public final class [InMemoryDexClassLoader](https://developer.android.com/reference/dalvik/system/InMemoryDexClassLoader.html)
    ///
    /// Required feature: dalvik-system-InMemoryDexClassLoader
    public final class InMemoryDexClassLoader ("dalvik/system/InMemoryDexClassLoader") extends crate::dalvik::system::BaseDexClassLoader {

        /// [InMemoryDexClassLoader](https://developer.android.com/reference/dalvik/system/InMemoryDexClassLoader.html#InMemoryDexClassLoader(java.nio.ByteBuffer%5B%5D,%20java.lang.String,%20java.lang.ClassLoader))
        ///
        /// Required features: "java-lang-ClassLoader", "java-lang-String", "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-lang-String", feature = "java-nio-ByteBuffer")))]
        pub fn new_ByteBuffer_array_String_ClassLoader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::ByteBuffer, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::dalvik::system::InMemoryDexClassLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "dalvik/system/InMemoryDexClassLoader", java.flags == PUBLIC, .name == "<init>", .descriptor == "([Ljava/nio/ByteBuffer;Ljava/lang/String;Ljava/lang/ClassLoader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("dalvik/system/InMemoryDexClassLoader\0", "<init>\0", "([Ljava/nio/ByteBuffer;Ljava/lang/String;Ljava/lang/ClassLoader;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InMemoryDexClassLoader](https://developer.android.com/reference/dalvik/system/InMemoryDexClassLoader.html#InMemoryDexClassLoader(java.nio.ByteBuffer%5B%5D,%20java.lang.ClassLoader))
        ///
        /// Required features: "java-lang-ClassLoader", "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-nio-ByteBuffer")))]
        pub fn new_ByteBuffer_array_ClassLoader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::nio::ByteBuffer, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::dalvik::system::InMemoryDexClassLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "dalvik/system/InMemoryDexClassLoader", java.flags == PUBLIC, .name == "<init>", .descriptor == "([Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("dalvik/system/InMemoryDexClassLoader\0", "<init>\0", "([Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InMemoryDexClassLoader](https://developer.android.com/reference/dalvik/system/InMemoryDexClassLoader.html#InMemoryDexClassLoader(java.nio.ByteBuffer,%20java.lang.ClassLoader))
        ///
        /// Required features: "java-lang-ClassLoader", "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-lang-ClassLoader", feature = "java-nio-ByteBuffer")))]
        pub fn new_ByteBuffer_ClassLoader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::ClassLoader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::dalvik::system::InMemoryDexClassLoader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "dalvik/system/InMemoryDexClassLoader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("dalvik/system/InMemoryDexClassLoader\0", "<init>\0", "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
