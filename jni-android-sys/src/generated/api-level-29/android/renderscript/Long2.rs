// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-Long2"))]
__jni_bindgen! {
    /// public class [Long2](https://developer.android.com/reference/android/renderscript/Long2.html)
    ///
    /// Required feature: android-renderscript-Long2
    public class Long2 ("android/renderscript/Long2") extends crate::java::lang::Object {

        /// [Long2](https://developer.android.com/reference/android/renderscript/Long2.html#Long2())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Long2>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Long2", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Long2\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Long2](https://developer.android.com/reference/android/renderscript/Long2.html#Long2(long,%20long))
        pub fn new_long_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Long2>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Long2", java.flags == PUBLIC, .name == "<init>", .descriptor == "(JJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Long2\0", "<init>\0", "(JJ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [x](https://developer.android.com/reference/android/renderscript/Long2.html#x)
        pub fn x<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Long2\0", "x\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [x](https://developer.android.com/reference/android/renderscript/Long2.html#x)
        pub fn set_x<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Long2\0", "x\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [y](https://developer.android.com/reference/android/renderscript/Long2.html#y)
        pub fn y<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Long2\0", "y\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [y](https://developer.android.com/reference/android/renderscript/Long2.html#y)
        pub fn set_y<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Long2\0", "y\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }
    }
}
