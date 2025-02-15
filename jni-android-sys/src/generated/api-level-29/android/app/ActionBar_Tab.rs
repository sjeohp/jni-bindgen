// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ActionBar_Tab"))]
__jni_bindgen! {
    /// public class [ActionBar.Tab](https://developer.android.com/reference/android/app/ActionBar.Tab.html)
    ///
    /// Required feature: android-app-ActionBar_Tab
    #[deprecated] public class ActionBar_Tab ("android/app/ActionBar$Tab") extends crate::java::lang::Object {

        /// [Tab](https://developer.android.com/reference/android/app/ActionBar.Tab.html#Tab())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPosition](https://developer.android.com/reference/android/app/ActionBar.Tab.html#getPosition())
        #[deprecated] pub fn getPosition<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "getPosition", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "getPosition\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIcon](https://developer.android.com/reference/android/app/ActionBar.Tab.html#getIcon())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        #[deprecated] pub fn getIcon<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "getIcon", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "getIcon\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getText](https://developer.android.com/reference/android/app/ActionBar.Tab.html#getText())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn getText<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "getText", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "getText\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIcon](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setIcon(android.graphics.drawable.Drawable))
        ///
        /// Required features: "android-app-ActionBar_Tab", "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "android-graphics-drawable-Drawable")))]
        #[deprecated] pub fn setIcon_Drawable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Drawable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setIcon", .descriptor == "(Landroid/graphics/drawable/Drawable;)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setIcon\0", "(Landroid/graphics/drawable/Drawable;)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIcon](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setIcon(int))
        ///
        /// Required features: "android-app-ActionBar_Tab"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab")))]
        #[deprecated] pub fn setIcon_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setIcon", .descriptor == "(I)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setIcon\0", "(I)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setText](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setText(java.lang.CharSequence))
        ///
        /// Required features: "android-app-ActionBar_Tab", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn setText_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setText", .descriptor == "(Ljava/lang/CharSequence;)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setText\0", "(Ljava/lang/CharSequence;)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setText](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setText(int))
        ///
        /// Required features: "android-app-ActionBar_Tab"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab")))]
        #[deprecated] pub fn setText_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setText", .descriptor == "(I)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setText\0", "(I)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCustomView](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setCustomView(android.view.View))
        ///
        /// Required features: "android-app-ActionBar_Tab", "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "android-view-View")))]
        #[deprecated] pub fn setCustomView_View<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setCustomView", .descriptor == "(Landroid/view/View;)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setCustomView\0", "(Landroid/view/View;)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCustomView](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setCustomView(int))
        ///
        /// Required features: "android-app-ActionBar_Tab"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab")))]
        #[deprecated] pub fn setCustomView_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setCustomView", .descriptor == "(I)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setCustomView\0", "(I)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCustomView](https://developer.android.com/reference/android/app/ActionBar.Tab.html#getCustomView())
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        #[deprecated] pub fn getCustomView<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "getCustomView", .descriptor == "()Landroid/view/View;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "getCustomView\0", "()Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTag](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setTag(java.lang.Object))
        ///
        /// Required features: "android-app-ActionBar_Tab", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "java-lang-Object")))]
        #[deprecated] pub fn setTag<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setTag", .descriptor == "(Ljava/lang/Object;)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setTag\0", "(Ljava/lang/Object;)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTag](https://developer.android.com/reference/android/app/ActionBar.Tab.html#getTag())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        #[deprecated] pub fn getTag<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "getTag", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "getTag\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTabListener](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setTabListener(android.app.ActionBar.TabListener))
        ///
        /// Required features: "android-app-ActionBar_Tab", "android-app-ActionBar_TabListener"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "android-app-ActionBar_TabListener")))]
        #[deprecated] pub fn setTabListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::ActionBar_TabListener>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setTabListener", .descriptor == "(Landroid/app/ActionBar$TabListener;)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setTabListener\0", "(Landroid/app/ActionBar$TabListener;)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [select](https://developer.android.com/reference/android/app/ActionBar.Tab.html#select())
        #[deprecated] pub fn select<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "select", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "select\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setContentDescription](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setContentDescription(int))
        ///
        /// Required features: "android-app-ActionBar_Tab"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab")))]
        #[deprecated] pub fn setContentDescription_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setContentDescription", .descriptor == "(I)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setContentDescription\0", "(I)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setContentDescription](https://developer.android.com/reference/android/app/ActionBar.Tab.html#setContentDescription(java.lang.CharSequence))
        ///
        /// Required features: "android-app-ActionBar_Tab", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn setContentDescription_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::ActionBar_Tab>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "setContentDescription", .descriptor == "(Ljava/lang/CharSequence;)Landroid/app/ActionBar$Tab;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "setContentDescription\0", "(Ljava/lang/CharSequence;)Landroid/app/ActionBar$Tab;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getContentDescription](https://developer.android.com/reference/android/app/ActionBar.Tab.html#getContentDescription())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn getContentDescription<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$Tab", java.flags == PUBLIC | ABSTRACT, .name == "getContentDescription", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$Tab\0", "getContentDescription\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [INVALID_POSITION](https://developer.android.com/reference/android/app/ActionBar.Tab.html#INVALID_POSITION)
        #[deprecated] pub const INVALID_POSITION : i32 = -1;
    }
}
