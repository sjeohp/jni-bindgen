// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-inspector-InspectionCompanion_UninitializedPropertyMapException"))]
__jni_bindgen! {
    /// public class [InspectionCompanion.UninitializedPropertyMapException](https://developer.android.com/reference/android/view/inspector/InspectionCompanion.UninitializedPropertyMapException.html)
    ///
    /// Required feature: android-view-inspector-InspectionCompanion_UninitializedPropertyMapException
    public class InspectionCompanion_UninitializedPropertyMapException ("android/view/inspector/InspectionCompanion$UninitializedPropertyMapException") extends crate::java::lang::RuntimeException {

        /// [UninitializedPropertyMapException](https://developer.android.com/reference/android/view/inspector/InspectionCompanion.UninitializedPropertyMapException.html#UninitializedPropertyMapException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::inspector::InspectionCompanion_UninitializedPropertyMapException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/inspector/InspectionCompanion$UninitializedPropertyMapException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/inspector/InspectionCompanion$UninitializedPropertyMapException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
