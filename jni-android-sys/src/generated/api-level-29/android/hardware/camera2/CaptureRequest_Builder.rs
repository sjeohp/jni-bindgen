// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-camera2-CaptureRequest_Builder"))]
__jni_bindgen! {
    /// public final class [CaptureRequest.Builder](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html)
    ///
    /// Required feature: android-hardware-camera2-CaptureRequest_Builder
    public final class CaptureRequest_Builder ("android/hardware/camera2/CaptureRequest$Builder") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Builder](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#Builder())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::camera2::CaptureRequest_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [addTarget](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#addTarget(android.view.Surface))
        ///
        /// Required features: "android-view-Surface"
        #[cfg(any(feature = "all", all(feature = "android-view-Surface")))]
        pub fn addTarget<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "addTarget", .descriptor == "(Landroid/view/Surface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "addTarget\0", "(Landroid/view/Surface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeTarget](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#removeTarget(android.view.Surface))
        ///
        /// Required features: "android-view-Surface"
        #[cfg(any(feature = "all", all(feature = "android-view-Surface")))]
        pub fn removeTarget<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "removeTarget", .descriptor == "(Landroid/view/Surface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "removeTarget\0", "(Landroid/view/Surface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#set(android.hardware.camera2.CaptureRequest.Key,%20java.lang.Object))
        ///
        /// Required features: "android-hardware-camera2-CaptureRequest_Key", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CaptureRequest_Key", feature = "java-lang-Object")))]
        pub fn set<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CaptureRequest_Key>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "set", .descriptor == "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "set\0", "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#get(android.hardware.camera2.CaptureRequest.Key))
        ///
        /// Required features: "android-hardware-camera2-CaptureRequest_Key", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CaptureRequest_Key", feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CaptureRequest_Key>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "get", .descriptor == "(Landroid/hardware/camera2/CaptureRequest$Key;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "get\0", "(Landroid/hardware/camera2/CaptureRequest$Key;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPhysicalCameraKey](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#setPhysicalCameraKey(android.hardware.camera2.CaptureRequest.Key,%20java.lang.Object,%20java.lang.String))
        ///
        /// Required features: "android-hardware-camera2-CaptureRequest_Builder", "android-hardware-camera2-CaptureRequest_Key", "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CaptureRequest_Builder", feature = "android-hardware-camera2-CaptureRequest_Key", feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn setPhysicalCameraKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CaptureRequest_Key>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::hardware::camera2::CaptureRequest_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "setPhysicalCameraKey", .descriptor == "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/Object;Ljava/lang/String;)Landroid/hardware/camera2/CaptureRequest$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "setPhysicalCameraKey\0", "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/Object;Ljava/lang/String;)Landroid/hardware/camera2/CaptureRequest$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPhysicalCameraKey](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#getPhysicalCameraKey(android.hardware.camera2.CaptureRequest.Key,%20java.lang.String))
        ///
        /// Required features: "android-hardware-camera2-CaptureRequest_Key", "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CaptureRequest_Key", feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn getPhysicalCameraKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::camera2::CaptureRequest_Key>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "getPhysicalCameraKey", .descriptor == "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/String;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "getPhysicalCameraKey\0", "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/String;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTag](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#setTag(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setTag<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "setTag", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "setTag\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/hardware/camera2/CaptureRequest.Builder.html#build())
        ///
        /// Required features: "android-hardware-camera2-CaptureRequest"
        #[cfg(any(feature = "all", all(feature = "android-hardware-camera2-CaptureRequest")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::hardware::camera2::CaptureRequest>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/camera2/CaptureRequest$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/hardware/camera2/CaptureRequest;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/camera2/CaptureRequest$Builder\0", "build\0", "()Landroid/hardware/camera2/CaptureRequest;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
