// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-RestrictionsManager"))]
__jni_bindgen! {
    /// public class [RestrictionsManager](https://developer.android.com/reference/android/content/RestrictionsManager.html)
    ///
    /// Required feature: android-content-RestrictionsManager
    public class RestrictionsManager ("android/content/RestrictionsManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [RestrictionsManager](https://developer.android.com/reference/android/content/RestrictionsManager.html#RestrictionsManager())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::content::RestrictionsManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/content/RestrictionsManager", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getApplicationRestrictions](https://developer.android.com/reference/android/content/RestrictionsManager.html#getApplicationRestrictions())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getApplicationRestrictions<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC, .name == "getApplicationRestrictions", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "getApplicationRestrictions\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasRestrictionsProvider](https://developer.android.com/reference/android/content/RestrictionsManager.html#hasRestrictionsProvider())
        pub fn hasRestrictionsProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC, .name == "hasRestrictionsProvider", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "hasRestrictionsProvider\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestPermission](https://developer.android.com/reference/android/content/RestrictionsManager.html#requestPermission(java.lang.String,%20java.lang.String,%20android.os.PersistableBundle))
        ///
        /// Required features: "android-os-PersistableBundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-PersistableBundle", feature = "java-lang-String")))]
        pub fn requestPermission<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::PersistableBundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC, .name == "requestPermission", .descriptor == "(Ljava/lang/String;Ljava/lang/String;Landroid/os/PersistableBundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "requestPermission\0", "(Ljava/lang/String;Ljava/lang/String;Landroid/os/PersistableBundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createLocalApprovalIntent](https://developer.android.com/reference/android/content/RestrictionsManager.html#createLocalApprovalIntent())
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn createLocalApprovalIntent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Intent>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC, .name == "createLocalApprovalIntent", .descriptor == "()Landroid/content/Intent;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "createLocalApprovalIntent\0", "()Landroid/content/Intent;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyPermissionResponse](https://developer.android.com/reference/android/content/RestrictionsManager.html#notifyPermissionResponse(java.lang.String,%20android.os.PersistableBundle))
        ///
        /// Required features: "android-os-PersistableBundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-PersistableBundle", feature = "java-lang-String")))]
        pub fn notifyPermissionResponse<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::PersistableBundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC, .name == "notifyPermissionResponse", .descriptor == "(Ljava/lang/String;Landroid/os/PersistableBundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "notifyPermissionResponse\0", "(Ljava/lang/String;Landroid/os/PersistableBundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getManifestRestrictions](https://developer.android.com/reference/android/content/RestrictionsManager.html#getManifestRestrictions(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-List")))]
        pub fn getManifestRestrictions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC, .name == "getManifestRestrictions", .descriptor == "(Ljava/lang/String;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/RestrictionsManager\0", "getManifestRestrictions\0", "(Ljava/lang/String;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [convertRestrictionsToBundle](https://developer.android.com/reference/android/content/RestrictionsManager.html#convertRestrictionsToBundle(java.util.List))
        ///
        /// Required features: "android-os-Bundle", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "java-util-List")))]
        pub fn convertRestrictionsToBundle<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/RestrictionsManager", java.flags == PUBLIC | STATIC, .name == "convertRestrictionsToBundle", .descriptor == "(Ljava/util/List;)Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/content/RestrictionsManager\0", "convertRestrictionsToBundle\0", "(Ljava/util/List;)Landroid/os/Bundle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_PERMISSION_RESPONSE_RECEIVED](https://developer.android.com/reference/android/content/RestrictionsManager.html#ACTION_PERMISSION_RESPONSE_RECEIVED)
        pub const ACTION_PERMISSION_RESPONSE_RECEIVED : &'static str = "android.content.action.PERMISSION_RESPONSE_RECEIVED";

        /// public static final [ACTION_REQUEST_LOCAL_APPROVAL](https://developer.android.com/reference/android/content/RestrictionsManager.html#ACTION_REQUEST_LOCAL_APPROVAL)
        pub const ACTION_REQUEST_LOCAL_APPROVAL : &'static str = "android.content.action.REQUEST_LOCAL_APPROVAL";

        /// public static final [ACTION_REQUEST_PERMISSION](https://developer.android.com/reference/android/content/RestrictionsManager.html#ACTION_REQUEST_PERMISSION)
        pub const ACTION_REQUEST_PERMISSION : &'static str = "android.content.action.REQUEST_PERMISSION";

        /// public static final [EXTRA_PACKAGE_NAME](https://developer.android.com/reference/android/content/RestrictionsManager.html#EXTRA_PACKAGE_NAME)
        pub const EXTRA_PACKAGE_NAME : &'static str = "android.content.extra.PACKAGE_NAME";

        /// public static final [EXTRA_REQUEST_BUNDLE](https://developer.android.com/reference/android/content/RestrictionsManager.html#EXTRA_REQUEST_BUNDLE)
        pub const EXTRA_REQUEST_BUNDLE : &'static str = "android.content.extra.REQUEST_BUNDLE";

        /// public static final [EXTRA_REQUEST_ID](https://developer.android.com/reference/android/content/RestrictionsManager.html#EXTRA_REQUEST_ID)
        pub const EXTRA_REQUEST_ID : &'static str = "android.content.extra.REQUEST_ID";

        /// public static final [EXTRA_REQUEST_TYPE](https://developer.android.com/reference/android/content/RestrictionsManager.html#EXTRA_REQUEST_TYPE)
        pub const EXTRA_REQUEST_TYPE : &'static str = "android.content.extra.REQUEST_TYPE";

        /// public static final [EXTRA_RESPONSE_BUNDLE](https://developer.android.com/reference/android/content/RestrictionsManager.html#EXTRA_RESPONSE_BUNDLE)
        pub const EXTRA_RESPONSE_BUNDLE : &'static str = "android.content.extra.RESPONSE_BUNDLE";

        /// public static final [META_DATA_APP_RESTRICTIONS](https://developer.android.com/reference/android/content/RestrictionsManager.html#META_DATA_APP_RESTRICTIONS)
        pub const META_DATA_APP_RESTRICTIONS : &'static str = "android.content.APP_RESTRICTIONS";

        /// public static final [REQUEST_KEY_APPROVE_LABEL](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_APPROVE_LABEL)
        pub const REQUEST_KEY_APPROVE_LABEL : &'static str = "android.request.approve_label";

        /// public static final [REQUEST_KEY_DATA](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_DATA)
        pub const REQUEST_KEY_DATA : &'static str = "android.request.data";

        /// public static final [REQUEST_KEY_DENY_LABEL](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_DENY_LABEL)
        pub const REQUEST_KEY_DENY_LABEL : &'static str = "android.request.deny_label";

        /// public static final [REQUEST_KEY_ICON](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_ICON)
        pub const REQUEST_KEY_ICON : &'static str = "android.request.icon";

        /// public static final [REQUEST_KEY_ID](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_ID)
        pub const REQUEST_KEY_ID : &'static str = "android.request.id";

        /// public static final [REQUEST_KEY_MESSAGE](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_MESSAGE)
        pub const REQUEST_KEY_MESSAGE : &'static str = "android.request.mesg";

        /// public static final [REQUEST_KEY_NEW_REQUEST](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_NEW_REQUEST)
        pub const REQUEST_KEY_NEW_REQUEST : &'static str = "android.request.new_request";

        /// public static final [REQUEST_KEY_TITLE](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_KEY_TITLE)
        pub const REQUEST_KEY_TITLE : &'static str = "android.request.title";

        /// public static final [REQUEST_TYPE_APPROVAL](https://developer.android.com/reference/android/content/RestrictionsManager.html#REQUEST_TYPE_APPROVAL)
        pub const REQUEST_TYPE_APPROVAL : &'static str = "android.request.type.approval";

        /// public static final [RESPONSE_KEY_ERROR_CODE](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESPONSE_KEY_ERROR_CODE)
        pub const RESPONSE_KEY_ERROR_CODE : &'static str = "android.response.errorcode";

        /// public static final [RESPONSE_KEY_MESSAGE](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESPONSE_KEY_MESSAGE)
        pub const RESPONSE_KEY_MESSAGE : &'static str = "android.response.msg";

        /// public static final [RESPONSE_KEY_RESPONSE_TIMESTAMP](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESPONSE_KEY_RESPONSE_TIMESTAMP)
        pub const RESPONSE_KEY_RESPONSE_TIMESTAMP : &'static str = "android.response.timestamp";

        /// public static final [RESPONSE_KEY_RESULT](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESPONSE_KEY_RESULT)
        pub const RESPONSE_KEY_RESULT : &'static str = "android.response.result";

        /// public static final [RESULT_APPROVED](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_APPROVED)
        pub const RESULT_APPROVED : i32 = 1;

        /// public static final [RESULT_DENIED](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_DENIED)
        pub const RESULT_DENIED : i32 = 2;

        /// public static final [RESULT_ERROR](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_ERROR)
        pub const RESULT_ERROR : i32 = 5;

        /// public static final [RESULT_ERROR_BAD_REQUEST](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_ERROR_BAD_REQUEST)
        pub const RESULT_ERROR_BAD_REQUEST : i32 = 1;

        /// public static final [RESULT_ERROR_INTERNAL](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_ERROR_INTERNAL)
        pub const RESULT_ERROR_INTERNAL : i32 = 3;

        /// public static final [RESULT_ERROR_NETWORK](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_ERROR_NETWORK)
        pub const RESULT_ERROR_NETWORK : i32 = 2;

        /// public static final [RESULT_NO_RESPONSE](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_NO_RESPONSE)
        pub const RESULT_NO_RESPONSE : i32 = 3;

        /// public static final [RESULT_UNKNOWN_REQUEST](https://developer.android.com/reference/android/content/RestrictionsManager.html#RESULT_UNKNOWN_REQUEST)
        pub const RESULT_UNKNOWN_REQUEST : i32 = 4;
    }
}
