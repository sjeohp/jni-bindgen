// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
__jni_bindgen! {
    /// public enum [ViewDebug.HierarchyTraceType](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html)
    ///
    /// Required feature: android-view-ViewDebug_HierarchyTraceType
    #[deprecated] public enum ViewDebug_HierarchyTraceType ("android/view/ViewDebug$HierarchyTraceType") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#values())
        ///
        /// Required features: "android-view-ViewDebug_HierarchyTraceType"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewDebug_HierarchyTraceType")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::ViewDebug_HierarchyTraceType, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug$HierarchyTraceType", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/view/ViewDebug$HierarchyTraceType;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug$HierarchyTraceType\0", "values\0", "()[Landroid/view/ViewDebug$HierarchyTraceType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-view-ViewDebug_HierarchyTraceType", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewDebug_HierarchyTraceType", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug$HierarchyTraceType", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/view/ViewDebug$HierarchyTraceType;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug$HierarchyTraceType\0", "valueOf\0", "(Ljava/lang/String;)Landroid/view/ViewDebug$HierarchyTraceType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [HierarchyTraceType](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#HierarchyTraceType(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/ViewDebug$HierarchyTraceType", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ViewDebug$HierarchyTraceType\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [INVALIDATE](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#INVALIDATE)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn INVALIDATE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "INVALIDATE\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INVALIDATE_CHILD](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#INVALIDATE_CHILD)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn INVALIDATE_CHILD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "INVALIDATE_CHILD\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INVALIDATE_CHILD_IN_PARENT](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#INVALIDATE_CHILD_IN_PARENT)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn INVALIDATE_CHILD_IN_PARENT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "INVALIDATE_CHILD_IN_PARENT\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REQUEST_LAYOUT](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#REQUEST_LAYOUT)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn REQUEST_LAYOUT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "REQUEST_LAYOUT\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ON_LAYOUT](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#ON_LAYOUT)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn ON_LAYOUT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "ON_LAYOUT\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ON_MEASURE](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#ON_MEASURE)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn ON_MEASURE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "ON_MEASURE\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DRAW](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#DRAW)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn DRAW<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "DRAW\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [BUILD_CACHE](https://developer.android.com/reference/android/view/ViewDebug.HierarchyTraceType.html#BUILD_CACHE)
        ///
        /// Required feature: android-view-ViewDebug_HierarchyTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_HierarchyTraceType"))]
        #[deprecated] pub fn BUILD_CACHE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_HierarchyTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$HierarchyTraceType\0", "BUILD_CACHE\0", "Landroid/view/ViewDebug$HierarchyTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::ViewDebug_HierarchyTraceType, crate::java::lang::Throwable>>> { ... }
    }
}
