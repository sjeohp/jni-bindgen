// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-DataSetObservable"))]
__jni_bindgen! {
    /// public class [DataSetObservable](https://developer.android.com/reference/android/database/DataSetObservable.html)
    ///
    /// Required feature: android-database-DataSetObservable
    public class DataSetObservable ("android/database/DataSetObservable") extends crate::android::database::Observable {

        /// [DataSetObservable](https://developer.android.com/reference/android/database/DataSetObservable.html#DataSetObservable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::DataSetObservable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/DataSetObservable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/DataSetObservable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyChanged](https://developer.android.com/reference/android/database/DataSetObservable.html#notifyChanged())
        pub fn notifyChanged<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/DataSetObservable", java.flags == PUBLIC, .name == "notifyChanged", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/DataSetObservable\0", "notifyChanged\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyInvalidated](https://developer.android.com/reference/android/database/DataSetObservable.html#notifyInvalidated())
        pub fn notifyInvalidated<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/DataSetObservable", java.flags == PUBLIC, .name == "notifyInvalidated", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/DataSetObservable\0", "notifyInvalidated\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
