// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-ConversationAction"))]
__jni_bindgen! {
    /// public final class [ConversationAction](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html)
    ///
    /// Required feature: android-view-textclassifier-ConversationAction
    public final class ConversationAction ("android/view/textclassifier/ConversationAction") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [ConversationAction](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#ConversationAction(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::textclassifier::ConversationAction>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/textclassifier/ConversationAction", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [writeToParcel](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getType](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#getType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "getType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "getType\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAction](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#getAction())
        ///
        /// Required features: "android-app-RemoteAction"
        #[cfg(any(feature = "all", all(feature = "android-app-RemoteAction")))]
        pub fn getAction<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::RemoteAction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "getAction", .descriptor == "()Landroid/app/RemoteAction;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "getAction\0", "()Landroid/app/RemoteAction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConfidenceScore](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#getConfidenceScore())
        pub fn getConfidenceScore<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "getConfidenceScore", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "getConfidenceScore\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTextReply](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#getTextReply())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getTextReply<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "getTextReply", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "getTextReply\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtras](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#getExtras())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getExtras<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationAction", java.flags == PUBLIC, .name == "getExtras", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationAction\0", "getExtras\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/ConversationAction\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TYPE_CALL_PHONE](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_CALL_PHONE)
        pub const TYPE_CALL_PHONE : &'static str = "call_phone";

        /// public static final [TYPE_CREATE_REMINDER](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_CREATE_REMINDER)
        pub const TYPE_CREATE_REMINDER : &'static str = "create_reminder";

        /// public static final [TYPE_OPEN_URL](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_OPEN_URL)
        pub const TYPE_OPEN_URL : &'static str = "open_url";

        /// public static final [TYPE_SEND_EMAIL](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_SEND_EMAIL)
        pub const TYPE_SEND_EMAIL : &'static str = "send_email";

        /// public static final [TYPE_SEND_SMS](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_SEND_SMS)
        pub const TYPE_SEND_SMS : &'static str = "send_sms";

        /// public static final [TYPE_SHARE_LOCATION](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_SHARE_LOCATION)
        pub const TYPE_SHARE_LOCATION : &'static str = "share_location";

        /// public static final [TYPE_TEXT_REPLY](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_TEXT_REPLY)
        pub const TYPE_TEXT_REPLY : &'static str = "text_reply";

        /// public static final [TYPE_TRACK_FLIGHT](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_TRACK_FLIGHT)
        pub const TYPE_TRACK_FLIGHT : &'static str = "track_flight";

        /// public static final [TYPE_VIEW_CALENDAR](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_VIEW_CALENDAR)
        pub const TYPE_VIEW_CALENDAR : &'static str = "view_calendar";

        /// public static final [TYPE_VIEW_MAP](https://developer.android.com/reference/android/view/textclassifier/ConversationAction.html#TYPE_VIEW_MAP)
        pub const TYPE_VIEW_MAP : &'static str = "view_map";
    }
}
