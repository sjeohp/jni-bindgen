// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-Shader_TileMode"))]
__jni_bindgen! {
    /// public enum [Shader.TileMode](https://developer.android.com/reference/android/graphics/Shader.TileMode.html)
    ///
    /// Required feature: android-graphics-Shader_TileMode
    public enum Shader_TileMode ("android/graphics/Shader$TileMode") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/Shader.TileMode.html#values())
        ///
        /// Required features: "android-graphics-Shader_TileMode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Shader_TileMode")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Shader_TileMode, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Shader$TileMode", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/Shader$TileMode;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Shader$TileMode\0", "values\0", "()[Landroid/graphics/Shader$TileMode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/Shader.TileMode.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-Shader_TileMode", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Shader_TileMode", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Shader_TileMode>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Shader$TileMode", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/Shader$TileMode;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Shader$TileMode\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/Shader$TileMode;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [TileMode](https://developer.android.com/reference/android/graphics/Shader.TileMode.html#TileMode(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::Shader_TileMode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/Shader$TileMode", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Shader$TileMode\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CLAMP](https://developer.android.com/reference/android/graphics/Shader.TileMode.html#CLAMP)
        ///
        /// Required feature: android-graphics-Shader_TileMode
        #[cfg(any(feature = "all", feature = "android-graphics-Shader_TileMode"))]
        pub fn CLAMP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Shader_TileMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Shader$TileMode\0", "CLAMP\0", "Landroid/graphics/Shader$TileMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REPEAT](https://developer.android.com/reference/android/graphics/Shader.TileMode.html#REPEAT)
        ///
        /// Required feature: android-graphics-Shader_TileMode
        #[cfg(any(feature = "all", feature = "android-graphics-Shader_TileMode"))]
        pub fn REPEAT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Shader_TileMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Shader$TileMode\0", "REPEAT\0", "Landroid/graphics/Shader$TileMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MIRROR](https://developer.android.com/reference/android/graphics/Shader.TileMode.html#MIRROR)
        ///
        /// Required feature: android-graphics-Shader_TileMode
        #[cfg(any(feature = "all", feature = "android-graphics-Shader_TileMode"))]
        pub fn MIRROR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Shader_TileMode>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Shader$TileMode\0", "MIRROR\0", "Landroid/graphics/Shader$TileMode;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Shader_TileMode, crate::java::lang::Throwable>>> { ... }
    }
}
