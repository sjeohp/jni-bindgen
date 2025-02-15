// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-Notification_InboxStyle"))]
__jni_bindgen! {
    /// public class [Notification.InboxStyle](https://developer.android.com/reference/android/app/Notification.InboxStyle.html)
    ///
    /// Required feature: android-app-Notification_InboxStyle
    public class Notification_InboxStyle ("android/app/Notification$InboxStyle") extends crate::android::app::Notification_Style {

        /// [InboxStyle](https://developer.android.com/reference/android/app/Notification.InboxStyle.html#InboxStyle())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::Notification_InboxStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$InboxStyle", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$InboxStyle\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InboxStyle](https://developer.android.com/reference/android/app/Notification.InboxStyle.html#InboxStyle(android.app.Notification.Builder))
        ///
        /// Required features: "android-app-Notification_Builder"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_Builder")))]
        #[deprecated] pub fn new_Builder<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Notification_Builder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::Notification_InboxStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$InboxStyle", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/app/Notification$Builder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$InboxStyle\0", "<init>\0", "(Landroid/app/Notification$Builder;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBigContentTitle](https://developer.android.com/reference/android/app/Notification.InboxStyle.html#setBigContentTitle(java.lang.CharSequence))
        ///
        /// Required features: "android-app-Notification_InboxStyle", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_InboxStyle", feature = "java-lang-CharSequence")))]
        pub fn setBigContentTitle<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Notification_InboxStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$InboxStyle", java.flags == PUBLIC, .name == "setBigContentTitle", .descriptor == "(Ljava/lang/CharSequence;)Landroid/app/Notification$InboxStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$InboxStyle\0", "setBigContentTitle\0", "(Ljava/lang/CharSequence;)Landroid/app/Notification$InboxStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSummaryText](https://developer.android.com/reference/android/app/Notification.InboxStyle.html#setSummaryText(java.lang.CharSequence))
        ///
        /// Required features: "android-app-Notification_InboxStyle", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_InboxStyle", feature = "java-lang-CharSequence")))]
        pub fn setSummaryText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Notification_InboxStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$InboxStyle", java.flags == PUBLIC, .name == "setSummaryText", .descriptor == "(Ljava/lang/CharSequence;)Landroid/app/Notification$InboxStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$InboxStyle\0", "setSummaryText\0", "(Ljava/lang/CharSequence;)Landroid/app/Notification$InboxStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addLine](https://developer.android.com/reference/android/app/Notification.InboxStyle.html#addLine(java.lang.CharSequence))
        ///
        /// Required features: "android-app-Notification_InboxStyle", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_InboxStyle", feature = "java-lang-CharSequence")))]
        pub fn addLine<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Notification_InboxStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$InboxStyle", java.flags == PUBLIC, .name == "addLine", .descriptor == "(Ljava/lang/CharSequence;)Landroid/app/Notification$InboxStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$InboxStyle\0", "addLine\0", "(Ljava/lang/CharSequence;)Landroid/app/Notification$InboxStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
