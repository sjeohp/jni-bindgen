// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-beans-IndexedPropertyChangeEvent"))]
__jni_bindgen! {
    /// public class [IndexedPropertyChangeEvent](https://developer.android.com/reference/java/beans/IndexedPropertyChangeEvent.html)
    ///
    /// Required feature: java-beans-IndexedPropertyChangeEvent
    public class IndexedPropertyChangeEvent ("java/beans/IndexedPropertyChangeEvent") extends crate::java::beans::PropertyChangeEvent {

        /// [IndexedPropertyChangeEvent](https://developer.android.com/reference/java/beans/IndexedPropertyChangeEvent.html#IndexedPropertyChangeEvent(java.lang.Object,%20java.lang.String,%20java.lang.Object,%20java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::beans::IndexedPropertyChangeEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/beans/IndexedPropertyChangeEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Object;Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/beans/IndexedPropertyChangeEvent\0", "<init>\0", "(Ljava/lang/Object;Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIndex](https://developer.android.com/reference/java/beans/IndexedPropertyChangeEvent.html#getIndex())
        pub fn getIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/beans/IndexedPropertyChangeEvent", java.flags == PUBLIC, .name == "getIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/beans/IndexedPropertyChangeEvent\0", "getIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
