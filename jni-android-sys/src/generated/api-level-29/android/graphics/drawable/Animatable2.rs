// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-drawable-Animatable2"))]
__jni_bindgen! {
    /// public interface [Animatable2](https://developer.android.com/reference/android/graphics/drawable/Animatable2.html)
    ///
    /// Required feature: android-graphics-drawable-Animatable2
    public interface Animatable2 ("android/graphics/drawable/Animatable2") extends crate::java::lang::Object, implements crate::android::graphics::drawable::Animatable {

        /// [registerAnimationCallback](https://developer.android.com/reference/android/graphics/drawable/Animatable2.html#registerAnimationCallback(android.graphics.drawable.Animatable2.AnimationCallback))
        ///
        /// Required features: "android-graphics-drawable-Animatable2_AnimationCallback"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Animatable2_AnimationCallback")))]
        pub fn registerAnimationCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Animatable2_AnimationCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/Animatable2", java.flags == PUBLIC | ABSTRACT, .name == "registerAnimationCallback", .descriptor == "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/Animatable2\0", "registerAnimationCallback\0", "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterAnimationCallback](https://developer.android.com/reference/android/graphics/drawable/Animatable2.html#unregisterAnimationCallback(android.graphics.drawable.Animatable2.AnimationCallback))
        ///
        /// Required features: "android-graphics-drawable-Animatable2_AnimationCallback"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Animatable2_AnimationCallback")))]
        pub fn unregisterAnimationCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Animatable2_AnimationCallback>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/Animatable2", java.flags == PUBLIC | ABSTRACT, .name == "unregisterAnimationCallback", .descriptor == "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/Animatable2\0", "unregisterAnimationCallback\0", "(Landroid/graphics/drawable/Animatable2$AnimationCallback;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clearAnimationCallbacks](https://developer.android.com/reference/android/graphics/drawable/Animatable2.html#clearAnimationCallbacks())
        pub fn clearAnimationCallbacks<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/Animatable2", java.flags == PUBLIC | ABSTRACT, .name == "clearAnimationCallbacks", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/Animatable2\0", "clearAnimationCallbacks\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
