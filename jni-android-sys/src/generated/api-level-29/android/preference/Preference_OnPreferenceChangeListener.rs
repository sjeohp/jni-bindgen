// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-preference-Preference_OnPreferenceChangeListener"))]
__jni_bindgen! {
    /// public interface [Preference.OnPreferenceChangeListener](https://developer.android.com/reference/android/preference/Preference.OnPreferenceChangeListener.html)
    ///
    /// Required feature: android-preference-Preference_OnPreferenceChangeListener
    #[deprecated] public interface Preference_OnPreferenceChangeListener ("android/preference/Preference$OnPreferenceChangeListener") extends crate::java::lang::Object {

        /// [onPreferenceChange](https://developer.android.com/reference/android/preference/Preference.OnPreferenceChangeListener.html#onPreferenceChange(android.preference.Preference,%20java.lang.Object))
        ///
        /// Required features: "android-preference-Preference", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-preference-Preference", feature = "java-lang-Object")))]
        #[deprecated] pub fn onPreferenceChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::preference::Preference>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/preference/Preference$OnPreferenceChangeListener", java.flags == PUBLIC | ABSTRACT, .name == "onPreferenceChange", .descriptor == "(Landroid/preference/Preference;Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/preference/Preference$OnPreferenceChangeListener\0", "onPreferenceChange\0", "(Landroid/preference/Preference;Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
