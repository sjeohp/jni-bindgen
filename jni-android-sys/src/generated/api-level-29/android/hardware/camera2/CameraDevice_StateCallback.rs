// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-camera2-CameraDevice_StateCallback"))]
__jni_bindgen! {
    /// public class [CameraDevice.StateCallback](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html)
    ///
    /// Required feature: android-hardware-camera2-CameraDevice_StateCallback
    public class CameraDevice_StateCallback ("android/hardware/camera2/CameraDevice$StateCallback") extends crate::java::lang::Object {

        /// [StateCallback](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#StateCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::camera2::CameraDevice_StateCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CameraDevice$StateCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CameraDevice$StateCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onOpened](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#onOpened(android.hardware.camera2.CameraDevice))
        ///
        /// Required features: "android-hardware-camera2-CameraDevice"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CameraDevice")))]
        pub fn onOpened<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CameraDevice>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CameraDevice$StateCallback", java.flags == PUBLIC | ABSTRACT, .name == "onOpened", .descriptor == "(Landroid/hardware/camera2/CameraDevice;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CameraDevice$StateCallback\0", "onOpened\0", "(Landroid/hardware/camera2/CameraDevice;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onClosed](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#onClosed(android.hardware.camera2.CameraDevice))
        ///
        /// Required features: "android-hardware-camera2-CameraDevice"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CameraDevice")))]
        pub fn onClosed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CameraDevice>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CameraDevice$StateCallback", java.flags == PUBLIC, .name == "onClosed", .descriptor == "(Landroid/hardware/camera2/CameraDevice;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CameraDevice$StateCallback\0", "onClosed\0", "(Landroid/hardware/camera2/CameraDevice;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisconnected](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#onDisconnected(android.hardware.camera2.CameraDevice))
        ///
        /// Required features: "android-hardware-camera2-CameraDevice"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CameraDevice")))]
        pub fn onDisconnected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CameraDevice>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CameraDevice$StateCallback", java.flags == PUBLIC | ABSTRACT, .name == "onDisconnected", .descriptor == "(Landroid/hardware/camera2/CameraDevice;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CameraDevice$StateCallback\0", "onDisconnected\0", "(Landroid/hardware/camera2/CameraDevice;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onError](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#onError(android.hardware.camera2.CameraDevice,%20int))
        ///
        /// Required features: "android-hardware-camera2-CameraDevice"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CameraDevice")))]
        pub fn onError<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CameraDevice>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CameraDevice$StateCallback", java.flags == PUBLIC | ABSTRACT, .name == "onError", .descriptor == "(Landroid/hardware/camera2/CameraDevice;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CameraDevice$StateCallback\0", "onError\0", "(Landroid/hardware/camera2/CameraDevice;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ERROR_CAMERA_DEVICE](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#ERROR_CAMERA_DEVICE)
        pub const ERROR_CAMERA_DEVICE : i32 = 4;

        /// public static final [ERROR_CAMERA_DISABLED](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#ERROR_CAMERA_DISABLED)
        pub const ERROR_CAMERA_DISABLED : i32 = 3;

        /// public static final [ERROR_CAMERA_IN_USE](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#ERROR_CAMERA_IN_USE)
        pub const ERROR_CAMERA_IN_USE : i32 = 1;

        /// public static final [ERROR_CAMERA_SERVICE](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#ERROR_CAMERA_SERVICE)
        pub const ERROR_CAMERA_SERVICE : i32 = 5;

        /// public static final [ERROR_MAX_CAMERAS_IN_USE](https://developer.android.com/reference/android/hardware/camera2/CameraDevice.StateCallback.html#ERROR_MAX_CAMERAS_IN_USE)
        pub const ERROR_MAX_CAMERAS_IN_USE : i32 = 2;
    }
}
