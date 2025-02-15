// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-Interpolator_Result"))]
__jni_bindgen! {
    /// public enum [Interpolator.Result](https://developer.android.com/reference/android/graphics/Interpolator.Result.html)
    ///
    /// Required feature: android-graphics-Interpolator_Result
    public enum Interpolator_Result ("android/graphics/Interpolator$Result") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/Interpolator.Result.html#values())
        ///
        /// Required features: "android-graphics-Interpolator_Result"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Interpolator_Result")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Interpolator_Result, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Interpolator$Result", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/Interpolator$Result;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Interpolator$Result\0", "values\0", "()[Landroid/graphics/Interpolator$Result;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/Interpolator.Result.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-Interpolator_Result", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Interpolator_Result", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Interpolator_Result>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Interpolator$Result", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/Interpolator$Result;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Interpolator$Result\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/Interpolator$Result;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Result](https://developer.android.com/reference/android/graphics/Interpolator.Result.html#Result(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::Interpolator_Result>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/Interpolator$Result", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Interpolator$Result\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [NORMAL](https://developer.android.com/reference/android/graphics/Interpolator.Result.html#NORMAL)
        ///
        /// Required feature: android-graphics-Interpolator_Result
        #[cfg(any(feature = "all", feature = "android-graphics-Interpolator_Result"))]
        pub fn NORMAL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Interpolator_Result>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Interpolator$Result\0", "NORMAL\0", "Landroid/graphics/Interpolator$Result;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FREEZE_START](https://developer.android.com/reference/android/graphics/Interpolator.Result.html#FREEZE_START)
        ///
        /// Required feature: android-graphics-Interpolator_Result
        #[cfg(any(feature = "all", feature = "android-graphics-Interpolator_Result"))]
        pub fn FREEZE_START<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Interpolator_Result>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Interpolator$Result\0", "FREEZE_START\0", "Landroid/graphics/Interpolator$Result;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FREEZE_END](https://developer.android.com/reference/android/graphics/Interpolator.Result.html#FREEZE_END)
        ///
        /// Required feature: android-graphics-Interpolator_Result
        #[cfg(any(feature = "all", feature = "android-graphics-Interpolator_Result"))]
        pub fn FREEZE_END<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Interpolator_Result>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Interpolator$Result\0", "FREEZE_END\0", "Landroid/graphics/Interpolator$Result;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Interpolator_Result, crate::java::lang::Throwable>>> { ... }
    }
}
