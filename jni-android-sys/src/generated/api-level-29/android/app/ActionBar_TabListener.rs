// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ActionBar_TabListener"))]
__jni_bindgen! {
    /// public interface [ActionBar.TabListener](https://developer.android.com/reference/android/app/ActionBar.TabListener.html)
    ///
    /// Required feature: android-app-ActionBar_TabListener
    #[deprecated] public interface ActionBar_TabListener ("android/app/ActionBar$TabListener") extends crate::java::lang::Object {

        /// [onTabSelected](https://developer.android.com/reference/android/app/ActionBar.TabListener.html#onTabSelected(android.app.ActionBar.Tab,%20android.app.FragmentTransaction))
        ///
        /// Required features: "android-app-ActionBar_Tab", "android-app-FragmentTransaction"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "android-app-FragmentTransaction")))]
        #[deprecated] pub fn onTabSelected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::ActionBar_Tab>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::FragmentTransaction>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$TabListener", java.flags == PUBLIC | ABSTRACT, .name == "onTabSelected", .descriptor == "(Landroid/app/ActionBar$Tab;Landroid/app/FragmentTransaction;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$TabListener\0", "onTabSelected\0", "(Landroid/app/ActionBar$Tab;Landroid/app/FragmentTransaction;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTabUnselected](https://developer.android.com/reference/android/app/ActionBar.TabListener.html#onTabUnselected(android.app.ActionBar.Tab,%20android.app.FragmentTransaction))
        ///
        /// Required features: "android-app-ActionBar_Tab", "android-app-FragmentTransaction"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "android-app-FragmentTransaction")))]
        #[deprecated] pub fn onTabUnselected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::ActionBar_Tab>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::FragmentTransaction>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$TabListener", java.flags == PUBLIC | ABSTRACT, .name == "onTabUnselected", .descriptor == "(Landroid/app/ActionBar$Tab;Landroid/app/FragmentTransaction;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$TabListener\0", "onTabUnselected\0", "(Landroid/app/ActionBar$Tab;Landroid/app/FragmentTransaction;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onTabReselected](https://developer.android.com/reference/android/app/ActionBar.TabListener.html#onTabReselected(android.app.ActionBar.Tab,%20android.app.FragmentTransaction))
        ///
        /// Required features: "android-app-ActionBar_Tab", "android-app-FragmentTransaction"
        #[cfg(any(feature = "all", all(feature = "android-app-ActionBar_Tab", feature = "android-app-FragmentTransaction")))]
        #[deprecated] pub fn onTabReselected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::ActionBar_Tab>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::FragmentTransaction>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ActionBar$TabListener", java.flags == PUBLIC | ABSTRACT, .name == "onTabReselected", .descriptor == "(Landroid/app/ActionBar$Tab;Landroid/app/FragmentTransaction;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ActionBar$TabListener\0", "onTabReselected\0", "(Landroid/app/ActionBar$Tab;Landroid/app/FragmentTransaction;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
