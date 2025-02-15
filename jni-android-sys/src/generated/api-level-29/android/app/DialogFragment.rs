// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-DialogFragment"))]
__jni_bindgen! {
    /// public class [DialogFragment](https://developer.android.com/reference/android/app/DialogFragment.html)
    ///
    /// Required feature: android-app-DialogFragment
    #[deprecated] public class DialogFragment ("android/app/DialogFragment") extends crate::android::app::Fragment, implements crate::android::content::DialogInterface_OnCancelListener, crate::android::content::DialogInterface_OnDismissListener {

        /// [DialogFragment](https://developer.android.com/reference/android/app/DialogFragment.html#DialogFragment())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::DialogFragment>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setStyle](https://developer.android.com/reference/android/app/DialogFragment.html#setStyle(int,%20int))
        #[deprecated] pub fn setStyle<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "setStyle", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "setStyle\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/DialogFragment.html#show(android.app.FragmentManager,%20java.lang.String))
        ///
        /// Required features: "android-app-FragmentManager", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-FragmentManager", feature = "java-lang-String")))]
        #[deprecated] pub fn show_FragmentManager_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::FragmentManager>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "show", .descriptor == "(Landroid/app/FragmentManager;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "show\0", "(Landroid/app/FragmentManager;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [show](https://developer.android.com/reference/android/app/DialogFragment.html#show(android.app.FragmentTransaction,%20java.lang.String))
        ///
        /// Required features: "android-app-FragmentTransaction", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-FragmentTransaction", feature = "java-lang-String")))]
        #[deprecated] pub fn show_FragmentTransaction_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::FragmentTransaction>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "show", .descriptor == "(Landroid/app/FragmentTransaction;Ljava/lang/String;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "show\0", "(Landroid/app/FragmentTransaction;Ljava/lang/String;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dismiss](https://developer.android.com/reference/android/app/DialogFragment.html#dismiss())
        #[deprecated] pub fn dismiss<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "dismiss", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "dismiss\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dismissAllowingStateLoss](https://developer.android.com/reference/android/app/DialogFragment.html#dismissAllowingStateLoss())
        #[deprecated] pub fn dismissAllowingStateLoss<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "dismissAllowingStateLoss", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "dismissAllowingStateLoss\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDialog](https://developer.android.com/reference/android/app/DialogFragment.html#getDialog())
        ///
        /// Required features: "android-app-Dialog"
        #[cfg(any(feature = "all", all(feature = "android-app-Dialog")))]
        #[deprecated] pub fn getDialog<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Dialog>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "getDialog", .descriptor == "()Landroid/app/Dialog;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "getDialog\0", "()Landroid/app/Dialog;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTheme](https://developer.android.com/reference/android/app/DialogFragment.html#getTheme())
        #[deprecated] pub fn getTheme<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "getTheme", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "getTheme\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCancelable](https://developer.android.com/reference/android/app/DialogFragment.html#setCancelable(boolean))
        #[deprecated] pub fn setCancelable<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "setCancelable", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "setCancelable\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isCancelable](https://developer.android.com/reference/android/app/DialogFragment.html#isCancelable())
        #[deprecated] pub fn isCancelable<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "isCancelable", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "isCancelable\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setShowsDialog](https://developer.android.com/reference/android/app/DialogFragment.html#setShowsDialog(boolean))
        #[deprecated] pub fn setShowsDialog<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "setShowsDialog", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "setShowsDialog\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getShowsDialog](https://developer.android.com/reference/android/app/DialogFragment.html#getShowsDialog())
        #[deprecated] pub fn getShowsDialog<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "getShowsDialog", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "getShowsDialog\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAttach](https://developer.android.com/reference/android/app/DialogFragment.html#onAttach(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        #[deprecated] pub fn onAttach<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onAttach", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onAttach\0", "(Landroid/content/Context;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDetach](https://developer.android.com/reference/android/app/DialogFragment.html#onDetach())
        #[deprecated] pub fn onDetach<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onDetach", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onDetach\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreate](https://developer.android.com/reference/android/app/DialogFragment.html#onCreate(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        #[deprecated] pub fn onCreate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onCreate", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onCreate\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGetLayoutInflater](https://developer.android.com/reference/android/app/DialogFragment.html#onGetLayoutInflater(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-view-LayoutInflater"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-view-LayoutInflater")))]
        pub fn onGetLayoutInflater<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onGetLayoutInflater", .descriptor == "(Landroid/os/Bundle;)Landroid/view/LayoutInflater;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onGetLayoutInflater\0", "(Landroid/os/Bundle;)Landroid/view/LayoutInflater;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateDialog](https://developer.android.com/reference/android/app/DialogFragment.html#onCreateDialog(android.os.Bundle))
        ///
        /// Required features: "android-app-Dialog", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-app-Dialog", feature = "android-os-Bundle")))]
        #[deprecated] pub fn onCreateDialog<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Dialog>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onCreateDialog", .descriptor == "(Landroid/os/Bundle;)Landroid/app/Dialog;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onCreateDialog\0", "(Landroid/os/Bundle;)Landroid/app/Dialog;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCancel](https://developer.android.com/reference/android/app/DialogFragment.html#onCancel(android.content.DialogInterface))
        ///
        /// Required features: "android-content-DialogInterface"
        #[cfg(any(feature = "all", all(feature = "android-content-DialogInterface")))]
        #[deprecated] pub fn onCancel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::DialogInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onCancel", .descriptor == "(Landroid/content/DialogInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onCancel\0", "(Landroid/content/DialogInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDismiss](https://developer.android.com/reference/android/app/DialogFragment.html#onDismiss(android.content.DialogInterface))
        ///
        /// Required features: "android-content-DialogInterface"
        #[cfg(any(feature = "all", all(feature = "android-content-DialogInterface")))]
        #[deprecated] pub fn onDismiss<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::DialogInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onDismiss", .descriptor == "(Landroid/content/DialogInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onDismiss\0", "(Landroid/content/DialogInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onActivityCreated](https://developer.android.com/reference/android/app/DialogFragment.html#onActivityCreated(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        #[deprecated] pub fn onActivityCreated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onActivityCreated", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onActivityCreated\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStart](https://developer.android.com/reference/android/app/DialogFragment.html#onStart())
        #[deprecated] pub fn onStart<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onStart", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onStart\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSaveInstanceState](https://developer.android.com/reference/android/app/DialogFragment.html#onSaveInstanceState(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        #[deprecated] pub fn onSaveInstanceState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onSaveInstanceState", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onSaveInstanceState\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStop](https://developer.android.com/reference/android/app/DialogFragment.html#onStop())
        #[deprecated] pub fn onStop<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onStop", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onStop\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDestroyView](https://developer.android.com/reference/android/app/DialogFragment.html#onDestroyView())
        #[deprecated] pub fn onDestroyView<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "onDestroyView", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "onDestroyView\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dump](https://developer.android.com/reference/android/app/DialogFragment.html#dump(java.lang.String,%20java.io.FileDescriptor,%20java.io.PrintWriter,%20java.lang.String%5B%5D))
        ///
        /// Required features: "java-io-FileDescriptor", "java-io-PrintWriter", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor", feature = "java-io-PrintWriter", feature = "java-lang-String")))]
        #[deprecated] pub fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PrintWriter>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DialogFragment", java.flags == PUBLIC, .name == "dump", .descriptor == "(Ljava/lang/String;Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DialogFragment\0", "dump\0", "(Ljava/lang/String;Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [STYLE_NORMAL](https://developer.android.com/reference/android/app/DialogFragment.html#STYLE_NORMAL)
        #[deprecated] pub const STYLE_NORMAL : i32 = 0;

        /// public static final [STYLE_NO_FRAME](https://developer.android.com/reference/android/app/DialogFragment.html#STYLE_NO_FRAME)
        #[deprecated] pub const STYLE_NO_FRAME : i32 = 2;

        /// public static final [STYLE_NO_INPUT](https://developer.android.com/reference/android/app/DialogFragment.html#STYLE_NO_INPUT)
        #[deprecated] pub const STYLE_NO_INPUT : i32 = 3;

        /// public static final [STYLE_NO_TITLE](https://developer.android.com/reference/android/app/DialogFragment.html#STYLE_NO_TITLE)
        #[deprecated] pub const STYLE_NO_TITLE : i32 = 1;
    }
}
