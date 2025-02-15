// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-text-LineBreaker_Result"))]
__jni_bindgen! {
    /// public class [LineBreaker.Result](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html)
    ///
    /// Required feature: android-graphics-text-LineBreaker_Result
    public class LineBreaker_Result ("android/graphics/text/LineBreaker$Result") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Result](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#Result(long))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::text::LineBreaker_Result>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/text/LineBreaker$Result", java.flags == (empty), .name == "<init>", .descriptor == "(J)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "<init>\0", "(J)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getLineCount](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getLineCount())
        pub fn getLineCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getLineCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getLineCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLineBreakOffset](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getLineBreakOffset(int))
        pub fn getLineBreakOffset<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getLineBreakOffset", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getLineBreakOffset\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLineWidth](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getLineWidth(int))
        pub fn getLineWidth<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getLineWidth", .descriptor == "(I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getLineWidth\0", "(I)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLineAscent](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getLineAscent(int))
        pub fn getLineAscent<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getLineAscent", .descriptor == "(I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getLineAscent\0", "(I)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLineDescent](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getLineDescent(int))
        pub fn getLineDescent<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getLineDescent", .descriptor == "(I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getLineDescent\0", "(I)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasLineTab](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#hasLineTab(int))
        pub fn hasLineTab<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "hasLineTab", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "hasLineTab\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStartLineHyphenEdit](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getStartLineHyphenEdit(int))
        pub fn getStartLineHyphenEdit<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getStartLineHyphenEdit", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getStartLineHyphenEdit\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEndLineHyphenEdit](https://developer.android.com/reference/android/graphics/text/LineBreaker.Result.html#getEndLineHyphenEdit(int))
        pub fn getEndLineHyphenEdit<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/text/LineBreaker$Result", java.flags == PUBLIC, .name == "getEndLineHyphenEdit", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/text/LineBreaker$Result\0", "getEndLineHyphenEdit\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
