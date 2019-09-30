// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaRouter_RouteGroup"))]
__jni_bindgen! {
    /// public class [MediaRouter.RouteGroup](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html)
    ///
    /// Required feature: android-media-MediaRouter_RouteGroup
    public class MediaRouter_RouteGroup ("android/media/MediaRouter$RouteGroup") extends crate::android::media::MediaRouter_RouteInfo {

        // // Not emitting: Non-public method
        // /// [RouteGroup](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#RouteGroup(android.media.MediaRouter.RouteCategory))
        // ///
        // /// Required features: "android-media-MediaRouter_RouteCategory"
        // #[cfg(any(feature = "all", all(feature = "android-media-MediaRouter_RouteCategory")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaRouter_RouteCategory>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaRouter_RouteGroup>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/MediaRouter$RouteGroup", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/media/MediaRouter$RouteCategory;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "<init>\0", "(Landroid/media/MediaRouter$RouteCategory;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [addRoute](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#addRoute(android.media.MediaRouter.RouteInfo))
        ///
        /// Required features: "android-media-MediaRouter_RouteInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaRouter_RouteInfo")))]
        pub fn addRoute_RouteInfo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaRouter_RouteInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "addRoute", .descriptor == "(Landroid/media/MediaRouter$RouteInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "addRoute\0", "(Landroid/media/MediaRouter$RouteInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addRoute](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#addRoute(android.media.MediaRouter.RouteInfo,%20int))
        ///
        /// Required features: "android-media-MediaRouter_RouteInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaRouter_RouteInfo")))]
        pub fn addRoute_RouteInfo_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaRouter_RouteInfo>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "addRoute", .descriptor == "(Landroid/media/MediaRouter$RouteInfo;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "addRoute\0", "(Landroid/media/MediaRouter$RouteInfo;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeRoute](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#removeRoute(android.media.MediaRouter.RouteInfo))
        ///
        /// Required features: "android-media-MediaRouter_RouteInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaRouter_RouteInfo")))]
        pub fn removeRoute_RouteInfo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaRouter_RouteInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "removeRoute", .descriptor == "(Landroid/media/MediaRouter$RouteInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "removeRoute\0", "(Landroid/media/MediaRouter$RouteInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeRoute](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#removeRoute(int))
        pub fn removeRoute_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "removeRoute", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "removeRoute\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRouteCount](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#getRouteCount())
        pub fn getRouteCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "getRouteCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "getRouteCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRouteAt](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#getRouteAt(int))
        ///
        /// Required features: "android-media-MediaRouter_RouteInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaRouter_RouteInfo")))]
        pub fn getRouteAt<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaRouter_RouteInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "getRouteAt", .descriptor == "(I)Landroid/media/MediaRouter$RouteInfo;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "getRouteAt\0", "(I)Landroid/media/MediaRouter$RouteInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIconDrawable](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#setIconDrawable(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn setIconDrawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "setIconDrawable", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "setIconDrawable\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIconResource](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#setIconResource(int))
        pub fn setIconResource<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "setIconResource", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "setIconResource\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestSetVolume](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#requestSetVolume(int))
        pub fn requestSetVolume<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "requestSetVolume", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "requestSetVolume\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestUpdateVolume](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#requestUpdateVolume(int))
        pub fn requestUpdateVolume<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "requestUpdateVolume", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "requestUpdateVolume\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/media/MediaRouter.RouteGroup.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaRouter$RouteGroup", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaRouter$RouteGroup\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
