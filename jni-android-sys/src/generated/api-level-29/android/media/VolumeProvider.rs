// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-VolumeProvider"))]
__jni_bindgen! {
    /// public class [VolumeProvider](https://developer.android.com/reference/android/media/VolumeProvider.html)
    ///
    /// Required feature: android-media-VolumeProvider
    public class VolumeProvider ("android/media/VolumeProvider") extends crate::java::lang::Object {

        /// [VolumeProvider](https://developer.android.com/reference/android/media/VolumeProvider.html#VolumeProvider(int,%20int,%20int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::VolumeProvider>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC, .name == "<init>", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "<init>\0", "(III)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVolumeControl](https://developer.android.com/reference/android/media/VolumeProvider.html#getVolumeControl())
        pub fn getVolumeControl<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC | FINAL, .name == "getVolumeControl", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "getVolumeControl\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxVolume](https://developer.android.com/reference/android/media/VolumeProvider.html#getMaxVolume())
        pub fn getMaxVolume<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC | FINAL, .name == "getMaxVolume", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "getMaxVolume\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentVolume](https://developer.android.com/reference/android/media/VolumeProvider.html#getCurrentVolume())
        pub fn getCurrentVolume<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC | FINAL, .name == "getCurrentVolume", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "getCurrentVolume\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCurrentVolume](https://developer.android.com/reference/android/media/VolumeProvider.html#setCurrentVolume(int))
        pub fn setCurrentVolume<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC | FINAL, .name == "setCurrentVolume", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "setCurrentVolume\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSetVolumeTo](https://developer.android.com/reference/android/media/VolumeProvider.html#onSetVolumeTo(int))
        pub fn onSetVolumeTo<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC, .name == "onSetVolumeTo", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "onSetVolumeTo\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAdjustVolume](https://developer.android.com/reference/android/media/VolumeProvider.html#onAdjustVolume(int))
        pub fn onAdjustVolume<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeProvider", java.flags == PUBLIC, .name == "onAdjustVolume", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeProvider\0", "onAdjustVolume\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [VOLUME_CONTROL_ABSOLUTE](https://developer.android.com/reference/android/media/VolumeProvider.html#VOLUME_CONTROL_ABSOLUTE)
        pub const VOLUME_CONTROL_ABSOLUTE : i32 = 2;

        /// public static final [VOLUME_CONTROL_FIXED](https://developer.android.com/reference/android/media/VolumeProvider.html#VOLUME_CONTROL_FIXED)
        pub const VOLUME_CONTROL_FIXED : i32 = 0;

        /// public static final [VOLUME_CONTROL_RELATIVE](https://developer.android.com/reference/android/media/VolumeProvider.html#VOLUME_CONTROL_RELATIVE)
        pub const VOLUME_CONTROL_RELATIVE : i32 = 1;
    }
}
