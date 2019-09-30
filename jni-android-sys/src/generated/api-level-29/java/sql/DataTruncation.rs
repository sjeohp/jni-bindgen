// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-sql-DataTruncation"))]
__jni_bindgen! {
    /// public class [DataTruncation](https://developer.android.com/reference/java/sql/DataTruncation.html)
    ///
    /// Required feature: java-sql-DataTruncation
    public class DataTruncation ("java/sql/DataTruncation") extends crate::java::sql::SQLWarning {

        /// [DataTruncation](https://developer.android.com/reference/java/sql/DataTruncation.html#DataTruncation(int,%20boolean,%20boolean,%20int,%20int))
        pub fn new_int_boolean_boolean_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: bool, arg2: bool, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::sql::DataTruncation>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IZZII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "<init>\0", "(IZZII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DataTruncation](https://developer.android.com/reference/java/sql/DataTruncation.html#DataTruncation(int,%20boolean,%20boolean,%20int,%20int,%20java.lang.Throwable))
        ///
        /// Required features: "java-lang-Throwable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Throwable")))]
        pub fn new_int_boolean_boolean_int_int_Throwable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: bool, arg2: bool, arg3: i32, arg4: i32, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::sql::DataTruncation>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IZZIILjava/lang/Throwable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "<init>\0", "(IZZIILjava/lang/Throwable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIndex](https://developer.android.com/reference/java/sql/DataTruncation.html#getIndex())
        pub fn getIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "getIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "getIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParameter](https://developer.android.com/reference/java/sql/DataTruncation.html#getParameter())
        pub fn getParameter<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "getParameter", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "getParameter\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRead](https://developer.android.com/reference/java/sql/DataTruncation.html#getRead())
        pub fn getRead<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "getRead", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "getRead\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDataSize](https://developer.android.com/reference/java/sql/DataTruncation.html#getDataSize())
        pub fn getDataSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "getDataSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "getDataSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTransferSize](https://developer.android.com/reference/java/sql/DataTruncation.html#getTransferSize())
        pub fn getTransferSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/DataTruncation", java.flags == PUBLIC, .name == "getTransferSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/DataTruncation\0", "getTransferSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
