// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-ScaleGestureDetector"))]
__jni_bindgen! {
    /// public class [ScaleGestureDetector](https://developer.android.com/reference/android/view/ScaleGestureDetector.html)
    ///
    /// Required feature: android-view-ScaleGestureDetector
    public class ScaleGestureDetector ("android/view/ScaleGestureDetector") extends crate::java::lang::Object {

        /// [ScaleGestureDetector](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#ScaleGestureDetector(android.content.Context,%20android.view.ScaleGestureDetector.OnScaleGestureListener))
        ///
        /// Required features: "android-content-Context", "android-view-ScaleGestureDetector_OnScaleGestureListener"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-ScaleGestureDetector_OnScaleGestureListener")))]
        pub fn new_Context_OnScaleGestureListener<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ScaleGestureDetector_OnScaleGestureListener>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::ScaleGestureDetector>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/view/ScaleGestureDetector$OnScaleGestureListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "<init>\0", "(Landroid/content/Context;Landroid/view/ScaleGestureDetector$OnScaleGestureListener;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ScaleGestureDetector](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#ScaleGestureDetector(android.content.Context,%20android.view.ScaleGestureDetector.OnScaleGestureListener,%20android.os.Handler))
        ///
        /// Required features: "android-content-Context", "android-os-Handler", "android-view-ScaleGestureDetector_OnScaleGestureListener"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-os-Handler", feature = "android-view-ScaleGestureDetector_OnScaleGestureListener")))]
        pub fn new_Context_OnScaleGestureListener_Handler<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ScaleGestureDetector_OnScaleGestureListener>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::ScaleGestureDetector>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/view/ScaleGestureDetector$OnScaleGestureListener;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "<init>\0", "(Landroid/content/Context;Landroid/view/ScaleGestureDetector$OnScaleGestureListener;Landroid/os/Handler;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTouchEvent](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#onTouchEvent(android.view.MotionEvent))
        ///
        /// Required features: "android-view-MotionEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent")))]
        pub fn onTouchEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "onTouchEvent", .descriptor == "(Landroid/view/MotionEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "onTouchEvent\0", "(Landroid/view/MotionEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setQuickScaleEnabled](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#setQuickScaleEnabled(boolean))
        pub fn setQuickScaleEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "setQuickScaleEnabled", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "setQuickScaleEnabled\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isQuickScaleEnabled](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#isQuickScaleEnabled())
        pub fn isQuickScaleEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "isQuickScaleEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "isQuickScaleEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setStylusScaleEnabled](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#setStylusScaleEnabled(boolean))
        pub fn setStylusScaleEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "setStylusScaleEnabled", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "setStylusScaleEnabled\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isStylusScaleEnabled](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#isStylusScaleEnabled())
        pub fn isStylusScaleEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "isStylusScaleEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "isStylusScaleEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isInProgress](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#isInProgress())
        pub fn isInProgress<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "isInProgress", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "isInProgress\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFocusX](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getFocusX())
        pub fn getFocusX<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getFocusX", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getFocusX\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFocusY](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getFocusY())
        pub fn getFocusY<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getFocusY", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getFocusY\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentSpan](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getCurrentSpan())
        pub fn getCurrentSpan<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getCurrentSpan", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getCurrentSpan\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentSpanX](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getCurrentSpanX())
        pub fn getCurrentSpanX<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getCurrentSpanX", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getCurrentSpanX\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentSpanY](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getCurrentSpanY())
        pub fn getCurrentSpanY<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getCurrentSpanY", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getCurrentSpanY\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPreviousSpan](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getPreviousSpan())
        pub fn getPreviousSpan<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getPreviousSpan", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getPreviousSpan\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPreviousSpanX](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getPreviousSpanX())
        pub fn getPreviousSpanX<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getPreviousSpanX", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getPreviousSpanX\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPreviousSpanY](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getPreviousSpanY())
        pub fn getPreviousSpanY<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getPreviousSpanY", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getPreviousSpanY\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getScaleFactor](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getScaleFactor())
        pub fn getScaleFactor<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getScaleFactor", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getScaleFactor\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeDelta](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getTimeDelta())
        pub fn getTimeDelta<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getTimeDelta", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getTimeDelta\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEventTime](https://developer.android.com/reference/android/view/ScaleGestureDetector.html#getEventTime())
        pub fn getEventTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ScaleGestureDetector", java.flags == PUBLIC, .name == "getEventTime", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ScaleGestureDetector\0", "getEventTime\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
