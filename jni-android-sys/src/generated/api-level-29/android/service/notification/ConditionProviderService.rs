// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-notification-ConditionProviderService"))]
__jni_bindgen! {
    /// public class [ConditionProviderService](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html)
    ///
    /// Required feature: android-service-notification-ConditionProviderService
    #[deprecated] public class ConditionProviderService ("android/service/notification/ConditionProviderService") extends crate::android::app::Service {

        /// [ConditionProviderService](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#ConditionProviderService())
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::notification::ConditionProviderService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnected](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#onConnected())
        #[deprecated] pub fn onConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | ABSTRACT, .name == "onConnected", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "onConnected\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRequestConditions](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#onRequestConditions(int))
        #[deprecated] pub fn onRequestConditions<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC, .name == "onRequestConditions", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "onRequestConditions\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSubscribe](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#onSubscribe(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        #[deprecated] pub fn onSubscribe<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | ABSTRACT, .name == "onSubscribe", .descriptor == "(Landroid/net/Uri;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "onSubscribe\0", "(Landroid/net/Uri;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onUnsubscribe](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#onUnsubscribe(android.net.Uri))
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        #[deprecated] pub fn onUnsubscribe<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | ABSTRACT, .name == "onUnsubscribe", .descriptor == "(Landroid/net/Uri;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "onUnsubscribe\0", "(Landroid/net/Uri;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestRebind](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#requestRebind(android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        #[deprecated] pub fn requestRebind<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | STATIC | FINAL, .name == "requestRebind", .descriptor == "(Landroid/content/ComponentName;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/service/notification/ConditionProviderService\0", "requestRebind\0", "(Landroid/content/ComponentName;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [requestUnbind](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#requestUnbind())
        #[deprecated] pub fn requestUnbind<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | FINAL, .name == "requestUnbind", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "requestUnbind\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyCondition](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#notifyCondition(android.service.notification.Condition))
        ///
        /// Required features: "android-service-notification-Condition"
        #[cfg(any(feature = "all", all(feature = "android-service-notification-Condition")))]
        #[deprecated] pub fn notifyCondition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::notification::Condition>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | FINAL, .name == "notifyCondition", .descriptor == "(Landroid/service/notification/Condition;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "notifyCondition\0", "(Landroid/service/notification/Condition;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyConditions](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#notifyConditions(android.service.notification.Condition...))
        ///
        /// Required features: "android-service-notification-Condition"
        #[cfg(any(feature = "all", all(feature = "android-service-notification-Condition")))]
        #[deprecated] pub fn notifyConditions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::service::notification::Condition, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC | FINAL | VARARGS, .name == "notifyConditions", .descriptor == "([Landroid/service/notification/Condition;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "notifyConditions\0", "([Landroid/service/notification/Condition;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        #[deprecated] pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/notification/ConditionProviderService", java.flags == PUBLIC, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/notification/ConditionProviderService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [EXTRA_RULE_ID](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#EXTRA_RULE_ID)
        #[deprecated] pub const EXTRA_RULE_ID : &'static str = "android.service.notification.extra.RULE_ID";

        /// public static final [META_DATA_CONFIGURATION_ACTIVITY](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#META_DATA_CONFIGURATION_ACTIVITY)
        #[deprecated] pub const META_DATA_CONFIGURATION_ACTIVITY : &'static str = "android.service.zen.automatic.configurationActivity";

        /// public static final [META_DATA_RULE_INSTANCE_LIMIT](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#META_DATA_RULE_INSTANCE_LIMIT)
        #[deprecated] pub const META_DATA_RULE_INSTANCE_LIMIT : &'static str = "android.service.zen.automatic.ruleInstanceLimit";

        /// public static final [META_DATA_RULE_TYPE](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#META_DATA_RULE_TYPE)
        #[deprecated] pub const META_DATA_RULE_TYPE : &'static str = "android.service.zen.automatic.ruleType";

        /// public static final [SERVICE_INTERFACE](https://developer.android.com/reference/android/service/notification/ConditionProviderService.html#SERVICE_INTERFACE)
        #[deprecated] pub const SERVICE_INTERFACE : &'static str = "android.service.notification.ConditionProviderService";
    }
}
