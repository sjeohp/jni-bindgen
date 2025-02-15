// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-ref-WeakReference"))]
__jni_bindgen! {
    /// public class [WeakReference](https://developer.android.com/reference/java/lang/ref/WeakReference.html)
    ///
    /// Required feature: java-lang-ref-WeakReference
    public class WeakReference ("java/lang/ref/WeakReference") extends crate::java::lang::r#ref::Reference {

        /// [WeakReference](https://developer.android.com/reference/java/lang/ref/WeakReference.html#WeakReference(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn new_Object<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::r#ref::WeakReference>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ref/WeakReference", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ref/WeakReference\0", "<init>\0", "(Ljava/lang/Object;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [WeakReference](https://developer.android.com/reference/java/lang/ref/WeakReference.html#WeakReference(java.lang.Object,%20java.lang.ref.ReferenceQueue))
        ///
        /// Required features: "java-lang-Object", "java-lang-ref-ReferenceQueue"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-ref-ReferenceQueue")))]
        pub fn new_Object_ReferenceQueue<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::r#ref::ReferenceQueue>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::r#ref::WeakReference>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/ref/WeakReference", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/ref/WeakReference\0", "<init>\0", "(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
