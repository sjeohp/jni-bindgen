// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-Filterable"))]
__jni_bindgen! {
    /// public interface [Filterable](https://developer.android.com/reference/android/widget/Filterable.html)
    ///
    /// Required feature: android-widget-Filterable
    public interface Filterable ("android/widget/Filterable") extends crate::java::lang::Object {

        /// [getFilter](https://developer.android.com/reference/android/widget/Filterable.html#getFilter())
        ///
        /// Required features: "android-widget-Filter"
        #[cfg(any(feature = "all", all(feature = "android-widget-Filter")))]
        pub fn getFilter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::Filter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/Filterable", java.flags == PUBLIC | ABSTRACT, .name == "getFilter", .descriptor == "()Landroid/widget/Filter;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/Filterable\0", "getFilter\0", "()Landroid/widget/Filter;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
