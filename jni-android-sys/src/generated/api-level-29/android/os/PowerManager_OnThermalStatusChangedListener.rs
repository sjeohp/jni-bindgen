// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-PowerManager_OnThermalStatusChangedListener"))]
__jni_bindgen! {
    /// public interface [PowerManager.OnThermalStatusChangedListener](https://developer.android.com/reference/android/os/PowerManager.OnThermalStatusChangedListener.html)
    ///
    /// Required feature: android-os-PowerManager_OnThermalStatusChangedListener
    public interface PowerManager_OnThermalStatusChangedListener ("android/os/PowerManager$OnThermalStatusChangedListener") extends crate::java::lang::Object {

        /// [onThermalStatusChanged](https://developer.android.com/reference/android/os/PowerManager.OnThermalStatusChangedListener.html#onThermalStatusChanged(int))
        pub fn onThermalStatusChanged<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/PowerManager$OnThermalStatusChangedListener", java.flags == PUBLIC | ABSTRACT, .name == "onThermalStatusChanged", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/PowerManager$OnThermalStatusChangedListener\0", "onThermalStatusChanged\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
