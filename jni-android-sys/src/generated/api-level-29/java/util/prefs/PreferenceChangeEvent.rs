// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-prefs-PreferenceChangeEvent"))]
__jni_bindgen! {
    /// public class [PreferenceChangeEvent](https://developer.android.com/reference/java/util/prefs/PreferenceChangeEvent.html)
    ///
    /// Required feature: java-util-prefs-PreferenceChangeEvent
    public class PreferenceChangeEvent ("java/util/prefs/PreferenceChangeEvent") extends crate::java::util::EventObject {

        /// [PreferenceChangeEvent](https://developer.android.com/reference/java/util/prefs/PreferenceChangeEvent.html#PreferenceChangeEvent(java.util.prefs.Preferences,%20java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-prefs-Preferences"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-prefs-Preferences")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::prefs::Preferences>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::prefs::PreferenceChangeEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/prefs/PreferenceChangeEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/prefs/Preferences;Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/prefs/PreferenceChangeEvent\0", "<init>\0", "(Ljava/util/prefs/Preferences;Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNode](https://developer.android.com/reference/java/util/prefs/PreferenceChangeEvent.html#getNode())
        ///
        /// Required features: "java-util-prefs-Preferences"
        #[cfg(any(feature = "all", all(feature = "java-util-prefs-Preferences")))]
        pub fn getNode<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::prefs::Preferences>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/prefs/PreferenceChangeEvent", java.flags == PUBLIC, .name == "getNode", .descriptor == "()Ljava/util/prefs/Preferences;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/prefs/PreferenceChangeEvent\0", "getNode\0", "()Ljava/util/prefs/Preferences;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKey](https://developer.android.com/reference/java/util/prefs/PreferenceChangeEvent.html#getKey())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getKey<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/prefs/PreferenceChangeEvent", java.flags == PUBLIC, .name == "getKey", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/prefs/PreferenceChangeEvent\0", "getKey\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNewValue](https://developer.android.com/reference/java/util/prefs/PreferenceChangeEvent.html#getNewValue())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getNewValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/prefs/PreferenceChangeEvent", java.flags == PUBLIC, .name == "getNewValue", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/prefs/PreferenceChangeEvent\0", "getNewValue\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
