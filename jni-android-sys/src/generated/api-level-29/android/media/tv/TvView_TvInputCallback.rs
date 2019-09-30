// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-tv-TvView_TvInputCallback"))]
__jni_bindgen! {
    /// public class [TvView.TvInputCallback](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html)
    ///
    /// Required feature: android-media-tv-TvView_TvInputCallback
    public class TvView_TvInputCallback ("android/media/tv/TvView$TvInputCallback") extends crate::java::lang::Object {

        /// [TvInputCallback](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#TvInputCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::tv::TvView_TvInputCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnectionFailed](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onConnectionFailed(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onConnectionFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onConnectionFailed", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onConnectionFailed\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisconnected](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onDisconnected(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onDisconnected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onDisconnected", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onDisconnected\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onChannelRetuned](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onChannelRetuned(java.lang.String,%20android.net.Uri))
        ///
        /// Required features: "android-net-Uri", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "java-lang-String")))]
        pub fn onChannelRetuned<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onChannelRetuned", .descriptor == "(Ljava/lang/String;Landroid/net/Uri;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onChannelRetuned\0", "(Ljava/lang/String;Landroid/net/Uri;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTracksChanged](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onTracksChanged(java.lang.String,%20java.util.List))
        ///
        /// Required features: "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-List")))]
        pub fn onTracksChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onTracksChanged", .descriptor == "(Ljava/lang/String;Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onTracksChanged\0", "(Ljava/lang/String;Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTrackSelected](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onTrackSelected(java.lang.String,%20int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onTrackSelected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onTrackSelected", .descriptor == "(Ljava/lang/String;ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onTrackSelected\0", "(Ljava/lang/String;ILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoSizeChanged](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onVideoSizeChanged(java.lang.String,%20int,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onVideoSizeChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onVideoSizeChanged", .descriptor == "(Ljava/lang/String;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onVideoSizeChanged\0", "(Ljava/lang/String;II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoAvailable](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onVideoAvailable(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onVideoAvailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onVideoAvailable", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onVideoAvailable\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoUnavailable](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onVideoUnavailable(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onVideoUnavailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onVideoUnavailable", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onVideoUnavailable\0", "(Ljava/lang/String;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onContentAllowed](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onContentAllowed(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onContentAllowed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onContentAllowed", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onContentAllowed\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onContentBlocked](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onContentBlocked(java.lang.String,%20android.media.tv.TvContentRating))
        ///
        /// Required features: "android-media-tv-TvContentRating", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-tv-TvContentRating", feature = "java-lang-String")))]
        pub fn onContentBlocked<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::tv::TvContentRating>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onContentBlocked", .descriptor == "(Ljava/lang/String;Landroid/media/tv/TvContentRating;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onContentBlocked\0", "(Ljava/lang/String;Landroid/media/tv/TvContentRating;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTimeShiftStatusChanged](https://developer.android.com/reference/android/media/tv/TvView.TvInputCallback.html#onTimeShiftStatusChanged(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onTimeShiftStatusChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvView$TvInputCallback", java.flags == PUBLIC, .name == "onTimeShiftStatusChanged", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvView$TvInputCallback\0", "onTimeShiftStatusChanged\0", "(Ljava/lang/String;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
