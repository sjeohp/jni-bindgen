// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-MutableLong"))]
__jni_bindgen! {
    /// public final class [MutableLong](https://developer.android.com/reference/android/util/MutableLong.html)
    ///
    /// Required feature: android-util-MutableLong
    #[deprecated] public final class MutableLong ("android/util/MutableLong") extends crate::java::lang::Object {

        /// [MutableLong](https://developer.android.com/reference/android/util/MutableLong.html#MutableLong(long))
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::MutableLong>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MutableLong", java.flags == PUBLIC, .name == "<init>", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MutableLong\0", "<init>\0", "(J)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [value](https://developer.android.com/reference/android/util/MutableLong.html#value)
        #[deprecated] pub fn value<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/MutableLong\0", "value\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [value](https://developer.android.com/reference/android/util/MutableLong.html#value)
        #[deprecated] pub fn set_value<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/MutableLong\0", "value\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }
    }
}
