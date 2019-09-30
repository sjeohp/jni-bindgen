// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-TwoLineListItem"))]
__jni_bindgen! {
    /// public class [TwoLineListItem](https://developer.android.com/reference/android/widget/TwoLineListItem.html)
    ///
    /// Required feature: android-widget-TwoLineListItem
    #[deprecated] public class TwoLineListItem ("android/widget/TwoLineListItem") extends crate::android::widget::RelativeLayout {

        /// [TwoLineListItem](https://developer.android.com/reference/android/widget/TwoLineListItem.html#TwoLineListItem(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        #[deprecated] pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::TwoLineListItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [TwoLineListItem](https://developer.android.com/reference/android/widget/TwoLineListItem.html#TwoLineListItem(android.content.Context,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        #[deprecated] pub fn new_Context_AttributeSet<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::TwoLineListItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [TwoLineListItem](https://developer.android.com/reference/android/widget/TwoLineListItem.html#TwoLineListItem(android.content.Context,%20android.util.AttributeSet,%20int))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        #[deprecated] pub fn new_Context_AttributeSet_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::TwoLineListItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [TwoLineListItem](https://developer.android.com/reference/android/widget/TwoLineListItem.html#TwoLineListItem(android.content.Context,%20android.util.AttributeSet,%20int,%20int))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet")))]
        #[deprecated] pub fn new_Context_AttributeSet_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::widget::TwoLineListItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/util/AttributeSet;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "<init>\0", "(Landroid/content/Context;Landroid/util/AttributeSet;II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onFinishInflate](https://developer.android.com/reference/android/widget/TwoLineListItem.html#onFinishInflate())
        // #[deprecated] fn onFinishInflate<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/widget/TwoLineListItem", java.flags == PROTECTED, .name == "onFinishInflate", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "onFinishInflate\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getText1](https://developer.android.com/reference/android/widget/TwoLineListItem.html#getText1())
        ///
        /// Required features: "android-widget-TextView"
        #[cfg(any(feature = "all", all(feature = "android-widget-TextView")))]
        #[deprecated] pub fn getText1<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::TextView>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "getText1", .descriptor == "()Landroid/widget/TextView;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "getText1\0", "()Landroid/widget/TextView;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getText2](https://developer.android.com/reference/android/widget/TwoLineListItem.html#getText2())
        ///
        /// Required features: "android-widget-TextView"
        #[cfg(any(feature = "all", all(feature = "android-widget-TextView")))]
        #[deprecated] pub fn getText2<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::TextView>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "getText2", .descriptor == "()Landroid/widget/TextView;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "getText2\0", "()Landroid/widget/TextView;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccessibilityClassName](https://developer.android.com/reference/android/widget/TwoLineListItem.html#getAccessibilityClassName())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn getAccessibilityClassName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/TwoLineListItem", java.flags == PUBLIC, .name == "getAccessibilityClassName", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/TwoLineListItem\0", "getAccessibilityClassName\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
