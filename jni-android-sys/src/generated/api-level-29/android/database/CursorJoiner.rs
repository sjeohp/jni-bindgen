// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-CursorJoiner"))]
__jni_bindgen! {
    /// public final class [CursorJoiner](https://developer.android.com/reference/android/database/CursorJoiner.html)
    ///
    /// Required feature: android-database-CursorJoiner
    public final class CursorJoiner ("android/database/CursorJoiner") extends crate::java::lang::Object, implements crate::java::util::Iterator, crate::java::lang::Iterable {

        /// [CursorJoiner](https://developer.android.com/reference/android/database/CursorJoiner.html#CursorJoiner(android.database.Cursor,%20java.lang.String%5B%5D,%20android.database.Cursor,%20java.lang.String%5B%5D))
        ///
        /// Required features: "android-database-Cursor", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-database-Cursor", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::Cursor>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::CursorJoiner>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorJoiner", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/database/Cursor;[Ljava/lang/String;Landroid/database/Cursor;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorJoiner\0", "<init>\0", "(Landroid/database/Cursor;[Ljava/lang/String;Landroid/database/Cursor;[Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/android/database/CursorJoiner.html#iterator())
        ///
        /// Required features: "java-util-Iterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator")))]
        pub fn iterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Iterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorJoiner", java.flags == PUBLIC, .name == "iterator", .descriptor == "()Ljava/util/Iterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorJoiner\0", "iterator\0", "()Ljava/util/Iterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasNext](https://developer.android.com/reference/android/database/CursorJoiner.html#hasNext())
        pub fn hasNext<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorJoiner", java.flags == PUBLIC, .name == "hasNext", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorJoiner\0", "hasNext\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [next](https://developer.android.com/reference/android/database/CursorJoiner.html#next())
        ///
        /// Required features: "android-database-CursorJoiner_Result"
        #[cfg(any(feature = "all", all(feature = "android-database-CursorJoiner_Result")))]
        pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::database::CursorJoiner_Result>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorJoiner", java.flags == PUBLIC, .name == "next", .descriptor == "()Landroid/database/CursorJoiner$Result;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorJoiner\0", "next\0", "()Landroid/database/CursorJoiner$Result;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remove](https://developer.android.com/reference/android/database/CursorJoiner.html#remove())
        pub fn remove<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/CursorJoiner", java.flags == PUBLIC, .name == "remove", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorJoiner\0", "remove\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [next](https://developer.android.com/reference/android/database/CursorJoiner.html#next())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn next<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/database/CursorJoiner", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "next", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/CursorJoiner\0", "next\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
