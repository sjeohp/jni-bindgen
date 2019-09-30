// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-job-JobWorkItem"))]
__jni_bindgen! {
    /// public final class [JobWorkItem](https://developer.android.com/reference/android/app/job/JobWorkItem.html)
    ///
    /// Required feature: android-app-job-JobWorkItem
    public final class JobWorkItem ("android/app/job/JobWorkItem") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [JobWorkItem](https://developer.android.com/reference/android/app/job/JobWorkItem.html#JobWorkItem(android.content.Intent))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn new_Intent<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::job::JobWorkItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Intent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "<init>\0", "(Landroid/content/Intent;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [JobWorkItem](https://developer.android.com/reference/android/app/job/JobWorkItem.html#JobWorkItem(android.content.Intent,%20long,%20long))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn new_Intent_long_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>, arg1: i64, arg2: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::job::JobWorkItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Intent;JJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "<init>\0", "(Landroid/content/Intent;JJ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntent](https://developer.android.com/reference/android/app/job/JobWorkItem.html#getIntent())
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn getIntent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Intent>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "getIntent", .descriptor == "()Landroid/content/Intent;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "getIntent\0", "()Landroid/content/Intent;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEstimatedNetworkDownloadBytes](https://developer.android.com/reference/android/app/job/JobWorkItem.html#getEstimatedNetworkDownloadBytes())
        pub fn getEstimatedNetworkDownloadBytes<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "getEstimatedNetworkDownloadBytes", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "getEstimatedNetworkDownloadBytes\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEstimatedNetworkUploadBytes](https://developer.android.com/reference/android/app/job/JobWorkItem.html#getEstimatedNetworkUploadBytes())
        pub fn getEstimatedNetworkUploadBytes<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "getEstimatedNetworkUploadBytes", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "getEstimatedNetworkUploadBytes\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDeliveryCount](https://developer.android.com/reference/android/app/job/JobWorkItem.html#getDeliveryCount())
        pub fn getDeliveryCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "getDeliveryCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "getDeliveryCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/app/job/JobWorkItem.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/job/JobWorkItem.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/job/JobWorkItem.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/job/JobWorkItem", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/job/JobWorkItem\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/job/JobWorkItem.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/job/JobWorkItem\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
