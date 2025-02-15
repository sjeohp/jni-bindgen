// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-TableRow"))]
__jni_bindgen! {
    /// public class [TableRow](https://developer.android.com/reference/android/widget/TableRow.html)
    ///
    /// Required feature: android-widget-TableRow
    public class TableRow ("android/widget/TableRow") extends crate::android::widget::LinearLayout {

        /// [TableRow](https://developer.android.com/reference/android/widget/TableRow.html#TableRow(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::TableRow>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [TableRow](https://developer.android.com/reference/android/widget/TableRow.html#TableRow(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::TableRow>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOnHierarchyChangeListener](https://developer.android.com/reference/android/widget/TableRow.html#setOnHierarchyChangeListener(android.view.ViewGroup.OnHierarchyChangeListener))
        ///
        /// Required features: "android-view-ViewGroup_OnHierarchyChangeListener"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_OnHierarchyChangeListener")))]
        pub fn setOnHierarchyChangeListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_OnHierarchyChangeListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "setOnHierarchyChangeListener", .descriptor == "(Landroid/view/ViewGroup$OnHierarchyChangeListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "setOnHierarchyChangeListener\0", "(Landroid/view/ViewGroup$OnHierarchyChangeListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onMeasure](https://developer.android.com/reference/android/widget/TableRow.html#onMeasure(int,%20int))
        // fn onMeasure<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED, .name == "onMeasure", .descriptor == "(II)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "onMeasure\0", "(II)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onLayout](https://developer.android.com/reference/android/widget/TableRow.html#onLayout(boolean,%20int,%20int,%20int,%20int))
        // fn onLayout<'env>(&'env self, arg0: bool, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED, .name == "onLayout", .descriptor == "(ZIIII)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "onLayout\0", "(ZIIII)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getVirtualChildAt](https://developer.android.com/reference/android/widget/TableRow.html#getVirtualChildAt(int))
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        pub fn getVirtualChildAt<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "getVirtualChildAt", .descriptor == "(I)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "getVirtualChildAt\0", "(I)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVirtualChildCount](https://developer.android.com/reference/android/widget/TableRow.html#getVirtualChildCount())
        pub fn getVirtualChildCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "getVirtualChildCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "getVirtualChildCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [generateLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateLayoutParams(android.util.AttributeSet))
        ///
        /// Required features: "android-util-AttributeSet", "android-widget-TableRow_LayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-widget-TableRow_LayoutParams")))]
        pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::TableRow_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/widget/TableRow$LayoutParams;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/widget/TableRow$LayoutParams;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [generateDefaultLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateDefaultLayoutParams())
        // ///
        // /// Required features: "android-widget-LinearLayout_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-widget-LinearLayout_LayoutParams")))]
        // fn generateDefaultLayoutParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::LinearLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED, .name == "generateDefaultLayoutParams", .descriptor == "()Landroid/widget/LinearLayout$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateDefaultLayoutParams\0", "()Landroid/widget/LinearLayout$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [checkLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#checkLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn checkLayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED, .name == "checkLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "checkLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams", "android-widget-LinearLayout_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams", feature = "android-widget-LinearLayout_LayoutParams")))]
        // fn generateLayoutParams_LayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::LinearLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED, .name == "generateLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Landroid/widget/LinearLayout$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Landroid/widget/LinearLayout$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getAccessibilityClassName](https://developer.android.com/reference/android/widget/TableRow.html#getAccessibilityClassName())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getAccessibilityClassName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TableRow", java.flags == PUBLIC, .name == "getAccessibilityClassName", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "getAccessibilityClassName\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateLayoutParams(android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-widget-LinearLayout_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-widget-LinearLayout_LayoutParams")))]
        // pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::LinearLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/widget/LinearLayout$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/widget/LinearLayout$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // // Not emitting: Bridge method - type erasure
        // /// [generateDefaultLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateDefaultLayoutParams())
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn generateDefaultLayoutParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED | BRIDGE | SYNTHETIC, .name == "generateDefaultLayoutParams", .descriptor == "()Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateDefaultLayoutParams\0", "()Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn generateLayoutParams_LayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PROTECTED | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/TableRow.html#generateLayoutParams(android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-view-ViewGroup_LayoutParams")))]
        // pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TableRow", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TableRow\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
