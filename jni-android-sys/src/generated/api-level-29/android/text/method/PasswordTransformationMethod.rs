// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-method-PasswordTransformationMethod"))]
__jni_bindgen! {
    /// public class [PasswordTransformationMethod](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html)
    ///
    /// Required feature: android-text-method-PasswordTransformationMethod
    public class PasswordTransformationMethod ("android/text/method/PasswordTransformationMethod") extends crate::java::lang::Object, implements crate::android::text::method::TransformationMethod, crate::android::text::TextWatcher {

        /// [PasswordTransformationMethod](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#PasswordTransformationMethod())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::method::PasswordTransformationMethod>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/PasswordTransformationMethod\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTransformation](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#getTransformation(java.lang.CharSequence,%20android.view.View))
        ///
        /// Required features: "android-view-View", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "java-lang-CharSequence")))]
        pub fn getTransformation<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC, .name == "getTransformation", .descriptor == "(Ljava/lang/CharSequence;Landroid/view/View;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/PasswordTransformationMethod\0", "getTransformation\0", "(Ljava/lang/CharSequence;Landroid/view/View;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#getInstance())
        ///
        /// Required features: "android-text-method-PasswordTransformationMethod"
        #[cfg(any(feature = "all", all(feature = "android-text-method-PasswordTransformationMethod")))]
        pub fn getInstance<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::method::PasswordTransformationMethod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "()Landroid/text/method/PasswordTransformationMethod;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/method/PasswordTransformationMethod\0", "getInstance\0", "()Landroid/text/method/PasswordTransformationMethod;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [beforeTextChanged](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#beforeTextChanged(java.lang.CharSequence,%20int,%20int,%20int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn beforeTextChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC, .name == "beforeTextChanged", .descriptor == "(Ljava/lang/CharSequence;III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/PasswordTransformationMethod\0", "beforeTextChanged\0", "(Ljava/lang/CharSequence;III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTextChanged](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#onTextChanged(java.lang.CharSequence,%20int,%20int,%20int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn onTextChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC, .name == "onTextChanged", .descriptor == "(Ljava/lang/CharSequence;III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/PasswordTransformationMethod\0", "onTextChanged\0", "(Ljava/lang/CharSequence;III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [afterTextChanged](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#afterTextChanged(android.text.Editable))
        ///
        /// Required features: "android-text-Editable"
        #[cfg(any(feature = "all", all(feature = "android-text-Editable")))]
        pub fn afterTextChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Editable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC, .name == "afterTextChanged", .descriptor == "(Landroid/text/Editable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/PasswordTransformationMethod\0", "afterTextChanged\0", "(Landroid/text/Editable;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFocusChanged](https://developer.android.com/reference/android/text/method/PasswordTransformationMethod.html#onFocusChanged(android.view.View,%20java.lang.CharSequence,%20boolean,%20int,%20android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect", "android-view-View", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect", feature = "android-view-View", feature = "java-lang-CharSequence")))]
        pub fn onFocusChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: bool, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/method/PasswordTransformationMethod", java.flags == PUBLIC, .name == "onFocusChanged", .descriptor == "(Landroid/view/View;Ljava/lang/CharSequence;ZILandroid/graphics/Rect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/method/PasswordTransformationMethod\0", "onFocusChanged\0", "(Landroid/view/View;Ljava/lang/CharSequence;ZILandroid/graphics/Rect;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
