// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-ProxyFileDescriptorCallback"))]
__jni_bindgen! {
    /// public class [ProxyFileDescriptorCallback](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html)
    ///
    /// Required feature: android-os-ProxyFileDescriptorCallback
    public class ProxyFileDescriptorCallback ("android/os/ProxyFileDescriptorCallback") extends crate::java::lang::Object {

        /// [ProxyFileDescriptorCallback](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html#ProxyFileDescriptorCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::ProxyFileDescriptorCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ProxyFileDescriptorCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ProxyFileDescriptorCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGetSize](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html#onGetSize())
        pub fn onGetSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ProxyFileDescriptorCallback", java.flags == PUBLIC, .name == "onGetSize", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ProxyFileDescriptorCallback\0", "onGetSize\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRead](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html#onRead(long,%20int,%20byte%5B%5D))
        pub fn onRead<'env>(&'env self, arg0: i64, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ProxyFileDescriptorCallback", java.flags == PUBLIC, .name == "onRead", .descriptor == "(JI[B)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ProxyFileDescriptorCallback\0", "onRead\0", "(JI[B)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onWrite](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html#onWrite(long,%20int,%20byte%5B%5D))
        pub fn onWrite<'env>(&'env self, arg0: i64, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ProxyFileDescriptorCallback", java.flags == PUBLIC, .name == "onWrite", .descriptor == "(JI[B)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ProxyFileDescriptorCallback\0", "onWrite\0", "(JI[B)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFsync](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html#onFsync())
        pub fn onFsync<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ProxyFileDescriptorCallback", java.flags == PUBLIC, .name == "onFsync", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ProxyFileDescriptorCallback\0", "onFsync\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRelease](https://developer.android.com/reference/android/os/ProxyFileDescriptorCallback.html#onRelease())
        pub fn onRelease<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ProxyFileDescriptorCallback", java.flags == PUBLIC | ABSTRACT, .name == "onRelease", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ProxyFileDescriptorCallback\0", "onRelease\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
