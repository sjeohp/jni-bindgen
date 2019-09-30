// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-Intent_ShortcutIconResource"))]
__jni_bindgen! {
    /// public class [Intent.ShortcutIconResource](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html)
    ///
    /// Required feature: android-content-Intent_ShortcutIconResource
    public class Intent_ShortcutIconResource ("android/content/Intent$ShortcutIconResource") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [ShortcutIconResource](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#ShortcutIconResource())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::Intent_ShortcutIconResource>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$ShortcutIconResource", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$ShortcutIconResource\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fromContext](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#fromContext(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-content-Intent_ShortcutIconResource"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-Intent_ShortcutIconResource")))]
        pub fn fromContext<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Intent_ShortcutIconResource>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$ShortcutIconResource", java.flags == PUBLIC | STATIC, .name == "fromContext", .descriptor == "(Landroid/content/Context;I)Landroid/content/Intent$ShortcutIconResource;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/content/Intent$ShortcutIconResource\0", "fromContext\0", "(Landroid/content/Context;I)Landroid/content/Intent$ShortcutIconResource;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$ShortcutIconResource", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$ShortcutIconResource\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$ShortcutIconResource", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$ShortcutIconResource\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Intent$ShortcutIconResource", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Intent$ShortcutIconResource\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/content/Intent$ShortcutIconResource\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public [packageName](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#packageName)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn packageName<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/Intent$ShortcutIconResource\0", "packageName\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [packageName](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#packageName)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_packageName<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/Intent$ShortcutIconResource\0", "packageName\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [resourceName](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#resourceName)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn resourceName<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/Intent$ShortcutIconResource\0", "resourceName\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [resourceName](https://developer.android.com/reference/android/content/Intent.ShortcutIconResource.html#resourceName)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn set_resourceName<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/content/Intent$ShortcutIconResource\0", "resourceName\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }
    }
}
