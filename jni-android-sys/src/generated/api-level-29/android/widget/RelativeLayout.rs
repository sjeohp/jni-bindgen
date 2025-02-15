// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-RelativeLayout"))]
__jni_bindgen! {
    /// public class [RelativeLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html)
    ///
    /// Required feature: android-widget-RelativeLayout
    public class RelativeLayout ("android/widget/RelativeLayout") extends crate::android::view::ViewGroup {

        /// [RelativeLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html#RelativeLayout(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::RelativeLayout>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [RelativeLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html#RelativeLayout(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::RelativeLayout>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [RelativeLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html#RelativeLayout(android.content.Context,%20android.util.AttributeSet,%20int))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::RelativeLayout>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [RelativeLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html#RelativeLayout(android.content.Context,%20android.util.AttributeSet,%20int,%20int))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::RelativeLayout>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [shouldDelayChildPressedState](https://developer.android.com/reference/android/widget/RelativeLayout.html#shouldDelayChildPressedState())
        pub fn shouldDelayChildPressedState<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "shouldDelayChildPressedState", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "shouldDelayChildPressedState\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIgnoreGravity](https://developer.android.com/reference/android/widget/RelativeLayout.html#setIgnoreGravity(int))
        pub fn setIgnoreGravity<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "setIgnoreGravity", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "setIgnoreGravity\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIgnoreGravity](https://developer.android.com/reference/android/widget/RelativeLayout.html#getIgnoreGravity())
        pub fn getIgnoreGravity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "getIgnoreGravity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "getIgnoreGravity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGravity](https://developer.android.com/reference/android/widget/RelativeLayout.html#getGravity())
        pub fn getGravity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "getGravity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "getGravity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setGravity](https://developer.android.com/reference/android/widget/RelativeLayout.html#setGravity(int))
        pub fn setGravity<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "setGravity", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "setGravity\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHorizontalGravity](https://developer.android.com/reference/android/widget/RelativeLayout.html#setHorizontalGravity(int))
        pub fn setHorizontalGravity<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "setHorizontalGravity", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "setHorizontalGravity\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setVerticalGravity](https://developer.android.com/reference/android/widget/RelativeLayout.html#setVerticalGravity(int))
        pub fn setVerticalGravity<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "setVerticalGravity", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "setVerticalGravity\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBaseline](https://developer.android.com/reference/android/widget/RelativeLayout.html#getBaseline())
        pub fn getBaseline<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "getBaseline", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "getBaseline\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html#requestLayout())
        pub fn requestLayout<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "requestLayout", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "requestLayout\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onMeasure](https://developer.android.com/reference/android/widget/RelativeLayout.html#onMeasure(int,%20int))
        // fn onMeasure<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/RelativeLayout", java.flags == PROTECTED, .name == "onMeasure", .descriptor == "(II)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "onMeasure\0", "(II)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onLayout](https://developer.android.com/reference/android/widget/RelativeLayout.html#onLayout(boolean,%20int,%20int,%20int,%20int))
        // fn onLayout<'env>(&'env self, arg0: bool, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/RelativeLayout", java.flags == PROTECTED, .name == "onLayout", .descriptor == "(ZIIII)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "onLayout\0", "(ZIIII)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [generateLayoutParams](https://developer.android.com/reference/android/widget/RelativeLayout.html#generateLayoutParams(android.util.AttributeSet))
        ///
        /// Required features: "android-util-AttributeSet", "android-widget-RelativeLayout_LayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-widget-RelativeLayout_LayoutParams")))]
        pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::RelativeLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/widget/RelativeLayout$LayoutParams;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/widget/RelativeLayout$LayoutParams;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [generateDefaultLayoutParams](https://developer.android.com/reference/android/widget/RelativeLayout.html#generateDefaultLayoutParams())
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn generateDefaultLayoutParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/RelativeLayout", java.flags == PROTECTED, .name == "generateDefaultLayoutParams", .descriptor == "()Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "generateDefaultLayoutParams\0", "()Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [checkLayoutParams](https://developer.android.com/reference/android/widget/RelativeLayout.html#checkLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn checkLayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/RelativeLayout", java.flags == PROTECTED, .name == "checkLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "checkLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/RelativeLayout.html#generateLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn generateLayoutParams_LayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/RelativeLayout", java.flags == PROTECTED, .name == "generateLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "generateLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getAccessibilityClassName](https://developer.android.com/reference/android/widget/RelativeLayout.html#getAccessibilityClassName())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getAccessibilityClassName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC, .name == "getAccessibilityClassName", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "getAccessibilityClassName\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/RelativeLayout.html#generateLayoutParams(android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-view-ViewGroup_LayoutParams")))]
        // pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/RelativeLayout", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/RelativeLayout\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [ABOVE](https://developer.android.com/reference/android/widget/RelativeLayout.html#ABOVE)
        pub const ABOVE : i32 = 2;

        /// public static final [ALIGN_BASELINE](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_BASELINE)
        pub const ALIGN_BASELINE : i32 = 4;

        /// public static final [ALIGN_BOTTOM](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_BOTTOM)
        pub const ALIGN_BOTTOM : i32 = 8;

        /// public static final [ALIGN_END](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_END)
        pub const ALIGN_END : i32 = 19;

        /// public static final [ALIGN_LEFT](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_LEFT)
        pub const ALIGN_LEFT : i32 = 5;

        /// public static final [ALIGN_PARENT_BOTTOM](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_PARENT_BOTTOM)
        pub const ALIGN_PARENT_BOTTOM : i32 = 12;

        /// public static final [ALIGN_PARENT_END](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_PARENT_END)
        pub const ALIGN_PARENT_END : i32 = 21;

        /// public static final [ALIGN_PARENT_LEFT](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_PARENT_LEFT)
        pub const ALIGN_PARENT_LEFT : i32 = 9;

        /// public static final [ALIGN_PARENT_RIGHT](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_PARENT_RIGHT)
        pub const ALIGN_PARENT_RIGHT : i32 = 11;

        /// public static final [ALIGN_PARENT_START](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_PARENT_START)
        pub const ALIGN_PARENT_START : i32 = 20;

        /// public static final [ALIGN_PARENT_TOP](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_PARENT_TOP)
        pub const ALIGN_PARENT_TOP : i32 = 10;

        /// public static final [ALIGN_RIGHT](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_RIGHT)
        pub const ALIGN_RIGHT : i32 = 7;

        /// public static final [ALIGN_START](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_START)
        pub const ALIGN_START : i32 = 18;

        /// public static final [ALIGN_TOP](https://developer.android.com/reference/android/widget/RelativeLayout.html#ALIGN_TOP)
        pub const ALIGN_TOP : i32 = 6;

        /// public static final [BELOW](https://developer.android.com/reference/android/widget/RelativeLayout.html#BELOW)
        pub const BELOW : i32 = 3;

        /// public static final [CENTER_HORIZONTAL](https://developer.android.com/reference/android/widget/RelativeLayout.html#CENTER_HORIZONTAL)
        pub const CENTER_HORIZONTAL : i32 = 14;

        /// public static final [CENTER_IN_PARENT](https://developer.android.com/reference/android/widget/RelativeLayout.html#CENTER_IN_PARENT)
        pub const CENTER_IN_PARENT : i32 = 13;

        /// public static final [CENTER_VERTICAL](https://developer.android.com/reference/android/widget/RelativeLayout.html#CENTER_VERTICAL)
        pub const CENTER_VERTICAL : i32 = 15;

        /// public static final [END_OF](https://developer.android.com/reference/android/widget/RelativeLayout.html#END_OF)
        pub const END_OF : i32 = 17;

        /// public static final [LEFT_OF](https://developer.android.com/reference/android/widget/RelativeLayout.html#LEFT_OF)
        pub const LEFT_OF : i32 = 0;

        /// public static final [RIGHT_OF](https://developer.android.com/reference/android/widget/RelativeLayout.html#RIGHT_OF)
        pub const RIGHT_OF : i32 = 1;

        /// public static final [START_OF](https://developer.android.com/reference/android/widget/RelativeLayout.html#START_OF)
        pub const START_OF : i32 = 16;

        /// public static final [TRUE](https://developer.android.com/reference/android/widget/RelativeLayout.html#TRUE)
        pub const TRUE : i32 = -1;
    }
}
