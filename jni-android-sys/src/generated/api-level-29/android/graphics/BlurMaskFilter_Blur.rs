// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-BlurMaskFilter_Blur"))]
__jni_bindgen! {
    /// public enum [BlurMaskFilter.Blur](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html)
    ///
    /// Required feature: android-graphics-BlurMaskFilter_Blur
    public enum BlurMaskFilter_Blur ("android/graphics/BlurMaskFilter$Blur") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#values())
        ///
        /// Required features: "android-graphics-BlurMaskFilter_Blur"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlurMaskFilter_Blur")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::BlurMaskFilter_Blur, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/BlurMaskFilter$Blur", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/BlurMaskFilter$Blur;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/BlurMaskFilter$Blur\0", "values\0", "()[Landroid/graphics/BlurMaskFilter$Blur;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-BlurMaskFilter_Blur", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlurMaskFilter_Blur", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlurMaskFilter_Blur>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/BlurMaskFilter$Blur", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/BlurMaskFilter$Blur;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/BlurMaskFilter$Blur\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/BlurMaskFilter$Blur;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Blur](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#Blur(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::BlurMaskFilter_Blur>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/BlurMaskFilter$Blur", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/BlurMaskFilter$Blur\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [NORMAL](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#NORMAL)
        ///
        /// Required feature: android-graphics-BlurMaskFilter_Blur
        #[cfg(any(feature = "all", feature = "android-graphics-BlurMaskFilter_Blur"))]
        pub fn NORMAL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlurMaskFilter_Blur>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlurMaskFilter$Blur\0", "NORMAL\0", "Landroid/graphics/BlurMaskFilter$Blur;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SOLID](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#SOLID)
        ///
        /// Required feature: android-graphics-BlurMaskFilter_Blur
        #[cfg(any(feature = "all", feature = "android-graphics-BlurMaskFilter_Blur"))]
        pub fn SOLID<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlurMaskFilter_Blur>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlurMaskFilter$Blur\0", "SOLID\0", "Landroid/graphics/BlurMaskFilter$Blur;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [OUTER](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#OUTER)
        ///
        /// Required feature: android-graphics-BlurMaskFilter_Blur
        #[cfg(any(feature = "all", feature = "android-graphics-BlurMaskFilter_Blur"))]
        pub fn OUTER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlurMaskFilter_Blur>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlurMaskFilter$Blur\0", "OUTER\0", "Landroid/graphics/BlurMaskFilter$Blur;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INNER](https://developer.android.com/reference/android/graphics/BlurMaskFilter.Blur.html#INNER)
        ///
        /// Required feature: android-graphics-BlurMaskFilter_Blur
        #[cfg(any(feature = "all", feature = "android-graphics-BlurMaskFilter_Blur"))]
        pub fn INNER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::BlurMaskFilter_Blur>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/BlurMaskFilter$Blur\0", "INNER\0", "Landroid/graphics/BlurMaskFilter$Blur;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::BlurMaskFilter_Blur, crate::java::lang::Throwable>>> { ... }
    }
}
