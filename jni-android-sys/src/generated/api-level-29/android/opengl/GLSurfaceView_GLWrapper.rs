// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-opengl-GLSurfaceView_GLWrapper"))]
__jni_bindgen! {
    /// public interface [GLSurfaceView.GLWrapper](https://developer.android.com/reference/android/opengl/GLSurfaceView.GLWrapper.html)
    ///
    /// Required feature: android-opengl-GLSurfaceView_GLWrapper
    public interface GLSurfaceView_GLWrapper ("android/opengl/GLSurfaceView$GLWrapper") extends crate::java::lang::Object {

        /// [wrap](https://developer.android.com/reference/android/opengl/GLSurfaceView.GLWrapper.html#wrap(javax.microedition.khronos.opengles.GL))
        ///
        /// Required features: "javax-microedition-khronos-opengles-GL"
        #[cfg(any(feature = "all", all(feature = "javax-microedition-khronos-opengles-GL")))]
        pub fn wrap<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::microedition::khronos::opengles::GL>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::microedition::khronos::opengles::GL>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/GLSurfaceView$GLWrapper", java.flags == PUBLIC | ABSTRACT, .name == "wrap", .descriptor == "(Ljavax/microedition/khronos/opengles/GL;)Ljavax/microedition/khronos/opengles/GL;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/GLSurfaceView$GLWrapper\0", "wrap\0", "(Ljavax/microedition/khronos/opengles/GL;)Ljavax/microedition/khronos/opengles/GL;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
