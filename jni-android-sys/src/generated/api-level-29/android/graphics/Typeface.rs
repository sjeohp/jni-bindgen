// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-Typeface"))]
__jni_bindgen! {
    /// public class [Typeface](https://developer.android.com/reference/android/graphics/Typeface.html)
    ///
    /// Required feature: android-graphics-Typeface
    public class Typeface ("android/graphics/Typeface") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Typeface](https://developer.android.com/reference/android/graphics/Typeface.html#Typeface(long))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/Typeface", java.flags == (empty), .name == "<init>", .descriptor == "(J)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "<init>\0", "(J)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getWeight](https://developer.android.com/reference/android/graphics/Typeface.html#getWeight())
        pub fn getWeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC, .name == "getWeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "getWeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStyle](https://developer.android.com/reference/android/graphics/Typeface.html#getStyle())
        pub fn getStyle<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC, .name == "getStyle", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "getStyle\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isBold](https://developer.android.com/reference/android/graphics/Typeface.html#isBold())
        pub fn isBold<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | FINAL, .name == "isBold", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "isBold\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isItalic](https://developer.android.com/reference/android/graphics/Typeface.html#isItalic())
        pub fn isItalic<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | FINAL, .name == "isItalic", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "isItalic\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/graphics/Typeface.html#create(java.lang.String,%20int))
        ///
        /// Required features: "android-graphics-Typeface", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface", feature = "java-lang-String")))]
        pub fn create_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Ljava/lang/String;I)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "create\0", "(Ljava/lang/String;I)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/graphics/Typeface.html#create(android.graphics.Typeface,%20int))
        ///
        /// Required features: "android-graphics-Typeface"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface")))]
        pub fn create_Typeface_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Typeface>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/graphics/Typeface;I)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "create\0", "(Landroid/graphics/Typeface;I)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [create](https://developer.android.com/reference/android/graphics/Typeface.html#create(android.graphics.Typeface,%20int,%20boolean))
        ///
        /// Required features: "android-graphics-Typeface"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface")))]
        pub fn create_Typeface_int_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Typeface>>, arg1: i32, arg2: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(Landroid/graphics/Typeface;IZ)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "create\0", "(Landroid/graphics/Typeface;IZ)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [defaultFromStyle](https://developer.android.com/reference/android/graphics/Typeface.html#defaultFromStyle(int))
        ///
        /// Required features: "android-graphics-Typeface"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface")))]
        pub fn defaultFromStyle<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "defaultFromStyle", .descriptor == "(I)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "defaultFromStyle\0", "(I)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createFromAsset](https://developer.android.com/reference/android/graphics/Typeface.html#createFromAsset(android.content.res.AssetManager,%20java.lang.String))
        ///
        /// Required features: "android-content-res-AssetManager", "android-graphics-Typeface", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-res-AssetManager", feature = "android-graphics-Typeface", feature = "java-lang-String")))]
        pub fn createFromAsset<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::AssetManager>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "createFromAsset", .descriptor == "(Landroid/content/res/AssetManager;Ljava/lang/String;)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "createFromAsset\0", "(Landroid/content/res/AssetManager;Ljava/lang/String;)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createFromFile](https://developer.android.com/reference/android/graphics/Typeface.html#createFromFile(java.io.File))
        ///
        /// Required features: "android-graphics-Typeface", "java-io-File"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface", feature = "java-io-File")))]
        pub fn createFromFile_File<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "createFromFile", .descriptor == "(Ljava/io/File;)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "createFromFile\0", "(Ljava/io/File;)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createFromFile](https://developer.android.com/reference/android/graphics/Typeface.html#createFromFile(java.lang.String))
        ///
        /// Required features: "android-graphics-Typeface", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Typeface", feature = "java-lang-String")))]
        pub fn createFromFile_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC | STATIC, .name == "createFromFile", .descriptor == "(Ljava/lang/String;)Landroid/graphics/Typeface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/graphics/Typeface\0", "createFromFile\0", "(Ljava/lang/String;)Landroid/graphics/Typeface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/graphics/Typeface.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/graphics/Typeface.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Typeface", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Typeface\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [BOLD](https://developer.android.com/reference/android/graphics/Typeface.html#BOLD)
        pub const BOLD : i32 = 1;

        /// public static final [BOLD_ITALIC](https://developer.android.com/reference/android/graphics/Typeface.html#BOLD_ITALIC)
        pub const BOLD_ITALIC : i32 = 3;

        /// **get** public static final [DEFAULT](https://developer.android.com/reference/android/graphics/Typeface.html#DEFAULT)
        ///
        /// Required feature: android-graphics-Typeface
        #[cfg(any(feature = "all", feature = "android-graphics-Typeface"))]
        pub fn DEFAULT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Typeface\0", "DEFAULT\0", "Landroid/graphics/Typeface;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DEFAULT_BOLD](https://developer.android.com/reference/android/graphics/Typeface.html#DEFAULT_BOLD)
        ///
        /// Required feature: android-graphics-Typeface
        #[cfg(any(feature = "all", feature = "android-graphics-Typeface"))]
        pub fn DEFAULT_BOLD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Typeface\0", "DEFAULT_BOLD\0", "Landroid/graphics/Typeface;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [ITALIC](https://developer.android.com/reference/android/graphics/Typeface.html#ITALIC)
        pub const ITALIC : i32 = 2;

        /// **get** public static final [MONOSPACE](https://developer.android.com/reference/android/graphics/Typeface.html#MONOSPACE)
        ///
        /// Required feature: android-graphics-Typeface
        #[cfg(any(feature = "all", feature = "android-graphics-Typeface"))]
        pub fn MONOSPACE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Typeface\0", "MONOSPACE\0", "Landroid/graphics/Typeface;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [NORMAL](https://developer.android.com/reference/android/graphics/Typeface.html#NORMAL)
        pub const NORMAL : i32 = 0;

        /// **get** public static final [SANS_SERIF](https://developer.android.com/reference/android/graphics/Typeface.html#SANS_SERIF)
        ///
        /// Required feature: android-graphics-Typeface
        #[cfg(any(feature = "all", feature = "android-graphics-Typeface"))]
        pub fn SANS_SERIF<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Typeface\0", "SANS_SERIF\0", "Landroid/graphics/Typeface;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SERIF](https://developer.android.com/reference/android/graphics/Typeface.html#SERIF)
        ///
        /// Required feature: android-graphics-Typeface
        #[cfg(any(feature = "all", feature = "android-graphics-Typeface"))]
        pub fn SERIF<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Typeface>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/graphics/Typeface\0", "SERIF\0", "Landroid/graphics/Typeface;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
