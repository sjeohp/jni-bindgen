// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-HardwareRenderer_FrameRenderRequest"))]
__jni_bindgen! {
    /// public final class [HardwareRenderer.FrameRenderRequest](https://developer.android.com/reference/android/graphics/HardwareRenderer.FrameRenderRequest.html)
    ///
    /// Required feature: android-graphics-HardwareRenderer_FrameRenderRequest
    public final class HardwareRenderer_FrameRenderRequest ("android/graphics/HardwareRenderer$FrameRenderRequest") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [FrameRenderRequest](https://developer.android.com/reference/android/graphics/HardwareRenderer.FrameRenderRequest.html#FrameRenderRequest(android.graphics.HardwareRenderer))
        // ///
        // /// Required features: "android-graphics-HardwareRenderer"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-HardwareRenderer")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::HardwareRenderer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::HardwareRenderer_FrameRenderRequest>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/HardwareRenderer$FrameRenderRequest", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/graphics/HardwareRenderer;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/HardwareRenderer$FrameRenderRequest\0", "<init>\0", "(Landroid/graphics/HardwareRenderer;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [setVsyncTime](https://developer.android.com/reference/android/graphics/HardwareRenderer.FrameRenderRequest.html#setVsyncTime(long))
        ///
        /// Required features: "android-graphics-HardwareRenderer_FrameRenderRequest"
        #[cfg(any(feature = "all", all(feature = "android-graphics-HardwareRenderer_FrameRenderRequest")))]
        pub fn setVsyncTime<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::HardwareRenderer_FrameRenderRequest>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/HardwareRenderer$FrameRenderRequest", java.flags == PUBLIC, .name == "setVsyncTime", .descriptor == "(J)Landroid/graphics/HardwareRenderer$FrameRenderRequest;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/HardwareRenderer$FrameRenderRequest\0", "setVsyncTime\0", "(J)Landroid/graphics/HardwareRenderer$FrameRenderRequest;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFrameCommitCallback](https://developer.android.com/reference/android/graphics/HardwareRenderer.FrameRenderRequest.html#setFrameCommitCallback(java.util.concurrent.Executor,%20java.lang.Runnable))
        ///
        /// Required features: "android-graphics-HardwareRenderer_FrameRenderRequest", "java-lang-Runnable", "java-util-concurrent-Executor"
        #[cfg(any(feature = "all", all(feature = "android-graphics-HardwareRenderer_FrameRenderRequest", feature = "java-lang-Runnable", feature = "java-util-concurrent-Executor")))]
        pub fn setFrameCommitCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::HardwareRenderer_FrameRenderRequest>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/HardwareRenderer$FrameRenderRequest", java.flags == PUBLIC, .name == "setFrameCommitCallback", .descriptor == "(Ljava/util/concurrent/Executor;Ljava/lang/Runnable;)Landroid/graphics/HardwareRenderer$FrameRenderRequest;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/HardwareRenderer$FrameRenderRequest\0", "setFrameCommitCallback\0", "(Ljava/util/concurrent/Executor;Ljava/lang/Runnable;)Landroid/graphics/HardwareRenderer$FrameRenderRequest;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWaitForPresent](https://developer.android.com/reference/android/graphics/HardwareRenderer.FrameRenderRequest.html#setWaitForPresent(boolean))
        ///
        /// Required features: "android-graphics-HardwareRenderer_FrameRenderRequest"
        #[cfg(any(feature = "all", all(feature = "android-graphics-HardwareRenderer_FrameRenderRequest")))]
        pub fn setWaitForPresent<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::HardwareRenderer_FrameRenderRequest>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/HardwareRenderer$FrameRenderRequest", java.flags == PUBLIC, .name == "setWaitForPresent", .descriptor == "(Z)Landroid/graphics/HardwareRenderer$FrameRenderRequest;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/HardwareRenderer$FrameRenderRequest\0", "setWaitForPresent\0", "(Z)Landroid/graphics/HardwareRenderer$FrameRenderRequest;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [syncAndDraw](https://developer.android.com/reference/android/graphics/HardwareRenderer.FrameRenderRequest.html#syncAndDraw())
        pub fn syncAndDraw<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/HardwareRenderer$FrameRenderRequest", java.flags == PUBLIC, .name == "syncAndDraw", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/HardwareRenderer$FrameRenderRequest\0", "syncAndDraw\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: this$N outer class pointer
        // pub fn get_"this$0"<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::HardwareRenderer>> { ... }
    }
}
