// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-LoginFilter"))]
__jni_bindgen! {
    /// public class [LoginFilter](https://developer.android.com/reference/android/text/LoginFilter.html)
    ///
    /// Required feature: android-text-LoginFilter
    public class LoginFilter ("android/text/LoginFilter") extends crate::java::lang::Object, implements crate::android::text::InputFilter {

        // // Not emitting: Non-public method
        // /// [LoginFilter](https://developer.android.com/reference/android/text/LoginFilter.html#LoginFilter())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::LoginFilter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/text/LoginFilter", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [filter](https://developer.android.com/reference/android/text/LoginFilter.html#filter(java.lang.CharSequence,%20int,%20int,%20android.text.Spanned,%20int,%20int))
        ///
        /// Required features: "android-text-Spanned", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-text-Spanned", feature = "java-lang-CharSequence")))]
        pub fn filter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spanned>>, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter", java.flags == PUBLIC, .name == "filter", .descriptor == "(Ljava/lang/CharSequence;IILandroid/text/Spanned;II)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter\0", "filter\0", "(Ljava/lang/CharSequence;IILandroid/text/Spanned;II)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStart](https://developer.android.com/reference/android/text/LoginFilter.html#onStart())
        pub fn onStart<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter", java.flags == PUBLIC, .name == "onStart", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter\0", "onStart\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onInvalidCharacter](https://developer.android.com/reference/android/text/LoginFilter.html#onInvalidCharacter(char))
        pub fn onInvalidCharacter<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter", java.flags == PUBLIC, .name == "onInvalidCharacter", .descriptor == "(C)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter\0", "onInvalidCharacter\0", "(C)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStop](https://developer.android.com/reference/android/text/LoginFilter.html#onStop())
        pub fn onStop<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter", java.flags == PUBLIC, .name == "onStop", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter\0", "onStop\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isAllowed](https://developer.android.com/reference/android/text/LoginFilter.html#isAllowed(char))
        pub fn isAllowed<'env>(&'env self, arg0: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/LoginFilter", java.flags == PUBLIC | ABSTRACT, .name == "isAllowed", .descriptor == "(C)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/LoginFilter\0", "isAllowed\0", "(C)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
