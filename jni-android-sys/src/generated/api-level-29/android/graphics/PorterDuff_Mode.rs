// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
__jni_bindgen! {
    /// public enum [PorterDuff.Mode](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html)
    ///
    /// Required feature: android-graphics-PorterDuff_Mode
    public enum PorterDuff_Mode ("android/graphics/PorterDuff$Mode") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#values())
        ///
        /// Required features: "android-graphics-PorterDuff_Mode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-PorterDuff_Mode")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::PorterDuff_Mode, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/PorterDuff$Mode", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/PorterDuff$Mode;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/PorterDuff$Mode\0", "values\0", "()[Landroid/graphics/PorterDuff$Mode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-PorterDuff_Mode", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-PorterDuff_Mode", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/PorterDuff$Mode", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/PorterDuff$Mode;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/PorterDuff$Mode\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/PorterDuff$Mode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Mode](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#Mode(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/PorterDuff$Mode", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/PorterDuff$Mode\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CLEAR](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#CLEAR)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn CLEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "CLEAR\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#SRC)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn SRC<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "SRC\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#DST)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn DST<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "DST\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_OVER](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#SRC_OVER)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn SRC_OVER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "SRC_OVER\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_OVER](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#DST_OVER)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn DST_OVER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "DST_OVER\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_IN](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#SRC_IN)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn SRC_IN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "SRC_IN\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_IN](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#DST_IN)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn DST_IN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "DST_IN\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_OUT](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#SRC_OUT)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn SRC_OUT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "SRC_OUT\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_OUT](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#DST_OUT)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn DST_OUT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "DST_OUT\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_ATOP](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#SRC_ATOP)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn SRC_ATOP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "SRC_ATOP\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_ATOP](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#DST_ATOP)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn DST_ATOP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "DST_ATOP\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [XOR](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#XOR)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn XOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "XOR\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DARKEN](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#DARKEN)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn DARKEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "DARKEN\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LIGHTEN](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#LIGHTEN)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn LIGHTEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "LIGHTEN\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MULTIPLY](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#MULTIPLY)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn MULTIPLY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "MULTIPLY\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SCREEN](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#SCREEN)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn SCREEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "SCREEN\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ADD](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#ADD)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn ADD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "ADD\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [OVERLAY](https://developer.android.com/reference/android/graphics/PorterDuff.Mode.html#OVERLAY)
        ///
        /// Required feature: android-graphics-PorterDuff_Mode
        #[cfg(any(feature = "all", feature = "android-graphics-PorterDuff_Mode"))]
        pub fn OVERLAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::PorterDuff_Mode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/PorterDuff$Mode\0", "OVERLAY\0", "Landroid/graphics/PorterDuff$Mode;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::PorterDuff_Mode, crate::java::lang::Throwable>>> { ... }
    }
}
