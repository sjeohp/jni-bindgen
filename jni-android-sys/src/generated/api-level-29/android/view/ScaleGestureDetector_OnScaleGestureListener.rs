// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-ScaleGestureDetector_OnScaleGestureListener"))]
__jni_bindgen! {
    /// public interface [ScaleGestureDetector.OnScaleGestureListener](https://developer.android.com/reference/android/view/ScaleGestureDetector.OnScaleGestureListener.html)
    ///
    /// Required feature: android-view-ScaleGestureDetector_OnScaleGestureListener
    public interface ScaleGestureDetector_OnScaleGestureListener ("android/view/ScaleGestureDetector$OnScaleGestureListener") extends crate::java::lang::Object {

        /// [onScale](https://developer.android.com/reference/android/view/ScaleGestureDetector.OnScaleGestureListener.html#onScale(android.view.ScaleGestureDetector))
        ///
        /// Required features: "android-view-ScaleGestureDetector"
        #[cfg(any(feature = "all", all(feature = "android-view-ScaleGestureDetector")))]
        pub fn onScale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ScaleGestureDetector>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector$OnScaleGestureListener", java.flags == PUBLIC | ABSTRACT, .name == "onScale", .descriptor == "(Landroid/view/ScaleGestureDetector;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector$OnScaleGestureListener\0", "onScale\0", "(Landroid/view/ScaleGestureDetector;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onScaleBegin](https://developer.android.com/reference/android/view/ScaleGestureDetector.OnScaleGestureListener.html#onScaleBegin(android.view.ScaleGestureDetector))
        ///
        /// Required features: "android-view-ScaleGestureDetector"
        #[cfg(any(feature = "all", all(feature = "android-view-ScaleGestureDetector")))]
        pub fn onScaleBegin<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ScaleGestureDetector>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector$OnScaleGestureListener", java.flags == PUBLIC | ABSTRACT, .name == "onScaleBegin", .descriptor == "(Landroid/view/ScaleGestureDetector;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector$OnScaleGestureListener\0", "onScaleBegin\0", "(Landroid/view/ScaleGestureDetector;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onScaleEnd](https://developer.android.com/reference/android/view/ScaleGestureDetector.OnScaleGestureListener.html#onScaleEnd(android.view.ScaleGestureDetector))
        ///
        /// Required features: "android-view-ScaleGestureDetector"
        #[cfg(any(feature = "all", all(feature = "android-view-ScaleGestureDetector")))]
        pub fn onScaleEnd<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ScaleGestureDetector>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector$OnScaleGestureListener", java.flags == PUBLIC | ABSTRACT, .name == "onScaleEnd", .descriptor == "(Landroid/view/ScaleGestureDetector;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector$OnScaleGestureListener\0", "onScaleEnd\0", "(Landroid/view/ScaleGestureDetector;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
