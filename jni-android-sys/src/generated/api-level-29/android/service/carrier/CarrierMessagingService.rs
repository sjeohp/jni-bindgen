// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-carrier-CarrierMessagingService"))]
__jni_bindgen! {
    /// public class [CarrierMessagingService](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html)
    ///
    /// Required feature: android-service-carrier-CarrierMessagingService
    public class CarrierMessagingService ("android/service/carrier/CarrierMessagingService") extends crate::android::app::Service {

        /// [CarrierMessagingService](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#CarrierMessagingService())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::carrier::CarrierMessagingService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFilterSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onFilterSms(android.service.carrier.MessagePdu,%20java.lang.String,%20int,%20int,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "android-service-carrier-MessagePdu", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "android-service-carrier-MessagePdu", feature = "java-lang-String")))]
        #[deprecated] pub fn onFilterSms<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::MessagePdu>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onFilterSms", .descriptor == "(Landroid/service/carrier/MessagePdu;Ljava/lang/String;IILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onFilterSms\0", "(Landroid/service/carrier/MessagePdu;Ljava/lang/String;IILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onReceiveTextSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onReceiveTextSms(android.service.carrier.MessagePdu,%20java.lang.String,%20int,%20int,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "android-service-carrier-MessagePdu", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "android-service-carrier-MessagePdu", feature = "java-lang-String")))]
        pub fn onReceiveTextSms<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::MessagePdu>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onReceiveTextSms", .descriptor == "(Landroid/service/carrier/MessagePdu;Ljava/lang/String;IILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onReceiveTextSms\0", "(Landroid/service/carrier/MessagePdu;Ljava/lang/String;IILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendTextSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendTextSms(java.lang.String,%20int,%20java.lang.String,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "java-lang-String")))]
        #[deprecated] pub fn onSendTextSms_String_int_String_ResultCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendTextSms", .descriptor == "(Ljava/lang/String;ILjava/lang/String;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendTextSms\0", "(Ljava/lang/String;ILjava/lang/String;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendTextSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendTextSms(java.lang.String,%20int,%20java.lang.String,%20int,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "java-lang-String")))]
        pub fn onSendTextSms_String_int_String_int_ResultCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendTextSms", .descriptor == "(Ljava/lang/String;ILjava/lang/String;ILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendTextSms\0", "(Ljava/lang/String;ILjava/lang/String;ILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendDataSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendDataSms(byte%5B%5D,%20int,%20java.lang.String,%20int,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "java-lang-String")))]
        #[deprecated] pub fn onSendDataSms_byte_array_int_String_int_ResultCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendDataSms", .descriptor == "([BILjava/lang/String;ILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendDataSms\0", "([BILjava/lang/String;ILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendDataSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendDataSms(byte%5B%5D,%20int,%20java.lang.String,%20int,%20int,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "java-lang-String")))]
        pub fn onSendDataSms_byte_array_int_String_int_int_ResultCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: i32, arg4: i32, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendDataSms", .descriptor == "([BILjava/lang/String;IILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendDataSms\0", "([BILjava/lang/String;IILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendMultipartTextSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendMultipartTextSms(java.util.List,%20int,%20java.lang.String,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "java-lang-String", feature = "java-util-List")))]
        #[deprecated] pub fn onSendMultipartTextSms_List_int_String_ResultCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendMultipartTextSms", .descriptor == "(Ljava/util/List;ILjava/lang/String;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendMultipartTextSms\0", "(Ljava/util/List;ILjava/lang/String;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendMultipartTextSms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendMultipartTextSms(java.util.List,%20int,%20java.lang.String,%20int,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-service-carrier-CarrierMessagingService_ResultCallback", "java-lang-String", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-service-carrier-CarrierMessagingService_ResultCallback", feature = "java-lang-String", feature = "java-util-List")))]
        pub fn onSendMultipartTextSms_List_int_String_int_ResultCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendMultipartTextSms", .descriptor == "(Ljava/util/List;ILjava/lang/String;ILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendMultipartTextSms\0", "(Ljava/util/List;ILjava/lang/String;ILandroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSendMms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onSendMms(android.net.Uri,%20int,%20android.net.Uri,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-net-Uri", "android-service-carrier-CarrierMessagingService_ResultCallback"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "android-service-carrier-CarrierMessagingService_ResultCallback")))]
        pub fn onSendMms<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onSendMms", .descriptor == "(Landroid/net/Uri;ILandroid/net/Uri;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onSendMms\0", "(Landroid/net/Uri;ILandroid/net/Uri;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDownloadMms](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onDownloadMms(android.net.Uri,%20int,%20android.net.Uri,%20android.service.carrier.CarrierMessagingService.ResultCallback))
        ///
        /// Required features: "android-net-Uri", "android-service-carrier-CarrierMessagingService_ResultCallback"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "android-service-carrier-CarrierMessagingService_ResultCallback")))]
        pub fn onDownloadMms<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::carrier::CarrierMessagingService_ResultCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onDownloadMms", .descriptor == "(Landroid/net/Uri;ILandroid/net/Uri;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onDownloadMms\0", "(Landroid/net/Uri;ILandroid/net/Uri;Landroid/service/carrier/CarrierMessagingService$ResultCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/carrier/CarrierMessagingService", java.flags == PUBLIC, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/carrier/CarrierMessagingService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DOWNLOAD_STATUS_ERROR](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#DOWNLOAD_STATUS_ERROR)
        pub const DOWNLOAD_STATUS_ERROR : i32 = 2;

        /// public static final [DOWNLOAD_STATUS_OK](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#DOWNLOAD_STATUS_OK)
        pub const DOWNLOAD_STATUS_OK : i32 = 0;

        /// public static final [DOWNLOAD_STATUS_RETRY_ON_CARRIER_NETWORK](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#DOWNLOAD_STATUS_RETRY_ON_CARRIER_NETWORK)
        pub const DOWNLOAD_STATUS_RETRY_ON_CARRIER_NETWORK : i32 = 1;

        /// public static final [RECEIVE_OPTIONS_DEFAULT](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#RECEIVE_OPTIONS_DEFAULT)
        pub const RECEIVE_OPTIONS_DEFAULT : i32 = 0;

        /// public static final [RECEIVE_OPTIONS_DROP](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#RECEIVE_OPTIONS_DROP)
        pub const RECEIVE_OPTIONS_DROP : i32 = 1;

        /// public static final [RECEIVE_OPTIONS_SKIP_NOTIFY_WHEN_CREDENTIAL_PROTECTED_STORAGE_UNAVAILABLE](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#RECEIVE_OPTIONS_SKIP_NOTIFY_WHEN_CREDENTIAL_PROTECTED_STORAGE_UNAVAILABLE)
        pub const RECEIVE_OPTIONS_SKIP_NOTIFY_WHEN_CREDENTIAL_PROTECTED_STORAGE_UNAVAILABLE : i32 = 2;

        /// public static final [SEND_FLAG_REQUEST_DELIVERY_STATUS](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#SEND_FLAG_REQUEST_DELIVERY_STATUS)
        pub const SEND_FLAG_REQUEST_DELIVERY_STATUS : i32 = 1;

        /// public static final [SEND_STATUS_ERROR](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#SEND_STATUS_ERROR)
        pub const SEND_STATUS_ERROR : i32 = 2;

        /// public static final [SEND_STATUS_OK](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#SEND_STATUS_OK)
        pub const SEND_STATUS_OK : i32 = 0;

        /// public static final [SEND_STATUS_RETRY_ON_CARRIER_NETWORK](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#SEND_STATUS_RETRY_ON_CARRIER_NETWORK)
        pub const SEND_STATUS_RETRY_ON_CARRIER_NETWORK : i32 = 1;

        /// public static final [SERVICE_INTERFACE](https://developer.android.com/reference/android/service/carrier/CarrierMessagingService.html#SERVICE_INTERFACE)
        pub const SERVICE_INTERFACE : &'static str = "android.service.carrier.CarrierMessagingService";
    }
}
