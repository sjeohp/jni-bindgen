// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-ConditionVariable"))]
__jni_bindgen! {
    /// public class [ConditionVariable](https://developer.android.com/reference/android/os/ConditionVariable.html)
    ///
    /// Required feature: android-os-ConditionVariable
    public class ConditionVariable ("android/os/ConditionVariable") extends crate::java::lang::Object {

        /// [ConditionVariable](https://developer.android.com/reference/android/os/ConditionVariable.html#ConditionVariable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::ConditionVariable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ConditionVariable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ConditionVariable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ConditionVariable](https://developer.android.com/reference/android/os/ConditionVariable.html#ConditionVariable(boolean))
        pub fn new_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::ConditionVariable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ConditionVariable", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ConditionVariable\0", "<init>\0", "(Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [open](https://developer.android.com/reference/android/os/ConditionVariable.html#open())
        pub fn open<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ConditionVariable", java.flags == PUBLIC, .name == "open", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ConditionVariable\0", "open\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/os/ConditionVariable.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ConditionVariable", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ConditionVariable\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [block](https://developer.android.com/reference/android/os/ConditionVariable.html#block())
        pub fn block<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ConditionVariable", java.flags == PUBLIC, .name == "block", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ConditionVariable\0", "block\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [block](https://developer.android.com/reference/android/os/ConditionVariable.html#block(long))
        pub fn block_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/ConditionVariable", java.flags == PUBLIC, .name == "block", .descriptor == "(J)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/ConditionVariable\0", "block\0", "(J)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
