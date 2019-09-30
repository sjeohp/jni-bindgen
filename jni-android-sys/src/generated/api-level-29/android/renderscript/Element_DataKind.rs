// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
__jni_bindgen! {
    /// public enum [Element.DataKind](https://developer.android.com/reference/android/renderscript/Element.DataKind.html)
    ///
    /// Required feature: android-renderscript-Element_DataKind
    public enum Element_DataKind ("android/renderscript/Element$DataKind") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#values())
        ///
        /// Required features: "android-renderscript-Element_DataKind"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Element_DataKind")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::renderscript::Element_DataKind, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Element$DataKind", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/renderscript/Element$DataKind;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/Element$DataKind\0", "values\0", "()[Landroid/renderscript/Element$DataKind;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-renderscript-Element_DataKind", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Element_DataKind", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Element$DataKind", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/renderscript/Element$DataKind;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/Element$DataKind\0", "valueOf\0", "(Ljava/lang/String;)Landroid/renderscript/Element$DataKind;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [DataKind](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#DataKind(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/renderscript/Element$DataKind", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Element$DataKind\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [USER](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#USER)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn USER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "USER\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_L](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_L)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_L<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_L\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_A](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_A)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_A<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_A\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_LA](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_LA)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_LA<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_LA\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_RGB](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_RGB)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_RGB<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_RGB\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_RGBA](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_RGBA)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_RGBA<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_RGBA\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_DEPTH](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_DEPTH)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_DEPTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_DEPTH\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PIXEL_YUV](https://developer.android.com/reference/android/renderscript/Element.DataKind.html#PIXEL_YUV)
        ///
        /// Required feature: android-renderscript-Element_DataKind
        #[cfg(any(feature = "all", feature = "android-renderscript-Element_DataKind"))]
        pub fn PIXEL_YUV<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Element_DataKind>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Element$DataKind\0", "PIXEL_YUV\0", "Landroid/renderscript/Element$DataKind;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::renderscript::Element_DataKind, crate::java::lang::Throwable>>> { ... }
    }
}
