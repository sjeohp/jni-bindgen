// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-Choreographer_FrameCallback"))]
__jni_bindgen! {
    /// public interface [Choreographer.FrameCallback](https://developer.android.com/reference/android/view/Choreographer.FrameCallback.html)
    ///
    /// Required feature: android-view-Choreographer_FrameCallback
    public interface Choreographer_FrameCallback ("android/view/Choreographer$FrameCallback") extends crate::java::lang::Object {

        /// [doFrame](https://developer.android.com/reference/android/view/Choreographer.FrameCallback.html#doFrame(long))
        pub fn doFrame<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/Choreographer$FrameCallback", java.flags == PUBLIC | ABSTRACT, .name == "doFrame", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/Choreographer$FrameCallback\0", "doFrame\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
