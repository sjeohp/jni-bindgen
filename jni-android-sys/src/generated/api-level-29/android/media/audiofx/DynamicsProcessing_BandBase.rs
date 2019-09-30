// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-DynamicsProcessing_BandBase"))]
__jni_bindgen! {
    /// public class [DynamicsProcessing.BandBase](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html)
    ///
    /// Required feature: android-media-audiofx-DynamicsProcessing_BandBase
    public class DynamicsProcessing_BandBase ("android/media/audiofx/DynamicsProcessing$BandBase") extends crate::java::lang::Object {

        /// [BandBase](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html#BandBase(boolean,%20float))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: bool, arg1: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::audiofx::DynamicsProcessing_BandBase>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/DynamicsProcessing$BandBase", java.flags == PUBLIC, .name == "<init>", .descriptor == "(ZF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/DynamicsProcessing$BandBase\0", "<init>\0", "(ZF)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/DynamicsProcessing$BandBase", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/DynamicsProcessing$BandBase\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEnabled](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html#isEnabled())
        pub fn isEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/DynamicsProcessing$BandBase", java.flags == PUBLIC, .name == "isEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/DynamicsProcessing$BandBase\0", "isEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEnabled](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html#setEnabled(boolean))
        pub fn setEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/DynamicsProcessing$BandBase", java.flags == PUBLIC, .name == "setEnabled", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/DynamicsProcessing$BandBase\0", "setEnabled\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCutoffFrequency](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html#getCutoffFrequency())
        pub fn getCutoffFrequency<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/DynamicsProcessing$BandBase", java.flags == PUBLIC, .name == "getCutoffFrequency", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/DynamicsProcessing$BandBase\0", "getCutoffFrequency\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCutoffFrequency](https://developer.android.com/reference/android/media/audiofx/DynamicsProcessing.BandBase.html#setCutoffFrequency(float))
        pub fn setCutoffFrequency<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/DynamicsProcessing$BandBase", java.flags == PUBLIC, .name == "setCutoffFrequency", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/DynamicsProcessing$BandBase\0", "setCutoffFrequency\0", "(F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
