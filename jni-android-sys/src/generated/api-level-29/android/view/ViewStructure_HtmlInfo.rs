// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-ViewStructure_HtmlInfo"))]
__jni_bindgen! {
    /// public class [ViewStructure.HtmlInfo](https://developer.android.com/reference/android/view/ViewStructure.HtmlInfo.html)
    ///
    /// Required feature: android-view-ViewStructure_HtmlInfo
    public class ViewStructure_HtmlInfo ("android/view/ViewStructure$HtmlInfo") extends crate::java::lang::Object {

        /// [HtmlInfo](https://developer.android.com/reference/android/view/ViewStructure.HtmlInfo.html#HtmlInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::ViewStructure_HtmlInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewStructure$HtmlInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ViewStructure$HtmlInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTag](https://developer.android.com/reference/android/view/ViewStructure.HtmlInfo.html#getTag())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getTag<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewStructure$HtmlInfo", java.flags == PUBLIC | ABSTRACT, .name == "getTag", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ViewStructure$HtmlInfo\0", "getTag\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAttributes](https://developer.android.com/reference/android/view/ViewStructure.HtmlInfo.html#getAttributes())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getAttributes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewStructure$HtmlInfo", java.flags == PUBLIC | ABSTRACT, .name == "getAttributes", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ViewStructure$HtmlInfo\0", "getAttributes\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
