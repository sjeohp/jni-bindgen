// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-sax-EndElementListener"))]
__jni_bindgen! {
    /// public interface [EndElementListener](https://developer.android.com/reference/android/sax/EndElementListener.html)
    ///
    /// Required feature: android-sax-EndElementListener
    public interface EndElementListener ("android/sax/EndElementListener") extends crate::java::lang::Object {

        /// [end](https://developer.android.com/reference/android/sax/EndElementListener.html#end())
        pub fn end<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/sax/EndElementListener", java.flags == PUBLIC | ABSTRACT, .name == "end", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/sax/EndElementListener\0", "end\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
