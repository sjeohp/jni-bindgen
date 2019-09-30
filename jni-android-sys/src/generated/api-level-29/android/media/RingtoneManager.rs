// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-RingtoneManager"))]
__jni_bindgen! {
    /// public class [RingtoneManager](https://developer.android.com/reference/android/media/RingtoneManager.html)
    ///
    /// Required feature: android-media-RingtoneManager
    public class RingtoneManager ("android/media/RingtoneManager") extends crate::java::lang::Object {

        /// [RingtoneManager](https://developer.android.com/reference/android/media/RingtoneManager.html#RingtoneManager(android.app.Activity))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        pub fn new_Activity<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::RingtoneManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "<init>\0", "(Landroid/app/Activity;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [RingtoneManager](https://developer.android.com/reference/android/media/RingtoneManager.html#RingtoneManager(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::RingtoneManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setType](https://developer.android.com/reference/android/media/RingtoneManager.html#setType(int))
        pub fn setType<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "setType", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "setType\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inferStreamType](https://developer.android.com/reference/android/media/RingtoneManager.html#inferStreamType())
        pub fn inferStreamType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "inferStreamType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "inferStreamType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setStopPreviousRingtone](https://developer.android.com/reference/android/media/RingtoneManager.html#setStopPreviousRingtone(boolean))
        pub fn setStopPreviousRingtone<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "setStopPreviousRingtone", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "setStopPreviousRingtone\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStopPreviousRingtone](https://developer.android.com/reference/android/media/RingtoneManager.html#getStopPreviousRingtone())
        pub fn getStopPreviousRingtone<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "getStopPreviousRingtone", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "getStopPreviousRingtone\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stopPreviousRingtone](https://developer.android.com/reference/android/media/RingtoneManager.html#stopPreviousRingtone())
        pub fn stopPreviousRingtone<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "stopPreviousRingtone", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "stopPreviousRingtone\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIncludeDrm](https://developer.android.com/reference/android/media/RingtoneManager.html#getIncludeDrm())
        #[deprecated] pub fn getIncludeDrm<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "getIncludeDrm", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "getIncludeDrm\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIncludeDrm](https://developer.android.com/reference/android/media/RingtoneManager.html#setIncludeDrm(boolean))
        #[deprecated] pub fn setIncludeDrm<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "setIncludeDrm", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "setIncludeDrm\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCursor](https://developer.android.com/reference/android/media/RingtoneManager.html#getCursor())
        ///
        /// Required features: "android-database-Cursor"
        #[cfg(any(feature = "all", all(feature = "android-database-Cursor")))]
        pub fn getCursor<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::database::Cursor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "getCursor", .descriptor == "()Landroid/database/Cursor;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "getCursor\0", "()Landroid/database/Cursor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRingtone](https://developer.android.com/reference/android/media/RingtoneManager.html#getRingtone(int))
        ///
        /// Required features: "android-media-Ringtone"
        #[cfg(any(feature = "all", all(feature = "android-media-Ringtone")))]
        pub fn getRingtone_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::Ringtone>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "getRingtone", .descriptor == "(I)Landroid/media/Ringtone;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "getRingtone\0", "(I)Landroid/media/Ringtone;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRingtoneUri](https://developer.android.com/reference/android/media/RingtoneManager.html#getRingtoneUri(int))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn getRingtoneUri<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "getRingtoneUri", .descriptor == "(I)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "getRingtoneUri\0", "(I)Landroid/net/Uri;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRingtonePosition](https://developer.android.com/reference/android/media/RingtoneManager.html#getRingtonePosition(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn getRingtonePosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "getRingtonePosition", .descriptor == "(Landroid/net/Uri;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "getRingtonePosition\0", "(Landroid/net/Uri;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getValidRingtoneUri](https://developer.android.com/reference/android/media/RingtoneManager.html#getValidRingtoneUri(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri")))]
        pub fn getValidRingtoneUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "getValidRingtoneUri", .descriptor == "(Landroid/content/Context;)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "getValidRingtoneUri\0", "(Landroid/content/Context;)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRingtone](https://developer.android.com/reference/android/media/RingtoneManager.html#getRingtone(android.content.Context,%20android.net.Uri))
        ///
        /// Required features: "android-content-Context", "android-media-Ringtone", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-media-Ringtone", feature = "android-net-Uri")))]
        pub fn getRingtone_Context_Uri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::Ringtone>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "getRingtone", .descriptor == "(Landroid/content/Context;Landroid/net/Uri;)Landroid/media/Ringtone;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "getRingtone\0", "(Landroid/content/Context;Landroid/net/Uri;)Landroid/media/Ringtone;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActualDefaultRingtoneUri](https://developer.android.com/reference/android/media/RingtoneManager.html#getActualDefaultRingtoneUri(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri")))]
        pub fn getActualDefaultRingtoneUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "getActualDefaultRingtoneUri", .descriptor == "(Landroid/content/Context;I)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "getActualDefaultRingtoneUri\0", "(Landroid/content/Context;I)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setActualDefaultRingtoneUri](https://developer.android.com/reference/android/media/RingtoneManager.html#setActualDefaultRingtoneUri(android.content.Context,%20int,%20android.net.Uri))
        ///
        /// Required features: "android-content-Context", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri")))]
        pub fn setActualDefaultRingtoneUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "setActualDefaultRingtoneUri", .descriptor == "(Landroid/content/Context;ILandroid/net/Uri;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "setActualDefaultRingtoneUri\0", "(Landroid/content/Context;ILandroid/net/Uri;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDefault](https://developer.android.com/reference/android/media/RingtoneManager.html#isDefault(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn isDefault<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "isDefault", .descriptor == "(Landroid/net/Uri;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "isDefault\0", "(Landroid/net/Uri;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultType](https://developer.android.com/reference/android/media/RingtoneManager.html#getDefaultType(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn getDefaultType<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "getDefaultType", .descriptor == "(Landroid/net/Uri;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "getDefaultType\0", "(Landroid/net/Uri;)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultUri](https://developer.android.com/reference/android/media/RingtoneManager.html#getDefaultUri(int))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn getDefaultUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "getDefaultUri", .descriptor == "(I)Landroid/net/Uri;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "getDefaultUri\0", "(I)Landroid/net/Uri;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [openDefaultRingtoneUri](https://developer.android.com/reference/android/media/RingtoneManager.html#openDefaultRingtoneUri(android.content.Context,%20android.net.Uri))
        ///
        /// Required features: "android-content-Context", "android-content-res-AssetFileDescriptor", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-res-AssetFileDescriptor", feature = "android-net-Uri")))]
        pub fn openDefaultRingtoneUri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::res::AssetFileDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "openDefaultRingtoneUri", .descriptor == "(Landroid/content/Context;Landroid/net/Uri;)Landroid/content/res/AssetFileDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "openDefaultRingtoneUri\0", "(Landroid/content/Context;Landroid/net/Uri;)Landroid/content/res/AssetFileDescriptor;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasHapticChannels](https://developer.android.com/reference/android/media/RingtoneManager.html#hasHapticChannels(int))
        pub fn hasHapticChannels_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC, .name == "hasHapticChannels", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/RingtoneManager\0", "hasHapticChannels\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasHapticChannels](https://developer.android.com/reference/android/media/RingtoneManager.html#hasHapticChannels(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn hasHapticChannels_Uri<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/RingtoneManager", java.flags == PUBLIC | STATIC, .name == "hasHapticChannels", .descriptor == "(Landroid/net/Uri;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/media/RingtoneManager\0", "hasHapticChannels\0", "(Landroid/net/Uri;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_RINGTONE_PICKER](https://developer.android.com/reference/android/media/RingtoneManager.html#ACTION_RINGTONE_PICKER)
        pub const ACTION_RINGTONE_PICKER : &'static str = "android.intent.action.RINGTONE_PICKER";

        /// public static final [EXTRA_RINGTONE_DEFAULT_URI](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_DEFAULT_URI)
        pub const EXTRA_RINGTONE_DEFAULT_URI : &'static str = "android.intent.extra.ringtone.DEFAULT_URI";

        /// public static final [EXTRA_RINGTONE_EXISTING_URI](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_EXISTING_URI)
        pub const EXTRA_RINGTONE_EXISTING_URI : &'static str = "android.intent.extra.ringtone.EXISTING_URI";

        /// public static final [EXTRA_RINGTONE_INCLUDE_DRM](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_INCLUDE_DRM)
        #[deprecated] pub const EXTRA_RINGTONE_INCLUDE_DRM : &'static str = "android.intent.extra.ringtone.INCLUDE_DRM";

        /// public static final [EXTRA_RINGTONE_PICKED_URI](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_PICKED_URI)
        pub const EXTRA_RINGTONE_PICKED_URI : &'static str = "android.intent.extra.ringtone.PICKED_URI";

        /// public static final [EXTRA_RINGTONE_SHOW_DEFAULT](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_SHOW_DEFAULT)
        pub const EXTRA_RINGTONE_SHOW_DEFAULT : &'static str = "android.intent.extra.ringtone.SHOW_DEFAULT";

        /// public static final [EXTRA_RINGTONE_SHOW_SILENT](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_SHOW_SILENT)
        pub const EXTRA_RINGTONE_SHOW_SILENT : &'static str = "android.intent.extra.ringtone.SHOW_SILENT";

        /// public static final [EXTRA_RINGTONE_TITLE](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_TITLE)
        pub const EXTRA_RINGTONE_TITLE : &'static str = "android.intent.extra.ringtone.TITLE";

        /// public static final [EXTRA_RINGTONE_TYPE](https://developer.android.com/reference/android/media/RingtoneManager.html#EXTRA_RINGTONE_TYPE)
        pub const EXTRA_RINGTONE_TYPE : &'static str = "android.intent.extra.ringtone.TYPE";

        /// public static final [ID_COLUMN_INDEX](https://developer.android.com/reference/android/media/RingtoneManager.html#ID_COLUMN_INDEX)
        pub const ID_COLUMN_INDEX : i32 = 0;

        /// public static final [TITLE_COLUMN_INDEX](https://developer.android.com/reference/android/media/RingtoneManager.html#TITLE_COLUMN_INDEX)
        pub const TITLE_COLUMN_INDEX : i32 = 1;

        /// public static final [TYPE_ALARM](https://developer.android.com/reference/android/media/RingtoneManager.html#TYPE_ALARM)
        pub const TYPE_ALARM : i32 = 4;

        /// public static final [TYPE_ALL](https://developer.android.com/reference/android/media/RingtoneManager.html#TYPE_ALL)
        pub const TYPE_ALL : i32 = 7;

        /// public static final [TYPE_NOTIFICATION](https://developer.android.com/reference/android/media/RingtoneManager.html#TYPE_NOTIFICATION)
        pub const TYPE_NOTIFICATION : i32 = 2;

        /// public static final [TYPE_RINGTONE](https://developer.android.com/reference/android/media/RingtoneManager.html#TYPE_RINGTONE)
        pub const TYPE_RINGTONE : i32 = 1;

        /// public static final [URI_COLUMN_INDEX](https://developer.android.com/reference/android/media/RingtoneManager.html#URI_COLUMN_INDEX)
        pub const URI_COLUMN_INDEX : i32 = 2;
    }
}
