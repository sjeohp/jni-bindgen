// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-nsd-NsdManager_ResolveListener"))]
__jni_bindgen! {
    /// public interface [NsdManager.ResolveListener](https://developer.android.com/reference/android/net/nsd/NsdManager.ResolveListener.html)
    ///
    /// Required feature: android-net-nsd-NsdManager_ResolveListener
    public interface NsdManager_ResolveListener ("android/net/nsd/NsdManager$ResolveListener") extends crate::java::lang::Object {

        /// [onResolveFailed](https://developer.android.com/reference/android/net/nsd/NsdManager.ResolveListener.html#onResolveFailed(android.net.nsd.NsdServiceInfo,%20int))
        ///
        /// Required features: "android-net-nsd-NsdServiceInfo"
        #[cfg(any(feature = "all", all(feature = "android-net-nsd-NsdServiceInfo")))]
        pub fn onResolveFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::nsd::NsdServiceInfo>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/nsd/NsdManager$ResolveListener", java.flags == PUBLIC | ABSTRACT, .name == "onResolveFailed", .descriptor == "(Landroid/net/nsd/NsdServiceInfo;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/nsd/NsdManager$ResolveListener\0", "onResolveFailed\0", "(Landroid/net/nsd/NsdServiceInfo;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onServiceResolved](https://developer.android.com/reference/android/net/nsd/NsdManager.ResolveListener.html#onServiceResolved(android.net.nsd.NsdServiceInfo))
        ///
        /// Required features: "android-net-nsd-NsdServiceInfo"
        #[cfg(any(feature = "all", all(feature = "android-net-nsd-NsdServiceInfo")))]
        pub fn onServiceResolved<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::nsd::NsdServiceInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/nsd/NsdManager$ResolveListener", java.flags == PUBLIC | ABSTRACT, .name == "onServiceResolved", .descriptor == "(Landroid/net/nsd/NsdServiceInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/nsd/NsdManager$ResolveListener\0", "onServiceResolved\0", "(Landroid/net/nsd/NsdServiceInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
