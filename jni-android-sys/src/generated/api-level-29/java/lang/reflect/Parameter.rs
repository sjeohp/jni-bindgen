// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-reflect-Parameter"))]
__jni_bindgen! {
    /// public final class [Parameter](https://developer.android.com/reference/java/lang/reflect/Parameter.html)
    ///
    /// Required feature: java-lang-reflect-Parameter
    public final class Parameter ("java/lang/reflect/Parameter") extends crate::java::lang::Object, implements crate::java::lang::reflect::AnnotatedElement {

        // // Not emitting: Non-public method
        // /// [Parameter](https://developer.android.com/reference/java/lang/reflect/Parameter.html#Parameter(java.lang.String,%20int,%20java.lang.reflect.Executable,%20int))
        // ///
        // /// Required features: "java-lang-String", "java-lang-reflect-Executable"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-lang-reflect-Executable")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::reflect::Executable>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::reflect::Parameter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/lang/reflect/Parameter", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/lang/String;ILjava/lang/reflect/Executable;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "<init>\0", "(Ljava/lang/String;ILjava/lang/reflect/Executable;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [equals](https://developer.android.com/reference/java/lang/reflect/Parameter.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/lang/reflect/Parameter.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isNamePresent](https://developer.android.com/reference/java/lang/reflect/Parameter.html#isNamePresent())
        pub fn isNamePresent<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "isNamePresent", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "isNamePresent\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/lang/reflect/Parameter.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDeclaringExecutable](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getDeclaringExecutable())
        ///
        /// Required features: "java-lang-reflect-Executable"
        #[cfg(any(feature = "all", all(feature = "java-lang-reflect-Executable")))]
        pub fn getDeclaringExecutable<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::reflect::Executable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getDeclaringExecutable", .descriptor == "()Ljava/lang/reflect/Executable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getDeclaringExecutable\0", "()Ljava/lang/reflect/Executable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getModifiers](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getModifiers())
        pub fn getModifiers<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getModifiers", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getModifiers\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParameterizedType](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getParameterizedType())
        ///
        /// Required features: "java-lang-reflect-Type"
        #[cfg(any(feature = "all", all(feature = "java-lang-reflect-Type")))]
        pub fn getParameterizedType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::reflect::Type>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getParameterizedType", .descriptor == "()Ljava/lang/reflect/Type;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getParameterizedType\0", "()Ljava/lang/reflect/Type;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getType](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getType())
        ///
        /// Required features: "java-lang-Class"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class")))]
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Class>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getType", .descriptor == "()Ljava/lang/Class;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getType\0", "()Ljava/lang/Class;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isImplicit](https://developer.android.com/reference/java/lang/reflect/Parameter.html#isImplicit())
        pub fn isImplicit<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "isImplicit", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "isImplicit\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSynthetic](https://developer.android.com/reference/java/lang/reflect/Parameter.html#isSynthetic())
        pub fn isSynthetic<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "isSynthetic", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "isSynthetic\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isVarArgs](https://developer.android.com/reference/java/lang/reflect/Parameter.html#isVarArgs())
        pub fn isVarArgs<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "isVarArgs", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "isVarArgs\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAnnotation](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getAnnotation(java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-lang-annotation-Annotation"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-annotation-Annotation")))]
        pub fn getAnnotation<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::annotation::Annotation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getAnnotation", .descriptor == "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getAnnotation\0", "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAnnotationsByType](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getAnnotationsByType(java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-lang-annotation-Annotation"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-annotation-Annotation")))]
        pub fn getAnnotationsByType<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::annotation::Annotation, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getAnnotationsByType", .descriptor == "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getAnnotationsByType\0", "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDeclaredAnnotations](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getDeclaredAnnotations())
        ///
        /// Required features: "java-lang-annotation-Annotation"
        #[cfg(any(feature = "all", all(feature = "java-lang-annotation-Annotation")))]
        pub fn getDeclaredAnnotations<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::annotation::Annotation, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getDeclaredAnnotations", .descriptor == "()[Ljava/lang/annotation/Annotation;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getDeclaredAnnotations\0", "()[Ljava/lang/annotation/Annotation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDeclaredAnnotation](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getDeclaredAnnotation(java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-lang-annotation-Annotation"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-annotation-Annotation")))]
        pub fn getDeclaredAnnotation<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::annotation::Annotation>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getDeclaredAnnotation", .descriptor == "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getDeclaredAnnotation\0", "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDeclaredAnnotationsByType](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getDeclaredAnnotationsByType(java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-lang-annotation-Annotation"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-annotation-Annotation")))]
        pub fn getDeclaredAnnotationsByType<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::annotation::Annotation, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getDeclaredAnnotationsByType", .descriptor == "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getDeclaredAnnotationsByType\0", "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAnnotations](https://developer.android.com/reference/java/lang/reflect/Parameter.html#getAnnotations())
        ///
        /// Required features: "java-lang-annotation-Annotation"
        #[cfg(any(feature = "all", all(feature = "java-lang-annotation-Annotation")))]
        pub fn getAnnotations<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::annotation::Annotation, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Parameter", java.flags == PUBLIC, .name == "getAnnotations", .descriptor == "()[Ljava/lang/annotation/Annotation;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Parameter\0", "getAnnotations\0", "()[Ljava/lang/annotation/Annotation;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
