// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-autofill-AutofillService"))]
__jni_bindgen! {
    /// public class [AutofillService](https://developer.android.com/reference/android/service/autofill/AutofillService.html)
    ///
    /// Required feature: android-service-autofill-AutofillService
    public class AutofillService ("android/service/autofill/AutofillService") extends crate::android::app::Service {

        /// [AutofillService](https://developer.android.com/reference/android/service/autofill/AutofillService.html#AutofillService())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::autofill::AutofillService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreate](https://developer.android.com/reference/android/service/autofill/AutofillService.html#onCreate())
        pub fn onCreate<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC, .name == "onCreate", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "onCreate\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/service/autofill/AutofillService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC | FINAL, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnected](https://developer.android.com/reference/android/service/autofill/AutofillService.html#onConnected())
        pub fn onConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC, .name == "onConnected", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "onConnected\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFillRequest](https://developer.android.com/reference/android/service/autofill/AutofillService.html#onFillRequest(android.service.autofill.FillRequest,%20android.os.CancellationSignal,%20android.service.autofill.FillCallback))
        ///
        /// Required features: "android-os-CancellationSignal", "android-service-autofill-FillCallback", "android-service-autofill-FillRequest"
        #[cfg(any(feature = "all", all(feature = "android-os-CancellationSignal", feature = "android-service-autofill-FillCallback", feature = "android-service-autofill-FillRequest")))]
        pub fn onFillRequest<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::autofill::FillRequest>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::CancellationSignal>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::autofill::FillCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC | ABSTRACT, .name == "onFillRequest", .descriptor == "(Landroid/service/autofill/FillRequest;Landroid/os/CancellationSignal;Landroid/service/autofill/FillCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "onFillRequest\0", "(Landroid/service/autofill/FillRequest;Landroid/os/CancellationSignal;Landroid/service/autofill/FillCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSaveRequest](https://developer.android.com/reference/android/service/autofill/AutofillService.html#onSaveRequest(android.service.autofill.SaveRequest,%20android.service.autofill.SaveCallback))
        ///
        /// Required features: "android-service-autofill-SaveCallback", "android-service-autofill-SaveRequest"
        #[cfg(any(feature = "all", all(feature = "android-service-autofill-SaveCallback", feature = "android-service-autofill-SaveRequest")))]
        pub fn onSaveRequest<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::autofill::SaveRequest>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::autofill::SaveCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC | ABSTRACT, .name == "onSaveRequest", .descriptor == "(Landroid/service/autofill/SaveRequest;Landroid/service/autofill/SaveCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "onSaveRequest\0", "(Landroid/service/autofill/SaveRequest;Landroid/service/autofill/SaveCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisconnected](https://developer.android.com/reference/android/service/autofill/AutofillService.html#onDisconnected())
        pub fn onDisconnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC, .name == "onDisconnected", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "onDisconnected\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFillEventHistory](https://developer.android.com/reference/android/service/autofill/AutofillService.html#getFillEventHistory())
        ///
        /// Required features: "android-service-autofill-FillEventHistory"
        #[cfg(any(feature = "all", all(feature = "android-service-autofill-FillEventHistory")))]
        pub fn getFillEventHistory<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::service::autofill::FillEventHistory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/autofill/AutofillService", java.flags == PUBLIC | FINAL, .name == "getFillEventHistory", .descriptor == "()Landroid/service/autofill/FillEventHistory;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/autofill/AutofillService\0", "getFillEventHistory\0", "()Landroid/service/autofill/FillEventHistory;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [SERVICE_INTERFACE](https://developer.android.com/reference/android/service/autofill/AutofillService.html#SERVICE_INTERFACE)
        pub const SERVICE_INTERFACE : &'static str = "android.service.autofill.AutofillService";

        /// public static final [SERVICE_META_DATA](https://developer.android.com/reference/android/service/autofill/AutofillService.html#SERVICE_META_DATA)
        pub const SERVICE_META_DATA : &'static str = "android.autofill";
    }
}
