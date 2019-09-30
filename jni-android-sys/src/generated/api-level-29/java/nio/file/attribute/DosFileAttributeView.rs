// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-attribute-DosFileAttributeView"))]
__jni_bindgen! {
    /// public interface [DosFileAttributeView](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html)
    ///
    /// Required feature: java-nio-file-attribute-DosFileAttributeView
    public interface DosFileAttributeView ("java/nio/file/attribute/DosFileAttributeView") extends crate::java::lang::Object, implements crate::java::nio::file::attribute::BasicFileAttributeView {

        /// [name](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#name())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn name<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | ABSTRACT, .name == "name", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "name\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readAttributes](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#readAttributes())
        ///
        /// Required features: "java-nio-file-attribute-DosFileAttributes"
        #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-DosFileAttributes")))]
        pub fn readAttributes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::attribute::DosFileAttributes>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | ABSTRACT, .name == "readAttributes", .descriptor == "()Ljava/nio/file/attribute/DosFileAttributes;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "readAttributes\0", "()Ljava/nio/file/attribute/DosFileAttributes;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setReadOnly](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#setReadOnly(boolean))
        pub fn setReadOnly<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | ABSTRACT, .name == "setReadOnly", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "setReadOnly\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHidden](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#setHidden(boolean))
        pub fn setHidden<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | ABSTRACT, .name == "setHidden", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "setHidden\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSystem](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#setSystem(boolean))
        pub fn setSystem<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | ABSTRACT, .name == "setSystem", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "setSystem\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setArchive](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#setArchive(boolean))
        pub fn setArchive<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | ABSTRACT, .name == "setArchive", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "setArchive\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [readAttributes](https://developer.android.com/reference/java/nio/file/attribute/DosFileAttributeView.html#readAttributes())
        // ///
        // /// Required features: "java-nio-file-attribute-BasicFileAttributes"
        // #[cfg(any(feature = "all", all(feature = "java-nio-file-attribute-BasicFileAttributes")))]
        // pub fn readAttributes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::file::attribute::BasicFileAttributes>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/file/attribute/DosFileAttributeView", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "readAttributes", .descriptor == "()Ljava/nio/file/attribute/BasicFileAttributes;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/DosFileAttributeView\0", "readAttributes\0", "()Ljava/nio/file/attribute/BasicFileAttributes;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
