// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-print-pdf-PrintedPdfDocument"))]
__jni_bindgen! {
    /// public class [PrintedPdfDocument](https://developer.android.com/reference/android/print/pdf/PrintedPdfDocument.html)
    ///
    /// Required feature: android-print-pdf-PrintedPdfDocument
    public class PrintedPdfDocument ("android/print/pdf/PrintedPdfDocument") extends crate::android::graphics::pdf::PdfDocument {

        /// [PrintedPdfDocument](https://developer.android.com/reference/android/print/pdf/PrintedPdfDocument.html#PrintedPdfDocument(android.content.Context,%20android.print.PrintAttributes))
        ///
        /// Required features: "android-content-Context", "android-print-PrintAttributes"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-print-PrintAttributes")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::print::PrintAttributes>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::print::pdf::PrintedPdfDocument>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/pdf/PrintedPdfDocument", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/print/PrintAttributes;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/pdf/PrintedPdfDocument\0", "<init>\0", "(Landroid/content/Context;Landroid/print/PrintAttributes;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startPage](https://developer.android.com/reference/android/print/pdf/PrintedPdfDocument.html#startPage(int))
        ///
        /// Required features: "android-graphics-pdf-PdfDocument_Page"
        #[cfg(any(feature = "all", all(feature = "android-graphics-pdf-PdfDocument_Page")))]
        pub fn startPage<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::pdf::PdfDocument_Page>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/pdf/PrintedPdfDocument", java.flags == PUBLIC, .name == "startPage", .descriptor == "(I)Landroid/graphics/pdf/PdfDocument$Page;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/pdf/PrintedPdfDocument\0", "startPage\0", "(I)Landroid/graphics/pdf/PdfDocument$Page;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPageWidth](https://developer.android.com/reference/android/print/pdf/PrintedPdfDocument.html#getPageWidth())
        pub fn getPageWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/pdf/PrintedPdfDocument", java.flags == PUBLIC, .name == "getPageWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/pdf/PrintedPdfDocument\0", "getPageWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPageHeight](https://developer.android.com/reference/android/print/pdf/PrintedPdfDocument.html#getPageHeight())
        pub fn getPageHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/pdf/PrintedPdfDocument", java.flags == PUBLIC, .name == "getPageHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/pdf/PrintedPdfDocument\0", "getPageHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPageContentRect](https://developer.android.com/reference/android/print/pdf/PrintedPdfDocument.html#getPageContentRect())
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn getPageContentRect<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Rect>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/pdf/PrintedPdfDocument", java.flags == PUBLIC, .name == "getPageContentRect", .descriptor == "()Landroid/graphics/Rect;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/pdf/PrintedPdfDocument\0", "getPageContentRect\0", "()Landroid/graphics/Rect;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
