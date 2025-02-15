// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-CrossProcessCursorWrapper"))]
__jni_bindgen! {
    /// public class [CrossProcessCursorWrapper](https://developer.android.com/reference/android/database/CrossProcessCursorWrapper.html)
    ///
    /// Required feature: android-database-CrossProcessCursorWrapper
    public class CrossProcessCursorWrapper ("android/database/CrossProcessCursorWrapper") extends crate::android::database::CursorWrapper, implements crate::android::database::CrossProcessCursor {

        /// [CrossProcessCursorWrapper](https://developer.android.com/reference/android/database/CrossProcessCursorWrapper.html#CrossProcessCursorWrapper(android.database.Cursor))
        ///
        /// Required features: "android-database-Cursor"
        #[cfg(any(feature = "all", all(feature = "android-database-Cursor")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::CrossProcessCursorWrapper>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CrossProcessCursorWrapper", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/database/Cursor;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CrossProcessCursorWrapper\0", "<init>\0", "(Landroid/database/Cursor;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fillWindow](https://developer.android.com/reference/android/database/CrossProcessCursorWrapper.html#fillWindow(int,%20android.database.CursorWindow))
        ///
        /// Required features: "android-database-CursorWindow"
        #[cfg(any(feature = "all", all(feature = "android-database-CursorWindow")))]
        pub fn fillWindow<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::CursorWindow>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CrossProcessCursorWrapper", java.flags == PUBLIC, .name == "fillWindow", .descriptor == "(ILandroid/database/CursorWindow;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CrossProcessCursorWrapper\0", "fillWindow\0", "(ILandroid/database/CursorWindow;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWindow](https://developer.android.com/reference/android/database/CrossProcessCursorWrapper.html#getWindow())
        ///
        /// Required features: "android-database-CursorWindow"
        #[cfg(any(feature = "all", all(feature = "android-database-CursorWindow")))]
        pub fn getWindow<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::database::CursorWindow>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CrossProcessCursorWrapper", java.flags == PUBLIC, .name == "getWindow", .descriptor == "()Landroid/database/CursorWindow;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CrossProcessCursorWrapper\0", "getWindow\0", "()Landroid/database/CursorWindow;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onMove](https://developer.android.com/reference/android/database/CrossProcessCursorWrapper.html#onMove(int,%20int))
        pub fn onMove<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CrossProcessCursorWrapper", java.flags == PUBLIC, .name == "onMove", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CrossProcessCursorWrapper\0", "onMove\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
