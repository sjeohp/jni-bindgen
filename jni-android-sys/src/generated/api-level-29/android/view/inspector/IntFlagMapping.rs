// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-inspector-IntFlagMapping"))]
__jni_bindgen! {
    /// public final class [IntFlagMapping](https://developer.android.com/reference/android/view/inspector/IntFlagMapping.html)
    ///
    /// Required feature: android-view-inspector-IntFlagMapping
    public final class IntFlagMapping ("android/view/inspector/IntFlagMapping") extends crate::java::lang::Object {

        /// [IntFlagMapping](https://developer.android.com/reference/android/view/inspector/IntFlagMapping.html#IntFlagMapping())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::inspector::IntFlagMapping>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inspector/IntFlagMapping", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inspector/IntFlagMapping\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/android/view/inspector/IntFlagMapping.html#get(int))
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn get<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inspector/IntFlagMapping", java.flags == PUBLIC, .name == "get", .descriptor == "(I)Ljava/util/Set;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inspector/IntFlagMapping\0", "get\0", "(I)Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/android/view/inspector/IntFlagMapping.html#add(int,%20int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn add<'env>(&'env self, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inspector/IntFlagMapping", java.flags == PUBLIC, .name == "add", .descriptor == "(IILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inspector/IntFlagMapping\0", "add\0", "(IILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
