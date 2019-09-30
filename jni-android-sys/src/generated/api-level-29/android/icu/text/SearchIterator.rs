// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-SearchIterator"))]
__jni_bindgen! {
    /// public class [SearchIterator](https://developer.android.com/reference/android/icu/text/SearchIterator.html)
    ///
    /// Required feature: android-icu-text-SearchIterator
    public class SearchIterator ("android/icu/text/SearchIterator") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SearchIterator](https://developer.android.com/reference/android/icu/text/SearchIterator.html#SearchIterator(java.text.CharacterIterator,%20android.icu.text.BreakIterator))
        // ///
        // /// Required features: "android-icu-text-BreakIterator", "java-text-CharacterIterator"
        // #[cfg(any(feature = "all", all(feature = "android-icu-text-BreakIterator", feature = "java-text-CharacterIterator")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::CharacterIterator>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::BreakIterator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/SearchIterator", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/text/CharacterIterator;Landroid/icu/text/BreakIterator;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "<init>\0", "(Ljava/text/CharacterIterator;Landroid/icu/text/BreakIterator;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [setIndex](https://developer.android.com/reference/android/icu/text/SearchIterator.html#setIndex(int))
        pub fn setIndex<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "setIndex", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "setIndex\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOverlapping](https://developer.android.com/reference/android/icu/text/SearchIterator.html#setOverlapping(boolean))
        pub fn setOverlapping<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "setOverlapping", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "setOverlapping\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBreakIterator](https://developer.android.com/reference/android/icu/text/SearchIterator.html#setBreakIterator(android.icu.text.BreakIterator))
        ///
        /// Required features: "android-icu-text-BreakIterator"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-BreakIterator")))]
        pub fn setBreakIterator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::BreakIterator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "setBreakIterator", .descriptor == "(Landroid/icu/text/BreakIterator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "setBreakIterator\0", "(Landroid/icu/text/BreakIterator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTarget](https://developer.android.com/reference/android/icu/text/SearchIterator.html#setTarget(java.text.CharacterIterator))
        ///
        /// Required features: "java-text-CharacterIterator"
        #[cfg(any(feature = "all", all(feature = "java-text-CharacterIterator")))]
        pub fn setTarget<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::CharacterIterator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "setTarget", .descriptor == "(Ljava/text/CharacterIterator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "setTarget\0", "(Ljava/text/CharacterIterator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMatchStart](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getMatchStart())
        pub fn getMatchStart<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "getMatchStart", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getMatchStart\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIndex](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getIndex())
        pub fn getIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC | ABSTRACT, .name == "getIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMatchLength](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getMatchLength())
        pub fn getMatchLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "getMatchLength", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getMatchLength\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBreakIterator](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getBreakIterator())
        ///
        /// Required features: "android-icu-text-BreakIterator"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-BreakIterator")))]
        pub fn getBreakIterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::BreakIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "getBreakIterator", .descriptor == "()Landroid/icu/text/BreakIterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getBreakIterator\0", "()Landroid/icu/text/BreakIterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTarget](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getTarget())
        ///
        /// Required features: "java-text-CharacterIterator"
        #[cfg(any(feature = "all", all(feature = "java-text-CharacterIterator")))]
        pub fn getTarget<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::CharacterIterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "getTarget", .descriptor == "()Ljava/text/CharacterIterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getTarget\0", "()Ljava/text/CharacterIterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMatchedText](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getMatchedText())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMatchedText<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "getMatchedText", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getMatchedText\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [next](https://developer.android.com/reference/android/icu/text/SearchIterator.html#next())
        pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "next", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "next\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [previous](https://developer.android.com/reference/android/icu/text/SearchIterator.html#previous())
        pub fn previous<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "previous", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "previous\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isOverlapping](https://developer.android.com/reference/android/icu/text/SearchIterator.html#isOverlapping())
        pub fn isOverlapping<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "isOverlapping", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "isOverlapping\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/android/icu/text/SearchIterator.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [first](https://developer.android.com/reference/android/icu/text/SearchIterator.html#first())
        pub fn first<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC | FINAL, .name == "first", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "first\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [following](https://developer.android.com/reference/android/icu/text/SearchIterator.html#following(int))
        pub fn following<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC | FINAL, .name == "following", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "following\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [last](https://developer.android.com/reference/android/icu/text/SearchIterator.html#last())
        pub fn last<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC | FINAL, .name == "last", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "last\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [preceding](https://developer.android.com/reference/android/icu/text/SearchIterator.html#preceding(int))
        pub fn preceding<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC | FINAL, .name == "preceding", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "preceding\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [setMatchLength](https://developer.android.com/reference/android/icu/text/SearchIterator.html#setMatchLength(int))
        // fn setMatchLength<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/SearchIterator", java.flags == PROTECTED, .name == "setMatchLength", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "setMatchLength\0", "(I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [handleNext](https://developer.android.com/reference/android/icu/text/SearchIterator.html#handleNext(int))
        // fn handleNext<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/SearchIterator", java.flags == PROTECTED | ABSTRACT, .name == "handleNext", .descriptor == "(I)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "handleNext\0", "(I)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [handlePrevious](https://developer.android.com/reference/android/icu/text/SearchIterator.html#handlePrevious(int))
        // fn handlePrevious<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/SearchIterator", java.flags == PROTECTED | ABSTRACT, .name == "handlePrevious", .descriptor == "(I)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "handlePrevious\0", "(I)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [setElementComparisonType](https://developer.android.com/reference/android/icu/text/SearchIterator.html#setElementComparisonType(android.icu.text.SearchIterator.ElementComparisonType))
        ///
        /// Required features: "android-icu-text-SearchIterator_ElementComparisonType"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-SearchIterator_ElementComparisonType")))]
        pub fn setElementComparisonType<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::SearchIterator_ElementComparisonType>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "setElementComparisonType", .descriptor == "(Landroid/icu/text/SearchIterator$ElementComparisonType;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "setElementComparisonType\0", "(Landroid/icu/text/SearchIterator$ElementComparisonType;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getElementComparisonType](https://developer.android.com/reference/android/icu/text/SearchIterator.html#getElementComparisonType())
        ///
        /// Required features: "android-icu-text-SearchIterator_ElementComparisonType"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-SearchIterator_ElementComparisonType")))]
        pub fn getElementComparisonType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator_ElementComparisonType>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator", java.flags == PUBLIC, .name == "getElementComparisonType", .descriptor == "()Landroid/icu/text/SearchIterator$ElementComparisonType;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator\0", "getElementComparisonType\0", "()Landroid/icu/text/SearchIterator$ElementComparisonType;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DONE](https://developer.android.com/reference/android/icu/text/SearchIterator.html#DONE)
        pub const DONE : i32 = -1;

        // // Not emitting: Non-public field
        // /// **get** protected [breakIterator](https://developer.android.com/reference/android/icu/text/SearchIterator.html#breakIterator)
        // ///
        // /// Required feature: android-icu-text-BreakIterator
        // #[cfg(any(feature = "all", feature = "android-icu-text-BreakIterator"))]
        // pub fn breakIterator<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::BreakIterator>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/icu/text/SearchIterator\0", "breakIterator\0", "Landroid/icu/text/BreakIterator;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [breakIterator](https://developer.android.com/reference/android/icu/text/SearchIterator.html#breakIterator)
        // ///
        // /// Required feature: android-icu-text-BreakIterator
        // #[cfg(any(feature = "all", feature = "android-icu-text-BreakIterator"))]
        // pub fn set_breakIterator<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::icu::text::BreakIterator>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/icu/text/SearchIterator\0", "breakIterator\0", "Landroid/icu/text/BreakIterator;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [matchLength](https://developer.android.com/reference/android/icu/text/SearchIterator.html#matchLength)
        // pub fn matchLength<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/icu/text/SearchIterator\0", "matchLength\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [matchLength](https://developer.android.com/reference/android/icu/text/SearchIterator.html#matchLength)
        // pub fn set_matchLength<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/icu/text/SearchIterator\0", "matchLength\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [targetText](https://developer.android.com/reference/android/icu/text/SearchIterator.html#targetText)
        // ///
        // /// Required feature: java-text-CharacterIterator
        // #[cfg(any(feature = "all", feature = "java-text-CharacterIterator"))]
        // pub fn targetText<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::CharacterIterator>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/icu/text/SearchIterator\0", "targetText\0", "Ljava/text/CharacterIterator;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [targetText](https://developer.android.com/reference/android/icu/text/SearchIterator.html#targetText)
        // ///
        // /// Required feature: java-text-CharacterIterator
        // #[cfg(any(feature = "all", feature = "java-text-CharacterIterator"))]
        // pub fn set_targetText<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::text::CharacterIterator>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/icu/text/SearchIterator\0", "targetText\0", "Ljava/text/CharacterIterator;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }
    }
}
