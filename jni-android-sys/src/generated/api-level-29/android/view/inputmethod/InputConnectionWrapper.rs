// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-inputmethod-InputConnectionWrapper"))]
__jni_bindgen! {
    /// public class [InputConnectionWrapper](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html)
    ///
    /// Required feature: android-view-inputmethod-InputConnectionWrapper
    public class InputConnectionWrapper ("android/view/inputmethod/InputConnectionWrapper") extends crate::java::lang::Object, implements crate::android::view::inputmethod::InputConnection {

        /// [InputConnectionWrapper](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#InputConnectionWrapper(android.view.inputmethod.InputConnection,%20boolean))
        ///
        /// Required features: "android-view-inputmethod-InputConnection"
        #[cfg(any(feature = "all", all(feature = "android-view-inputmethod-InputConnection")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inputmethod::InputConnection>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::inputmethod::InputConnectionWrapper>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/view/inputmethod/InputConnection;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "<init>\0", "(Landroid/view/inputmethod/InputConnection;Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTarget](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#setTarget(android.view.inputmethod.InputConnection))
        ///
        /// Required features: "android-view-inputmethod-InputConnection"
        #[cfg(any(feature = "all", all(feature = "android-view-inputmethod-InputConnection")))]
        pub fn setTarget<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inputmethod::InputConnection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "setTarget", .descriptor == "(Landroid/view/inputmethod/InputConnection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "setTarget\0", "(Landroid/view/inputmethod/InputConnection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTextBeforeCursor](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#getTextBeforeCursor(int,%20int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getTextBeforeCursor<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "getTextBeforeCursor", .descriptor == "(II)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "getTextBeforeCursor\0", "(II)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTextAfterCursor](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#getTextAfterCursor(int,%20int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getTextAfterCursor<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "getTextAfterCursor", .descriptor == "(II)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "getTextAfterCursor\0", "(II)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSelectedText](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#getSelectedText(int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getSelectedText<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "getSelectedText", .descriptor == "(I)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "getSelectedText\0", "(I)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCursorCapsMode](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#getCursorCapsMode(int))
        pub fn getCursorCapsMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "getCursorCapsMode", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "getCursorCapsMode\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtractedText](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#getExtractedText(android.view.inputmethod.ExtractedTextRequest,%20int))
        ///
        /// Required features: "android-view-inputmethod-ExtractedText", "android-view-inputmethod-ExtractedTextRequest"
        #[cfg(any(feature = "all", all(feature = "android-view-inputmethod-ExtractedText", feature = "android-view-inputmethod-ExtractedTextRequest")))]
        pub fn getExtractedText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inputmethod::ExtractedTextRequest>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::inputmethod::ExtractedText>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "getExtractedText", .descriptor == "(Landroid/view/inputmethod/ExtractedTextRequest;I)Landroid/view/inputmethod/ExtractedText;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "getExtractedText\0", "(Landroid/view/inputmethod/ExtractedTextRequest;I)Landroid/view/inputmethod/ExtractedText;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deleteSurroundingTextInCodePoints](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#deleteSurroundingTextInCodePoints(int,%20int))
        pub fn deleteSurroundingTextInCodePoints<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "deleteSurroundingTextInCodePoints", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "deleteSurroundingTextInCodePoints\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deleteSurroundingText](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#deleteSurroundingText(int,%20int))
        pub fn deleteSurroundingText<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "deleteSurroundingText", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "deleteSurroundingText\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setComposingText](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#setComposingText(java.lang.CharSequence,%20int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn setComposingText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "setComposingText", .descriptor == "(Ljava/lang/CharSequence;I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "setComposingText\0", "(Ljava/lang/CharSequence;I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setComposingRegion](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#setComposingRegion(int,%20int))
        pub fn setComposingRegion<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "setComposingRegion", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "setComposingRegion\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [finishComposingText](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#finishComposingText())
        pub fn finishComposingText<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "finishComposingText", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "finishComposingText\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [commitText](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#commitText(java.lang.CharSequence,%20int))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn commitText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "commitText", .descriptor == "(Ljava/lang/CharSequence;I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "commitText\0", "(Ljava/lang/CharSequence;I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [commitCompletion](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#commitCompletion(android.view.inputmethod.CompletionInfo))
        ///
        /// Required features: "android-view-inputmethod-CompletionInfo"
        #[cfg(any(feature = "all", all(feature = "android-view-inputmethod-CompletionInfo")))]
        pub fn commitCompletion<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inputmethod::CompletionInfo>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "commitCompletion", .descriptor == "(Landroid/view/inputmethod/CompletionInfo;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "commitCompletion\0", "(Landroid/view/inputmethod/CompletionInfo;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [commitCorrection](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#commitCorrection(android.view.inputmethod.CorrectionInfo))
        ///
        /// Required features: "android-view-inputmethod-CorrectionInfo"
        #[cfg(any(feature = "all", all(feature = "android-view-inputmethod-CorrectionInfo")))]
        pub fn commitCorrection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inputmethod::CorrectionInfo>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "commitCorrection", .descriptor == "(Landroid/view/inputmethod/CorrectionInfo;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "commitCorrection\0", "(Landroid/view/inputmethod/CorrectionInfo;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSelection](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#setSelection(int,%20int))
        pub fn setSelection<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "setSelection", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "setSelection\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [performEditorAction](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#performEditorAction(int))
        pub fn performEditorAction<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "performEditorAction", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "performEditorAction\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [performContextMenuAction](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#performContextMenuAction(int))
        pub fn performContextMenuAction<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "performContextMenuAction", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "performContextMenuAction\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [beginBatchEdit](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#beginBatchEdit())
        pub fn beginBatchEdit<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "beginBatchEdit", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "beginBatchEdit\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [endBatchEdit](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#endBatchEdit())
        pub fn endBatchEdit<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "endBatchEdit", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "endBatchEdit\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sendKeyEvent](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#sendKeyEvent(android.view.KeyEvent))
        ///
        /// Required features: "android-view-KeyEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent")))]
        pub fn sendKeyEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "sendKeyEvent", .descriptor == "(Landroid/view/KeyEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "sendKeyEvent\0", "(Landroid/view/KeyEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clearMetaKeyStates](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#clearMetaKeyStates(int))
        pub fn clearMetaKeyStates<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "clearMetaKeyStates", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "clearMetaKeyStates\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reportFullscreenMode](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#reportFullscreenMode(boolean))
        pub fn reportFullscreenMode<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "reportFullscreenMode", .descriptor == "(Z)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "reportFullscreenMode\0", "(Z)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [performPrivateCommand](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#performPrivateCommand(java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn performPrivateCommand<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "performPrivateCommand", .descriptor == "(Ljava/lang/String;Landroid/os/Bundle;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "performPrivateCommand\0", "(Ljava/lang/String;Landroid/os/Bundle;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestCursorUpdates](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#requestCursorUpdates(int))
        pub fn requestCursorUpdates<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "requestCursorUpdates", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "requestCursorUpdates\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHandler](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#getHandler())
        ///
        /// Required features: "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-os-Handler")))]
        pub fn getHandler<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Handler>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "getHandler", .descriptor == "()Landroid/os/Handler;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "getHandler\0", "()Landroid/os/Handler;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [closeConnection](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#closeConnection())
        pub fn closeConnection<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "closeConnection", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "closeConnection\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [commitContent](https://developer.android.com/reference/android/view/inputmethod/InputConnectionWrapper.html#commitContent(android.view.inputmethod.InputContentInfo,%20int,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-view-inputmethod-InputContentInfo"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-view-inputmethod-InputContentInfo")))]
        pub fn commitContent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::inputmethod::InputContentInfo>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inputmethod/InputConnectionWrapper", java.flags == PUBLIC, .name == "commitContent", .descriptor == "(Landroid/view/inputmethod/InputContentInfo;ILandroid/os/Bundle;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inputmethod/InputConnectionWrapper\0", "commitContent\0", "(Landroid/view/inputmethod/InputContentInfo;ILandroid/os/Bundle;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
