// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-animation-AnimatorSet"))]
__jni_bindgen! {
    /// public final class [AnimatorSet](https://developer.android.com/reference/android/animation/AnimatorSet.html)
    ///
    /// Required feature: android-animation-AnimatorSet
    public final class AnimatorSet ("android/animation/AnimatorSet") extends crate::android::animation::Animator {

        /// [AnimatorSet](https://developer.android.com/reference/android/animation/AnimatorSet.html#AnimatorSet())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::animation::AnimatorSet>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [playTogether](https://developer.android.com/reference/android/animation/AnimatorSet.html#playTogether(android.animation.Animator...))
        ///
        /// Required features: "android-animation-Animator"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator")))]
        pub fn playTogether_Animator_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::animation::Animator, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC | VARARGS, .name == "playTogether", .descriptor == "([Landroid/animation/Animator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "playTogether\0", "([Landroid/animation/Animator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [playTogether](https://developer.android.com/reference/android/animation/AnimatorSet.html#playTogether(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn playTogether_Collection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "playTogether", .descriptor == "(Ljava/util/Collection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "playTogether\0", "(Ljava/util/Collection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [playSequentially](https://developer.android.com/reference/android/animation/AnimatorSet.html#playSequentially(android.animation.Animator...))
        ///
        /// Required features: "android-animation-Animator"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator")))]
        pub fn playSequentially_Animator_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::animation::Animator, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC | VARARGS, .name == "playSequentially", .descriptor == "([Landroid/animation/Animator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "playSequentially\0", "([Landroid/animation/Animator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [playSequentially](https://developer.android.com/reference/android/animation/AnimatorSet.html#playSequentially(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn playSequentially_List<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "playSequentially", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "playSequentially\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChildAnimations](https://developer.android.com/reference/android/animation/AnimatorSet.html#getChildAnimations())
        ///
        /// Required features: "java-util-ArrayList"
        #[cfg(any(feature = "all", all(feature = "java-util-ArrayList")))]
        pub fn getChildAnimations<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ArrayList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "getChildAnimations", .descriptor == "()Ljava/util/ArrayList;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "getChildAnimations\0", "()Ljava/util/ArrayList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTarget](https://developer.android.com/reference/android/animation/AnimatorSet.html#setTarget(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setTarget<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setTarget", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setTarget\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInterpolator](https://developer.android.com/reference/android/animation/AnimatorSet.html#setInterpolator(android.animation.TimeInterpolator))
        ///
        /// Required features: "android-animation-TimeInterpolator"
        #[cfg(any(feature = "all", all(feature = "android-animation-TimeInterpolator")))]
        pub fn setInterpolator<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::animation::TimeInterpolator>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setInterpolator", .descriptor == "(Landroid/animation/TimeInterpolator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setInterpolator\0", "(Landroid/animation/TimeInterpolator;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInterpolator](https://developer.android.com/reference/android/animation/AnimatorSet.html#getInterpolator())
        ///
        /// Required features: "android-animation-TimeInterpolator"
        #[cfg(any(feature = "all", all(feature = "android-animation-TimeInterpolator")))]
        pub fn getInterpolator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::TimeInterpolator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "getInterpolator", .descriptor == "()Landroid/animation/TimeInterpolator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "getInterpolator\0", "()Landroid/animation/TimeInterpolator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [play](https://developer.android.com/reference/android/animation/AnimatorSet.html#play(android.animation.Animator))
        ///
        /// Required features: "android-animation-Animator", "android-animation-AnimatorSet_Builder"
        #[cfg(any(feature = "all", all(feature = "android-animation-Animator", feature = "android-animation-AnimatorSet_Builder")))]
        pub fn play<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::animation::Animator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::AnimatorSet_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "play", .descriptor == "(Landroid/animation/Animator;)Landroid/animation/AnimatorSet$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "play\0", "(Landroid/animation/Animator;)Landroid/animation/AnimatorSet$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cancel](https://developer.android.com/reference/android/animation/AnimatorSet.html#cancel())
        pub fn cancel<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "cancel", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "cancel\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [end](https://developer.android.com/reference/android/animation/AnimatorSet.html#end())
        pub fn end<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "end", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "end\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRunning](https://developer.android.com/reference/android/animation/AnimatorSet.html#isRunning())
        pub fn isRunning<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "isRunning", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "isRunning\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isStarted](https://developer.android.com/reference/android/animation/AnimatorSet.html#isStarted())
        pub fn isStarted<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "isStarted", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "isStarted\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStartDelay](https://developer.android.com/reference/android/animation/AnimatorSet.html#getStartDelay())
        pub fn getStartDelay<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "getStartDelay", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "getStartDelay\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setStartDelay](https://developer.android.com/reference/android/animation/AnimatorSet.html#setStartDelay(long))
        pub fn setStartDelay<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setStartDelay", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setStartDelay\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDuration](https://developer.android.com/reference/android/animation/AnimatorSet.html#getDuration())
        pub fn getDuration<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "getDuration", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "getDuration\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDuration](https://developer.android.com/reference/android/animation/AnimatorSet.html#setDuration(long))
        ///
        /// Required features: "android-animation-AnimatorSet"
        #[cfg(any(feature = "all", all(feature = "android-animation-AnimatorSet")))]
        pub fn setDuration_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::AnimatorSet>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setDuration", .descriptor == "(J)Landroid/animation/AnimatorSet;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setDuration\0", "(J)Landroid/animation/AnimatorSet;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setupStartValues](https://developer.android.com/reference/android/animation/AnimatorSet.html#setupStartValues())
        pub fn setupStartValues<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setupStartValues", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setupStartValues\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setupEndValues](https://developer.android.com/reference/android/animation/AnimatorSet.html#setupEndValues())
        pub fn setupEndValues<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setupEndValues", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setupEndValues\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [pause](https://developer.android.com/reference/android/animation/AnimatorSet.html#pause())
        pub fn pause<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "pause", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "pause\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [resume](https://developer.android.com/reference/android/animation/AnimatorSet.html#resume())
        pub fn resume<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "resume", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "resume\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [start](https://developer.android.com/reference/android/animation/AnimatorSet.html#start())
        pub fn start<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "start", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "start\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCurrentPlayTime](https://developer.android.com/reference/android/animation/AnimatorSet.html#setCurrentPlayTime(long))
        pub fn setCurrentPlayTime<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "setCurrentPlayTime", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setCurrentPlayTime\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentPlayTime](https://developer.android.com/reference/android/animation/AnimatorSet.html#getCurrentPlayTime())
        pub fn getCurrentPlayTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "getCurrentPlayTime", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "getCurrentPlayTime\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/android/animation/AnimatorSet.html#clone())
        ///
        /// Required features: "android-animation-AnimatorSet"
        #[cfg(any(feature = "all", all(feature = "android-animation-AnimatorSet")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::AnimatorSet>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "clone", .descriptor == "()Landroid/animation/AnimatorSet;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "clone\0", "()Landroid/animation/AnimatorSet;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reverse](https://developer.android.com/reference/android/animation/AnimatorSet.html#reverse())
        pub fn reverse<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "reverse", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "reverse\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/animation/AnimatorSet.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTotalDuration](https://developer.android.com/reference/android/animation/AnimatorSet.html#getTotalDuration())
        pub fn getTotalDuration<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC, .name == "getTotalDuration", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "getTotalDuration\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [clone](https://developer.android.com/reference/android/animation/AnimatorSet.html#clone())
        // ///
        // /// Required features: "android-animation-Animator"
        // #[cfg(any(feature = "all", all(feature = "android-animation-Animator")))]
        // pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "clone", .descriptor == "()Landroid/animation/Animator;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "clone\0", "()Landroid/animation/Animator;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [setDuration](https://developer.android.com/reference/android/animation/AnimatorSet.html#setDuration(long))
        // ///
        // /// Required features: "android-animation-Animator"
        // #[cfg(any(feature = "all", all(feature = "android-animation-Animator")))]
        // pub fn setDuration_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::animation::Animator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "setDuration", .descriptor == "(J)Landroid/animation/Animator;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "setDuration\0", "(J)Landroid/animation/Animator;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [clone](https://developer.android.com/reference/android/animation/AnimatorSet.html#clone())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/animation/AnimatorSet", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/AnimatorSet\0", "clone\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
