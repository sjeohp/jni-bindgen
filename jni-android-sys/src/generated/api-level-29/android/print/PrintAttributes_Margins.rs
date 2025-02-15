// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-print-PrintAttributes_Margins"))]
__jni_bindgen! {
    /// public final class [PrintAttributes.Margins](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html)
    ///
    /// Required feature: android-print-PrintAttributes_Margins
    public final class PrintAttributes_Margins ("android/print/PrintAttributes$Margins") extends crate::java::lang::Object {

        /// [Margins](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#Margins(int,%20int,%20int,%20int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::print::PrintAttributes_Margins>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "<init>\0", "(IIII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLeftMils](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#getLeftMils())
        pub fn getLeftMils<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "getLeftMils", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "getLeftMils\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTopMils](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#getTopMils())
        pub fn getTopMils<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "getTopMils", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "getTopMils\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRightMils](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#getRightMils())
        pub fn getRightMils<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "getRightMils", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "getRightMils\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBottomMils](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#getBottomMils())
        pub fn getBottomMils<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "getBottomMils", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "getBottomMils\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintAttributes$Margins", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintAttributes$Margins\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [NO_MARGINS](https://developer.android.com/reference/android/print/PrintAttributes.Margins.html#NO_MARGINS)
        ///
        /// Required feature: android-print-PrintAttributes_Margins
        #[cfg(any(feature = "all", feature = "android-print-PrintAttributes_Margins"))]
        pub fn NO_MARGINS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrintAttributes_Margins>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/print/PrintAttributes$Margins\0", "NO_MARGINS\0", "Landroid/print/PrintAttributes$Margins;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
