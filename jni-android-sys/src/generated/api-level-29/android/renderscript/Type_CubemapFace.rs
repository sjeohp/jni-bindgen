// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
__jni_bindgen! {
    /// public enum [Type.CubemapFace](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html)
    ///
    /// Required feature: android-renderscript-Type_CubemapFace
    public enum Type_CubemapFace ("android/renderscript/Type$CubemapFace") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#values())
        ///
        /// Required features: "android-renderscript-Type_CubemapFace"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Type_CubemapFace")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::renderscript::Type_CubemapFace, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Type$CubemapFace", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/renderscript/Type$CubemapFace;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/Type$CubemapFace\0", "values\0", "()[Landroid/renderscript/Type$CubemapFace;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-renderscript-Type_CubemapFace", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Type_CubemapFace", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/Type$CubemapFace", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/renderscript/Type$CubemapFace;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/Type$CubemapFace\0", "valueOf\0", "(Ljava/lang/String;)Landroid/renderscript/Type$CubemapFace;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [CubemapFace](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#CubemapFace(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/renderscript/Type$CubemapFace", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/Type$CubemapFace\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [POSITIVE_X](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#POSITIVE_X)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        pub fn POSITIVE_X<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "POSITIVE_X\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NEGATIVE_X](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#NEGATIVE_X)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        pub fn NEGATIVE_X<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "NEGATIVE_X\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [POSITIVE_Y](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#POSITIVE_Y)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        pub fn POSITIVE_Y<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "POSITIVE_Y\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NEGATIVE_Y](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#NEGATIVE_Y)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        pub fn NEGATIVE_Y<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "NEGATIVE_Y\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [POSITIVE_Z](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#POSITIVE_Z)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        pub fn POSITIVE_Z<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "POSITIVE_Z\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NEGATIVE_Z](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#NEGATIVE_Z)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        pub fn NEGATIVE_Z<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "NEGATIVE_Z\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [POSITVE_X](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#POSITVE_X)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        #[deprecated] pub fn POSITVE_X<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "POSITVE_X\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [POSITVE_Y](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#POSITVE_Y)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        #[deprecated] pub fn POSITVE_Y<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "POSITVE_Y\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [POSITVE_Z](https://developer.android.com/reference/android/renderscript/Type.CubemapFace.html#POSITVE_Z)
        ///
        /// Required feature: android-renderscript-Type_CubemapFace
        #[cfg(any(feature = "all", feature = "android-renderscript-Type_CubemapFace"))]
        #[deprecated] pub fn POSITVE_Z<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Type_CubemapFace>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/renderscript/Type$CubemapFace\0", "POSITVE_Z\0", "Landroid/renderscript/Type$CubemapFace;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::renderscript::Type_CubemapFace, crate::java::lang::Throwable>>> { ... }
    }
}
