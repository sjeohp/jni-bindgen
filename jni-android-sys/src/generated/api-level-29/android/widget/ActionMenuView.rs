// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-ActionMenuView"))]
__jni_bindgen! {
    /// public class [ActionMenuView](https://developer.android.com/reference/android/widget/ActionMenuView.html)
    ///
    /// Required feature: android-widget-ActionMenuView
    public class ActionMenuView ("android/widget/ActionMenuView") extends crate::android::widget::LinearLayout {

        /// [ActionMenuView](https://developer.android.com/reference/android/widget/ActionMenuView.html#ActionMenuView(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::ActionMenuView>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ActionMenuView](https://developer.android.com/reference/android/widget/ActionMenuView.html#ActionMenuView(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::ActionMenuView>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPopupTheme](https://developer.android.com/reference/android/widget/ActionMenuView.html#setPopupTheme(int))
        pub fn setPopupTheme<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "setPopupTheme", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "setPopupTheme\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPopupTheme](https://developer.android.com/reference/android/widget/ActionMenuView.html#getPopupTheme())
        pub fn getPopupTheme<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "getPopupTheme", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "getPopupTheme\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConfigurationChanged](https://developer.android.com/reference/android/widget/ActionMenuView.html#onConfigurationChanged(android.content.res.Configuration))
        ///
        /// Required features: "android-content-res-Configuration"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Configuration")))]
        pub fn onConfigurationChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Configuration>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "onConfigurationChanged", .descriptor == "(Landroid/content/res/Configuration;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "onConfigurationChanged\0", "(Landroid/content/res/Configuration;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOnMenuItemClickListener](https://developer.android.com/reference/android/widget/ActionMenuView.html#setOnMenuItemClickListener(android.widget.ActionMenuView.OnMenuItemClickListener))
        ///
        /// Required features: "android-widget-ActionMenuView_OnMenuItemClickListener"
        #[cfg(any(feature = "all", all(feature = "android-widget-ActionMenuView_OnMenuItemClickListener")))]
        pub fn setOnMenuItemClickListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::ActionMenuView_OnMenuItemClickListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "setOnMenuItemClickListener", .descriptor == "(Landroid/widget/ActionMenuView$OnMenuItemClickListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "setOnMenuItemClickListener\0", "(Landroid/widget/ActionMenuView$OnMenuItemClickListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onMeasure](https://developer.android.com/reference/android/widget/ActionMenuView.html#onMeasure(int,%20int))
        // fn onMeasure<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED, .name == "onMeasure", .descriptor == "(II)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "onMeasure\0", "(II)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onLayout](https://developer.android.com/reference/android/widget/ActionMenuView.html#onLayout(boolean,%20int,%20int,%20int,%20int))
        // fn onLayout<'env>(&'env self, arg0: bool, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED, .name == "onLayout", .descriptor == "(ZIIII)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "onLayout\0", "(ZIIII)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [onDetachedFromWindow](https://developer.android.com/reference/android/widget/ActionMenuView.html#onDetachedFromWindow())
        pub fn onDetachedFromWindow<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "onDetachedFromWindow", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "onDetachedFromWindow\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOverflowIcon](https://developer.android.com/reference/android/widget/ActionMenuView.html#setOverflowIcon(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn setOverflowIcon<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "setOverflowIcon", .descriptor == "(Landroid/graphics/drawable/Drawable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "setOverflowIcon\0", "(Landroid/graphics/drawable/Drawable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOverflowIcon](https://developer.android.com/reference/android/widget/ActionMenuView.html#getOverflowIcon())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn getOverflowIcon<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "getOverflowIcon", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "getOverflowIcon\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [generateDefaultLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateDefaultLayoutParams())
        // ///
        // /// Required features: "android-widget-ActionMenuView_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-widget-ActionMenuView_LayoutParams")))]
        // fn generateDefaultLayoutParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::ActionMenuView_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED, .name == "generateDefaultLayoutParams", .descriptor == "()Landroid/widget/ActionMenuView$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateDefaultLayoutParams\0", "()Landroid/widget/ActionMenuView$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [generateLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateLayoutParams(android.util.AttributeSet))
        ///
        /// Required features: "android-util-AttributeSet", "android-widget-ActionMenuView_LayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-widget-ActionMenuView_LayoutParams")))]
        pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::ActionMenuView_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/widget/ActionMenuView$LayoutParams;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/widget/ActionMenuView$LayoutParams;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams", "android-widget-ActionMenuView_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams", feature = "android-widget-ActionMenuView_LayoutParams")))]
        // fn generateLayoutParams_LayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::ActionMenuView_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED, .name == "generateLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Landroid/widget/ActionMenuView$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Landroid/widget/ActionMenuView$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [checkLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#checkLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn checkLayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED, .name == "checkLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "checkLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getMenu](https://developer.android.com/reference/android/widget/ActionMenuView.html#getMenu())
        ///
        /// Required features: "android-view-Menu"
        #[cfg(any(feature = "all", all(feature = "android-view-Menu")))]
        pub fn getMenu<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::Menu>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "getMenu", .descriptor == "()Landroid/view/Menu;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "getMenu\0", "()Landroid/view/Menu;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [showOverflowMenu](https://developer.android.com/reference/android/widget/ActionMenuView.html#showOverflowMenu())
        pub fn showOverflowMenu<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "showOverflowMenu", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "showOverflowMenu\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hideOverflowMenu](https://developer.android.com/reference/android/widget/ActionMenuView.html#hideOverflowMenu())
        pub fn hideOverflowMenu<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "hideOverflowMenu", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "hideOverflowMenu\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isOverflowMenuShowing](https://developer.android.com/reference/android/widget/ActionMenuView.html#isOverflowMenuShowing())
        pub fn isOverflowMenuShowing<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "isOverflowMenuShowing", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "isOverflowMenuShowing\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dismissPopupMenus](https://developer.android.com/reference/android/widget/ActionMenuView.html#dismissPopupMenus())
        pub fn dismissPopupMenus<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC, .name == "dismissPopupMenus", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "dismissPopupMenus\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams", "android-widget-LinearLayout_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams", feature = "android-widget-LinearLayout_LayoutParams")))]
        // fn generateLayoutParams_LayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::LinearLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Landroid/widget/LinearLayout$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Landroid/widget/LinearLayout$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // // Not emitting: Bridge method - type erasure
        // /// [generateDefaultLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateDefaultLayoutParams())
        // ///
        // /// Required features: "android-widget-LinearLayout_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-widget-LinearLayout_LayoutParams")))]
        // fn generateDefaultLayoutParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::LinearLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED | BRIDGE | SYNTHETIC, .name == "generateDefaultLayoutParams", .descriptor == "()Landroid/widget/LinearLayout$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateDefaultLayoutParams\0", "()Landroid/widget/LinearLayout$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateLayoutParams(android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-widget-LinearLayout_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-widget-LinearLayout_LayoutParams")))]
        // pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::LinearLayout_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/widget/LinearLayout$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/widget/LinearLayout$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // // Not emitting: Bridge method - type erasure
        // /// [generateDefaultLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateDefaultLayoutParams())
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn generateDefaultLayoutParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED | BRIDGE | SYNTHETIC, .name == "generateDefaultLayoutParams", .descriptor == "()Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateDefaultLayoutParams\0", "()Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateLayoutParams(android.view.ViewGroup.LayoutParams))
        // ///
        // /// Required features: "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        // fn generateLayoutParams_LayoutParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PROTECTED | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateLayoutParams\0", "(Landroid/view/ViewGroup$LayoutParams;)Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [generateLayoutParams](https://developer.android.com/reference/android/widget/ActionMenuView.html#generateLayoutParams(android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-view-ViewGroup_LayoutParams"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-view-ViewGroup_LayoutParams")))]
        // pub fn generateLayoutParams_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::ViewGroup_LayoutParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/ActionMenuView", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "generateLayoutParams", .descriptor == "(Landroid/util/AttributeSet;)Landroid/view/ViewGroup$LayoutParams;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ActionMenuView\0", "generateLayoutParams\0", "(Landroid/util/AttributeSet;)Landroid/view/ViewGroup$LayoutParams;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
