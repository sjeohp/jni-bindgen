// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-renderscript-ScriptGroup_Builder2"))]
__jni_bindgen! {
    /// public final class [ScriptGroup.Builder2](https://developer.android.com/reference/android/renderscript/ScriptGroup.Builder2.html)
    ///
    /// Required feature: android-renderscript-ScriptGroup_Builder2
    public final class ScriptGroup_Builder2 ("android/renderscript/ScriptGroup$Builder2") extends crate::java::lang::Object {

        /// [Builder2](https://developer.android.com/reference/android/renderscript/ScriptGroup.Builder2.html#Builder2(android.renderscript.RenderScript))
        ///
        /// Required features: "android-renderscript-RenderScript"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-RenderScript")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::RenderScript>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptGroup_Builder2>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptGroup$Builder2", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/renderscript/RenderScript;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptGroup$Builder2\0", "<init>\0", "(Landroid/renderscript/RenderScript;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addInput](https://developer.android.com/reference/android/renderscript/ScriptGroup.Builder2.html#addInput())
        ///
        /// Required features: "android-renderscript-ScriptGroup_Input"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-ScriptGroup_Input")))]
        pub fn addInput<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptGroup_Input>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptGroup$Builder2", java.flags == PUBLIC, .name == "addInput", .descriptor == "()Landroid/renderscript/ScriptGroup$Input;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptGroup$Builder2\0", "addInput\0", "()Landroid/renderscript/ScriptGroup$Input;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addKernel](https://developer.android.com/reference/android/renderscript/ScriptGroup.Builder2.html#addKernel(android.renderscript.Script.KernelID,%20android.renderscript.Type,%20java.lang.Object...))
        ///
        /// Required features: "android-renderscript-ScriptGroup_Closure", "android-renderscript-Script_KernelID", "android-renderscript-Type", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-ScriptGroup_Closure", feature = "android-renderscript-Script_KernelID", feature = "android-renderscript-Type", feature = "java-lang-Object")))]
        pub fn addKernel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Script_KernelID>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Type>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptGroup_Closure>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptGroup$Builder2", java.flags == PUBLIC | VARARGS, .name == "addKernel", .descriptor == "(Landroid/renderscript/Script$KernelID;Landroid/renderscript/Type;[Ljava/lang/Object;)Landroid/renderscript/ScriptGroup$Closure;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptGroup$Builder2\0", "addKernel\0", "(Landroid/renderscript/Script$KernelID;Landroid/renderscript/Type;[Ljava/lang/Object;)Landroid/renderscript/ScriptGroup$Closure;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addInvoke](https://developer.android.com/reference/android/renderscript/ScriptGroup.Builder2.html#addInvoke(android.renderscript.Script.InvokeID,%20java.lang.Object...))
        ///
        /// Required features: "android-renderscript-ScriptGroup_Closure", "android-renderscript-Script_InvokeID", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-ScriptGroup_Closure", feature = "android-renderscript-Script_InvokeID", feature = "java-lang-Object")))]
        pub fn addInvoke<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::renderscript::Script_InvokeID>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptGroup_Closure>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptGroup$Builder2", java.flags == PUBLIC | VARARGS, .name == "addInvoke", .descriptor == "(Landroid/renderscript/Script$InvokeID;[Ljava/lang/Object;)Landroid/renderscript/ScriptGroup$Closure;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptGroup$Builder2\0", "addInvoke\0", "(Landroid/renderscript/Script$InvokeID;[Ljava/lang/Object;)Landroid/renderscript/ScriptGroup$Closure;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/renderscript/ScriptGroup.Builder2.html#create(java.lang.String,%20android.renderscript.ScriptGroup.Future...))
        ///
        /// Required features: "android-renderscript-ScriptGroup", "android-renderscript-ScriptGroup_Future", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-renderscript-ScriptGroup", feature = "android-renderscript-ScriptGroup_Future", feature = "java-lang-String")))]
        pub fn create<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::renderscript::ScriptGroup_Future, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::renderscript::ScriptGroup>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/renderscript/ScriptGroup$Builder2", java.flags == PUBLIC | VARARGS, .name == "create", .descriptor == "(Ljava/lang/String;[Landroid/renderscript/ScriptGroup$Future;)Landroid/renderscript/ScriptGroup;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/renderscript/ScriptGroup$Builder2\0", "create\0", "(Ljava/lang/String;[Landroid/renderscript/ScriptGroup$Future;)Landroid/renderscript/ScriptGroup;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
