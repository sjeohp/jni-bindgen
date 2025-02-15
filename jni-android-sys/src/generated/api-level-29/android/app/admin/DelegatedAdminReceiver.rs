// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-admin-DelegatedAdminReceiver"))]
__jni_bindgen! {
    /// public class [DelegatedAdminReceiver](https://developer.android.com/reference/android/app/admin/DelegatedAdminReceiver.html)
    ///
    /// Required feature: android-app-admin-DelegatedAdminReceiver
    public class DelegatedAdminReceiver ("android/app/admin/DelegatedAdminReceiver") extends crate::android::content::BroadcastReceiver {

        /// [DelegatedAdminReceiver](https://developer.android.com/reference/android/app/admin/DelegatedAdminReceiver.html#DelegatedAdminReceiver())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::admin::DelegatedAdminReceiver>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DelegatedAdminReceiver", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DelegatedAdminReceiver\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onChoosePrivateKeyAlias](https://developer.android.com/reference/android/app/admin/DelegatedAdminReceiver.html#onChoosePrivateKeyAlias(android.content.Context,%20android.content.Intent,%20int,%20android.net.Uri,%20java.lang.String))
        ///
        /// Required features: "android-content-Context", "android-content-Intent", "android-net-Uri", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-Intent", feature = "android-net-Uri", feature = "java-lang-String")))]
        pub fn onChoosePrivateKeyAlias<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DelegatedAdminReceiver", java.flags == PUBLIC, .name == "onChoosePrivateKeyAlias", .descriptor == "(Landroid/content/Context;Landroid/content/Intent;ILandroid/net/Uri;Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DelegatedAdminReceiver\0", "onChoosePrivateKeyAlias\0", "(Landroid/content/Context;Landroid/content/Intent;ILandroid/net/Uri;Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onNetworkLogsAvailable](https://developer.android.com/reference/android/app/admin/DelegatedAdminReceiver.html#onNetworkLogsAvailable(android.content.Context,%20android.content.Intent,%20long,%20int))
        ///
        /// Required features: "android-content-Context", "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-Intent")))]
        pub fn onNetworkLogsAvailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>, arg2: i64, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DelegatedAdminReceiver", java.flags == PUBLIC, .name == "onNetworkLogsAvailable", .descriptor == "(Landroid/content/Context;Landroid/content/Intent;JI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DelegatedAdminReceiver\0", "onNetworkLogsAvailable\0", "(Landroid/content/Context;Landroid/content/Intent;JI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onReceive](https://developer.android.com/reference/android/app/admin/DelegatedAdminReceiver.html#onReceive(android.content.Context,%20android.content.Intent))
        ///
        /// Required features: "android-content-Context", "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-Intent")))]
        pub fn onReceive<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/admin/DelegatedAdminReceiver", java.flags == PUBLIC | FINAL, .name == "onReceive", .descriptor == "(Landroid/content/Context;Landroid/content/Intent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/admin/DelegatedAdminReceiver\0", "onReceive\0", "(Landroid/content/Context;Landroid/content/Intent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
