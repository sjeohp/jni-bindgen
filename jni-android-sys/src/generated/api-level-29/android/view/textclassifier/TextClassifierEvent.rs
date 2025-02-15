// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassifierEvent"))]
__jni_bindgen! {
    /// public class [TextClassifierEvent](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html)
    ///
    /// Required feature: android-view-textclassifier-TextClassifierEvent
    public class TextClassifierEvent ("android/view/textclassifier/TextClassifierEvent") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [TextClassifierEvent](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TextClassifierEvent(android.view.textclassifier.TextClassifierEvent.Builder))
        // ///
        // /// Required features: "android-view-textclassifier-TextClassifierEvent_Builder"
        // #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifierEvent_Builder")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextClassifierEvent_Builder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifierEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/view/textclassifier/TextClassifierEvent$Builder;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "<init>\0", "(Landroid/view/textclassifier/TextClassifierEvent$Builder;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [describeContents](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEventCategory](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getEventCategory())
        pub fn getEventCategory<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getEventCategory", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getEventCategory\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEventType](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getEventType())
        pub fn getEventType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getEventType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getEventType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEntityTypes](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getEntityTypes())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEntityTypes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getEntityTypes", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getEntityTypes\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEventContext](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getEventContext())
        ///
        /// Required features: "android-view-textclassifier-TextClassificationContext"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassificationContext")))]
        pub fn getEventContext<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassificationContext>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getEventContext", .descriptor == "()Landroid/view/textclassifier/TextClassificationContext;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getEventContext\0", "()Landroid/view/textclassifier/TextClassificationContext;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getResultId](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getResultId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getResultId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getResultId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getResultId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEventIndex](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getEventIndex())
        pub fn getEventIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getEventIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getEventIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getScores](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getScores())
        pub fn getScores<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::FloatArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getScores", .descriptor == "()[F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getScores\0", "()[F\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getModelName](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getModelName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getModelName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getModelName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getModelName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActionIndices](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getActionIndices())
        pub fn getActionIndices<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getActionIndices", .descriptor == "()[I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getActionIndices\0", "()[I\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocale](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getLocale())
        ///
        /// Required features: "android-icu-util-ULocale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale")))]
        pub fn getLocale<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::ULocale>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getLocale", .descriptor == "()Landroid/icu/util/ULocale;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getLocale\0", "()Landroid/icu/util/ULocale;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtras](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#getExtras())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getExtras<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "getExtras", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "getExtras\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CATEGORY_CONVERSATION_ACTIONS](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#CATEGORY_CONVERSATION_ACTIONS)
        pub const CATEGORY_CONVERSATION_ACTIONS : i32 = 3;

        /// public static final [CATEGORY_LANGUAGE_DETECTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#CATEGORY_LANGUAGE_DETECTION)
        pub const CATEGORY_LANGUAGE_DETECTION : i32 = 4;

        /// public static final [CATEGORY_LINKIFY](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#CATEGORY_LINKIFY)
        pub const CATEGORY_LINKIFY : i32 = 2;

        /// public static final [CATEGORY_SELECTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#CATEGORY_SELECTION)
        pub const CATEGORY_SELECTION : i32 = 1;

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/TextClassifierEvent\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TYPE_ACTIONS_GENERATED](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_ACTIONS_GENERATED)
        pub const TYPE_ACTIONS_GENERATED : i32 = 20;

        /// public static final [TYPE_ACTIONS_SHOWN](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_ACTIONS_SHOWN)
        pub const TYPE_ACTIONS_SHOWN : i32 = 6;

        /// public static final [TYPE_AUTO_SELECTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_AUTO_SELECTION)
        pub const TYPE_AUTO_SELECTION : i32 = 5;

        /// public static final [TYPE_COPY_ACTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_COPY_ACTION)
        pub const TYPE_COPY_ACTION : i32 = 9;

        /// public static final [TYPE_CUT_ACTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_CUT_ACTION)
        pub const TYPE_CUT_ACTION : i32 = 11;

        /// public static final [TYPE_LINK_CLICKED](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_LINK_CLICKED)
        pub const TYPE_LINK_CLICKED : i32 = 7;

        /// public static final [TYPE_MANUAL_REPLY](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_MANUAL_REPLY)
        pub const TYPE_MANUAL_REPLY : i32 = 19;

        /// public static final [TYPE_OTHER_ACTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_OTHER_ACTION)
        pub const TYPE_OTHER_ACTION : i32 = 16;

        /// public static final [TYPE_OVERTYPE](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_OVERTYPE)
        pub const TYPE_OVERTYPE : i32 = 8;

        /// public static final [TYPE_PASTE_ACTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_PASTE_ACTION)
        pub const TYPE_PASTE_ACTION : i32 = 10;

        /// public static final [TYPE_SELECTION_DESTROYED](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SELECTION_DESTROYED)
        pub const TYPE_SELECTION_DESTROYED : i32 = 15;

        /// public static final [TYPE_SELECTION_DRAG](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SELECTION_DRAG)
        pub const TYPE_SELECTION_DRAG : i32 = 14;

        /// public static final [TYPE_SELECTION_MODIFIED](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SELECTION_MODIFIED)
        pub const TYPE_SELECTION_MODIFIED : i32 = 2;

        /// public static final [TYPE_SELECTION_RESET](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SELECTION_RESET)
        pub const TYPE_SELECTION_RESET : i32 = 18;

        /// public static final [TYPE_SELECTION_STARTED](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SELECTION_STARTED)
        pub const TYPE_SELECTION_STARTED : i32 = 1;

        /// public static final [TYPE_SELECT_ALL](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SELECT_ALL)
        pub const TYPE_SELECT_ALL : i32 = 17;

        /// public static final [TYPE_SHARE_ACTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SHARE_ACTION)
        pub const TYPE_SHARE_ACTION : i32 = 12;

        /// public static final [TYPE_SMART_ACTION](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SMART_ACTION)
        pub const TYPE_SMART_ACTION : i32 = 13;

        /// public static final [TYPE_SMART_SELECTION_MULTI](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SMART_SELECTION_MULTI)
        pub const TYPE_SMART_SELECTION_MULTI : i32 = 4;

        /// public static final [TYPE_SMART_SELECTION_SINGLE](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.html#TYPE_SMART_SELECTION_SINGLE)
        pub const TYPE_SMART_SELECTION_SINGLE : i32 = 3;
    }
}
