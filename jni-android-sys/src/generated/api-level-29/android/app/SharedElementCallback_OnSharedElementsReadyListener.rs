// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-SharedElementCallback_OnSharedElementsReadyListener"))]
__jni_bindgen! {
    /// public interface [SharedElementCallback.OnSharedElementsReadyListener](https://developer.android.com/reference/android/app/SharedElementCallback.OnSharedElementsReadyListener.html)
    ///
    /// Required feature: android-app-SharedElementCallback_OnSharedElementsReadyListener
    public interface SharedElementCallback_OnSharedElementsReadyListener ("android/app/SharedElementCallback$OnSharedElementsReadyListener") extends crate::java::lang::Object {

        /// [onSharedElementsReady](https://developer.android.com/reference/android/app/SharedElementCallback.OnSharedElementsReadyListener.html#onSharedElementsReady())
        pub fn onSharedElementsReady<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/SharedElementCallback$OnSharedElementsReadyListener", java.flags == PUBLIC | ABSTRACT, .name == "onSharedElementsReady", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/SharedElementCallback$OnSharedElementsReadyListener\0", "onSharedElementsReady\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
