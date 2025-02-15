// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-RecoverableSecurityException"))]
__jni_bindgen! {
    /// public final class [RecoverableSecurityException](https://developer.android.com/reference/android/app/RecoverableSecurityException.html)
    ///
    /// Required feature: android-app-RecoverableSecurityException
    public final class RecoverableSecurityException ("android/app/RecoverableSecurityException") extends crate::java::lang::SecurityException, implements crate::android::os::Parcelable {

        /// [RecoverableSecurityException](https://developer.android.com/reference/android/app/RecoverableSecurityException.html#RecoverableSecurityException(java.lang.Throwable,%20java.lang.CharSequence,%20android.app.RemoteAction))
        ///
        /// Required features: "android-app-RemoteAction", "java-lang-CharSequence", "java-lang-Throwable"
        #[cfg(any(feature = "all", all(feature = "android-app-RemoteAction", feature = "java-lang-CharSequence", feature = "java-lang-Throwable")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::RemoteAction>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::RecoverableSecurityException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/RecoverableSecurityException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Throwable;Ljava/lang/CharSequence;Landroid/app/RemoteAction;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/RecoverableSecurityException\0", "<init>\0", "(Ljava/lang/Throwable;Ljava/lang/CharSequence;Landroid/app/RemoteAction;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUserMessage](https://developer.android.com/reference/android/app/RecoverableSecurityException.html#getUserMessage())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getUserMessage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/RecoverableSecurityException", java.flags == PUBLIC, .name == "getUserMessage", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/RecoverableSecurityException\0", "getUserMessage\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUserAction](https://developer.android.com/reference/android/app/RecoverableSecurityException.html#getUserAction())
        ///
        /// Required features: "android-app-RemoteAction"
        #[cfg(any(feature = "all", all(feature = "android-app-RemoteAction")))]
        pub fn getUserAction<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::RemoteAction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/RecoverableSecurityException", java.flags == PUBLIC, .name == "getUserAction", .descriptor == "()Landroid/app/RemoteAction;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/RecoverableSecurityException\0", "getUserAction\0", "()Landroid/app/RemoteAction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/RecoverableSecurityException.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/RecoverableSecurityException", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/RecoverableSecurityException\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/RecoverableSecurityException.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/RecoverableSecurityException", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/RecoverableSecurityException\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/RecoverableSecurityException.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/RecoverableSecurityException\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
