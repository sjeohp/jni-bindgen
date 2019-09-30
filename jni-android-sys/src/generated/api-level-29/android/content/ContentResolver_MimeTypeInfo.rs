// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-ContentResolver_MimeTypeInfo"))]
__jni_bindgen! {
    /// public final class [ContentResolver.MimeTypeInfo](https://developer.android.com/reference/android/content/ContentResolver.MimeTypeInfo.html)
    ///
    /// Required feature: android-content-ContentResolver_MimeTypeInfo
    public final class ContentResolver_MimeTypeInfo ("android/content/ContentResolver$MimeTypeInfo") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [MimeTypeInfo](https://developer.android.com/reference/android/content/ContentResolver.MimeTypeInfo.html#MimeTypeInfo(android.graphics.drawable.Icon,%20java.lang.CharSequence,%20java.lang.CharSequence))
        // ///
        // /// Required features: "android-graphics-drawable-Icon", "java-lang-CharSequence"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Icon", feature = "java-lang-CharSequence")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Icon>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::ContentResolver_MimeTypeInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/ContentResolver$MimeTypeInfo", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/graphics/drawable/Icon;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ContentResolver$MimeTypeInfo\0", "<init>\0", "(Landroid/graphics/drawable/Icon;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getIcon](https://developer.android.com/reference/android/content/ContentResolver.MimeTypeInfo.html#getIcon())
        ///
        /// Required features: "android-graphics-drawable-Icon"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Icon")))]
        pub fn getIcon<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Icon>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ContentResolver$MimeTypeInfo", java.flags == PUBLIC, .name == "getIcon", .descriptor == "()Landroid/graphics/drawable/Icon;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ContentResolver$MimeTypeInfo\0", "getIcon\0", "()Landroid/graphics/drawable/Icon;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLabel](https://developer.android.com/reference/android/content/ContentResolver.MimeTypeInfo.html#getLabel())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getLabel<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ContentResolver$MimeTypeInfo", java.flags == PUBLIC, .name == "getLabel", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ContentResolver$MimeTypeInfo\0", "getLabel\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getContentDescription](https://developer.android.com/reference/android/content/ContentResolver.MimeTypeInfo.html#getContentDescription())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getContentDescription<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ContentResolver$MimeTypeInfo", java.flags == PUBLIC, .name == "getContentDescription", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ContentResolver$MimeTypeInfo\0", "getContentDescription\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
