// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-text-LineBreaker_ParagraphConstraints"))]
__jni_bindgen! {
    /// public class [LineBreaker.ParagraphConstraints](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html)
    ///
    /// Required feature: android-graphics-text-LineBreaker_ParagraphConstraints
    public class LineBreaker_ParagraphConstraints ("android/graphics/text/LineBreaker$ParagraphConstraints") extends crate::java::lang::Object {

        /// [ParagraphConstraints](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#ParagraphConstraints())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::text::LineBreaker_ParagraphConstraints>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWidth](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#setWidth(float))
        pub fn setWidth<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "setWidth", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "setWidth\0", "(F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIndent](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#setIndent(float,%20int))
        pub fn setIndent<'env>(&'env self, arg0: f32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "setIndent", .descriptor == "(FI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "setIndent\0", "(FI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTabStops](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#setTabStops(float%5B%5D,%20float))
        pub fn setTabStops<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg1: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "setTabStops", .descriptor == "([FF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "setTabStops\0", "([FF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWidth](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#getWidth())
        pub fn getWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "getWidth", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "getWidth\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFirstWidth](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#getFirstWidth())
        pub fn getFirstWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "getFirstWidth", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "getFirstWidth\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFirstWidthLineCount](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#getFirstWidthLineCount())
        pub fn getFirstWidthLineCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "getFirstWidthLineCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "getFirstWidthLineCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTabStops](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#getTabStops())
        pub fn getTabStops<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::FloatArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "getTabStops", .descriptor == "()[F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "getTabStops\0", "()[F\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultTabStop](https://developer.android.com/reference/android/graphics/text/LineBreaker.ParagraphConstraints.html#getDefaultTabStop())
        pub fn getDefaultTabStop<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$ParagraphConstraints", java.flags == PUBLIC, .name == "getDefaultTabStop", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$ParagraphConstraints\0", "getDefaultTabStop\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
