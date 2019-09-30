// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-Person_Builder"))]
__jni_bindgen! {
    /// public class [Person.Builder](https://developer.android.com/reference/android/app/Person.Builder.html)
    ///
    /// Required feature: android-app-Person_Builder
    public class Person_Builder ("android/app/Person$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/app/Person.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setName](https://developer.android.com/reference/android/app/Person.Builder.html#setName(java.lang.CharSequence))
        ///
        /// Required features: "android-app-Person_Builder", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-app-Person_Builder", feature = "java-lang-CharSequence")))]
        pub fn setName<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "setName", .descriptor == "(Ljava/lang/CharSequence;)Landroid/app/Person$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "setName\0", "(Ljava/lang/CharSequence;)Landroid/app/Person$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIcon](https://developer.android.com/reference/android/app/Person.Builder.html#setIcon(android.graphics.drawable.Icon))
        ///
        /// Required features: "android-app-Person_Builder", "android-graphics-drawable-Icon"
        #[cfg(any(feature = "all", all(feature = "android-app-Person_Builder", feature = "android-graphics-drawable-Icon")))]
        pub fn setIcon<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::Icon>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "setIcon", .descriptor == "(Landroid/graphics/drawable/Icon;)Landroid/app/Person$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "setIcon\0", "(Landroid/graphics/drawable/Icon;)Landroid/app/Person$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setUri](https://developer.android.com/reference/android/app/Person.Builder.html#setUri(java.lang.String))
        ///
        /// Required features: "android-app-Person_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-Person_Builder", feature = "java-lang-String")))]
        pub fn setUri<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "setUri", .descriptor == "(Ljava/lang/String;)Landroid/app/Person$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "setUri\0", "(Ljava/lang/String;)Landroid/app/Person$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setKey](https://developer.android.com/reference/android/app/Person.Builder.html#setKey(java.lang.String))
        ///
        /// Required features: "android-app-Person_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-Person_Builder", feature = "java-lang-String")))]
        pub fn setKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "setKey", .descriptor == "(Ljava/lang/String;)Landroid/app/Person$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "setKey\0", "(Ljava/lang/String;)Landroid/app/Person$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setImportant](https://developer.android.com/reference/android/app/Person.Builder.html#setImportant(boolean))
        ///
        /// Required features: "android-app-Person_Builder"
        #[cfg(any(feature = "all", all(feature = "android-app-Person_Builder")))]
        pub fn setImportant<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "setImportant", .descriptor == "(Z)Landroid/app/Person$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "setImportant\0", "(Z)Landroid/app/Person$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBot](https://developer.android.com/reference/android/app/Person.Builder.html#setBot(boolean))
        ///
        /// Required features: "android-app-Person_Builder"
        #[cfg(any(feature = "all", all(feature = "android-app-Person_Builder")))]
        pub fn setBot<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "setBot", .descriptor == "(Z)Landroid/app/Person$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "setBot\0", "(Z)Landroid/app/Person$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/app/Person.Builder.html#build())
        ///
        /// Required features: "android-app-Person"
        #[cfg(any(feature = "all", all(feature = "android-app-Person")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Person>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Person$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/app/Person;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Person$Builder\0", "build\0", "()Landroid/app/Person;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
