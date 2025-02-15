// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-ScriptIntrinsicColorMatrix"))]
__jni_bindgen! {
    /// public final class [ScriptIntrinsicColorMatrix](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html)
    ///
    /// Required feature: android-renderscript-ScriptIntrinsicColorMatrix
    public final class ScriptIntrinsicColorMatrix ("android/renderscript/ScriptIntrinsicColorMatrix") extends crate::android::renderscript::ScriptIntrinsic {

        // // Not emitting: Non-public method
        // /// [ScriptIntrinsicColorMatrix](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#ScriptIntrinsicColorMatrix(long,%20android.renderscript.RenderScript))
        // ///
        // /// Required features: "android-renderscript-RenderScript"
        // #[cfg(any(feature = "all", all(feature = "android-renderscript-RenderScript")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptIntrinsicColorMatrix>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == (empty), .name == "<init>", .descriptor == "(JLandroid/renderscript/RenderScript;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "<init>\0", "(JLandroid/renderscript/RenderScript;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [create](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#create(android.renderscript.RenderScript,%20android.renderscript.Element))
        ///
        /// Required features: "android-renderscript-Element", "android-renderscript-RenderScript", "android-renderscript-ScriptIntrinsicColorMatrix"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Element", feature = "android-renderscript-RenderScript", feature = "android-renderscript-ScriptIntrinsicColorMatrix")))]
        #[deprecated] pub fn create_RenderScript_Element<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Element>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptIntrinsicColorMatrix>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/renderscript/RenderScript;Landroid/renderscript/Element;)Landroid/renderscript/ScriptIntrinsicColorMatrix;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "create\0", "(Landroid/renderscript/RenderScript;Landroid/renderscript/Element;)Landroid/renderscript/ScriptIntrinsicColorMatrix;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#create(android.renderscript.RenderScript))
        ///
        /// Required features: "android-renderscript-RenderScript", "android-renderscript-ScriptIntrinsicColorMatrix"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-RenderScript", feature = "android-renderscript-ScriptIntrinsicColorMatrix")))]
        pub fn create_RenderScript<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptIntrinsicColorMatrix>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/renderscript/RenderScript;)Landroid/renderscript/ScriptIntrinsicColorMatrix;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "create\0", "(Landroid/renderscript/RenderScript;)Landroid/renderscript/ScriptIntrinsicColorMatrix;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setColorMatrix](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setColorMatrix(android.renderscript.Matrix4f))
        ///
        /// Required features: "android-renderscript-Matrix4f"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Matrix4f")))]
        pub fn setColorMatrix_Matrix4f<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Matrix4f>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setColorMatrix", .descriptor == "(Landroid/renderscript/Matrix4f;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setColorMatrix\0", "(Landroid/renderscript/Matrix4f;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setColorMatrix](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setColorMatrix(android.renderscript.Matrix3f))
        ///
        /// Required features: "android-renderscript-Matrix3f"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Matrix3f")))]
        pub fn setColorMatrix_Matrix3f<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Matrix3f>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setColorMatrix", .descriptor == "(Landroid/renderscript/Matrix3f;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setColorMatrix\0", "(Landroid/renderscript/Matrix3f;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAdd](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setAdd(android.renderscript.Float4))
        ///
        /// Required features: "android-renderscript-Float4"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Float4")))]
        pub fn setAdd_Float4<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Float4>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setAdd", .descriptor == "(Landroid/renderscript/Float4;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setAdd\0", "(Landroid/renderscript/Float4;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAdd](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setAdd(float,%20float,%20float,%20float))
        pub fn setAdd_float_float_float_float<'env>(&'env self, arg0: f32, arg1: f32, arg2: f32, arg3: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setAdd", .descriptor == "(FFFF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setAdd\0", "(FFFF)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setGreyscale](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setGreyscale())
        pub fn setGreyscale<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setGreyscale", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setGreyscale\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setYUVtoRGB](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setYUVtoRGB())
        pub fn setYUVtoRGB<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setYUVtoRGB", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setYUVtoRGB\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRGBtoYUV](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#setRGBtoYUV())
        pub fn setRGBtoYUV<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "setRGBtoYUV", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "setRGBtoYUV\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEach](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#forEach(android.renderscript.Allocation,%20android.renderscript.Allocation))
        ///
        /// Required features: "android-renderscript-Allocation"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Allocation")))]
        pub fn forEach_Allocation_Allocation<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "forEach", .descriptor == "(Landroid/renderscript/Allocation;Landroid/renderscript/Allocation;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "forEach\0", "(Landroid/renderscript/Allocation;Landroid/renderscript/Allocation;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forEach](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#forEach(android.renderscript.Allocation,%20android.renderscript.Allocation,%20android.renderscript.Script.LaunchOptions))
        ///
        /// Required features: "android-renderscript-Allocation", "android-renderscript-Script_LaunchOptions"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Allocation", feature = "android-renderscript-Script_LaunchOptions")))]
        pub fn forEach_Allocation_Allocation_LaunchOptions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Allocation>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Script_LaunchOptions>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "forEach", .descriptor == "(Landroid/renderscript/Allocation;Landroid/renderscript/Allocation;Landroid/renderscript/Script$LaunchOptions;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "forEach\0", "(Landroid/renderscript/Allocation;Landroid/renderscript/Allocation;Landroid/renderscript/Script$LaunchOptions;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKernelID](https://developer.android.com/reference/android/renderscript/ScriptIntrinsicColorMatrix.html#getKernelID())
        ///
        /// Required features: "android-renderscript-Script_KernelID"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-Script_KernelID")))]
        pub fn getKernelID<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::Script_KernelID>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptIntrinsicColorMatrix", java.flags == PUBLIC, .name == "getKernelID", .descriptor == "()Landroid/renderscript/Script$KernelID;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptIntrinsicColorMatrix\0", "getKernelID\0", "()Landroid/renderscript/Script$KernelID;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
