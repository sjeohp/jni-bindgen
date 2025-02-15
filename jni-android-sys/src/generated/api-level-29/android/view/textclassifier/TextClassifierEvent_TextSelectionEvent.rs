// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassifierEvent_TextSelectionEvent"))]
__jni_bindgen! {
    /// public final class [TextClassifierEvent.TextSelectionEvent](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html)
    ///
    /// Required feature: android-view-textclassifier-TextClassifierEvent_TextSelectionEvent
    public final class TextClassifierEvent_TextSelectionEvent ("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent") extends crate::android::view::textclassifier::TextClassifierEvent, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [TextSelectionEvent](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#TextSelectionEvent(android.view.textclassifier.TextClassifierEvent.TextSelectionEvent.Builder))
        // ///
        // /// Required features: "android-view-textclassifier-TextClassifierEvent_TextSelectionEvent_Builder"
        // #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifierEvent_TextSelectionEvent_Builder")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextClassifierEvent_TextSelectionEvent_Builder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifierEvent_TextSelectionEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/textclassifier/TextClassifierEvent$TextSelectionEvent", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/view/textclassifier/TextClassifierEvent$TextSelectionEvent$Builder;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "<init>\0", "(Landroid/view/textclassifier/TextClassifierEvent$TextSelectionEvent$Builder;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [writeToParcel](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent$TextSelectionEvent", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRelativeWordStartIndex](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#getRelativeWordStartIndex())
        pub fn getRelativeWordStartIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent$TextSelectionEvent", java.flags == PUBLIC, .name == "getRelativeWordStartIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "getRelativeWordStartIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRelativeWordEndIndex](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#getRelativeWordEndIndex())
        pub fn getRelativeWordEndIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent$TextSelectionEvent", java.flags == PUBLIC, .name == "getRelativeWordEndIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "getRelativeWordEndIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRelativeSuggestedWordStartIndex](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#getRelativeSuggestedWordStartIndex())
        pub fn getRelativeSuggestedWordStartIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent$TextSelectionEvent", java.flags == PUBLIC, .name == "getRelativeSuggestedWordStartIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "getRelativeSuggestedWordStartIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRelativeSuggestedWordEndIndex](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#getRelativeSuggestedWordEndIndex())
        pub fn getRelativeSuggestedWordEndIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifierEvent$TextSelectionEvent", java.flags == PUBLIC, .name == "getRelativeSuggestedWordEndIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "getRelativeSuggestedWordEndIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/view/textclassifier/TextClassifierEvent.TextSelectionEvent.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/TextClassifierEvent$TextSelectionEvent\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
