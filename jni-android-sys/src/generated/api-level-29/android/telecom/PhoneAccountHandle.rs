// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-PhoneAccountHandle"))]
__jni_bindgen! {
    /// public final class [PhoneAccountHandle](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html)
    ///
    /// Required feature: android-telecom-PhoneAccountHandle
    public final class PhoneAccountHandle ("android/telecom/PhoneAccountHandle") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [PhoneAccountHandle](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#PhoneAccountHandle(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String")))]
        pub fn new_ComponentName_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::PhoneAccountHandle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "<init>\0", "(Landroid/content/ComponentName;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PhoneAccountHandle](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#PhoneAccountHandle(android.content.ComponentName,%20java.lang.String,%20android.os.UserHandle))
        ///
        /// Required features: "android-content-ComponentName", "android-os-UserHandle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-os-UserHandle", feature = "java-lang-String")))]
        pub fn new_ComponentName_String_UserHandle<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::UserHandle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::PhoneAccountHandle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;Landroid/os/UserHandle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "<init>\0", "(Landroid/content/ComponentName;Ljava/lang/String;Landroid/os/UserHandle;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getComponentName](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#getComponentName())
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn getComponentName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "getComponentName", .descriptor == "()Landroid/content/ComponentName;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "getComponentName\0", "()Landroid/content/ComponentName;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getId](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#getId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "getId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "getId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUserHandle](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#getUserHandle())
        ///
        /// Required features: "android-os-UserHandle"
        #[cfg(any(feature = "all", all(feature = "android-os-UserHandle")))]
        pub fn getUserHandle<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::UserHandle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "getUserHandle", .descriptor == "()Landroid/os/UserHandle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "getUserHandle\0", "()Landroid/os/UserHandle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/PhoneAccountHandle", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/PhoneAccountHandle\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/telecom/PhoneAccountHandle.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telecom/PhoneAccountHandle\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
