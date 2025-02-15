// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-contentcapture-ContentCaptureContext_Builder"))]
__jni_bindgen! {
    /// public final class [ContentCaptureContext.Builder](https://developer.android.com/reference/android/view/contentcapture/ContentCaptureContext.Builder.html)
    ///
    /// Required feature: android-view-contentcapture-ContentCaptureContext_Builder
    public final class ContentCaptureContext_Builder ("android/view/contentcapture/ContentCaptureContext$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/view/contentcapture/ContentCaptureContext.Builder.html#Builder(android.content.LocusId))
        ///
        /// Required features: "android-content-LocusId"
        #[cfg(any(feature = "all", all(feature = "android-content-LocusId")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::LocusId>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::contentcapture::ContentCaptureContext_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/contentcapture/ContentCaptureContext$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/LocusId;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/contentcapture/ContentCaptureContext$Builder\0", "<init>\0", "(Landroid/content/LocusId;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setExtras](https://developer.android.com/reference/android/view/contentcapture/ContentCaptureContext.Builder.html#setExtras(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-view-contentcapture-ContentCaptureContext_Builder"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-view-contentcapture-ContentCaptureContext_Builder")))]
        pub fn setExtras<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::contentcapture::ContentCaptureContext_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/contentcapture/ContentCaptureContext$Builder", java.flags == PUBLIC, .name == "setExtras", .descriptor == "(Landroid/os/Bundle;)Landroid/view/contentcapture/ContentCaptureContext$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/contentcapture/ContentCaptureContext$Builder\0", "setExtras\0", "(Landroid/os/Bundle;)Landroid/view/contentcapture/ContentCaptureContext$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/view/contentcapture/ContentCaptureContext.Builder.html#build())
        ///
        /// Required features: "android-view-contentcapture-ContentCaptureContext"
        #[cfg(any(feature = "all", all(feature = "android-view-contentcapture-ContentCaptureContext")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::contentcapture::ContentCaptureContext>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/contentcapture/ContentCaptureContext$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/view/contentcapture/ContentCaptureContext;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/contentcapture/ContentCaptureContext$Builder\0", "build\0", "()Landroid/view/contentcapture/ContentCaptureContext;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
