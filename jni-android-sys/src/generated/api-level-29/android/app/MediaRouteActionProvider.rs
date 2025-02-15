// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-MediaRouteActionProvider"))]
__jni_bindgen! {
    /// public class [MediaRouteActionProvider](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html)
    ///
    /// Required feature: android-app-MediaRouteActionProvider
    public class MediaRouteActionProvider ("android/app/MediaRouteActionProvider") extends crate::android::view::ActionProvider {

        /// [MediaRouteActionProvider](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#MediaRouteActionProvider(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::MediaRouteActionProvider>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRouteTypes](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#setRouteTypes(int))
        pub fn setRouteTypes<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "setRouteTypes", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "setRouteTypes\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setExtendedSettingsClickListener](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#setExtendedSettingsClickListener(android.view.View.OnClickListener))
        ///
        /// Required features: "android-view-View_OnClickListener"
        #[cfg(any(feature = "all", all(feature = "android-view-View_OnClickListener")))]
        pub fn setExtendedSettingsClickListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View_OnClickListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "setExtendedSettingsClickListener", .descriptor == "(Landroid/view/View$OnClickListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "setExtendedSettingsClickListener\0", "(Landroid/view/View$OnClickListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateActionView](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#onCreateActionView())
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        pub fn onCreateActionView<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "onCreateActionView", .descriptor == "()Landroid/view/View;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "onCreateActionView\0", "()Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateActionView](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#onCreateActionView(android.view.MenuItem))
        ///
        /// Required features: "android-view-MenuItem", "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-MenuItem", feature = "android-view-View")))]
        pub fn onCreateActionView_MenuItem<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MenuItem>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "onCreateActionView", .descriptor == "(Landroid/view/MenuItem;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "onCreateActionView\0", "(Landroid/view/MenuItem;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPerformDefaultAction](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#onPerformDefaultAction())
        pub fn onPerformDefaultAction<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "onPerformDefaultAction", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "onPerformDefaultAction\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [overridesItemVisibility](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#overridesItemVisibility())
        pub fn overridesItemVisibility<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "overridesItemVisibility", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "overridesItemVisibility\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isVisible](https://developer.android.com/reference/android/app/MediaRouteActionProvider.html#isVisible())
        pub fn isVisible<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/MediaRouteActionProvider", java.flags == PUBLIC, .name == "isVisible", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/MediaRouteActionProvider\0", "isVisible\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
