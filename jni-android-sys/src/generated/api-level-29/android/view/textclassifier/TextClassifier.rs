// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassifier"))]
__jni_bindgen! {
    /// public interface [TextClassifier](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html)
    ///
    /// Required feature: android-view-textclassifier-TextClassifier
    public interface TextClassifier ("android/view/textclassifier/TextClassifier") extends crate::java::lang::Object {

        /// [suggestSelection](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#suggestSelection(android.view.textclassifier.TextSelection.Request))
        ///
        /// Required features: "android-view-textclassifier-TextSelection", "android-view-textclassifier-TextSelection_Request"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextSelection", feature = "android-view-textclassifier-TextSelection_Request")))]
        pub fn suggestSelection_Request<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextSelection_Request>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextSelection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "suggestSelection", .descriptor == "(Landroid/view/textclassifier/TextSelection$Request;)Landroid/view/textclassifier/TextSelection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "suggestSelection\0", "(Landroid/view/textclassifier/TextSelection$Request;)Landroid/view/textclassifier/TextSelection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [suggestSelection](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#suggestSelection(java.lang.CharSequence,%20int,%20int,%20android.os.LocaleList))
        ///
        /// Required features: "android-os-LocaleList", "android-view-textclassifier-TextSelection", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-os-LocaleList", feature = "android-view-textclassifier-TextSelection", feature = "java-lang-CharSequence")))]
        pub fn suggestSelection_CharSequence_int_int_LocaleList<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::LocaleList>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextSelection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "suggestSelection", .descriptor == "(Ljava/lang/CharSequence;IILandroid/os/LocaleList;)Landroid/view/textclassifier/TextSelection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "suggestSelection\0", "(Ljava/lang/CharSequence;IILandroid/os/LocaleList;)Landroid/view/textclassifier/TextSelection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [classifyText](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#classifyText(android.view.textclassifier.TextClassification.Request))
        ///
        /// Required features: "android-view-textclassifier-TextClassification", "android-view-textclassifier-TextClassification_Request"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassification", feature = "android-view-textclassifier-TextClassification_Request")))]
        pub fn classifyText_Request<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextClassification_Request>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassification>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "classifyText", .descriptor == "(Landroid/view/textclassifier/TextClassification$Request;)Landroid/view/textclassifier/TextClassification;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "classifyText\0", "(Landroid/view/textclassifier/TextClassification$Request;)Landroid/view/textclassifier/TextClassification;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [classifyText](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#classifyText(java.lang.CharSequence,%20int,%20int,%20android.os.LocaleList))
        ///
        /// Required features: "android-os-LocaleList", "android-view-textclassifier-TextClassification", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-os-LocaleList", feature = "android-view-textclassifier-TextClassification", feature = "java-lang-CharSequence")))]
        pub fn classifyText_CharSequence_int_int_LocaleList<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::LocaleList>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassification>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "classifyText", .descriptor == "(Ljava/lang/CharSequence;IILandroid/os/LocaleList;)Landroid/view/textclassifier/TextClassification;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "classifyText\0", "(Ljava/lang/CharSequence;IILandroid/os/LocaleList;)Landroid/view/textclassifier/TextClassification;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [generateLinks](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#generateLinks(android.view.textclassifier.TextLinks.Request))
        ///
        /// Required features: "android-view-textclassifier-TextLinks", "android-view-textclassifier-TextLinks_Request"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextLinks", feature = "android-view-textclassifier-TextLinks_Request")))]
        pub fn generateLinks<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextLinks_Request>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextLinks>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "generateLinks", .descriptor == "(Landroid/view/textclassifier/TextLinks$Request;)Landroid/view/textclassifier/TextLinks;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "generateLinks\0", "(Landroid/view/textclassifier/TextLinks$Request;)Landroid/view/textclassifier/TextLinks;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxGenerateLinksTextLength](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#getMaxGenerateLinksTextLength())
        pub fn getMaxGenerateLinksTextLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "getMaxGenerateLinksTextLength", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "getMaxGenerateLinksTextLength\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [detectLanguage](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#detectLanguage(android.view.textclassifier.TextLanguage.Request))
        ///
        /// Required features: "android-view-textclassifier-TextLanguage", "android-view-textclassifier-TextLanguage_Request"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextLanguage", feature = "android-view-textclassifier-TextLanguage_Request")))]
        pub fn detectLanguage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextLanguage_Request>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextLanguage>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "detectLanguage", .descriptor == "(Landroid/view/textclassifier/TextLanguage$Request;)Landroid/view/textclassifier/TextLanguage;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "detectLanguage\0", "(Landroid/view/textclassifier/TextLanguage$Request;)Landroid/view/textclassifier/TextLanguage;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [suggestConversationActions](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#suggestConversationActions(android.view.textclassifier.ConversationActions.Request))
        ///
        /// Required features: "android-view-textclassifier-ConversationActions", "android-view-textclassifier-ConversationActions_Request"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-ConversationActions", feature = "android-view-textclassifier-ConversationActions_Request")))]
        pub fn suggestConversationActions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::ConversationActions_Request>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::ConversationActions>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "suggestConversationActions", .descriptor == "(Landroid/view/textclassifier/ConversationActions$Request;)Landroid/view/textclassifier/ConversationActions;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "suggestConversationActions\0", "(Landroid/view/textclassifier/ConversationActions$Request;)Landroid/view/textclassifier/ConversationActions;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSelectionEvent](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#onSelectionEvent(android.view.textclassifier.SelectionEvent))
        ///
        /// Required features: "android-view-textclassifier-SelectionEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-SelectionEvent")))]
        pub fn onSelectionEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::SelectionEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "onSelectionEvent", .descriptor == "(Landroid/view/textclassifier/SelectionEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "onSelectionEvent\0", "(Landroid/view/textclassifier/SelectionEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTextClassifierEvent](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#onTextClassifierEvent(android.view.textclassifier.TextClassifierEvent))
        ///
        /// Required features: "android-view-textclassifier-TextClassifierEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-textclassifier-TextClassifierEvent")))]
        pub fn onTextClassifierEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::textclassifier::TextClassifierEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "onTextClassifierEvent", .descriptor == "(Landroid/view/textclassifier/TextClassifierEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "onTextClassifierEvent\0", "(Landroid/view/textclassifier/TextClassifierEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [destroy](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#destroy())
        pub fn destroy<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "destroy", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "destroy\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDestroyed](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#isDestroyed())
        pub fn isDestroyed<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/textclassifier/TextClassifier", java.flags == PUBLIC, .name == "isDestroyed", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/textclassifier/TextClassifier\0", "isDestroyed\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [EXTRA_FROM_TEXT_CLASSIFIER](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#EXTRA_FROM_TEXT_CLASSIFIER)
        pub const EXTRA_FROM_TEXT_CLASSIFIER : &'static str = "android.view.textclassifier.extra.FROM_TEXT_CLASSIFIER";

        /// public static final [HINT_TEXT_IS_EDITABLE](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#HINT_TEXT_IS_EDITABLE)
        pub const HINT_TEXT_IS_EDITABLE : &'static str = "android.text_is_editable";

        /// public static final [HINT_TEXT_IS_NOT_EDITABLE](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#HINT_TEXT_IS_NOT_EDITABLE)
        pub const HINT_TEXT_IS_NOT_EDITABLE : &'static str = "android.text_is_not_editable";

        /// **get** public static final [NO_OP](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#NO_OP)
        ///
        /// Required feature: android-view-textclassifier-TextClassifier
        #[cfg(any(feature = "all", feature = "android-view-textclassifier-TextClassifier"))]
        pub fn NO_OP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::textclassifier::TextClassifier>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/textclassifier/TextClassifier\0", "NO_OP\0", "Landroid/view/textclassifier/TextClassifier;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TYPE_ADDRESS](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_ADDRESS)
        pub const TYPE_ADDRESS : &'static str = "address";

        /// public static final [TYPE_DATE](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_DATE)
        pub const TYPE_DATE : &'static str = "date";

        /// public static final [TYPE_DATE_TIME](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_DATE_TIME)
        pub const TYPE_DATE_TIME : &'static str = "datetime";

        /// public static final [TYPE_EMAIL](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_EMAIL)
        pub const TYPE_EMAIL : &'static str = "email";

        /// public static final [TYPE_FLIGHT_NUMBER](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_FLIGHT_NUMBER)
        pub const TYPE_FLIGHT_NUMBER : &'static str = "flight";

        /// public static final [TYPE_OTHER](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_OTHER)
        pub const TYPE_OTHER : &'static str = "other";

        /// public static final [TYPE_PHONE](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_PHONE)
        pub const TYPE_PHONE : &'static str = "phone";

        /// public static final [TYPE_UNKNOWN](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_UNKNOWN)
        pub const TYPE_UNKNOWN : &'static str = "";

        /// public static final [TYPE_URL](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#TYPE_URL)
        pub const TYPE_URL : &'static str = "url";

        /// public static final [WIDGET_TYPE_CUSTOM_EDITTEXT](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_CUSTOM_EDITTEXT)
        pub const WIDGET_TYPE_CUSTOM_EDITTEXT : &'static str = "customedit";

        /// public static final [WIDGET_TYPE_CUSTOM_TEXTVIEW](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_CUSTOM_TEXTVIEW)
        pub const WIDGET_TYPE_CUSTOM_TEXTVIEW : &'static str = "customview";

        /// public static final [WIDGET_TYPE_CUSTOM_UNSELECTABLE_TEXTVIEW](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_CUSTOM_UNSELECTABLE_TEXTVIEW)
        pub const WIDGET_TYPE_CUSTOM_UNSELECTABLE_TEXTVIEW : &'static str = "nosel-customview";

        /// public static final [WIDGET_TYPE_EDITTEXT](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_EDITTEXT)
        pub const WIDGET_TYPE_EDITTEXT : &'static str = "edittext";

        /// public static final [WIDGET_TYPE_EDIT_WEBVIEW](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_EDIT_WEBVIEW)
        pub const WIDGET_TYPE_EDIT_WEBVIEW : &'static str = "edit-webview";

        /// public static final [WIDGET_TYPE_NOTIFICATION](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_NOTIFICATION)
        pub const WIDGET_TYPE_NOTIFICATION : &'static str = "notification";

        /// public static final [WIDGET_TYPE_TEXTVIEW](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_TEXTVIEW)
        pub const WIDGET_TYPE_TEXTVIEW : &'static str = "textview";

        /// public static final [WIDGET_TYPE_UNKNOWN](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_UNKNOWN)
        pub const WIDGET_TYPE_UNKNOWN : &'static str = "unknown";

        /// public static final [WIDGET_TYPE_UNSELECTABLE_TEXTVIEW](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_UNSELECTABLE_TEXTVIEW)
        pub const WIDGET_TYPE_UNSELECTABLE_TEXTVIEW : &'static str = "nosel-textview";

        /// public static final [WIDGET_TYPE_WEBVIEW](https://developer.android.com/reference/android/view/textclassifier/TextClassifier.html#WIDGET_TYPE_WEBVIEW)
        pub const WIDGET_TYPE_WEBVIEW : &'static str = "webview";
    }
}
