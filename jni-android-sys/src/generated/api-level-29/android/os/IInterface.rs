// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-IInterface"))]
__jni_bindgen! {
    /// public interface [IInterface](https://developer.android.com/reference/android/os/IInterface.html)
    ///
    /// Required feature: android-os-IInterface
    public interface IInterface ("android/os/IInterface") extends crate::java::lang::Object {

        /// [asBinder](https://developer.android.com/reference/android/os/IInterface.html#asBinder())
        ///
        /// Required features: "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-os-IBinder")))]
        pub fn asBinder<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/IInterface", java.flags == PUBLIC | ABSTRACT, .name == "asBinder", .descriptor == "()Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/IInterface\0", "asBinder\0", "()Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
