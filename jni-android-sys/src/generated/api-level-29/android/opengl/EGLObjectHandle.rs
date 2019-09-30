// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-opengl-EGLObjectHandle"))]
__jni_bindgen! {
    /// public class [EGLObjectHandle](https://developer.android.com/reference/android/opengl/EGLObjectHandle.html)
    ///
    /// Required feature: android-opengl-EGLObjectHandle
    public class EGLObjectHandle ("android/opengl/EGLObjectHandle") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [EGLObjectHandle](https://developer.android.com/reference/android/opengl/EGLObjectHandle.html#EGLObjectHandle(int))
        // #[deprecated] fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::opengl::EGLObjectHandle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/opengl/EGLObjectHandle", java.flags == PROTECTED, .name == "<init>", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/EGLObjectHandle\0", "<init>\0", "(I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [EGLObjectHandle](https://developer.android.com/reference/android/opengl/EGLObjectHandle.html#EGLObjectHandle(long))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::opengl::EGLObjectHandle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/opengl/EGLObjectHandle", java.flags == PROTECTED, .name == "<init>", .descriptor == "(J)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/EGLObjectHandle\0", "<init>\0", "(J)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getHandle](https://developer.android.com/reference/android/opengl/EGLObjectHandle.html#getHandle())
        #[deprecated] pub fn getHandle<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGLObjectHandle", java.flags == PUBLIC, .name == "getHandle", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/EGLObjectHandle\0", "getHandle\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNativeHandle](https://developer.android.com/reference/android/opengl/EGLObjectHandle.html#getNativeHandle())
        pub fn getNativeHandle<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGLObjectHandle", java.flags == PUBLIC, .name == "getNativeHandle", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/EGLObjectHandle\0", "getNativeHandle\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/opengl/EGLObjectHandle.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGLObjectHandle", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/EGLObjectHandle\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
