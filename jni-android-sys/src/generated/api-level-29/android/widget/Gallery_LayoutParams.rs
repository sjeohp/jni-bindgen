// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-Gallery_LayoutParams"))]
__jni_bindgen! {
    /// public class [Gallery.LayoutParams](https://developer.android.com/reference/android/widget/Gallery.LayoutParams.html)
    ///
    /// Required feature: android-widget-Gallery_LayoutParams
    #[deprecated] public class Gallery_LayoutParams ("android/widget/Gallery$LayoutParams") extends crate::android::view::ViewGroup_LayoutParams {

        /// [LayoutParams](https://developer.android.com/reference/android/widget/Gallery.LayoutParams.html#LayoutParams(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        #[deprecated] pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::Gallery_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Gallery$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Gallery$LayoutParams\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/Gallery.LayoutParams.html#LayoutParams(int,%20int))
        #[deprecated] pub fn new_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::Gallery_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Gallery$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Gallery$LayoutParams\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [LayoutParams](https://developer.android.com/reference/android/widget/Gallery.LayoutParams.html#LayoutParams(android.view.ViewGroup.LayoutParams))
        ///
        /// Required features: "android-view-ViewGroup_LayoutParams"
        #[cfg(any(feature = "all", all(feature = "android-view-ViewGroup_LayoutParams")))]
        #[deprecated] pub fn new_LayoutParams<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup_LayoutParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::Gallery_LayoutParams>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Gallery$LayoutParams", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/ViewGroup$LayoutParams;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Gallery$LayoutParams\0", "<init>\0", "(Landroid/view/ViewGroup$LayoutParams;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
