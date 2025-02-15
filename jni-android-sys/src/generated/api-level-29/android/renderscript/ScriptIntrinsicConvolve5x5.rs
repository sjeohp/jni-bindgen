// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-ScriptIntrinsicConvolve5x5"))]
__jni_bindgen! {
    /// public final class [ScriptIntrinsicConvolve5x5](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html)
    ///
    /// Required feature: android-renderscript-ScriptIntrinsicConvolve5x5
    public final class ScriptIntrinsicConvolve5x5 ("android/renderscript/ScriptIntrinsicConvolve5x5") extends crate::android::renderscript::ScriptIntrinsic {

        // // Not emitting: Non-public method
        // /// [ScriptIntrinsicConvolve5x5](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#ScriptIntrinsicConvolve5x5(long,%20android.renderscript.RenderScript))
        // ///
        // /// Required features: "android-renderscript-RenderScript"
        // #[cfg(any(feature = "all", all(feature = "android-renderscript-RenderScript")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptIntrinsicConvolve5x5>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == (empty), .name == "<init>", .descriptor == "(JLandroid/renderscript/RenderScript;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "<init>\0", "(JLandroid/renderscript/RenderScript;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [create](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#create(android.renderscript.RenderScript,%20android.renderscript.Element))
        ///
        /// Required features: "android-renderscript-Element", "android-renderscript-RenderScript", "android-renderscript-ScriptIntrinsicConvolve5x5"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Element", feature = "android-renderscript-RenderScript", feature = "android-renderscript-ScriptIntrinsicConvolve5x5")))]
        pub fn create<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Element>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptIntrinsicConvolve5x5>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/renderscript/RenderScript;Landroid/renderscript/Element;)Landroid/renderscript/ScriptIntrinsicConvolve5x5;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "create\0", "(Landroid/renderscript/RenderScript;Landroid/renderscript/Element;)Landroid/renderscript/ScriptIntrinsicConvolve5x5;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInput](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#setInput(android.renderscript.Allocation))
        ///
        /// Required features: "android-renderscript-Allocation"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Allocation")))]
        pub fn setInput<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC, .name == "setInput", .descriptor == "(Landroid/renderscript/Allocation;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "setInput\0", "(Landroid/renderscript/Allocation;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCoefficients](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#setCoefficients(float%5B%5D))
        pub fn setCoefficients<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC, .name == "setCoefficients", .descriptor == "([F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "setCoefficients\0", "([F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEach](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#forEach(android.renderscript.Allocation))
        ///
        /// Required features: "android-renderscript-Allocation"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Allocation")))]
        pub fn forEach_Allocation<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC, .name == "forEach", .descriptor == "(Landroid/renderscript/Allocation;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "forEach\0", "(Landroid/renderscript/Allocation;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEach](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#forEach(android.renderscript.Allocation,%20android.renderscript.Script.LaunchOptions))
        ///
        /// Required features: "android-renderscript-Allocation", "android-renderscript-Script_LaunchOptions"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Allocation", feature = "android-renderscript-Script_LaunchOptions")))]
        pub fn forEach_Allocation_LaunchOptions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Script_LaunchOptions>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC, .name == "forEach", .descriptor == "(Landroid/renderscript/Allocation;Landroid/renderscript/Script$LaunchOptions;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "forEach\0", "(Landroid/renderscript/Allocation;Landroid/renderscript/Script$LaunchOptions;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKernelID](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#getKernelID())
        ///
        /// Required features: "android-renderscript-Script_KernelID"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Script_KernelID")))]
        pub fn getKernelID<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Script_KernelID>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC, .name == "getKernelID", .descriptor == "()Landroid/renderscript/Script$KernelID;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "getKernelID\0", "()Landroid/renderscript/Script$KernelID;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFieldID_Input](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicConvolve5x5.html#getFieldID_Input())
        ///
        /// Required features: "android-renderscript-Script_FieldID"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Script_FieldID")))]
        pub fn getFieldID_Input<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Script_FieldID>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicConvolve5x5", java.flags == PUBLIC, .name == "getFieldID_Input", .descriptor == "()Landroid/renderscript/Script$FieldID;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicConvolve5x5\0", "getFieldID_Input\0", "()Landroid/renderscript/Script$FieldID;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
