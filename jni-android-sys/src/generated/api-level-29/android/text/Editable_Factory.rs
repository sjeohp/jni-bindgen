// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-Editable_Factory"))]
__jni_bindgen! {
    /// public class [Editable.Factory](https://developer.android.com/reference/android/text/Editable.Factory.html)
    ///
    /// Required feature: android-text-Editable_Factory
    public class Editable_Factory ("android/text/Editable$Factory") extends crate::java::lang::Object {

        /// [Factory](https://developer.android.com/reference/android/text/Editable.Factory.html#Factory())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::Editable_Factory>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/Editable$Factory", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/Editable$Factory\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/text/Editable.Factory.html#getInstance())
        ///
        /// Required features: "android-text-Editable_Factory"
        #[cfg(any(feature = "all", all(feature = "android-text-Editable_Factory")))]
        pub fn getInstance<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::Editable_Factory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/Editable$Factory", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "()Landroid/text/Editable$Factory;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/Editable$Factory\0", "getInstance\0", "()Landroid/text/Editable$Factory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newEditable](https://developer.android.com/reference/android/text/Editable.Factory.html#newEditable(java.lang.CharSequence))
        ///
        /// Required features: "android-text-Editable", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-text-Editable", feature = "java-lang-CharSequence")))]
        pub fn newEditable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::Editable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/Editable$Factory", java.flags == PUBLIC, .name == "newEditable", .descriptor == "(Ljava/lang/CharSequence;)Landroid/text/Editable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/Editable$Factory\0", "newEditable\0", "(Ljava/lang/CharSequence;)Landroid/text/Editable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
