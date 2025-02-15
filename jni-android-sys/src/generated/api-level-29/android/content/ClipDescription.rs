// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-ClipDescription"))]
__jni_bindgen! {
    /// public class [ClipDescription](https://developer.android.com/reference/android/content/ClipDescription.html)
    ///
    /// Required feature: android-content-ClipDescription
    public class ClipDescription ("android/content/ClipDescription") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [ClipDescription](https://developer.android.com/reference/android/content/ClipDescription.html#ClipDescription(java.lang.CharSequence,%20java.lang.String%5B%5D))
        ///
        /// Required features: "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn new_CharSequence_String_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::ClipDescription>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/CharSequence;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "<init>\0", "(Ljava/lang/CharSequence;[Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ClipDescription](https://developer.android.com/reference/android/content/ClipDescription.html#ClipDescription(android.content.ClipDescription))
        ///
        /// Required features: "android-content-ClipDescription"
        #[cfg(any(feature = "all", all(feature = "android-content-ClipDescription")))]
        pub fn new_ClipDescription<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ClipDescription>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::ClipDescription>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/ClipDescription;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "<init>\0", "(Landroid/content/ClipDescription;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareMimeTypes](https://developer.android.com/reference/android/content/ClipDescription.html#compareMimeTypes(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn compareMimeTypes<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC | STATIC, .name == "compareMimeTypes", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/content/ClipDescription\0", "compareMimeTypes\0", "(Ljava/lang/String;Ljava/lang/String;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimestamp](https://developer.android.com/reference/android/content/ClipDescription.html#getTimestamp())
        pub fn getTimestamp<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "getTimestamp", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "getTimestamp\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLabel](https://developer.android.com/reference/android/content/ClipDescription.html#getLabel())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getLabel<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "getLabel", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "getLabel\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasMimeType](https://developer.android.com/reference/android/content/ClipDescription.html#hasMimeType(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn hasMimeType<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "hasMimeType", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "hasMimeType\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [filterMimeTypes](https://developer.android.com/reference/android/content/ClipDescription.html#filterMimeTypes(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn filterMimeTypes<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "filterMimeTypes", .descriptor == "(Ljava/lang/String;)[Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "filterMimeTypes\0", "(Ljava/lang/String;)[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMimeTypeCount](https://developer.android.com/reference/android/content/ClipDescription.html#getMimeTypeCount())
        pub fn getMimeTypeCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "getMimeTypeCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "getMimeTypeCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMimeType](https://developer.android.com/reference/android/content/ClipDescription.html#getMimeType(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMimeType<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "getMimeType", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "getMimeType\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtras](https://developer.android.com/reference/android/content/ClipDescription.html#getExtras())
        ///
        /// Required features: "android-os-PersistableBundle"
        #[cfg(any(feature = "all", all(feature = "android-os-PersistableBundle")))]
        pub fn getExtras<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::PersistableBundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "getExtras", .descriptor == "()Landroid/os/PersistableBundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "getExtras\0", "()Landroid/os/PersistableBundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setExtras](https://developer.android.com/reference/android/content/ClipDescription.html#setExtras(android.os.PersistableBundle))
        ///
        /// Required features: "android-os-PersistableBundle"
        #[cfg(any(feature = "all", all(feature = "android-os-PersistableBundle")))]
        pub fn setExtras<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::PersistableBundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "setExtras", .descriptor == "(Landroid/os/PersistableBundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "setExtras\0", "(Landroid/os/PersistableBundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/content/ClipDescription.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/content/ClipDescription.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/content/ClipDescription.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/ClipDescription", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/ClipDescription\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/content/ClipDescription.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/content/ClipDescription\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [MIMETYPE_TEXT_HTML](https://developer.android.com/reference/android/content/ClipDescription.html#MIMETYPE_TEXT_HTML)
        pub const MIMETYPE_TEXT_HTML : &'static str = "text/html";

        /// public static final [MIMETYPE_TEXT_INTENT](https://developer.android.com/reference/android/content/ClipDescription.html#MIMETYPE_TEXT_INTENT)
        pub const MIMETYPE_TEXT_INTENT : &'static str = "text/vnd.android.intent";

        /// public static final [MIMETYPE_TEXT_PLAIN](https://developer.android.com/reference/android/content/ClipDescription.html#MIMETYPE_TEXT_PLAIN)
        pub const MIMETYPE_TEXT_PLAIN : &'static str = "text/plain";

        /// public static final [MIMETYPE_TEXT_URILIST](https://developer.android.com/reference/android/content/ClipDescription.html#MIMETYPE_TEXT_URILIST)
        pub const MIMETYPE_TEXT_URILIST : &'static str = "text/uri-list";
    }
}
