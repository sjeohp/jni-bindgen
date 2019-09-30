// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-Path_FillType"))]
__jni_bindgen! {
    /// public enum [Path.FillType](https://developer.android.com/reference/android/graphics/Path.FillType.html)
    ///
    /// Required feature: android-graphics-Path_FillType
    public enum Path_FillType ("android/graphics/Path$FillType") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/Path.FillType.html#values())
        ///
        /// Required features: "android-graphics-Path_FillType"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path_FillType")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Path_FillType, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Path$FillType", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/Path$FillType;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Path$FillType\0", "values\0", "()[Landroid/graphics/Path$FillType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/Path.FillType.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-Path_FillType", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path_FillType", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_FillType>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Path$FillType", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/Path$FillType;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Path$FillType\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/Path$FillType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [FillType](https://developer.android.com/reference/android/graphics/Path.FillType.html#FillType(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::Path_FillType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/Path$FillType", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Path$FillType\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [WINDING](https://developer.android.com/reference/android/graphics/Path.FillType.html#WINDING)
        ///
        /// Required feature: android-graphics-Path_FillType
        #[cfg(any(feature = "all", feature = "android-graphics-Path_FillType"))]
        pub fn WINDING<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_FillType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$FillType\0", "WINDING\0", "Landroid/graphics/Path$FillType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EVEN_ODD](https://developer.android.com/reference/android/graphics/Path.FillType.html#EVEN_ODD)
        ///
        /// Required feature: android-graphics-Path_FillType
        #[cfg(any(feature = "all", feature = "android-graphics-Path_FillType"))]
        pub fn EVEN_ODD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_FillType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$FillType\0", "EVEN_ODD\0", "Landroid/graphics/Path$FillType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INVERSE_WINDING](https://developer.android.com/reference/android/graphics/Path.FillType.html#INVERSE_WINDING)
        ///
        /// Required feature: android-graphics-Path_FillType
        #[cfg(any(feature = "all", feature = "android-graphics-Path_FillType"))]
        pub fn INVERSE_WINDING<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_FillType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$FillType\0", "INVERSE_WINDING\0", "Landroid/graphics/Path$FillType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INVERSE_EVEN_ODD](https://developer.android.com/reference/android/graphics/Path.FillType.html#INVERSE_EVEN_ODD)
        ///
        /// Required feature: android-graphics-Path_FillType
        #[cfg(any(feature = "all", feature = "android-graphics-Path_FillType"))]
        pub fn INVERSE_EVEN_ODD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_FillType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$FillType\0", "INVERSE_EVEN_ODD\0", "Landroid/graphics/Path$FillType;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Path_FillType, crate::java::lang::Throwable>>> { ... }
    }
}
