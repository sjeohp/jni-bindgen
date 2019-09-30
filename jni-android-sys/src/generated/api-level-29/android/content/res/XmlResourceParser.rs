// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-res-XmlResourceParser"))]
__jni_bindgen! {
    /// public interface [XmlResourceParser](https://developer.android.com/reference/android/content/res/XmlResourceParser.html)
    ///
    /// Required feature: android-content-res-XmlResourceParser
    public interface XmlResourceParser ("android/content/res/XmlResourceParser") extends crate::java::lang::Object, implements crate::org::xmlpull::v1::XmlPullParser, crate::android::util::AttributeSet, crate::java::lang::AutoCloseable {

        /// [getAttributeNamespace](https://developer.android.com/reference/android/content/res/XmlResourceParser.html#getAttributeNamespace(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAttributeNamespace<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/res/XmlResourceParser", java.flags == PUBLIC | ABSTRACT, .name == "getAttributeNamespace", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/res/XmlResourceParser\0", "getAttributeNamespace\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/content/res/XmlResourceParser.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/res/XmlResourceParser", java.flags == PUBLIC | ABSTRACT, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/res/XmlResourceParser\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
