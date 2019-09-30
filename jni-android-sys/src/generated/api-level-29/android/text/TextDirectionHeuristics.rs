// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristics"))]
__jni_bindgen! {
    /// public class [TextDirectionHeuristics](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html)
    ///
    /// Required feature: android-text-TextDirectionHeuristics
    public class TextDirectionHeuristics ("android/text/TextDirectionHeuristics") extends crate::java::lang::Object {

        /// [TextDirectionHeuristics](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#TextDirectionHeuristics())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristics>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/TextDirectionHeuristics", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/TextDirectionHeuristics\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [ANYRTL_LTR](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#ANYRTL_LTR)
        ///
        /// Required feature: android-text-TextDirectionHeuristic
        #[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristic"))]
        pub fn ANYRTL_LTR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristic>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/TextDirectionHeuristics\0", "ANYRTL_LTR\0", "Landroid/text/TextDirectionHeuristic;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FIRSTSTRONG_LTR](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#FIRSTSTRONG_LTR)
        ///
        /// Required feature: android-text-TextDirectionHeuristic
        #[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristic"))]
        pub fn FIRSTSTRONG_LTR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristic>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/TextDirectionHeuristics\0", "FIRSTSTRONG_LTR\0", "Landroid/text/TextDirectionHeuristic;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FIRSTSTRONG_RTL](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#FIRSTSTRONG_RTL)
        ///
        /// Required feature: android-text-TextDirectionHeuristic
        #[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristic"))]
        pub fn FIRSTSTRONG_RTL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristic>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/TextDirectionHeuristics\0", "FIRSTSTRONG_RTL\0", "Landroid/text/TextDirectionHeuristic;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LOCALE](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#LOCALE)
        ///
        /// Required feature: android-text-TextDirectionHeuristic
        #[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristic"))]
        pub fn LOCALE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristic>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/TextDirectionHeuristics\0", "LOCALE\0", "Landroid/text/TextDirectionHeuristic;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LTR](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#LTR)
        ///
        /// Required feature: android-text-TextDirectionHeuristic
        #[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristic"))]
        pub fn LTR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristic>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/TextDirectionHeuristics\0", "LTR\0", "Landroid/text/TextDirectionHeuristic;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [RTL](https://developer.android.com/reference/android/text/TextDirectionHeuristics.html#RTL)
        ///
        /// Required feature: android-text-TextDirectionHeuristic
        #[cfg(any(feature = "all", feature = "android-text-TextDirectionHeuristic"))]
        pub fn RTL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::TextDirectionHeuristic>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/TextDirectionHeuristics\0", "RTL\0", "Landroid/text/TextDirectionHeuristic;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
