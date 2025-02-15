// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-wallpaper-WallpaperService_Engine"))]
__jni_bindgen! {
    /// public class [WallpaperService.Engine](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html)
    ///
    /// Required feature: android-service-wallpaper-WallpaperService_Engine
    public class WallpaperService_Engine ("android/service/wallpaper/WallpaperService$Engine") extends crate::java::lang::Object {

        /// [Engine](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#Engine(android.service.wallpaper.WallpaperService))
        ///
        /// Required features: "android-service-wallpaper-WallpaperService"
        #[cfg(any(feature = "all", all(feature = "android-service-wallpaper-WallpaperService")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::wallpaper::WallpaperService>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::wallpaper::WallpaperService_Engine>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/service/wallpaper/WallpaperService;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "<init>\0", "(Landroid/service/wallpaper/WallpaperService;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSurfaceHolder](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#getSurfaceHolder())
        ///
        /// Required features: "android-view-SurfaceHolder"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceHolder")))]
        pub fn getSurfaceHolder<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceHolder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "getSurfaceHolder", .descriptor == "()Landroid/view/SurfaceHolder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "getSurfaceHolder\0", "()Landroid/view/SurfaceHolder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDesiredMinimumWidth](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#getDesiredMinimumWidth())
        pub fn getDesiredMinimumWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "getDesiredMinimumWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "getDesiredMinimumWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDesiredMinimumHeight](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#getDesiredMinimumHeight())
        pub fn getDesiredMinimumHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "getDesiredMinimumHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "getDesiredMinimumHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isVisible](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#isVisible())
        pub fn isVisible<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "isVisible", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "isVisible\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isPreview](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#isPreview())
        pub fn isPreview<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "isPreview", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "isPreview\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTouchEventsEnabled](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#setTouchEventsEnabled(boolean))
        pub fn setTouchEventsEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "setTouchEventsEnabled", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "setTouchEventsEnabled\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOffsetNotificationsEnabled](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#setOffsetNotificationsEnabled(boolean))
        pub fn setOffsetNotificationsEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "setOffsetNotificationsEnabled", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "setOffsetNotificationsEnabled\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreate](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onCreate(android.view.SurfaceHolder))
        ///
        /// Required features: "android-view-SurfaceHolder"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceHolder")))]
        pub fn onCreate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceHolder>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onCreate", .descriptor == "(Landroid/view/SurfaceHolder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onCreate\0", "(Landroid/view/SurfaceHolder;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDestroy](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onDestroy())
        pub fn onDestroy<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onDestroy", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onDestroy\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVisibilityChanged](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onVisibilityChanged(boolean))
        pub fn onVisibilityChanged<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onVisibilityChanged", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onVisibilityChanged\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onApplyWindowInsets](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onApplyWindowInsets(android.view.WindowInsets))
        ///
        /// Required features: "android-view-WindowInsets"
        #[cfg(any(feature = "all", all(feature = "android-view-WindowInsets")))]
        pub fn onApplyWindowInsets<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::WindowInsets>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onApplyWindowInsets", .descriptor == "(Landroid/view/WindowInsets;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onApplyWindowInsets\0", "(Landroid/view/WindowInsets;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTouchEvent](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onTouchEvent(android.view.MotionEvent))
        ///
        /// Required features: "android-view-MotionEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent")))]
        pub fn onTouchEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onTouchEvent", .descriptor == "(Landroid/view/MotionEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onTouchEvent\0", "(Landroid/view/MotionEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onOffsetsChanged](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onOffsetsChanged(float,%20float,%20float,%20float,%20int,%20int))
        pub fn onOffsetsChanged<'env>(&'env self, arg0: f32, arg1: f32, arg2: f32, arg3: f32, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onOffsetsChanged", .descriptor == "(FFFFII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onOffsetsChanged\0", "(FFFFII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCommand](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onCommand(java.lang.String,%20int,%20int,%20int,%20android.os.Bundle,%20boolean))
        ///
        /// Required features: "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn onCommand<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>, arg5: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onCommand", .descriptor == "(Ljava/lang/String;IIILandroid/os/Bundle;Z)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onCommand\0", "(Ljava/lang/String;IIILandroid/os/Bundle;Z)Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDesiredSizeChanged](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onDesiredSizeChanged(int,%20int))
        pub fn onDesiredSizeChanged<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onDesiredSizeChanged", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onDesiredSizeChanged\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSurfaceChanged](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onSurfaceChanged(android.view.SurfaceHolder,%20int,%20int,%20int))
        ///
        /// Required features: "android-view-SurfaceHolder"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceHolder")))]
        pub fn onSurfaceChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceHolder>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onSurfaceChanged", .descriptor == "(Landroid/view/SurfaceHolder;III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onSurfaceChanged\0", "(Landroid/view/SurfaceHolder;III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSurfaceRedrawNeeded](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onSurfaceRedrawNeeded(android.view.SurfaceHolder))
        ///
        /// Required features: "android-view-SurfaceHolder"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceHolder")))]
        pub fn onSurfaceRedrawNeeded<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceHolder>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onSurfaceRedrawNeeded", .descriptor == "(Landroid/view/SurfaceHolder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onSurfaceRedrawNeeded\0", "(Landroid/view/SurfaceHolder;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSurfaceCreated](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onSurfaceCreated(android.view.SurfaceHolder))
        ///
        /// Required features: "android-view-SurfaceHolder"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceHolder")))]
        pub fn onSurfaceCreated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceHolder>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onSurfaceCreated", .descriptor == "(Landroid/view/SurfaceHolder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onSurfaceCreated\0", "(Landroid/view/SurfaceHolder;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSurfaceDestroyed](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onSurfaceDestroyed(android.view.SurfaceHolder))
        ///
        /// Required features: "android-view-SurfaceHolder"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceHolder")))]
        pub fn onSurfaceDestroyed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceHolder>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onSurfaceDestroyed", .descriptor == "(Landroid/view/SurfaceHolder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onSurfaceDestroyed\0", "(Landroid/view/SurfaceHolder;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyColorsChanged](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#notifyColorsChanged())
        pub fn notifyColorsChanged<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "notifyColorsChanged", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "notifyColorsChanged\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onComputeColors](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#onComputeColors())
        ///
        /// Required features: "android-app-WallpaperColors"
        #[cfg(any(feature = "all", all(feature = "android-app-WallpaperColors")))]
        pub fn onComputeColors<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::WallpaperColors>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "onComputeColors", .descriptor == "()Landroid/app/WallpaperColors;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "onComputeColors\0", "()Landroid/app/WallpaperColors;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [dump](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#dump(java.lang.String,%20java.io.FileDescriptor,%20java.io.PrintWriter,%20java.lang.String%5B%5D))
        // ///
        // /// Required features: "java-io-FileDescriptor", "java-io-PrintWriter", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor", feature = "java-io-PrintWriter", feature = "java-lang-String")))]
        // fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PrintWriter>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PROTECTED, .name == "dump", .descriptor == "(Ljava/lang/String;Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "dump\0", "(Ljava/lang/String;Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDisplayContext](https://developer.android.com/reference/android/service/wallpaper/WallpaperService.Engine.html#getDisplayContext())
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn getDisplayContext<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Context>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/wallpaper/WallpaperService$Engine", java.flags == PUBLIC, .name == "getDisplayContext", .descriptor == "()Landroid/content/Context;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/wallpaper/WallpaperService$Engine\0", "getDisplayContext\0", "()Landroid/content/Context;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: this$N outer class pointer
        // pub fn get_"this$0"<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::service::wallpaper::WallpaperService>> { ... }
    }
}
