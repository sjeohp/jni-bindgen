// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-inputmethodservice-KeyboardView_OnKeyboardActionListener"))]
__jni_bindgen! {
    /// public interface [KeyboardView.OnKeyboardActionListener](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html)
    ///
    /// Required feature: android-inputmethodservice-KeyboardView_OnKeyboardActionListener
    #[deprecated] public interface KeyboardView_OnKeyboardActionListener ("android/inputmethodservice/KeyboardView$OnKeyboardActionListener") extends crate::java::lang::Object {

        /// [onPress](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#onPress(int))
        #[deprecated] pub fn onPress<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "onPress", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "onPress\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRelease](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#onRelease(int))
        #[deprecated] pub fn onRelease<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "onRelease", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "onRelease\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onKey](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#onKey(int,%20int%5B%5D))
        #[deprecated] pub fn onKey<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "onKey", .descriptor == "(I[I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "onKey\0", "(I[I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onText](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#onText(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn onText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "onText", .descriptor == "(Ljava/lang/CharSequence;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "onText\0", "(Ljava/lang/CharSequence;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [swipeLeft](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#swipeLeft())
        #[deprecated] pub fn swipeLeft<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "swipeLeft", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "swipeLeft\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [swipeRight](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#swipeRight())
        #[deprecated] pub fn swipeRight<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "swipeRight", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "swipeRight\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [swipeDown](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#swipeDown())
        #[deprecated] pub fn swipeDown<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "swipeDown", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "swipeDown\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [swipeUp](https://developer.android.com/reference/android/inputmethodservice/KeyboardView.OnKeyboardActionListener.html#swipeUp())
        #[deprecated] pub fn swipeUp<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/KeyboardView$OnKeyboardActionListener", java.flags == PUBLIC | ABSTRACT, .name == "swipeUp", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/KeyboardView$OnKeyboardActionListener\0", "swipeUp\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
