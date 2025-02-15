// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-ConversationActions_Message"))]
__jni_bindgen! {
    /// public final class [ConversationActions.Message](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html)
    ///
    /// Required feature: android-view-textclassifier-ConversationActions_Message
    public final class ConversationActions_Message ("android/view/textclassifier/ConversationActions$Message") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [Message](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#Message(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::textclassifier::ConversationActions_Message>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [writeToParcel](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAuthor](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#getAuthor())
        ///
        /// Required features: "android-app-Person"
        #[cfg(any(feature = "all", all(feature = "android-app-Person")))]
        pub fn getAuthor<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == PUBLIC, .name == "getAuthor", .descriptor == "()Landroid/app/Person;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "getAuthor\0", "()Landroid/app/Person;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReferenceTime](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#getReferenceTime())
        ///
        /// Required features: "java-time-ZonedDateTime"
        #[cfg(any(feature = "all", all(feature = "java-time-ZonedDateTime")))]
        pub fn getReferenceTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::ZonedDateTime>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == PUBLIC, .name == "getReferenceTime", .descriptor == "()Ljava/time/ZonedDateTime;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "getReferenceTime\0", "()Ljava/time/ZonedDateTime;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getText](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#getText())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getText<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == PUBLIC, .name == "getText", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "getText\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtras](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#getExtras())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getExtras<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/ConversationActions$Message", java.flags == PUBLIC, .name == "getExtras", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/ConversationActions$Message\0", "getExtras\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/ConversationActions$Message\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PERSON_USER_OTHERS](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#PERSON_USER_OTHERS)
        ///
        /// Required feature: android-app-Person
        #[cfg(any(feature = "all", feature = "android-app-Person"))]
        pub fn PERSON_USER_OTHERS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/ConversationActions$Message\0", "PERSON_USER_OTHERS\0", "Landroid/app/Person;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PERSON_USER_SELF](https://developer.android.com/reference/android/view/textclassifier/ConversationActions.Message.html#PERSON_USER_SELF)
        ///
        /// Required feature: android-app-Person
        #[cfg(any(feature = "all", feature = "android-app-Person"))]
        pub fn PERSON_USER_SELF<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/ConversationActions$Message\0", "PERSON_USER_SELF\0", "Landroid/app/Person;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
