// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-animation-IntEvaluator"))]
__jni_bindgen! {
    /// public class [IntEvaluator](https://developer.android.com/reference/android/animation/IntEvaluator.html)
    ///
    /// Required feature: android-animation-IntEvaluator
    public class IntEvaluator ("android/animation/IntEvaluator") extends crate::java::lang::Object, implements crate::android::animation::TypeEvaluator {

        /// [IntEvaluator](https://developer.android.com/reference/android/animation/IntEvaluator.html#IntEvaluator())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::animation::IntEvaluator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/IntEvaluator", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/IntEvaluator\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [evaluate](https://developer.android.com/reference/android/animation/IntEvaluator.html#evaluate(float,%20java.lang.Integer,%20java.lang.Integer))
        ///
        /// Required features: "java-lang-Integer"
        #[cfg(any(feature = "all", all(feature = "java-lang-Integer")))]
        pub fn evaluate_float_Integer_Integer<'env>(&'env self, arg0: f32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Integer>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Integer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Integer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/animation/IntEvaluator", java.flags == PUBLIC, .name == "evaluate", .descriptor == "(FLjava/lang/Integer;Ljava/lang/Integer;)Ljava/lang/Integer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/IntEvaluator\0", "evaluate\0", "(FLjava/lang/Integer;Ljava/lang/Integer;)Ljava/lang/Integer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [evaluate](https://developer.android.com/reference/android/animation/IntEvaluator.html#evaluate(float,%20java.lang.Object,%20java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn evaluate_float_Object_Object<'env>(&'env self, arg0: f32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/animation/IntEvaluator", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "evaluate", .descriptor == "(FLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/animation/IntEvaluator\0", "evaluate\0", "(FLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
