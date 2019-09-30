// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
__jni_bindgen! {
    /// public enum [BlendMode](https://developer.android.com/reference/android/graphics/BlendMode.html)
    ///
    /// Required feature: android-graphics-BlendMode
    public enum BlendMode ("android/graphics/BlendMode") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/BlendMode.html#values())
        ///
        /// Required features: "android-graphics-BlendMode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlendMode")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::BlendMode, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/BlendMode", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/BlendMode;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/BlendMode\0", "values\0", "()[Landroid/graphics/BlendMode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/BlendMode.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-BlendMode", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlendMode", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/BlendMode", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/BlendMode;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/BlendMode\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/BlendMode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [BlendMode](https://developer.android.com/reference/android/graphics/BlendMode.html#BlendMode(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/BlendMode", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/BlendMode\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CLEAR](https://developer.android.com/reference/android/graphics/BlendMode.html#CLEAR)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn CLEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "CLEAR\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC](https://developer.android.com/reference/android/graphics/BlendMode.html#SRC)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SRC<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SRC\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST](https://developer.android.com/reference/android/graphics/BlendMode.html#DST)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DST<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DST\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_OVER](https://developer.android.com/reference/android/graphics/BlendMode.html#SRC_OVER)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SRC_OVER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SRC_OVER\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_OVER](https://developer.android.com/reference/android/graphics/BlendMode.html#DST_OVER)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DST_OVER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DST_OVER\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_IN](https://developer.android.com/reference/android/graphics/BlendMode.html#SRC_IN)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SRC_IN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SRC_IN\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_IN](https://developer.android.com/reference/android/graphics/BlendMode.html#DST_IN)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DST_IN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DST_IN\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_OUT](https://developer.android.com/reference/android/graphics/BlendMode.html#SRC_OUT)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SRC_OUT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SRC_OUT\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_OUT](https://developer.android.com/reference/android/graphics/BlendMode.html#DST_OUT)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DST_OUT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DST_OUT\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SRC_ATOP](https://developer.android.com/reference/android/graphics/BlendMode.html#SRC_ATOP)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SRC_ATOP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SRC_ATOP\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DST_ATOP](https://developer.android.com/reference/android/graphics/BlendMode.html#DST_ATOP)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DST_ATOP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DST_ATOP\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [XOR](https://developer.android.com/reference/android/graphics/BlendMode.html#XOR)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn XOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "XOR\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PLUS](https://developer.android.com/reference/android/graphics/BlendMode.html#PLUS)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn PLUS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "PLUS\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MODULATE](https://developer.android.com/reference/android/graphics/BlendMode.html#MODULATE)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn MODULATE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "MODULATE\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SCREEN](https://developer.android.com/reference/android/graphics/BlendMode.html#SCREEN)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SCREEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SCREEN\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [OVERLAY](https://developer.android.com/reference/android/graphics/BlendMode.html#OVERLAY)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn OVERLAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "OVERLAY\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DARKEN](https://developer.android.com/reference/android/graphics/BlendMode.html#DARKEN)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DARKEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DARKEN\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LIGHTEN](https://developer.android.com/reference/android/graphics/BlendMode.html#LIGHTEN)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn LIGHTEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "LIGHTEN\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [COLOR_DODGE](https://developer.android.com/reference/android/graphics/BlendMode.html#COLOR_DODGE)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn COLOR_DODGE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "COLOR_DODGE\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [COLOR_BURN](https://developer.android.com/reference/android/graphics/BlendMode.html#COLOR_BURN)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn COLOR_BURN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "COLOR_BURN\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HARD_LIGHT](https://developer.android.com/reference/android/graphics/BlendMode.html#HARD_LIGHT)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn HARD_LIGHT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "HARD_LIGHT\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SOFT_LIGHT](https://developer.android.com/reference/android/graphics/BlendMode.html#SOFT_LIGHT)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SOFT_LIGHT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SOFT_LIGHT\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DIFFERENCE](https://developer.android.com/reference/android/graphics/BlendMode.html#DIFFERENCE)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn DIFFERENCE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "DIFFERENCE\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXCLUSION](https://developer.android.com/reference/android/graphics/BlendMode.html#EXCLUSION)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn EXCLUSION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "EXCLUSION\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MULTIPLY](https://developer.android.com/reference/android/graphics/BlendMode.html#MULTIPLY)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn MULTIPLY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "MULTIPLY\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HUE](https://developer.android.com/reference/android/graphics/BlendMode.html#HUE)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn HUE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "HUE\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SATURATION](https://developer.android.com/reference/android/graphics/BlendMode.html#SATURATION)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn SATURATION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "SATURATION\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [COLOR](https://developer.android.com/reference/android/graphics/BlendMode.html#COLOR)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn COLOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "COLOR\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LUMINOSITY](https://developer.android.com/reference/android/graphics/BlendMode.html#LUMINOSITY)
        ///
        /// Required feature: android-graphics-BlendMode
        #[cfg(any(feature = "all", feature = "android-graphics-BlendMode"))]
        pub fn LUMINOSITY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlendMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlendMode\0", "LUMINOSITY\0", "Landroid/graphics/BlendMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::BlendMode, crate::java::lang::Throwable>>> { ... }
    }
}
