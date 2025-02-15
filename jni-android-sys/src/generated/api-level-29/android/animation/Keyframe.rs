// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-animation-Keyframe"))]
__jni_bindgen! {
    /// public class [Keyframe](https://developer.android.com/reference/android/animation/Keyframe.html)
    ///
    /// Required feature: android-animation-Keyframe
    public class Keyframe ("android/animation/Keyframe") extends crate::java::lang::Object, implements crate::java::lang::Cloneable {

        /// [Keyframe](https://developer.android.com/reference/android/animation/Keyframe.html#Keyframe())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofInt](https://developer.android.com/reference/android/animation/Keyframe.html#ofInt(float,%20int))
        ///
        /// Required features: "android-animation-Keyframe"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe")))]
        pub fn ofInt_float_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | STATIC, .name == "ofInt", .descriptor == "(FI)Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/animation/Keyframe\0", "ofInt\0", "(FI)Landroid/animation/Keyframe;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofInt](https://developer.android.com/reference/android/animation/Keyframe.html#ofInt(float))
        ///
        /// Required features: "android-animation-Keyframe"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe")))]
        pub fn ofInt_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | STATIC, .name == "ofInt", .descriptor == "(F)Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/animation/Keyframe\0", "ofInt\0", "(F)Landroid/animation/Keyframe;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofFloat](https://developer.android.com/reference/android/animation/Keyframe.html#ofFloat(float,%20float))
        ///
        /// Required features: "android-animation-Keyframe"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe")))]
        pub fn ofFloat_float_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | STATIC, .name == "ofFloat", .descriptor == "(FF)Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/animation/Keyframe\0", "ofFloat\0", "(FF)Landroid/animation/Keyframe;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofFloat](https://developer.android.com/reference/android/animation/Keyframe.html#ofFloat(float))
        ///
        /// Required features: "android-animation-Keyframe"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe")))]
        pub fn ofFloat_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | STATIC, .name == "ofFloat", .descriptor == "(F)Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/animation/Keyframe\0", "ofFloat\0", "(F)Landroid/animation/Keyframe;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofObject](https://developer.android.com/reference/android/animation/Keyframe.html#ofObject(float,%20java.lang.Object))
        ///
        /// Required features: "android-animation-Keyframe", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe", feature = "java-lang-Object")))]
        pub fn ofObject_float_Object<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | STATIC, .name == "ofObject", .descriptor == "(FLjava/lang/Object;)Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/animation/Keyframe\0", "ofObject\0", "(FLjava/lang/Object;)Landroid/animation/Keyframe;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ofObject](https://developer.android.com/reference/android/animation/Keyframe.html#ofObject(float))
        ///
        /// Required features: "android-animation-Keyframe"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe")))]
        pub fn ofObject_float<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | STATIC, .name == "ofObject", .descriptor == "(F)Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/animation/Keyframe\0", "ofObject\0", "(F)Landroid/animation/Keyframe;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasValue](https://developer.android.com/reference/android/animation/Keyframe.html#hasValue())
        pub fn hasValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "hasValue", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "hasValue\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getValue](https://developer.android.com/reference/android/animation/Keyframe.html#getValue())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | ABSTRACT, .name == "getValue", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "getValue\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setValue](https://developer.android.com/reference/android/animation/Keyframe.html#setValue(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setValue<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | ABSTRACT, .name == "setValue", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "setValue\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFraction](https://developer.android.com/reference/android/animation/Keyframe.html#getFraction())
        pub fn getFraction<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "getFraction", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "getFraction\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFraction](https://developer.android.com/reference/android/animation/Keyframe.html#setFraction(float))
        pub fn setFraction<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "setFraction", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "setFraction\0", "(F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInterpolator](https://developer.android.com/reference/android/animation/Keyframe.html#getInterpolator())
        ///
        /// Required features: "android-animation-TimeInterpolator"
        #[cfg(any(feature = "all", all(feature = "android-animation-TimeInterpolator")))]
        pub fn getInterpolator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::TimeInterpolator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "getInterpolator", .descriptor == "()Landroid/animation/TimeInterpolator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "getInterpolator\0", "()Landroid/animation/TimeInterpolator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInterpolator](https://developer.android.com/reference/android/animation/Keyframe.html#setInterpolator(android.animation.TimeInterpolator))
        ///
        /// Required features: "android-animation-TimeInterpolator"
        #[cfg(any(feature = "all", all(feature = "android-animation-TimeInterpolator")))]
        pub fn setInterpolator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::animation::TimeInterpolator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "setInterpolator", .descriptor == "(Landroid/animation/TimeInterpolator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "setInterpolator\0", "(Landroid/animation/TimeInterpolator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getType](https://developer.android.com/reference/android/animation/Keyframe.html#getType())
        ///
        /// Required features: "java-lang-Class"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class")))]
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Class>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC, .name == "getType", .descriptor == "()Ljava/lang/Class;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "getType\0", "()Ljava/lang/Class;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/android/animation/Keyframe.html#clone())
        ///
        /// Required features: "android-animation-Keyframe"
        #[cfg(any(feature = "all", all(feature = "android-animation-Keyframe")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Keyframe>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/Keyframe", java.flags == PUBLIC | ABSTRACT, .name == "clone", .descriptor == "()Landroid/animation/Keyframe;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "clone\0", "()Landroid/animation/Keyframe;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [clone](https://developer.android.com/reference/android/animation/Keyframe.html#clone())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/animation/Keyframe", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/Keyframe\0", "clone\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
