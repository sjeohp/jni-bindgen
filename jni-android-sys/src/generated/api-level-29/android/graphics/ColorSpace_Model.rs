// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-ColorSpace_Model"))]
__jni_bindgen! {
    /// public enum [ColorSpace.Model](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html)
    ///
    /// Required feature: android-graphics-ColorSpace_Model
    public enum ColorSpace_Model ("android/graphics/ColorSpace$Model") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#values())
        ///
        /// Required features: "android-graphics-ColorSpace_Model"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorSpace_Model")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::ColorSpace_Model, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ColorSpace$Model", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/graphics/ColorSpace$Model;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/ColorSpace$Model\0", "values\0", "()[Landroid/graphics/ColorSpace$Model;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-graphics-ColorSpace_Model", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorSpace_Model", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorSpace_Model>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ColorSpace$Model", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/graphics/ColorSpace$Model;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/ColorSpace$Model\0", "valueOf\0", "(Ljava/lang/String;)Landroid/graphics/ColorSpace$Model;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Model](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#Model(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::ColorSpace_Model>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/ColorSpace$Model", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ColorSpace$Model\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getComponentCount](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#getComponentCount())
        pub fn getComponentCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/ColorSpace$Model", java.flags == PUBLIC, .name == "getComponentCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/ColorSpace$Model\0", "getComponentCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [RGB](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#RGB)
        ///
        /// Required feature: android-graphics-ColorSpace_Model
        #[cfg(any(feature = "all", feature = "android-graphics-ColorSpace_Model"))]
        pub fn RGB<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorSpace_Model>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/ColorSpace$Model\0", "RGB\0", "Landroid/graphics/ColorSpace$Model;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [XYZ](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#XYZ)
        ///
        /// Required feature: android-graphics-ColorSpace_Model
        #[cfg(any(feature = "all", feature = "android-graphics-ColorSpace_Model"))]
        pub fn XYZ<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorSpace_Model>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/ColorSpace$Model\0", "XYZ\0", "Landroid/graphics/ColorSpace$Model;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LAB](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#LAB)
        ///
        /// Required feature: android-graphics-ColorSpace_Model
        #[cfg(any(feature = "all", feature = "android-graphics-ColorSpace_Model"))]
        pub fn LAB<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorSpace_Model>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/ColorSpace$Model\0", "LAB\0", "Landroid/graphics/ColorSpace$Model;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CMYK](https://developer.android.com/reference/android/graphics/ColorSpace.Model.html#CMYK)
        ///
        /// Required feature: android-graphics-ColorSpace_Model
        #[cfg(any(feature = "all", feature = "android-graphics-ColorSpace_Model"))]
        pub fn CMYK<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::ColorSpace_Model>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/ColorSpace$Model\0", "CMYK\0", "Landroid/graphics/ColorSpace$Model;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::ColorSpace_Model, crate::java::lang::Throwable>>> { ... }
    }
}
