// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-AcousticEchoCanceler"))]
__jni_bindgen! {
    /// public class [AcousticEchoCanceler](https://developer.android.com/reference/android/media/audiofx/AcousticEchoCanceler.html)
    ///
    /// Required feature: android-media-audiofx-AcousticEchoCanceler
    public class AcousticEchoCanceler ("android/media/audiofx/AcousticEchoCanceler") extends crate::android::media::audiofx::AudioEffect {

        // // Not emitting: Non-public method
        // /// [AcousticEchoCanceler](https://developer.android.com/reference/android/media/audiofx/AcousticEchoCanceler.html#AcousticEchoCanceler(int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::audiofx::AcousticEchoCanceler>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/audiofx/AcousticEchoCanceler", java.flags == (empty), .name == "<init>", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/AcousticEchoCanceler\0", "<init>\0", "(I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isAvailable](https://developer.android.com/reference/android/media/audiofx/AcousticEchoCanceler.html#isAvailable())
        pub fn isAvailable<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/AcousticEchoCanceler", java.flags == PUBLIC | STATIC, .name == "isAvailable", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/audiofx/AcousticEchoCanceler\0", "isAvailable\0", "()Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/media/audiofx/AcousticEchoCanceler.html#create(int))
        ///
        /// Required features: "android-media-audiofx-AcousticEchoCanceler"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-AcousticEchoCanceler")))]
        pub fn create<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::audiofx::AcousticEchoCanceler>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/AcousticEchoCanceler", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(I)Landroid/media/audiofx/AcousticEchoCanceler;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/audiofx/AcousticEchoCanceler\0", "create\0", "(I)Landroid/media/audiofx/AcousticEchoCanceler;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
