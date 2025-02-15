// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-Entity"))]
__jni_bindgen! {
    /// public final class [Entity](https://developer.android.com/reference/android/content/Entity.html)
    ///
    /// Required feature: android-content-Entity
    public final class Entity ("android/content/Entity") extends crate::java::lang::Object {

        /// [Entity](https://developer.android.com/reference/android/content/Entity.html#Entity(android.content.ContentValues))
        ///
        /// Required features: "android-content-ContentValues"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentValues")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::Entity>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Entity", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/ContentValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Entity\0", "<init>\0", "(Landroid/content/ContentValues;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEntityValues](https://developer.android.com/reference/android/content/Entity.html#getEntityValues())
        ///
        /// Required features: "android-content-ContentValues"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentValues")))]
        pub fn getEntityValues<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ContentValues>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Entity", java.flags == PUBLIC, .name == "getEntityValues", .descriptor == "()Landroid/content/ContentValues;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Entity\0", "getEntityValues\0", "()Landroid/content/ContentValues;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubValues](https://developer.android.com/reference/android/content/Entity.html#getSubValues())
        ///
        /// Required features: "java-util-ArrayList"
        #[cfg(any(feature = "all", all(feature = "java-util-ArrayList")))]
        pub fn getSubValues<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ArrayList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Entity", java.flags == PUBLIC, .name == "getSubValues", .descriptor == "()Ljava/util/ArrayList;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Entity\0", "getSubValues\0", "()Ljava/util/ArrayList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addSubValue](https://developer.android.com/reference/android/content/Entity.html#addSubValue(android.net.Uri,%20android.content.ContentValues))
        ///
        /// Required features: "android-content-ContentValues", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentValues", feature = "android-net-Uri")))]
        pub fn addSubValue<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentValues>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Entity", java.flags == PUBLIC, .name == "addSubValue", .descriptor == "(Landroid/net/Uri;Landroid/content/ContentValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Entity\0", "addSubValue\0", "(Landroid/net/Uri;Landroid/content/ContentValues;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/content/Entity.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Entity", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Entity\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
