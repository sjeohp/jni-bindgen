// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-nfc-cardemulation-CardEmulation"))]
__jni_bindgen! {
    /// public final class [CardEmulation](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html)
    ///
    /// Required feature: android-nfc-cardemulation-CardEmulation
    public final class CardEmulation ("android/nfc/cardemulation/CardEmulation") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [CardEmulation](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#CardEmulation())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::nfc::cardemulation::CardEmulation>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#getInstance(android.nfc.NfcAdapter))
        ///
        /// Required features: "android-nfc-NfcAdapter", "android-nfc-cardemulation-CardEmulation"
        #[cfg(any(feature = "all", all(feature = "android-nfc-NfcAdapter", feature = "android-nfc-cardemulation-CardEmulation")))]
        pub fn getInstance<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NfcAdapter>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::nfc::cardemulation::CardEmulation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC | STATIC | SYNCRONIZED, .name == "getInstance", .descriptor == "(Landroid/nfc/NfcAdapter;)Landroid/nfc/cardemulation/CardEmulation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/nfc/cardemulation/CardEmulation\0", "getInstance\0", "(Landroid/nfc/NfcAdapter;)Landroid/nfc/cardemulation/CardEmulation;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDefaultServiceForCategory](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#isDefaultServiceForCategory(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String")))]
        pub fn isDefaultServiceForCategory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "isDefaultServiceForCategory", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "isDefaultServiceForCategory\0", "(Landroid/content/ComponentName;Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDefaultServiceForAid](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#isDefaultServiceForAid(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String")))]
        pub fn isDefaultServiceForAid<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "isDefaultServiceForAid", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "isDefaultServiceForAid\0", "(Landroid/content/ComponentName;Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [categoryAllowsForegroundPreference](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#categoryAllowsForegroundPreference(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn categoryAllowsForegroundPreference<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "categoryAllowsForegroundPreference", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "categoryAllowsForegroundPreference\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSelectionModeForCategory](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#getSelectionModeForCategory(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSelectionModeForCategory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "getSelectionModeForCategory", .descriptor == "(Ljava/lang/String;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "getSelectionModeForCategory\0", "(Ljava/lang/String;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerAidsForService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#registerAidsForService(android.content.ComponentName,%20java.lang.String,%20java.util.List))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String", feature = "java-util-List")))]
        pub fn registerAidsForService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "registerAidsForService", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;Ljava/util/List;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "registerAidsForService\0", "(Landroid/content/ComponentName;Ljava/lang/String;Ljava/util/List;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unsetOffHostForService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#unsetOffHostForService(android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn unsetOffHostForService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "unsetOffHostForService", .descriptor == "(Landroid/content/ComponentName;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "unsetOffHostForService\0", "(Landroid/content/ComponentName;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOffHostForService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#setOffHostForService(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String")))]
        pub fn setOffHostForService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "setOffHostForService", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "setOffHostForService\0", "(Landroid/content/ComponentName;Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAidsForService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#getAidsForService(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String", feature = "java-util-List")))]
        pub fn getAidsForService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "getAidsForService", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "getAidsForService\0", "(Landroid/content/ComponentName;Ljava/lang/String;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeAidsForService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#removeAidsForService(android.content.ComponentName,%20java.lang.String))
        ///
        /// Required features: "android-content-ComponentName", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-lang-String")))]
        pub fn removeAidsForService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "removeAidsForService", .descriptor == "(Landroid/content/ComponentName;Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "removeAidsForService\0", "(Landroid/content/ComponentName;Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPreferredService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#setPreferredService(android.app.Activity,%20android.content.ComponentName))
        ///
        /// Required features: "android-app-Activity", "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-content-ComponentName")))]
        pub fn setPreferredService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "setPreferredService", .descriptor == "(Landroid/app/Activity;Landroid/content/ComponentName;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "setPreferredService\0", "(Landroid/app/Activity;Landroid/content/ComponentName;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unsetPreferredService](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#unsetPreferredService(android.app.Activity))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        pub fn unsetPreferredService<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "unsetPreferredService", .descriptor == "(Landroid/app/Activity;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "unsetPreferredService\0", "(Landroid/app/Activity;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [supportsAidPrefixRegistration](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#supportsAidPrefixRegistration())
        pub fn supportsAidPrefixRegistration<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/cardemulation/CardEmulation", java.flags == PUBLIC, .name == "supportsAidPrefixRegistration", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/cardemulation/CardEmulation\0", "supportsAidPrefixRegistration\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_CHANGE_DEFAULT](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#ACTION_CHANGE_DEFAULT)
        pub const ACTION_CHANGE_DEFAULT : &'static str = "android.nfc.cardemulation.action.ACTION_CHANGE_DEFAULT";

        /// public static final [CATEGORY_OTHER](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#CATEGORY_OTHER)
        pub const CATEGORY_OTHER : &'static str = "other";

        /// public static final [CATEGORY_PAYMENT](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#CATEGORY_PAYMENT)
        pub const CATEGORY_PAYMENT : &'static str = "payment";

        /// public static final [EXTRA_CATEGORY](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#EXTRA_CATEGORY)
        pub const EXTRA_CATEGORY : &'static str = "category";

        /// public static final [EXTRA_SERVICE_COMPONENT](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#EXTRA_SERVICE_COMPONENT)
        pub const EXTRA_SERVICE_COMPONENT : &'static str = "component";

        /// public static final [SELECTION_MODE_ALWAYS_ASK](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#SELECTION_MODE_ALWAYS_ASK)
        pub const SELECTION_MODE_ALWAYS_ASK : i32 = 1;

        /// public static final [SELECTION_MODE_ASK_IF_CONFLICT](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#SELECTION_MODE_ASK_IF_CONFLICT)
        pub const SELECTION_MODE_ASK_IF_CONFLICT : i32 = 2;

        /// public static final [SELECTION_MODE_PREFER_DEFAULT](https://developer.android.com/reference/android/nfc/cardemulation/CardEmulation.html#SELECTION_MODE_PREFER_DEFAULT)
        pub const SELECTION_MODE_PREFER_DEFAULT : i32 = 0;
    }
}
