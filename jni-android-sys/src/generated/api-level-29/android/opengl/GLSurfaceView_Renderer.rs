// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-opengl-GLSurfaceView_Renderer"))]
__jni_bindgen! {
    /// public interface [GLSurfaceView.Renderer](https://developer.android.com/reference/android/opengl/GLSurfaceView.Renderer.html)
    ///
    /// Required feature: android-opengl-GLSurfaceView_Renderer
    public interface GLSurfaceView_Renderer ("android/opengl/GLSurfaceView$Renderer") extends crate::java::lang::Object {

        /// [onSurfaceCreated](https://developer.android.com/reference/android/opengl/GLSurfaceView.Renderer.html#onSurfaceCreated(javax.microedition.khronos.opengles.GL10,%20javax.microedition.khronos.egl.EGLConfig))
        ///
        /// Required features: "javax-microedition-khronos-egl-EGLConfig", "javax-microedition-khronos-opengles-GL10"
        #[cfg(any(feature = "all", all(feature = "javax-microedition-khronos-egl-EGLConfig", feature = "javax-microedition-khronos-opengles-GL10")))]
        pub fn onSurfaceCreated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::microedition::khronos::opengles::GL10>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::microedition::khronos::egl::EGLConfig>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/GLSurfaceView$Renderer", java.flags == PUBLIC | ABSTRACT, .name == "onSurfaceCreated", .descriptor == "(Ljavax/microedition/khronos/opengles/GL10;Ljavax/microedition/khronos/egl/EGLConfig;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/GLSurfaceView$Renderer\0", "onSurfaceCreated\0", "(Ljavax/microedition/khronos/opengles/GL10;Ljavax/microedition/khronos/egl/EGLConfig;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSurfaceChanged](https://developer.android.com/reference/android/opengl/GLSurfaceView.Renderer.html#onSurfaceChanged(javax.microedition.khronos.opengles.GL10,%20int,%20int))
        ///
        /// Required features: "javax-microedition-khronos-opengles-GL10"
        #[cfg(any(feature = "all", all(feature = "javax-microedition-khronos-opengles-GL10")))]
        pub fn onSurfaceChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::microedition::khronos::opengles::GL10>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/GLSurfaceView$Renderer", java.flags == PUBLIC | ABSTRACT, .name == "onSurfaceChanged", .descriptor == "(Ljavax/microedition/khronos/opengles/GL10;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/GLSurfaceView$Renderer\0", "onSurfaceChanged\0", "(Ljavax/microedition/khronos/opengles/GL10;II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDrawFrame](https://developer.android.com/reference/android/opengl/GLSurfaceView.Renderer.html#onDrawFrame(javax.microedition.khronos.opengles.GL10))
        ///
        /// Required features: "javax-microedition-khronos-opengles-GL10"
        #[cfg(any(feature = "all", all(feature = "javax-microedition-khronos-opengles-GL10")))]
        pub fn onDrawFrame<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::microedition::khronos::opengles::GL10>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/GLSurfaceView$Renderer", java.flags == PUBLIC | ABSTRACT, .name == "onDrawFrame", .descriptor == "(Ljavax/microedition/khronos/opengles/GL10;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/GLSurfaceView$Renderer\0", "onDrawFrame\0", "(Ljavax/microedition/khronos/opengles/GL10;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
