// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-TracingConfig_Builder"))]
__jni_bindgen! {
    /// public class [TracingConfig.Builder](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html)
    ///
    /// Required feature: android-webkit-TracingConfig_Builder
    public class TracingConfig_Builder ("android/webkit/TracingConfig$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::TracingConfig_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/TracingConfig$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/TracingConfig$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html#build())
        ///
        /// Required features: "android-webkit-TracingConfig"
        #[cfg(any(feature = "all", all(feature = "android-webkit-TracingConfig")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::webkit::TracingConfig>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/TracingConfig$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/webkit/TracingConfig;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/TracingConfig$Builder\0", "build\0", "()Landroid/webkit/TracingConfig;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addCategories](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html#addCategories(int...))
        ///
        /// Required features: "android-webkit-TracingConfig_Builder"
        #[cfg(any(feature = "all", all(feature = "android-webkit-TracingConfig_Builder")))]
        pub fn addCategories_int_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::webkit::TracingConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/TracingConfig$Builder", java.flags == PUBLIC | VARARGS, .name == "addCategories", .descriptor == "([I)Landroid/webkit/TracingConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/TracingConfig$Builder\0", "addCategories\0", "([I)Landroid/webkit/TracingConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addCategories](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html#addCategories(java.lang.String...))
        ///
        /// Required features: "android-webkit-TracingConfig_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-webkit-TracingConfig_Builder", feature = "java-lang-String")))]
        pub fn addCategories_String_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::webkit::TracingConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/TracingConfig$Builder", java.flags == PUBLIC | VARARGS, .name == "addCategories", .descriptor == "([Ljava/lang/String;)Landroid/webkit/TracingConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/TracingConfig$Builder\0", "addCategories\0", "([Ljava/lang/String;)Landroid/webkit/TracingConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addCategories](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html#addCategories(java.util.Collection))
        ///
        /// Required features: "android-webkit-TracingConfig_Builder", "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "android-webkit-TracingConfig_Builder", feature = "java-util-Collection")))]
        pub fn addCategories_Collection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::webkit::TracingConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/TracingConfig$Builder", java.flags == PUBLIC, .name == "addCategories", .descriptor == "(Ljava/util/Collection;)Landroid/webkit/TracingConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/TracingConfig$Builder\0", "addCategories\0", "(Ljava/util/Collection;)Landroid/webkit/TracingConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTracingMode](https://developer.android.com/reference/android/webkit/TracingConfig.Builder.html#setTracingMode(int))
        ///
        /// Required features: "android-webkit-TracingConfig_Builder"
        #[cfg(any(feature = "all", all(feature = "android-webkit-TracingConfig_Builder")))]
        pub fn setTracingMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::webkit::TracingConfig_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/TracingConfig$Builder", java.flags == PUBLIC, .name == "setTracingMode", .descriptor == "(I)Landroid/webkit/TracingConfig$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/TracingConfig$Builder\0", "setTracingMode\0", "(I)Landroid/webkit/TracingConfig$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
