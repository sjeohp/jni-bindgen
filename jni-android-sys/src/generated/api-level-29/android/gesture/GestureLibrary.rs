// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-gesture-GestureLibrary"))]
__jni_bindgen! {
    /// public class [GestureLibrary](https://developer.android.com/reference/android/gesture/GestureLibrary.html)
    ///
    /// Required feature: android-gesture-GestureLibrary
    public class GestureLibrary ("android/gesture/GestureLibrary") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [GestureLibrary](https://developer.android.com/reference/android/gesture/GestureLibrary.html#GestureLibrary())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::gesture::GestureLibrary>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/gesture/GestureLibrary", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [save](https://developer.android.com/reference/android/gesture/GestureLibrary.html#save())
        pub fn save<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC | ABSTRACT, .name == "save", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "save\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [load](https://developer.android.com/reference/android/gesture/GestureLibrary.html#load())
        pub fn load<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC | ABSTRACT, .name == "load", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "load\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isReadOnly](https://developer.android.com/reference/android/gesture/GestureLibrary.html#isReadOnly())
        pub fn isReadOnly<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "isReadOnly", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "isReadOnly\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOrientationStyle](https://developer.android.com/reference/android/gesture/GestureLibrary.html#setOrientationStyle(int))
        pub fn setOrientationStyle<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "setOrientationStyle", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "setOrientationStyle\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOrientationStyle](https://developer.android.com/reference/android/gesture/GestureLibrary.html#getOrientationStyle())
        pub fn getOrientationStyle<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "getOrientationStyle", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "getOrientationStyle\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSequenceType](https://developer.android.com/reference/android/gesture/GestureLibrary.html#setSequenceType(int))
        pub fn setSequenceType<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "setSequenceType", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "setSequenceType\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSequenceType](https://developer.android.com/reference/android/gesture/GestureLibrary.html#getSequenceType())
        pub fn getSequenceType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "getSequenceType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "getSequenceType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGestureEntries](https://developer.android.com/reference/android/gesture/GestureLibrary.html#getGestureEntries())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getGestureEntries<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "getGestureEntries", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "getGestureEntries\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [recognize](https://developer.android.com/reference/android/gesture/GestureLibrary.html#recognize(android.gesture.Gesture))
        ///
        /// Required features: "android-gesture-Gesture", "java-util-ArrayList"
        #[cfg(any(feature = "all", all(feature = "android-gesture-Gesture", feature = "java-util-ArrayList")))]
        pub fn recognize<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::gesture::Gesture>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ArrayList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "recognize", .descriptor == "(Landroid/gesture/Gesture;)Ljava/util/ArrayList;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "recognize\0", "(Landroid/gesture/Gesture;)Ljava/util/ArrayList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addGesture](https://developer.android.com/reference/android/gesture/GestureLibrary.html#addGesture(java.lang.String,%20android.gesture.Gesture))
        ///
        /// Required features: "android-gesture-Gesture", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-gesture-Gesture", feature = "java-lang-String")))]
        pub fn addGesture<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::gesture::Gesture>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "addGesture", .descriptor == "(Ljava/lang/String;Landroid/gesture/Gesture;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "addGesture\0", "(Ljava/lang/String;Landroid/gesture/Gesture;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeGesture](https://developer.android.com/reference/android/gesture/GestureLibrary.html#removeGesture(java.lang.String,%20android.gesture.Gesture))
        ///
        /// Required features: "android-gesture-Gesture", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-gesture-Gesture", feature = "java-lang-String")))]
        pub fn removeGesture<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::gesture::Gesture>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "removeGesture", .descriptor == "(Ljava/lang/String;Landroid/gesture/Gesture;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "removeGesture\0", "(Ljava/lang/String;Landroid/gesture/Gesture;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeEntry](https://developer.android.com/reference/android/gesture/GestureLibrary.html#removeEntry(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn removeEntry<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "removeEntry", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "removeEntry\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGestures](https://developer.android.com/reference/android/gesture/GestureLibrary.html#getGestures(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-ArrayList"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-ArrayList")))]
        pub fn getGestures<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ArrayList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureLibrary", java.flags == PUBLIC, .name == "getGestures", .descriptor == "(Ljava/lang/String;)Ljava/util/ArrayList;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureLibrary\0", "getGestures\0", "(Ljava/lang/String;)Ljava/util/ArrayList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// **get** protected final [mStore](https://developer.android.com/reference/android/gesture/GestureLibrary.html#mStore)
        // ///
        // /// Required feature: android-gesture-GestureStore
        // #[cfg(any(feature = "all", feature = "android-gesture-GestureStore"))]
        // pub fn mStore<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::gesture::GestureStore>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/gesture/GestureLibrary\0", "mStore\0", "Landroid/gesture/GestureStore;\0");
        //         env.get_object_field(class, field)
        //     }
        // }
    }
}
