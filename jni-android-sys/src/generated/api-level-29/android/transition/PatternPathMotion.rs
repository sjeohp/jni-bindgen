// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-transition-PatternPathMotion"))]
__jni_bindgen! {
    /// public class [PatternPathMotion](https://developer.android.com/reference/android/transition/PatternPathMotion.html)
    ///
    /// Required feature: android-transition-PatternPathMotion
    public class PatternPathMotion ("android/transition/PatternPathMotion") extends crate::android::transition::PathMotion {

        /// [PatternPathMotion](https://developer.android.com/reference/android/transition/PatternPathMotion.html#PatternPathMotion())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::PatternPathMotion>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/PatternPathMotion", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/PatternPathMotion\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PatternPathMotion](https://developer.android.com/reference/android/transition/PatternPathMotion.html#PatternPathMotion(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::PatternPathMotion>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/PatternPathMotion", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/PatternPathMotion\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PatternPathMotion](https://developer.android.com/reference/android/transition/PatternPathMotion.html#PatternPathMotion(android.graphics.Path))
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn new_Path<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Path>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::transition::PatternPathMotion>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/PatternPathMotion", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/Path;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/PatternPathMotion\0", "<init>\0", "(Landroid/graphics/Path;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPatternPath](https://developer.android.com/reference/android/transition/PatternPathMotion.html#getPatternPath())
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn getPatternPath<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/PatternPathMotion", java.flags == PUBLIC, .name == "getPatternPath", .descriptor == "()Landroid/graphics/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/PatternPathMotion\0", "getPatternPath\0", "()Landroid/graphics/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPatternPath](https://developer.android.com/reference/android/transition/PatternPathMotion.html#setPatternPath(android.graphics.Path))
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn setPatternPath<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Path>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/PatternPathMotion", java.flags == PUBLIC, .name == "setPatternPath", .descriptor == "(Landroid/graphics/Path;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/PatternPathMotion\0", "setPatternPath\0", "(Landroid/graphics/Path;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPath](https://developer.android.com/reference/android/transition/PatternPathMotion.html#getPath(float,%20float,%20float,%20float))
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn getPath<'env>(&'env self, arg0: f32, arg1: f32, arg2: f32, arg3: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/transition/PatternPathMotion", java.flags == PUBLIC, .name == "getPath", .descriptor == "(FFFF)Landroid/graphics/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/transition/PatternPathMotion\0", "getPath\0", "(FFFF)Landroid/graphics/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
