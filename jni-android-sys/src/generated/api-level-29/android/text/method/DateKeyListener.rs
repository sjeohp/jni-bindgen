// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-method-DateKeyListener"))]
__jni_bindgen! {
    /// public class [DateKeyListener](https://developer.android.com/reference/android/text/method/DateKeyListener.html)
    ///
    /// Required feature: android-text-method-DateKeyListener
    public class DateKeyListener ("android/text/method/DateKeyListener") extends crate::android::text::method::NumberKeyListener {

        /// [DateKeyListener](https://developer.android.com/reference/android/text/method/DateKeyListener.html#DateKeyListener())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::method::DateKeyListener>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/DateKeyListener", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/DateKeyListener\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DateKeyListener](https://developer.android.com/reference/android/text/method/DateKeyListener.html#DateKeyListener(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn new_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::method::DateKeyListener>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/DateKeyListener", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/DateKeyListener\0", "<init>\0", "(Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInputType](https://developer.android.com/reference/android/text/method/DateKeyListener.html#getInputType())
        pub fn getInputType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/DateKeyListener", java.flags == PUBLIC, .name == "getInputType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/DateKeyListener\0", "getInputType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [getAcceptedChars](https://developer.android.com/reference/android/text/method/DateKeyListener.html#getAcceptedChars())
        // fn getAcceptedChars<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::CharArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/method/DateKeyListener", java.flags == PROTECTED, .name == "getAcceptedChars", .descriptor == "()[C"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/DateKeyListener\0", "getAcceptedChars\0", "()[C\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/android/text/method/DateKeyListener.html#getInstance())
        ///
        /// Required features: "android-text-method-DateKeyListener"
        #[cfg(any(feature = "all", all(feature = "android-text-method-DateKeyListener")))]
        #[deprecated] pub fn getInstance<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::method::DateKeyListener>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/DateKeyListener", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "()Landroid/text/method/DateKeyListener;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/method/DateKeyListener\0", "getInstance\0", "()Landroid/text/method/DateKeyListener;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/text/method/DateKeyListener.html#getInstance(java.util.Locale))
        ///
        /// Required features: "android-text-method-DateKeyListener", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-text-method-DateKeyListener", feature = "java-util-Locale")))]
        pub fn getInstance_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::method::DateKeyListener>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/DateKeyListener", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/util/Locale;)Landroid/text/method/DateKeyListener;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/method/DateKeyListener\0", "getInstance\0", "(Ljava/util/Locale;)Landroid/text/method/DateKeyListener;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CHARACTERS](https://developer.android.com/reference/android/text/method/DateKeyListener.html#CHARACTERS)
        #[deprecated] pub fn CHARACTERS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::CharArray>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/text/method/DateKeyListener\0", "CHARACTERS\0", "[C\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
