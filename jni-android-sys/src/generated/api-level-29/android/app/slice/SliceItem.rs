// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-slice-SliceItem"))]
__jni_bindgen! {
    /// public final class [SliceItem](https://developer.android.com/reference/android/app/slice/SliceItem.html)
    ///
    /// Required feature: android-app-slice-SliceItem
    public final class SliceItem ("android/app/slice/SliceItem") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [SliceItem](https://developer.android.com/reference/android/app/slice/SliceItem.html#SliceItem(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::slice::SliceItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/slice/SliceItem", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getHints](https://developer.android.com/reference/android/app/slice/SliceItem.html#getHints())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getHints<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getHints", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getHints\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFormat](https://developer.android.com/reference/android/app/slice/SliceItem.html#getFormat())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getFormat<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getFormat", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getFormat\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubType](https://developer.android.com/reference/android/app/slice/SliceItem.html#getSubType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSubType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getSubType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getSubType\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getText](https://developer.android.com/reference/android/app/slice/SliceItem.html#getText())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getText<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getText", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getText\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBundle](https://developer.android.com/reference/android/app/slice/SliceItem.html#getBundle())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getBundle<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getBundle", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getBundle\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIcon](https://developer.android.com/reference/android/app/slice/SliceItem.html#getIcon())
        ///
        /// Required features: "android-graphics-drawable-Icon"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Icon")))]
        pub fn getIcon<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Icon>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getIcon", .descriptor == "()Landroid/graphics/drawable/Icon;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getIcon\0", "()Landroid/graphics/drawable/Icon;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAction](https://developer.android.com/reference/android/app/slice/SliceItem.html#getAction())
        ///
        /// Required features: "android-app-PendingIntent"
        #[cfg(any(feature = "all", all(feature = "android-app-PendingIntent")))]
        pub fn getAction<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::PendingIntent>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getAction", .descriptor == "()Landroid/app/PendingIntent;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getAction\0", "()Landroid/app/PendingIntent;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRemoteInput](https://developer.android.com/reference/android/app/slice/SliceItem.html#getRemoteInput())
        ///
        /// Required features: "android-app-RemoteInput"
        #[cfg(any(feature = "all", all(feature = "android-app-RemoteInput")))]
        pub fn getRemoteInput<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::RemoteInput>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getRemoteInput", .descriptor == "()Landroid/app/RemoteInput;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getRemoteInput\0", "()Landroid/app/RemoteInput;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInt](https://developer.android.com/reference/android/app/slice/SliceItem.html#getInt())
        pub fn getInt<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getInt", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getInt\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSlice](https://developer.android.com/reference/android/app/slice/SliceItem.html#getSlice())
        ///
        /// Required features: "android-app-slice-Slice"
        #[cfg(any(feature = "all", all(feature = "android-app-slice-Slice")))]
        pub fn getSlice<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::slice::Slice>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getSlice", .descriptor == "()Landroid/app/slice/Slice;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getSlice\0", "()Landroid/app/slice/Slice;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLong](https://developer.android.com/reference/android/app/slice/SliceItem.html#getLong())
        pub fn getLong<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "getLong", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "getLong\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasHint](https://developer.android.com/reference/android/app/slice/SliceItem.html#hasHint(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn hasHint<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "hasHint", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "hasHint\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/app/slice/SliceItem.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/app/slice/SliceItem.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/slice/SliceItem", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/slice/SliceItem\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/app/slice/SliceItem.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/app/slice/SliceItem\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [FORMAT_ACTION](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_ACTION)
        pub const FORMAT_ACTION : &'static str = "action";

        /// public static final [FORMAT_BUNDLE](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_BUNDLE)
        pub const FORMAT_BUNDLE : &'static str = "bundle";

        /// public static final [FORMAT_IMAGE](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_IMAGE)
        pub const FORMAT_IMAGE : &'static str = "image";

        /// public static final [FORMAT_INT](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_INT)
        pub const FORMAT_INT : &'static str = "int";

        /// public static final [FORMAT_LONG](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_LONG)
        pub const FORMAT_LONG : &'static str = "long";

        /// public static final [FORMAT_REMOTE_INPUT](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_REMOTE_INPUT)
        pub const FORMAT_REMOTE_INPUT : &'static str = "input";

        /// public static final [FORMAT_SLICE](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_SLICE)
        pub const FORMAT_SLICE : &'static str = "slice";

        /// public static final [FORMAT_TEXT](https://developer.android.com/reference/android/app/slice/SliceItem.html#FORMAT_TEXT)
        pub const FORMAT_TEXT : &'static str = "text";
    }
}
