// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-ColorMatrixColorFilter"))]
__jni_bindgen! {
    /// public class [ColorMatrixColorFilter](https://developer.android.com/reference/android/graphics/ColorMatrixColorFilter.html)
    ///
    /// Required feature: android-graphics-ColorMatrixColorFilter
    public class ColorMatrixColorFilter ("android/graphics/ColorMatrixColorFilter") extends crate::android::graphics::ColorFilter {

        /// [ColorMatrixColorFilter](https://developer.android.com/reference/android/graphics/ColorMatrixColorFilter.html#ColorMatrixColorFilter(android.graphics.ColorMatrix))
        ///
        /// Required features: "android-graphics-ColorMatrix"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorMatrix")))]
        pub fn new_ColorMatrix<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::ColorMatrix>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::ColorMatrixColorFilter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ColorMatrixColorFilter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/ColorMatrix;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ColorMatrixColorFilter\0", "<init>\0", "(Landroid/graphics/ColorMatrix;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ColorMatrixColorFilter](https://developer.android.com/reference/android/graphics/ColorMatrixColorFilter.html#ColorMatrixColorFilter(float%5B%5D))
        pub fn new_float_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::ColorMatrixColorFilter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ColorMatrixColorFilter", java.flags == PUBLIC, .name == "<init>", .descriptor == "([F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ColorMatrixColorFilter\0", "<init>\0", "([F)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getColorMatrix](https://developer.android.com/reference/android/graphics/ColorMatrixColorFilter.html#getColorMatrix(android.graphics.ColorMatrix))
        ///
        /// Required features: "android-graphics-ColorMatrix"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorMatrix")))]
        pub fn getColorMatrix<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::ColorMatrix>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ColorMatrixColorFilter", java.flags == PUBLIC, .name == "getColorMatrix", .descriptor == "(Landroid/graphics/ColorMatrix;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ColorMatrixColorFilter\0", "getColorMatrix\0", "(Landroid/graphics/ColorMatrix;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
