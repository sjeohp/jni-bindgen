// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-KeyCharacterMap"))]
__jni_bindgen! {
    /// public class [KeyCharacterMap](https://developer.android.com/reference/android/view/KeyCharacterMap.html)
    ///
    /// Required feature: android-view-KeyCharacterMap
    public class KeyCharacterMap ("android/view/KeyCharacterMap") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [KeyCharacterMap](https://developer.android.com/reference/android/view/KeyCharacterMap.html#KeyCharacterMap(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::KeyCharacterMap>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/KeyCharacterMap", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/view/KeyCharacterMap.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/KeyCharacterMap", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [load](https://developer.android.com/reference/android/view/KeyCharacterMap.html#load(int))
        ///
        /// Required features: "android-view-KeyCharacterMap"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyCharacterMap")))]
        pub fn load<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::KeyCharacterMap>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC | STATIC, .name == "load", .descriptor == "(I)Landroid/view/KeyCharacterMap;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/KeyCharacterMap\0", "load\0", "(I)Landroid/view/KeyCharacterMap;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/android/view/KeyCharacterMap.html#get(int,%20int))
        pub fn get<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "get", .descriptor == "(II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "get\0", "(II)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumber](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getNumber(int))
        pub fn getNumber<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getNumber", .descriptor == "(I)C"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getNumber\0", "(I)C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMatch](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getMatch(int,%20char%5B%5D))
        pub fn getMatch_int_char_array<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getMatch", .descriptor == "(I[C)C"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getMatch\0", "(I[C)C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMatch](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getMatch(int,%20char%5B%5D,%20int))
        pub fn getMatch_int_char_array_int<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getMatch", .descriptor == "(I[CI)C"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getMatch\0", "(I[CI)C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDisplayLabel](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getDisplayLabel(int))
        pub fn getDisplayLabel<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getDisplayLabel", .descriptor == "(I)C"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getDisplayLabel\0", "(I)C\0");
                __jni_env.call_char_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDeadChar](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getDeadChar(int,%20int))
        pub fn getDeadChar<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC | STATIC, .name == "getDeadChar", .descriptor == "(II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/KeyCharacterMap\0", "getDeadChar\0", "(II)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyData](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getKeyData(int,%20android.view.KeyCharacterMap.KeyData))
        ///
        /// Required features: "android-view-KeyCharacterMap_KeyData"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyCharacterMap_KeyData")))]
        #[deprecated] pub fn getKeyData<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyCharacterMap_KeyData>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getKeyData", .descriptor == "(ILandroid/view/KeyCharacterMap$KeyData;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getKeyData\0", "(ILandroid/view/KeyCharacterMap$KeyData;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEvents](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getEvents(char%5B%5D))
        ///
        /// Required features: "android-view-KeyEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent")))]
        pub fn getEvents<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::KeyEvent, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getEvents", .descriptor == "([C)[Landroid/view/KeyEvent;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getEvents\0", "([C)[Landroid/view/KeyEvent;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isPrintingKey](https://developer.android.com/reference/android/view/KeyCharacterMap.html#isPrintingKey(int))
        pub fn isPrintingKey<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "isPrintingKey", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "isPrintingKey\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyboardType](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getKeyboardType())
        pub fn getKeyboardType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getKeyboardType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getKeyboardType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getModifierBehavior](https://developer.android.com/reference/android/view/KeyCharacterMap.html#getModifierBehavior())
        pub fn getModifierBehavior<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "getModifierBehavior", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "getModifierBehavior\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deviceHasKey](https://developer.android.com/reference/android/view/KeyCharacterMap.html#deviceHasKey(int))
        pub fn deviceHasKey<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC | STATIC, .name == "deviceHasKey", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/KeyCharacterMap\0", "deviceHasKey\0", "(I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deviceHasKeys](https://developer.android.com/reference/android/view/KeyCharacterMap.html#deviceHasKeys(int%5B%5D))
        pub fn deviceHasKeys<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::BooleanArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC | STATIC, .name == "deviceHasKeys", .descriptor == "([I)[Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/KeyCharacterMap\0", "deviceHasKeys\0", "([I)[Z\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/view/KeyCharacterMap.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/view/KeyCharacterMap.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ALPHA](https://developer.android.com/reference/android/view/KeyCharacterMap.html#ALPHA)
        pub const ALPHA : i32 = 3;

        /// public static final [BUILT_IN_KEYBOARD](https://developer.android.com/reference/android/view/KeyCharacterMap.html#BUILT_IN_KEYBOARD)
        #[deprecated] pub const BUILT_IN_KEYBOARD : i32 = 0;

        /// public static final [COMBINING_ACCENT](https://developer.android.com/reference/android/view/KeyCharacterMap.html#COMBINING_ACCENT)
        pub const COMBINING_ACCENT : i32 = -2147483648;

        /// public static final [COMBINING_ACCENT_MASK](https://developer.android.com/reference/android/view/KeyCharacterMap.html#COMBINING_ACCENT_MASK)
        pub const COMBINING_ACCENT_MASK : i32 = 2147483647;

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/view/KeyCharacterMap.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/KeyCharacterMap\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [FULL](https://developer.android.com/reference/android/view/KeyCharacterMap.html#FULL)
        pub const FULL : i32 = 4;

        /// public static final [HEX_INPUT](https://developer.android.com/reference/android/view/KeyCharacterMap.html#HEX_INPUT)
        pub const HEX_INPUT : __jni_bindgen::jchar = __jni_bindgen::jchar(61184);

        /// public static final [MODIFIER_BEHAVIOR_CHORDED](https://developer.android.com/reference/android/view/KeyCharacterMap.html#MODIFIER_BEHAVIOR_CHORDED)
        pub const MODIFIER_BEHAVIOR_CHORDED : i32 = 0;

        /// public static final [MODIFIER_BEHAVIOR_CHORDED_OR_TOGGLED](https://developer.android.com/reference/android/view/KeyCharacterMap.html#MODIFIER_BEHAVIOR_CHORDED_OR_TOGGLED)
        pub const MODIFIER_BEHAVIOR_CHORDED_OR_TOGGLED : i32 = 1;

        /// public static final [NUMERIC](https://developer.android.com/reference/android/view/KeyCharacterMap.html#NUMERIC)
        pub const NUMERIC : i32 = 1;

        /// public static final [PICKER_DIALOG_INPUT](https://developer.android.com/reference/android/view/KeyCharacterMap.html#PICKER_DIALOG_INPUT)
        pub const PICKER_DIALOG_INPUT : __jni_bindgen::jchar = __jni_bindgen::jchar(61185);

        /// public static final [PREDICTIVE](https://developer.android.com/reference/android/view/KeyCharacterMap.html#PREDICTIVE)
        pub const PREDICTIVE : i32 = 2;

        /// public static final [SPECIAL_FUNCTION](https://developer.android.com/reference/android/view/KeyCharacterMap.html#SPECIAL_FUNCTION)
        pub const SPECIAL_FUNCTION : i32 = 5;

        /// public static final [VIRTUAL_KEYBOARD](https://developer.android.com/reference/android/view/KeyCharacterMap.html#VIRTUAL_KEYBOARD)
        pub const VIRTUAL_KEYBOARD : i32 = -1;
    }
}
