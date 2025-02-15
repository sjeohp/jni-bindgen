// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder"))]
__jni_bindgen! {
    /// public final class [ContactsContract.Contacts.AggregationSuggestions.Builder](https://developer.android.com/reference/android/provider/ContactsContract.Contacts.AggregationSuggestions.Builder.html)
    ///
    /// Required feature: android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder
    public final class ContactsContract_Contacts_AggregationSuggestions_Builder ("android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/provider/ContactsContract.Contacts.AggregationSuggestions.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_Contacts_AggregationSuggestions_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setContactId](https://developer.android.com/reference/android/provider/ContactsContract.Contacts.AggregationSuggestions.Builder.html#setContactId(long))
        ///
        /// Required features: "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder"
        #[cfg(any(feature = "all", all(feature = "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder")))]
        pub fn setContactId<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_Contacts_AggregationSuggestions_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder", java.flags == PUBLIC, .name == "setContactId", .descriptor == "(J)Landroid/provider/ContactsContract$Contacts$AggregationSuggestions$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder\0", "setContactId\0", "(J)Landroid/provider/ContactsContract$Contacts$AggregationSuggestions$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addNameParameter](https://developer.android.com/reference/android/provider/ContactsContract.Contacts.AggregationSuggestions.Builder.html#addNameParameter(java.lang.String))
        ///
        /// Required features: "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder", feature = "java-lang-String")))]
        pub fn addNameParameter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_Contacts_AggregationSuggestions_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder", java.flags == PUBLIC, .name == "addNameParameter", .descriptor == "(Ljava/lang/String;)Landroid/provider/ContactsContract$Contacts$AggregationSuggestions$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder\0", "addNameParameter\0", "(Ljava/lang/String;)Landroid/provider/ContactsContract$Contacts$AggregationSuggestions$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLimit](https://developer.android.com/reference/android/provider/ContactsContract.Contacts.AggregationSuggestions.Builder.html#setLimit(int))
        ///
        /// Required features: "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder"
        #[cfg(any(feature = "all", all(feature = "android-provider-ContactsContract_Contacts_AggregationSuggestions_Builder")))]
        pub fn setLimit<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::provider::ContactsContract_Contacts_AggregationSuggestions_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder", java.flags == PUBLIC, .name == "setLimit", .descriptor == "(I)Landroid/provider/ContactsContract$Contacts$AggregationSuggestions$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder\0", "setLimit\0", "(I)Landroid/provider/ContactsContract$Contacts$AggregationSuggestions$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/provider/ContactsContract.Contacts.AggregationSuggestions.Builder.html#build())
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/net/Uri;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/ContactsContract$Contacts$AggregationSuggestions$Builder\0", "build\0", "()Landroid/net/Uri;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
