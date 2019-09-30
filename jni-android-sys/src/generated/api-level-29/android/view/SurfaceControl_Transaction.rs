// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-SurfaceControl_Transaction"))]
__jni_bindgen! {
    /// public class [SurfaceControl.Transaction](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html)
    ///
    /// Required feature: android-view-SurfaceControl_Transaction
    public class SurfaceControl_Transaction ("android/view/SurfaceControl$Transaction") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        /// [Transaction](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#Transaction())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#apply())
        pub fn apply<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "apply", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "apply\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setVisibility](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#setVisibility(android.view.SurfaceControl,%20boolean))
        ///
        /// Required features: "android-view-SurfaceControl", "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceControl", feature = "android-view-SurfaceControl_Transaction")))]
        pub fn setVisibility<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "setVisibility", .descriptor == "(Landroid/view/SurfaceControl;Z)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "setVisibility\0", "(Landroid/view/SurfaceControl;Z)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBufferSize](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#setBufferSize(android.view.SurfaceControl,%20int,%20int))
        ///
        /// Required features: "android-view-SurfaceControl", "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceControl", feature = "android-view-SurfaceControl_Transaction")))]
        pub fn setBufferSize<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "setBufferSize", .descriptor == "(Landroid/view/SurfaceControl;II)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "setBufferSize\0", "(Landroid/view/SurfaceControl;II)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLayer](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#setLayer(android.view.SurfaceControl,%20int))
        ///
        /// Required features: "android-view-SurfaceControl", "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceControl", feature = "android-view-SurfaceControl_Transaction")))]
        pub fn setLayer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "setLayer", .descriptor == "(Landroid/view/SurfaceControl;I)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "setLayer\0", "(Landroid/view/SurfaceControl;I)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAlpha](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#setAlpha(android.view.SurfaceControl,%20float))
        ///
        /// Required features: "android-view-SurfaceControl", "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceControl", feature = "android-view-SurfaceControl_Transaction")))]
        pub fn setAlpha<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>, arg1: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "setAlpha", .descriptor == "(Landroid/view/SurfaceControl;F)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "setAlpha\0", "(Landroid/view/SurfaceControl;F)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setGeometry](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#setGeometry(android.view.SurfaceControl,%20android.graphics.Rect,%20android.graphics.Rect,%20int))
        ///
        /// Required features: "android-graphics-Rect", "android-view-SurfaceControl", "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect", feature = "android-view-SurfaceControl", feature = "android-view-SurfaceControl_Transaction")))]
        pub fn setGeometry<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "setGeometry", .descriptor == "(Landroid/view/SurfaceControl;Landroid/graphics/Rect;Landroid/graphics/Rect;I)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "setGeometry\0", "(Landroid/view/SurfaceControl;Landroid/graphics/Rect;Landroid/graphics/Rect;I)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reparent](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#reparent(android.view.SurfaceControl,%20android.view.SurfaceControl))
        ///
        /// Required features: "android-view-SurfaceControl", "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceControl", feature = "android-view-SurfaceControl_Transaction")))]
        pub fn reparent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "reparent", .descriptor == "(Landroid/view/SurfaceControl;Landroid/view/SurfaceControl;)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "reparent\0", "(Landroid/view/SurfaceControl;Landroid/view/SurfaceControl;)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [merge](https://developer.android.com/reference/android/view/SurfaceControl.Transaction.html#merge(android.view.SurfaceControl.Transaction))
        ///
        /// Required features: "android-view-SurfaceControl_Transaction"
        #[cfg(any(feature = "all", all(feature = "android-view-SurfaceControl_Transaction")))]
        pub fn merge<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::SurfaceControl_Transaction>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::SurfaceControl_Transaction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/SurfaceControl$Transaction", java.flags == PUBLIC, .name == "merge", .descriptor == "(Landroid/view/SurfaceControl$Transaction;)Landroid/view/SurfaceControl$Transaction;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/SurfaceControl$Transaction\0", "merge\0", "(Landroid/view/SurfaceControl$Transaction;)Landroid/view/SurfaceControl$Transaction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
