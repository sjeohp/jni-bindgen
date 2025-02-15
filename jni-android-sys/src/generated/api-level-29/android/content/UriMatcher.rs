// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-UriMatcher"))]
__jni_bindgen! {
    /// public class [UriMatcher](https://developer.android.com/reference/android/content/UriMatcher.html)
    ///
    /// Required feature: android-content-UriMatcher
    public class UriMatcher ("android/content/UriMatcher") extends crate::java::lang::Object {

        /// [UriMatcher](https://developer.android.com/reference/android/content/UriMatcher.html#UriMatcher(int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::UriMatcher>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/UriMatcher", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/UriMatcher\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addURI](https://developer.android.com/reference/android/content/UriMatcher.html#addURI(java.lang.String,%20java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn addURI<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/UriMatcher", java.flags == PUBLIC, .name == "addURI", .descriptor == "(Ljava/lang/String;Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/UriMatcher\0", "addURI\0", "(Ljava/lang/String;Ljava/lang/String;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [match](https://developer.android.com/reference/android/content/UriMatcher.html#match(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn r#match<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/UriMatcher", java.flags == PUBLIC, .name == "match", .descriptor == "(Landroid/net/Uri;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/UriMatcher\0", "match\0", "(Landroid/net/Uri;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [NO_MATCH](https://developer.android.com/reference/android/content/UriMatcher.html#NO_MATCH)
        pub const NO_MATCH : i32 = -1;
    }
}
