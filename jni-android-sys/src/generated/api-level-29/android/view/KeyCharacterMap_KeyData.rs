// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-KeyCharacterMap_KeyData"))]
__jni_bindgen! {
    /// public class [KeyCharacterMap.KeyData](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html)
    ///
    /// Required feature: android-view-KeyCharacterMap_KeyData
    #[deprecated] public class KeyCharacterMap_KeyData ("android/view/KeyCharacterMap$KeyData") extends crate::java::lang::Object {

        /// [KeyData](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#KeyData())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::KeyCharacterMap_KeyData>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyCharacterMap$KeyData", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyCharacterMap$KeyData\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [META_LENGTH](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#META_LENGTH)
        #[deprecated] pub const META_LENGTH : i32 = 4;

        /// **get** public [displayLabel](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#displayLabel)
        #[deprecated] pub fn displayLabel<'env>(&'env self) -> __jni_bindgen::jchar {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/KeyCharacterMap$KeyData\0", "displayLabel\0", "C\0");
                env.get_char_field(class, field)
            }
        }

        /// **set** public [displayLabel](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#displayLabel)
        #[deprecated] pub fn set_displayLabel<'env>(&'env self, value: __jni_bindgen::jchar) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/KeyCharacterMap$KeyData\0", "displayLabel\0", "C\0");
                env.set_char_field(class, field, value)
            }
        }

        /// **get** public [meta](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#meta)
        #[deprecated] pub fn meta<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::CharArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/KeyCharacterMap$KeyData\0", "meta\0", "[C\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [meta](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#meta)
        #[deprecated] pub fn set_meta<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::CharArray>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/KeyCharacterMap$KeyData\0", "meta\0", "[C\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [number](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#number)
        #[deprecated] pub fn number<'env>(&'env self) -> __jni_bindgen::jchar {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/KeyCharacterMap$KeyData\0", "number\0", "C\0");
                env.get_char_field(class, field)
            }
        }

        /// **set** public [number](https://developer.android.com/reference/android/view/KeyCharacterMap.KeyData.html#number)
        #[deprecated] pub fn set_number<'env>(&'env self, value: __jni_bindgen::jchar) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/view/KeyCharacterMap$KeyData\0", "number\0", "C\0");
                env.set_char_field(class, field, value)
            }
        }
    }
}
