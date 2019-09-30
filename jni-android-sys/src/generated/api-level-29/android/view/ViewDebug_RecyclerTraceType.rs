// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
__jni_bindgen! {
    /// public enum [ViewDebug.RecyclerTraceType](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html)
    ///
    /// Required feature: android-view-ViewDebug_RecyclerTraceType
    #[deprecated] public enum ViewDebug_RecyclerTraceType ("android/view/ViewDebug$RecyclerTraceType") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#values())
        ///
        /// Required features: "android-view-ViewDebug_RecyclerTraceType"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewDebug_RecyclerTraceType")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::ViewDebug_RecyclerTraceType, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug$RecyclerTraceType", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/view/ViewDebug$RecyclerTraceType;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug$RecyclerTraceType\0", "values\0", "()[Landroid/view/ViewDebug$RecyclerTraceType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-view-ViewDebug_RecyclerTraceType", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewDebug_RecyclerTraceType", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug$RecyclerTraceType", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/view/ViewDebug$RecyclerTraceType;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug$RecyclerTraceType\0", "valueOf\0", "(Ljava/lang/String;)Landroid/view/ViewDebug$RecyclerTraceType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [RecyclerTraceType](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#RecyclerTraceType(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/ViewDebug$RecyclerTraceType", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ViewDebug$RecyclerTraceType\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [NEW_VIEW](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#NEW_VIEW)
        ///
        /// Required feature: android-view-ViewDebug_RecyclerTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
        #[deprecated] pub fn NEW_VIEW<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$RecyclerTraceType\0", "NEW_VIEW\0", "Landroid/view/ViewDebug$RecyclerTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [BIND_VIEW](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#BIND_VIEW)
        ///
        /// Required feature: android-view-ViewDebug_RecyclerTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
        #[deprecated] pub fn BIND_VIEW<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$RecyclerTraceType\0", "BIND_VIEW\0", "Landroid/view/ViewDebug$RecyclerTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [RECYCLE_FROM_ACTIVE_HEAP](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#RECYCLE_FROM_ACTIVE_HEAP)
        ///
        /// Required feature: android-view-ViewDebug_RecyclerTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
        #[deprecated] pub fn RECYCLE_FROM_ACTIVE_HEAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$RecyclerTraceType\0", "RECYCLE_FROM_ACTIVE_HEAP\0", "Landroid/view/ViewDebug$RecyclerTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [RECYCLE_FROM_SCRAP_HEAP](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#RECYCLE_FROM_SCRAP_HEAP)
        ///
        /// Required feature: android-view-ViewDebug_RecyclerTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
        #[deprecated] pub fn RECYCLE_FROM_SCRAP_HEAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$RecyclerTraceType\0", "RECYCLE_FROM_SCRAP_HEAP\0", "Landroid/view/ViewDebug$RecyclerTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MOVE_TO_SCRAP_HEAP](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#MOVE_TO_SCRAP_HEAP)
        ///
        /// Required feature: android-view-ViewDebug_RecyclerTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
        #[deprecated] pub fn MOVE_TO_SCRAP_HEAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$RecyclerTraceType\0", "MOVE_TO_SCRAP_HEAP\0", "Landroid/view/ViewDebug$RecyclerTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MOVE_FROM_ACTIVE_TO_SCRAP_HEAP](https://developer.android.com/reference/android/view/ViewDebug.RecyclerTraceType.html#MOVE_FROM_ACTIVE_TO_SCRAP_HEAP)
        ///
        /// Required feature: android-view-ViewDebug_RecyclerTraceType
        #[cfg(any(feature = "all", feature = "android-view-ViewDebug_RecyclerTraceType"))]
        #[deprecated] pub fn MOVE_FROM_ACTIVE_TO_SCRAP_HEAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewDebug_RecyclerTraceType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/view/ViewDebug$RecyclerTraceType\0", "MOVE_FROM_ACTIVE_TO_SCRAP_HEAP\0", "Landroid/view/ViewDebug$RecyclerTraceType;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::ViewDebug_RecyclerTraceType, crate::java::lang::Throwable>>> { ... }
    }
}
