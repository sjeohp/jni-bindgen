// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-Session2CommandGroup_Builder"))]
__jni_bindgen! {
    /// public final class [Session2CommandGroup.Builder](https://developer.android.com/reference/android/media/Session2CommandGroup.Builder.html)
    ///
    /// Required feature: android-media-Session2CommandGroup_Builder
    public final class Session2CommandGroup_Builder ("android/media/Session2CommandGroup$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/media/Session2CommandGroup.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::Session2CommandGroup_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/Session2CommandGroup$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/Session2CommandGroup$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Builder](https://developer.android.com/reference/android/media/Session2CommandGroup.Builder.html#Builder(android.media.Session2CommandGroup))
        ///
        /// Required features: "android-media-Session2CommandGroup"
        #[cfg(any(feature = "all", all(feature = "android-media-Session2CommandGroup")))]
        pub fn new_Session2CommandGroup<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2CommandGroup>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::Session2CommandGroup_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/Session2CommandGroup$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/media/Session2CommandGroup;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/Session2CommandGroup$Builder\0", "<init>\0", "(Landroid/media/Session2CommandGroup;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addCommand](https://developer.android.com/reference/android/media/Session2CommandGroup.Builder.html#addCommand(android.media.Session2Command))
        ///
        /// Required features: "android-media-Session2Command", "android-media-Session2CommandGroup_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-Session2Command", feature = "android-media-Session2CommandGroup_Builder")))]
        pub fn addCommand<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Command>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::Session2CommandGroup_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/Session2CommandGroup$Builder", java.flags == PUBLIC, .name == "addCommand", .descriptor == "(Landroid/media/Session2Command;)Landroid/media/Session2CommandGroup$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/Session2CommandGroup$Builder\0", "addCommand\0", "(Landroid/media/Session2Command;)Landroid/media/Session2CommandGroup$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeCommand](https://developer.android.com/reference/android/media/Session2CommandGroup.Builder.html#removeCommand(android.media.Session2Command))
        ///
        /// Required features: "android-media-Session2Command", "android-media-Session2CommandGroup_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-Session2Command", feature = "android-media-Session2CommandGroup_Builder")))]
        pub fn removeCommand<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Command>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::Session2CommandGroup_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/Session2CommandGroup$Builder", java.flags == PUBLIC, .name == "removeCommand", .descriptor == "(Landroid/media/Session2Command;)Landroid/media/Session2CommandGroup$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/Session2CommandGroup$Builder\0", "removeCommand\0", "(Landroid/media/Session2Command;)Landroid/media/Session2CommandGroup$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/media/Session2CommandGroup.Builder.html#build())
        ///
        /// Required features: "android-media-Session2CommandGroup"
        #[cfg(any(feature = "all", all(feature = "android-media-Session2CommandGroup")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::Session2CommandGroup>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/Session2CommandGroup$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/media/Session2CommandGroup;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/Session2CommandGroup$Builder\0", "build\0", "()Landroid/media/Session2CommandGroup;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
