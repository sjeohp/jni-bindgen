// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_QuickContact"))]
__jni_bindgen! {
    /// public final class [ContactsContract.QuickContact](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html)
    ///
    /// Required feature: android-provider-ContactsContract_QuickContact
    public final class ContactsContract_QuickContact ("android/provider/ContactsContract$QuickContact") extends crate::java::lang::Object {

        /// [QuickContact](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#QuickContact())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_QuickContact>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$QuickContact", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$QuickContact\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [showQuickContact](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#showQuickContact(android.content.Context,%20android.view.View,%20android.net.Uri,%20int,%20java.lang.String%5B%5D))
        ///
        /// Required features: "android-content-Context", "android-net-Uri", "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri", feature = "android-view-View", feature = "java-lang-String")))]
        pub fn showQuickContact_Context_View_Uri_int_String_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$QuickContact", java.flags == PUBLIC | STATIC, .name == "showQuickContact", .descriptor == "(Landroid/content/Context;Landroid/view/View;Landroid/net/Uri;I[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$QuickContact\0", "showQuickContact\0", "(Landroid/content/Context;Landroid/view/View;Landroid/net/Uri;I[Ljava/lang/String;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [showQuickContact](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#showQuickContact(android.content.Context,%20android.graphics.Rect,%20android.net.Uri,%20int,%20java.lang.String%5B%5D))
        ///
        /// Required features: "android-content-Context", "android-graphics-Rect", "android-net-Uri", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-graphics-Rect", feature = "android-net-Uri", feature = "java-lang-String")))]
        pub fn showQuickContact_Context_Rect_Uri_int_String_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$QuickContact", java.flags == PUBLIC | STATIC, .name == "showQuickContact", .descriptor == "(Landroid/content/Context;Landroid/graphics/Rect;Landroid/net/Uri;I[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$QuickContact\0", "showQuickContact\0", "(Landroid/content/Context;Landroid/graphics/Rect;Landroid/net/Uri;I[Ljava/lang/String;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [showQuickContact](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#showQuickContact(android.content.Context,%20android.view.View,%20android.net.Uri,%20java.lang.String%5B%5D,%20java.lang.String))
        ///
        /// Required features: "android-content-Context", "android-net-Uri", "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-net-Uri", feature = "android-view-View", feature = "java-lang-String")))]
        pub fn showQuickContact_Context_View_Uri_String_array_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$QuickContact", java.flags == PUBLIC | STATIC, .name == "showQuickContact", .descriptor == "(Landroid/content/Context;Landroid/view/View;Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$QuickContact\0", "showQuickContact\0", "(Landroid/content/Context;Landroid/view/View;Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [showQuickContact](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#showQuickContact(android.content.Context,%20android.graphics.Rect,%20android.net.Uri,%20java.lang.String%5B%5D,%20java.lang.String))
        ///
        /// Required features: "android-content-Context", "android-graphics-Rect", "android-net-Uri", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-graphics-Rect", feature = "android-net-Uri", feature = "java-lang-String")))]
        pub fn showQuickContact_Context_Rect_Uri_String_array_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$QuickContact", java.flags == PUBLIC | STATIC, .name == "showQuickContact", .descriptor == "(Landroid/content/Context;Landroid/graphics/Rect;Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/ContactsContract$QuickContact\0", "showQuickContact\0", "(Landroid/content/Context;Landroid/graphics/Rect;Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_QUICK_CONTACT](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#ACTION_QUICK_CONTACT)
        pub const ACTION_QUICK_CONTACT : &'static str = "android.provider.action.QUICK_CONTACT";

        /// public static final [EXTRA_EXCLUDE_MIMES](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#EXTRA_EXCLUDE_MIMES)
        pub const EXTRA_EXCLUDE_MIMES : &'static str = "android.provider.extra.EXCLUDE_MIMES";

        /// public static final [EXTRA_MODE](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#EXTRA_MODE)
        pub const EXTRA_MODE : &'static str = "android.provider.extra.MODE";

        /// public static final [EXTRA_PRIORITIZED_MIMETYPE](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#EXTRA_PRIORITIZED_MIMETYPE)
        pub const EXTRA_PRIORITIZED_MIMETYPE : &'static str = "android.provider.extra.PRIORITIZED_MIMETYPE";

        /// public static final [MODE_LARGE](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#MODE_LARGE)
        pub const MODE_LARGE : i32 = 3;

        /// public static final [MODE_MEDIUM](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#MODE_MEDIUM)
        pub const MODE_MEDIUM : i32 = 2;

        /// public static final [MODE_SMALL](https://developer.android.com/reference/android/provider/ContactsContract.QuickContact.html#MODE_SMALL)
        pub const MODE_SMALL : i32 = 1;
    }
}
