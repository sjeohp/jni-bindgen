// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-MutableChar"))]
__jni_bindgen! {
    /// public final class [MutableChar](https://developer.android.com/reference/android/util/MutableChar.html)
    ///
    /// Required feature: android-util-MutableChar
    #[deprecated] public final class MutableChar ("android/util/MutableChar") extends crate::java::lang::Object {

        /// [MutableChar](https://developer.android.com/reference/android/util/MutableChar.html#MutableChar(char))
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::MutableChar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MutableChar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(C)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MutableChar\0", "<init>\0", "(C)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [value](https://developer.android.com/reference/android/util/MutableChar.html#value)
        #[deprecated] pub fn value<'env>(&'env self) -> __jni_bindgen::jchar {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/MutableChar\0", "value\0", "C\0");
                env.get_char_field(class, field)
            }
        }

        /// **set** public [value](https://developer.android.com/reference/android/util/MutableChar.html#value)
        #[deprecated] pub fn set_value<'env>(&'env self, value: __jni_bindgen::jchar) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/MutableChar\0", "value\0", "C\0");
                env.set_char_field(class, field, value)
            }
        }
    }
}
