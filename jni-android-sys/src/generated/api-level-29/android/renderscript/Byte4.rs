// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-Byte4"))]
__jni_bindgen! {
    /// public class [Byte4](https://developer.android.com/reference/android/renderscript/Byte4.html)
    ///
    /// Required feature: android-renderscript-Byte4
    public class Byte4 ("android/renderscript/Byte4") extends crate::java::lang::Object {

        /// [Byte4](https://developer.android.com/reference/android/renderscript/Byte4.html#Byte4())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Byte4>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Byte4", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Byte4\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Byte4](https://developer.android.com/reference/android/renderscript/Byte4.html#Byte4(byte,%20byte,%20byte,%20byte))
        pub fn new_byte_byte_byte_byte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8, arg1: i8, arg2: i8, arg3: i8) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Byte4>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Byte4", java.flags == PUBLIC, .name == "<init>", .descriptor == "(BBBB)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Byte4\0", "<init>\0", "(BBBB)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [w](https://developer.android.com/reference/android/renderscript/Byte4.html#w)
        pub fn w<'env>(&'env self) -> i8 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "w\0", "B\0");
                env.get_byte_field(class, field)
            }
        }

        /// **set** public [w](https://developer.android.com/reference/android/renderscript/Byte4.html#w)
        pub fn set_w<'env>(&'env self, value: i8) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "w\0", "B\0");
                env.set_byte_field(class, field, value)
            }
        }

        /// **get** public [x](https://developer.android.com/reference/android/renderscript/Byte4.html#x)
        pub fn x<'env>(&'env self) -> i8 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "x\0", "B\0");
                env.get_byte_field(class, field)
            }
        }

        /// **set** public [x](https://developer.android.com/reference/android/renderscript/Byte4.html#x)
        pub fn set_x<'env>(&'env self, value: i8) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "x\0", "B\0");
                env.set_byte_field(class, field, value)
            }
        }

        /// **get** public [y](https://developer.android.com/reference/android/renderscript/Byte4.html#y)
        pub fn y<'env>(&'env self) -> i8 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "y\0", "B\0");
                env.get_byte_field(class, field)
            }
        }

        /// **set** public [y](https://developer.android.com/reference/android/renderscript/Byte4.html#y)
        pub fn set_y<'env>(&'env self, value: i8) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "y\0", "B\0");
                env.set_byte_field(class, field, value)
            }
        }

        /// **get** public [z](https://developer.android.com/reference/android/renderscript/Byte4.html#z)
        pub fn z<'env>(&'env self) -> i8 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "z\0", "B\0");
                env.get_byte_field(class, field)
            }
        }

        /// **set** public [z](https://developer.android.com/reference/android/renderscript/Byte4.html#z)
        pub fn set_z<'env>(&'env self, value: i8) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/renderscript/Byte4\0", "z\0", "B\0");
                env.set_byte_field(class, field, value)
            }
        }
    }
}
