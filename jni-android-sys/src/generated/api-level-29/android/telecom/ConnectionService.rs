// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-ConnectionService"))]
__jni_bindgen! {
    /// public class [ConnectionService](https://developer.android.com/reference/android/telecom/ConnectionService.html)
    ///
    /// Required feature: android-telecom-ConnectionService
    public class ConnectionService ("android/telecom/ConnectionService") extends crate::android::app::Service {

        /// [ConnectionService](https://developer.android.com/reference/android/telecom/ConnectionService.html#ConnectionService())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::ConnectionService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/telecom/ConnectionService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onUnbind](https://developer.android.com/reference/android/telecom/ConnectionService.html#onUnbind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn onUnbind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onUnbind", .descriptor == "(Landroid/content/Intent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onUnbind\0", "(Landroid/content/Intent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createRemoteIncomingConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#createRemoteIncomingConnection(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle", "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle", feature = "android-telecom-RemoteConnection")))]
        pub fn createRemoteIncomingConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telecom::RemoteConnection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "createRemoteIncomingConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/RemoteConnection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "createRemoteIncomingConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/RemoteConnection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createRemoteOutgoingConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#createRemoteOutgoingConnection(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle", "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle", feature = "android-telecom-RemoteConnection")))]
        pub fn createRemoteOutgoingConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telecom::RemoteConnection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "createRemoteOutgoingConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/RemoteConnection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "createRemoteOutgoingConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/RemoteConnection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [conferenceRemoteConnections](https://developer.android.com/reference/android/telecom/ConnectionService.html#conferenceRemoteConnections(android.telecom.RemoteConnection,%20android.telecom.RemoteConnection))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn conferenceRemoteConnections<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "conferenceRemoteConnections", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/telecom/RemoteConnection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "conferenceRemoteConnections\0", "(Landroid/telecom/RemoteConnection;Landroid/telecom/RemoteConnection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addConference](https://developer.android.com/reference/android/telecom/ConnectionService.html#addConference(android.telecom.Conference))
        ///
        /// Required features: "android-telecom-Conference"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Conference")))]
        pub fn addConference<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::Conference>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "addConference", .descriptor == "(Landroid/telecom/Conference;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "addConference\0", "(Landroid/telecom/Conference;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addExistingConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#addExistingConnection(android.telecom.PhoneAccountHandle,%20android.telecom.Connection))
        ///
        /// Required features: "android-telecom-Connection", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Connection", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn addExistingConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::Connection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "addExistingConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/Connection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "addExistingConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/Connection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connectionServiceFocusReleased](https://developer.android.com/reference/android/telecom/ConnectionService.html#connectionServiceFocusReleased())
        pub fn connectionServiceFocusReleased<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "connectionServiceFocusReleased", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "connectionServiceFocusReleased\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAllConnections](https://developer.android.com/reference/android/telecom/ConnectionService.html#getAllConnections())
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn getAllConnections<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "getAllConnections", .descriptor == "()Ljava/util/Collection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "getAllConnections\0", "()Ljava/util/Collection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAllConferences](https://developer.android.com/reference/android/telecom/ConnectionService.html#getAllConferences())
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn getAllConferences<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC | FINAL, .name == "getAllConferences", .descriptor == "()Ljava/util/Collection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "getAllConferences\0", "()Ljava/util/Collection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateIncomingConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#onCreateIncomingConnection(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-Connection", "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Connection", feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn onCreateIncomingConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telecom::Connection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onCreateIncomingConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onCreateIncomingConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateIncomingConnectionFailed](https://developer.android.com/reference/android/telecom/ConnectionService.html#onCreateIncomingConnectionFailed(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn onCreateIncomingConnectionFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onCreateIncomingConnectionFailed", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onCreateIncomingConnectionFailed\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateOutgoingConnectionFailed](https://developer.android.com/reference/android/telecom/ConnectionService.html#onCreateOutgoingConnectionFailed(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn onCreateOutgoingConnectionFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onCreateOutgoingConnectionFailed", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onCreateOutgoingConnectionFailed\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateOutgoingConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#onCreateOutgoingConnection(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-Connection", "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Connection", feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn onCreateOutgoingConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telecom::Connection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onCreateOutgoingConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onCreateOutgoingConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateOutgoingHandoverConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#onCreateOutgoingHandoverConnection(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-Connection", "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Connection", feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn onCreateOutgoingHandoverConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telecom::Connection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onCreateOutgoingHandoverConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onCreateOutgoingHandoverConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateIncomingHandoverConnection](https://developer.android.com/reference/android/telecom/ConnectionService.html#onCreateIncomingHandoverConnection(android.telecom.PhoneAccountHandle,%20android.telecom.ConnectionRequest))
        ///
        /// Required features: "android-telecom-Connection", "android-telecom-ConnectionRequest", "android-telecom-PhoneAccountHandle"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Connection", feature = "android-telecom-ConnectionRequest", feature = "android-telecom-PhoneAccountHandle")))]
        pub fn onCreateIncomingHandoverConnection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::PhoneAccountHandle>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::telecom::Connection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onCreateIncomingHandoverConnection", .descriptor == "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onCreateIncomingHandoverConnection\0", "(Landroid/telecom/PhoneAccountHandle;Landroid/telecom/ConnectionRequest;)Landroid/telecom/Connection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onHandoverFailed](https://developer.android.com/reference/android/telecom/ConnectionService.html#onHandoverFailed(android.telecom.ConnectionRequest,%20int))
        ///
        /// Required features: "android-telecom-ConnectionRequest"
        #[cfg(any(feature = "all", all(feature = "android-telecom-ConnectionRequest")))]
        pub fn onHandoverFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::ConnectionRequest>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onHandoverFailed", .descriptor == "(Landroid/telecom/ConnectionRequest;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onHandoverFailed\0", "(Landroid/telecom/ConnectionRequest;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConference](https://developer.android.com/reference/android/telecom/ConnectionService.html#onConference(android.telecom.Connection,%20android.telecom.Connection))
        ///
        /// Required features: "android-telecom-Connection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-Connection")))]
        pub fn onConference<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::Connection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::Connection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onConference", .descriptor == "(Landroid/telecom/Connection;Landroid/telecom/Connection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onConference\0", "(Landroid/telecom/Connection;Landroid/telecom/Connection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRemoteConferenceAdded](https://developer.android.com/reference/android/telecom/ConnectionService.html#onRemoteConferenceAdded(android.telecom.RemoteConference))
        ///
        /// Required features: "android-telecom-RemoteConference"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConference")))]
        pub fn onRemoteConferenceAdded<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConference>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onRemoteConferenceAdded", .descriptor == "(Landroid/telecom/RemoteConference;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onRemoteConferenceAdded\0", "(Landroid/telecom/RemoteConference;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRemoteExistingConnectionAdded](https://developer.android.com/reference/android/telecom/ConnectionService.html#onRemoteExistingConnectionAdded(android.telecom.RemoteConnection))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onRemoteExistingConnectionAdded<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onRemoteExistingConnectionAdded", .descriptor == "(Landroid/telecom/RemoteConnection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onRemoteExistingConnectionAdded\0", "(Landroid/telecom/RemoteConnection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnectionServiceFocusLost](https://developer.android.com/reference/android/telecom/ConnectionService.html#onConnectionServiceFocusLost())
        pub fn onConnectionServiceFocusLost<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onConnectionServiceFocusLost", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onConnectionServiceFocusLost\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnectionServiceFocusGained](https://developer.android.com/reference/android/telecom/ConnectionService.html#onConnectionServiceFocusGained())
        pub fn onConnectionServiceFocusGained<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/ConnectionService", java.flags == PUBLIC, .name == "onConnectionServiceFocusGained", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/ConnectionService\0", "onConnectionServiceFocusGained\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [SERVICE_INTERFACE](https://developer.android.com/reference/android/telecom/ConnectionService.html#SERVICE_INTERFACE)
        pub const SERVICE_INTERFACE : &'static str = "android.telecom.ConnectionService";
    }
}
