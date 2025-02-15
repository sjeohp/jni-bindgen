// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-DateSorter"))]
__jni_bindgen! {
    /// public class [DateSorter](https://developer.android.com/reference/android/webkit/DateSorter.html)
    ///
    /// Required feature: android-webkit-DateSorter
    public class DateSorter ("android/webkit/DateSorter") extends crate::java::lang::Object {

        /// [DateSorter](https://developer.android.com/reference/android/webkit/DateSorter.html#DateSorter(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::DateSorter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/DateSorter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/DateSorter\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIndex](https://developer.android.com/reference/android/webkit/DateSorter.html#getIndex(long))
        pub fn getIndex<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/DateSorter", java.flags == PUBLIC, .name == "getIndex", .descriptor == "(J)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/DateSorter\0", "getIndex\0", "(J)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLabel](https://developer.android.com/reference/android/webkit/DateSorter.html#getLabel(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getLabel<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/DateSorter", java.flags == PUBLIC, .name == "getLabel", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/DateSorter\0", "getLabel\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBoundary](https://developer.android.com/reference/android/webkit/DateSorter.html#getBoundary(int))
        pub fn getBoundary<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/DateSorter", java.flags == PUBLIC, .name == "getBoundary", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/DateSorter\0", "getBoundary\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DAY_COUNT](https://developer.android.com/reference/android/webkit/DateSorter.html#DAY_COUNT)
        pub const DAY_COUNT : i32 = 5;
    }
}
