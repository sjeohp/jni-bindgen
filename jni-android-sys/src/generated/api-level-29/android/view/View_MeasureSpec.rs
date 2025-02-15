// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-View_MeasureSpec"))]
__jni_bindgen! {
    /// public class [View.MeasureSpec](https://developer.android.com/reference/android/view/View.MeasureSpec.html)
    ///
    /// Required feature: android-view-View_MeasureSpec
    public class View_MeasureSpec ("android/view/View$MeasureSpec") extends crate::java::lang::Object {

        /// [MeasureSpec](https://developer.android.com/reference/android/view/View.MeasureSpec.html#MeasureSpec())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::View_MeasureSpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$MeasureSpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/View$MeasureSpec\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [makeMeasureSpec](https://developer.android.com/reference/android/view/View.MeasureSpec.html#makeMeasureSpec(int,%20int))
        pub fn makeMeasureSpec<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$MeasureSpec", java.flags == PUBLIC | STATIC, .name == "makeMeasureSpec", .descriptor == "(II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/View$MeasureSpec\0", "makeMeasureSpec\0", "(II)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMode](https://developer.android.com/reference/android/view/View.MeasureSpec.html#getMode(int))
        pub fn getMode<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$MeasureSpec", java.flags == PUBLIC | STATIC, .name == "getMode", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/View$MeasureSpec\0", "getMode\0", "(I)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSize](https://developer.android.com/reference/android/view/View.MeasureSpec.html#getSize(int))
        pub fn getSize<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$MeasureSpec", java.flags == PUBLIC | STATIC, .name == "getSize", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/View$MeasureSpec\0", "getSize\0", "(I)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/view/View.MeasureSpec.html#toString(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$MeasureSpec", java.flags == PUBLIC | STATIC, .name == "toString", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/View$MeasureSpec\0", "toString\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [AT_MOST](https://developer.android.com/reference/android/view/View.MeasureSpec.html#AT_MOST)
        pub const AT_MOST : i32 = -2147483648;

        /// public static final [EXACTLY](https://developer.android.com/reference/android/view/View.MeasureSpec.html#EXACTLY)
        pub const EXACTLY : i32 = 1073741824;

        /// public static final [UNSPECIFIED](https://developer.android.com/reference/android/view/View.MeasureSpec.html#UNSPECIFIED)
        pub const UNSPECIFIED : i32 = 0;
    }
}
