// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-print-PrintDocumentInfo_Builder"))]
__jni_bindgen! {
    /// public final class [PrintDocumentInfo.Builder](https://developer.android.com/reference/android/print/PrintDocumentInfo.Builder.html)
    ///
    /// Required feature: android-print-PrintDocumentInfo_Builder
    public final class PrintDocumentInfo_Builder ("android/print/PrintDocumentInfo$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/print/PrintDocumentInfo.Builder.html#Builder(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::print::PrintDocumentInfo_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintDocumentInfo$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintDocumentInfo$Builder\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPageCount](https://developer.android.com/reference/android/print/PrintDocumentInfo.Builder.html#setPageCount(int))
        ///
        /// Required features: "android-print-PrintDocumentInfo_Builder"
        #[cfg(any(feature = "all", all(feature = "android-print-PrintDocumentInfo_Builder")))]
        pub fn setPageCount<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrintDocumentInfo_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintDocumentInfo$Builder", java.flags == PUBLIC, .name == "setPageCount", .descriptor == "(I)Landroid/print/PrintDocumentInfo$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintDocumentInfo$Builder\0", "setPageCount\0", "(I)Landroid/print/PrintDocumentInfo$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setContentType](https://developer.android.com/reference/android/print/PrintDocumentInfo.Builder.html#setContentType(int))
        ///
        /// Required features: "android-print-PrintDocumentInfo_Builder"
        #[cfg(any(feature = "all", all(feature = "android-print-PrintDocumentInfo_Builder")))]
        pub fn setContentType<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrintDocumentInfo_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintDocumentInfo$Builder", java.flags == PUBLIC, .name == "setContentType", .descriptor == "(I)Landroid/print/PrintDocumentInfo$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintDocumentInfo$Builder\0", "setContentType\0", "(I)Landroid/print/PrintDocumentInfo$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/print/PrintDocumentInfo.Builder.html#build())
        ///
        /// Required features: "android-print-PrintDocumentInfo"
        #[cfg(any(feature = "all", all(feature = "android-print-PrintDocumentInfo")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrintDocumentInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintDocumentInfo$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/print/PrintDocumentInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintDocumentInfo$Builder\0", "build\0", "()Landroid/print/PrintDocumentInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
