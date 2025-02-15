// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-fonts-Font"))]
__jni_bindgen! {
    /// public final class [Font](https://developer.android.com/reference/android/graphics/fonts/Font.html)
    ///
    /// Required feature: android-graphics-fonts-Font
    public final class Font ("android/graphics/fonts/Font") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Font](https://developer.android.com/reference/android/graphics/fonts/Font.html#Font(long,%20java.nio.ByteBuffer,%20java.io.File,%20android.graphics.fonts.FontStyle,%20int,%20android.graphics.fonts.FontVariationAxis%5B%5D,%20java.lang.String))
        // ///
        // /// Required features: "android-graphics-fonts-FontStyle", "android-graphics-fonts-FontVariationAxis", "java-io-File", "java-lang-String", "java-nio-ByteBuffer"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-fonts-FontStyle", feature = "android-graphics-fonts-FontVariationAxis", feature = "java-io-File", feature = "java-lang-String", feature = "java-nio-ByteBuffer")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::fonts::FontStyle>>, arg4: i32, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::graphics::fonts::FontVariationAxis, crate::java::lang::Throwable>>>, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::fonts::Font>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/fonts/Font", java.flags == (empty), .name == "<init>", .descriptor == "(JLjava/nio/ByteBuffer;Ljava/io/File;Landroid/graphics/fonts/FontStyle;I[Landroid/graphics/fonts/FontVariationAxis;Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "<init>\0", "(JLjava/nio/ByteBuffer;Ljava/io/File;Landroid/graphics/fonts/FontStyle;I[Landroid/graphics/fonts/FontVariationAxis;Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getBuffer](https://developer.android.com/reference/android/graphics/fonts/Font.html#getBuffer())
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn getBuffer<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ByteBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "getBuffer", .descriptor == "()Ljava/nio/ByteBuffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "getBuffer\0", "()Ljava/nio/ByteBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFile](https://developer.android.com/reference/android/graphics/fonts/Font.html#getFile())
        ///
        /// Required features: "java-io-File"
        #[cfg(any(feature = "all", all(feature = "java-io-File")))]
        pub fn getFile<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::File>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "getFile", .descriptor == "()Ljava/io/File;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "getFile\0", "()Ljava/io/File;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStyle](https://developer.android.com/reference/android/graphics/fonts/Font.html#getStyle())
        ///
        /// Required features: "android-graphics-fonts-FontStyle"
        #[cfg(any(feature = "all", all(feature = "android-graphics-fonts-FontStyle")))]
        pub fn getStyle<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::fonts::FontStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "getStyle", .descriptor == "()Landroid/graphics/fonts/FontStyle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "getStyle\0", "()Landroid/graphics/fonts/FontStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTtcIndex](https://developer.android.com/reference/android/graphics/fonts/Font.html#getTtcIndex())
        pub fn getTtcIndex<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "getTtcIndex", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "getTtcIndex\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAxes](https://developer.android.com/reference/android/graphics/fonts/Font.html#getAxes())
        ///
        /// Required features: "android-graphics-fonts-FontVariationAxis"
        #[cfg(any(feature = "all", all(feature = "android-graphics-fonts-FontVariationAxis")))]
        pub fn getAxes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::graphics::fonts::FontVariationAxis, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "getAxes", .descriptor == "()[Landroid/graphics/fonts/FontVariationAxis;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "getAxes\0", "()[Landroid/graphics/fonts/FontVariationAxis;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocaleList](https://developer.android.com/reference/android/graphics/fonts/Font.html#getLocaleList())
        ///
        /// Required features: "android-os-LocaleList"
        #[cfg(any(feature = "all", all(feature = "android-os-LocaleList")))]
        pub fn getLocaleList<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::LocaleList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "getLocaleList", .descriptor == "()Landroid/os/LocaleList;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "getLocaleList\0", "()Landroid/os/LocaleList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/graphics/fonts/Font.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/graphics/fonts/Font.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/graphics/fonts/Font.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/fonts/Font", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/fonts/Font\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
