// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-WebViewFragment"))]
__jni_bindgen! {
    /// public class [WebViewFragment](https://developer.android.com/reference/android/webkit/WebViewFragment.html)
    ///
    /// Required feature: android-webkit-WebViewFragment
    #[deprecated] public class WebViewFragment ("android/webkit/WebViewFragment") extends crate::android::app::Fragment {

        /// [WebViewFragment](https://developer.android.com/reference/android/webkit/WebViewFragment.html#WebViewFragment())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::WebViewFragment>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateView](https://developer.android.com/reference/android/webkit/WebViewFragment.html#onCreateView(android.view.LayoutInflater,%20android.view.ViewGroup,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-view-LayoutInflater", "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-view-LayoutInflater", feature = "android-view-View", feature = "android-view-ViewGroup")))]
        #[deprecated] pub fn onCreateView<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::LayoutInflater>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "onCreateView", .descriptor == "(Landroid/view/LayoutInflater;Landroid/view/ViewGroup;Landroid/os/Bundle;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "onCreateView\0", "(Landroid/view/LayoutInflater;Landroid/view/ViewGroup;Landroid/os/Bundle;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPause](https://developer.android.com/reference/android/webkit/WebViewFragment.html#onPause())
        #[deprecated] pub fn onPause<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "onPause", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "onPause\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onResume](https://developer.android.com/reference/android/webkit/WebViewFragment.html#onResume())
        #[deprecated] pub fn onResume<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "onResume", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "onResume\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDestroyView](https://developer.android.com/reference/android/webkit/WebViewFragment.html#onDestroyView())
        #[deprecated] pub fn onDestroyView<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "onDestroyView", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "onDestroyView\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDestroy](https://developer.android.com/reference/android/webkit/WebViewFragment.html#onDestroy())
        #[deprecated] pub fn onDestroy<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "onDestroy", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "onDestroy\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWebView](https://developer.android.com/reference/android/webkit/WebViewFragment.html#getWebView())
        ///
        /// Required features: "android-webkit-WebView"
        #[cfg(any(feature = "all", all(feature = "android-webkit-WebView")))]
        #[deprecated] pub fn getWebView<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::webkit::WebView>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebViewFragment", java.flags == PUBLIC, .name == "getWebView", .descriptor == "()Landroid/webkit/WebView;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebViewFragment\0", "getWebView\0", "()Landroid/webkit/WebView;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
