// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-Path_Op"))]
__jni_bindgen! {
    /// public enum [Path.Op](https://developer.android.com/reference/android/graphics/Path.Op.html)
    ///
    /// Required feature: android-graphics-Path_Op
    public enum Path_Op ("android/graphics/Path$Op") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/Path.Op.html#values())
        ///
        /// Required features: "android-graphics-Path_Op"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path_Op")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Path_Op, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Path$Op", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/Path$Op;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Path$Op\0", "values\0", "()[Landroid/graphics/Path$Op;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/Path.Op.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-Path_Op", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path_Op", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Path$Op", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/Path$Op;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Path$Op\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/Path$Op;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Op](https://developer.android.com/reference/android/graphics/Path.Op.html#Op(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/Path$Op", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Path$Op\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [DIFFERENCE](https://developer.android.com/reference/android/graphics/Path.Op.html#DIFFERENCE)
        ///
        /// Required feature: android-graphics-Path_Op
        #[cfg(any(feature = "all", feature = "android-graphics-Path_Op"))]
        pub fn DIFFERENCE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$Op\0", "DIFFERENCE\0", "Landroid/graphics/Path$Op;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INTERSECT](https://developer.android.com/reference/android/graphics/Path.Op.html#INTERSECT)
        ///
        /// Required feature: android-graphics-Path_Op
        #[cfg(any(feature = "all", feature = "android-graphics-Path_Op"))]
        pub fn INTERSECT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$Op\0", "INTERSECT\0", "Landroid/graphics/Path$Op;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [UNION](https://developer.android.com/reference/android/graphics/Path.Op.html#UNION)
        ///
        /// Required feature: android-graphics-Path_Op
        #[cfg(any(feature = "all", feature = "android-graphics-Path_Op"))]
        pub fn UNION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$Op\0", "UNION\0", "Landroid/graphics/Path$Op;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [XOR](https://developer.android.com/reference/android/graphics/Path.Op.html#XOR)
        ///
        /// Required feature: android-graphics-Path_Op
        #[cfg(any(feature = "all", feature = "android-graphics-Path_Op"))]
        pub fn XOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$Op\0", "XOR\0", "Landroid/graphics/Path$Op;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REVERSE_DIFFERENCE](https://developer.android.com/reference/android/graphics/Path.Op.html#REVERSE_DIFFERENCE)
        ///
        /// Required feature: android-graphics-Path_Op
        #[cfg(any(feature = "all", feature = "android-graphics-Path_Op"))]
        pub fn REVERSE_DIFFERENCE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path_Op>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Path$Op\0", "REVERSE_DIFFERENCE\0", "Landroid/graphics/Path$Op;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::Path_Op, crate::java::lang::Throwable>>> { ... }
    }
}
