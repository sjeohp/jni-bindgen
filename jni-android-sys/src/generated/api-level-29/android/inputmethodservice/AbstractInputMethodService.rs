// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-inputmethodservice-AbstractInputMethodService"))]
__jni_bindgen! {
    /// public class [AbstractInputMethodService](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html)
    ///
    /// Required feature: android-inputmethodservice-AbstractInputMethodService
    public class AbstractInputMethodService ("android/inputmethodservice/AbstractInputMethodService") extends crate::android::app::Service, implements crate::android::view::KeyEvent_Callback {

        /// [AbstractInputMethodService](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#AbstractInputMethodService())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::inputmethodservice::AbstractInputMethodService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyDispatcherState](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#getKeyDispatcherState())
        ///
        /// Required features: "android-view-KeyEvent_DispatcherState"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent_DispatcherState")))]
        pub fn getKeyDispatcherState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::KeyEvent_DispatcherState>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC, .name == "getKeyDispatcherState", .descriptor == "()Landroid/view/KeyEvent$DispatcherState;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "getKeyDispatcherState\0", "()Landroid/view/KeyEvent$DispatcherState;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateInputMethodInterface](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#onCreateInputMethodInterface())
        ///
        /// Required features: "android-inputmethodservice-AbstractInputMethodService_AbstractInputMethodImpl"
        #[cfg(any(feature = "all", all(feature = "android-inputmethodservice-AbstractInputMethodService_AbstractInputMethodImpl")))]
        pub fn onCreateInputMethodInterface<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::inputmethodservice::AbstractInputMethodService_AbstractInputMethodImpl>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC | ABSTRACT, .name == "onCreateInputMethodInterface", .descriptor == "()Landroid/inputmethodservice/AbstractInputMethodService$AbstractInputMethodImpl;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "onCreateInputMethodInterface\0", "()Landroid/inputmethodservice/AbstractInputMethodService$AbstractInputMethodImpl;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateInputMethodSessionInterface](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#onCreateInputMethodSessionInterface())
        ///
        /// Required features: "android-inputmethodservice-AbstractInputMethodService_AbstractInputMethodSessionImpl"
        #[cfg(any(feature = "all", all(feature = "android-inputmethodservice-AbstractInputMethodService_AbstractInputMethodSessionImpl")))]
        pub fn onCreateInputMethodSessionInterface<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::inputmethodservice::AbstractInputMethodService_AbstractInputMethodSessionImpl>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC | ABSTRACT, .name == "onCreateInputMethodSessionInterface", .descriptor == "()Landroid/inputmethodservice/AbstractInputMethodService$AbstractInputMethodSessionImpl;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "onCreateInputMethodSessionInterface\0", "()Landroid/inputmethodservice/AbstractInputMethodService$AbstractInputMethodSessionImpl;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [dump](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#dump(java.io.FileDescriptor,%20java.io.PrintWriter,%20java.lang.String%5B%5D))
        // ///
        // /// Required features: "java-io-FileDescriptor", "java-io-PrintWriter", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor", feature = "java-io-PrintWriter", feature = "java-lang-String")))]
        // fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PrintWriter>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PROTECTED, .name == "dump", .descriptor == "(Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "dump\0", "(Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [onBind](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC | FINAL, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTrackballEvent](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#onTrackballEvent(android.view.MotionEvent))
        ///
        /// Required features: "android-view-MotionEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent")))]
        pub fn onTrackballEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC, .name == "onTrackballEvent", .descriptor == "(Landroid/view/MotionEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "onTrackballEvent\0", "(Landroid/view/MotionEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGenericMotionEvent](https://developer.android.com/reference/android/inputmethodservice/AbstractInputMethodService.html#onGenericMotionEvent(android.view.MotionEvent))
        ///
        /// Required features: "android-view-MotionEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent")))]
        pub fn onGenericMotionEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/AbstractInputMethodService", java.flags == PUBLIC, .name == "onGenericMotionEvent", .descriptor == "(Landroid/view/MotionEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/AbstractInputMethodService\0", "onGenericMotionEvent\0", "(Landroid/view/MotionEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
