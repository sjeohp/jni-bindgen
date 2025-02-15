// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-Entity_NamedContentValues"))]
__jni_bindgen! {
    /// public class [Entity.NamedContentValues](https://developer.android.com/reference/android/content/Entity.NamedContentValues.html)
    ///
    /// Required feature: android-content-Entity_NamedContentValues
    public class Entity_NamedContentValues ("android/content/Entity$NamedContentValues") extends crate::java::lang::Object {

        /// [NamedContentValues](https://developer.android.com/reference/android/content/Entity.NamedContentValues.html#NamedContentValues(android.net.Uri,%20android.content.ContentValues))
        ///
        /// Required features: "android-content-ContentValues", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-ContentValues", feature = "android-net-Uri")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentValues>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::Entity_NamedContentValues>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Entity$NamedContentValues", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/net/Uri;Landroid/content/ContentValues;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Entity$NamedContentValues\0", "<init>\0", "(Landroid/net/Uri;Landroid/content/ContentValues;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public final [uri](https://developer.android.com/reference/android/content/Entity.NamedContentValues.html#uri)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn uri<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/Entity$NamedContentValues\0", "uri\0", "Landroid/net/Uri;\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [values](https://developer.android.com/reference/android/content/Entity.NamedContentValues.html#values)
        ///
        /// Required feature: android-content-ContentValues
        #[cfg(any(feature = "all", feature = "android-content-ContentValues"))]
        pub fn values<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ContentValues>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/Entity$NamedContentValues\0", "values\0", "Landroid/content/ContentValues;\0");
                env.get_object_field(class, field)
            }
        }
    }
}
