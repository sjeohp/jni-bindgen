// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-FrameLayout_LayoutParams"))]
__jni_bindgen! {
    /// public class [FrameLayout.LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html)
    ///
    /// Required feature: android-widget-FrameLayout_LayoutParams
    public class FrameLayout_LayoutParams ("android/widget/FrameLayout$LayoutParams") extends crate::android::view::ViewGroup_MarginLayoutParams {

        /// [LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#LayoutParams(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::FrameLayout_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FrameLayout$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FrameLayout$LayoutParams\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#LayoutParams(int,%20int))
        pub fn new_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::FrameLayout_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FrameLayout$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FrameLayout$LayoutParams\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#LayoutParams(int,%20int,%20int))
        pub fn new_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::FrameLayout_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FrameLayout$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FrameLayout$LayoutParams\0", "<init>\0", "(III)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#LayoutParams(android.view.ViewGroup.LayoutParams))
        ///
        /// Required features: "android-view-ViewGroup_LayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        pub fn new_ViewGroup_LayoutParams<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::FrameLayout_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FrameLayout$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FrameLayout$LayoutParams\0", "<init>\0", "(Landroid/view/ViewGroup$LayoutParams;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#LayoutParams(android.view.ViewGroup.MarginLayoutParams))
        ///
        /// Required features: "android-view-ViewGroup_MarginLayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_MarginLayoutParams")))]
        pub fn new_ViewGroup_MarginLayoutParams<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_MarginLayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::FrameLayout_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FrameLayout$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/ViewGroup$MarginLayoutParams;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FrameLayout$LayoutParams\0", "<init>\0", "(Landroid/view/ViewGroup$MarginLayoutParams;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#LayoutParams(android.widget.FrameLayout.LayoutParams))
        ///
        /// Required features: "android-widget-FrameLayout_LayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-widget-FrameLayout_LayoutParams")))]
        pub fn new_LayoutParams<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::FrameLayout_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::FrameLayout_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/FrameLayout$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/widget/FrameLayout$LayoutParams;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/FrameLayout$LayoutParams\0", "<init>\0", "(Landroid/widget/FrameLayout$LayoutParams;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [UNSPECIFIED_GRAVITY](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#UNSPECIFIED_GRAVITY)
        pub const UNSPECIFIED_GRAVITY : i32 = -1;

        /// **get** public [gravity](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#gravity)
        pub fn gravity<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/widget/FrameLayout$LayoutParams\0", "gravity\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [gravity](https://developer.android.com/reference/android/widget/FrameLayout.LayoutParams.html#gravity)
        pub fn set_gravity<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/widget/FrameLayout$LayoutParams\0", "gravity\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
