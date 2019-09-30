// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-RemoteConnection_VideoProvider"))]
__jni_bindgen! {
    /// public class [RemoteConnection.VideoProvider](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html)
    ///
    /// Required feature: android-telecom-RemoteConnection_VideoProvider
    public class RemoteConnection_VideoProvider ("android/telecom/RemoteConnection$VideoProvider") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [VideoProvider](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#VideoProvider())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::RemoteConnection_VideoProvider>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [registerCallback](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#registerCallback(android.telecom.RemoteConnection.VideoProvider.Callback))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider_Callback"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider_Callback")))]
        pub fn registerCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider_Callback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "registerCallback", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider$Callback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "registerCallback\0", "(Landroid/telecom/RemoteConnection$VideoProvider$Callback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterCallback](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#unregisterCallback(android.telecom.RemoteConnection.VideoProvider.Callback))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider_Callback"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider_Callback")))]
        pub fn unregisterCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider_Callback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "unregisterCallback", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider$Callback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "unregisterCallback\0", "(Landroid/telecom/RemoteConnection$VideoProvider$Callback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCamera](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#setCamera(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setCamera<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "setCamera", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "setCamera\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPreviewSurface](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#setPreviewSurface(android.view.Surface))
        ///
        /// Required features: "android-view-Surface"
        #[cfg(any(feature = "all", all(feature = "android-view-Surface")))]
        pub fn setPreviewSurface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "setPreviewSurface", .descriptor == "(Landroid/view/Surface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "setPreviewSurface\0", "(Landroid/view/Surface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDisplaySurface](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#setDisplaySurface(android.view.Surface))
        ///
        /// Required features: "android-view-Surface"
        #[cfg(any(feature = "all", all(feature = "android-view-Surface")))]
        pub fn setDisplaySurface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "setDisplaySurface", .descriptor == "(Landroid/view/Surface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "setDisplaySurface\0", "(Landroid/view/Surface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDeviceOrientation](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#setDeviceOrientation(int))
        pub fn setDeviceOrientation<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "setDeviceOrientation", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "setDeviceOrientation\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setZoom](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#setZoom(float))
        pub fn setZoom<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "setZoom", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "setZoom\0", "(F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sendSessionModifyRequest](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#sendSessionModifyRequest(android.telecom.VideoProfile,%20android.telecom.VideoProfile))
        ///
        /// Required features: "android-telecom-VideoProfile"
        #[cfg(any(feature = "all", all(feature = "android-telecom-VideoProfile")))]
        pub fn sendSessionModifyRequest<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "sendSessionModifyRequest", .descriptor == "(Landroid/telecom/VideoProfile;Landroid/telecom/VideoProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "sendSessionModifyRequest\0", "(Landroid/telecom/VideoProfile;Landroid/telecom/VideoProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sendSessionModifyResponse](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#sendSessionModifyResponse(android.telecom.VideoProfile))
        ///
        /// Required features: "android-telecom-VideoProfile"
        #[cfg(any(feature = "all", all(feature = "android-telecom-VideoProfile")))]
        pub fn sendSessionModifyResponse<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "sendSessionModifyResponse", .descriptor == "(Landroid/telecom/VideoProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "sendSessionModifyResponse\0", "(Landroid/telecom/VideoProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestCameraCapabilities](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#requestCameraCapabilities())
        pub fn requestCameraCapabilities<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "requestCameraCapabilities", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "requestCameraCapabilities\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestCallDataUsage](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#requestCallDataUsage())
        pub fn requestCallDataUsage<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "requestCallDataUsage", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "requestCallDataUsage\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPauseImage](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.html#setPauseImage(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn setPauseImage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider", java.flags == PUBLIC, .name == "setPauseImage", .descriptor == "(Landroid/net/Uri;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider\0", "setPauseImage\0", "(Landroid/net/Uri;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
