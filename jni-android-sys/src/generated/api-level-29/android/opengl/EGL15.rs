// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-opengl-EGL15"))]
__jni_bindgen! {
    /// public final class [EGL15](https://developer.android.com/reference/android/opengl/EGL15.html)
    ///
    /// Required feature: android-opengl-EGL15
    public final class EGL15 ("android/opengl/EGL15") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [EGL15](https://developer.android.com/reference/android/opengl/EGL15.html#EGL15())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::opengl::EGL15>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/opengl/EGL15", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/opengl/EGL15\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [eglCreateSync](https://developer.android.com/reference/android/opengl/EGL15.html#eglCreateSync(android.opengl.EGLDisplay,%20int,%20long%5B%5D,%20int))
        ///
        /// Required features: "android-opengl-EGLDisplay", "android-opengl-EGLSync"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSync")))]
        pub fn eglCreateSync<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLSync>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglCreateSync", .descriptor == "(Landroid/opengl/EGLDisplay;I[JI)Landroid/opengl/EGLSync;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglCreateSync\0", "(Landroid/opengl/EGLDisplay;I[JI)Landroid/opengl/EGLSync;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglGetSyncAttrib](https://developer.android.com/reference/android/opengl/EGL15.html#eglGetSyncAttrib(android.opengl.EGLDisplay,%20android.opengl.EGLSync,%20int,%20long%5B%5D,%20int))
        ///
        /// Required features: "android-opengl-EGLDisplay", "android-opengl-EGLSync"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSync")))]
        pub fn eglGetSyncAttrib<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLSync>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg4: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglGetSyncAttrib", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;I[JI)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglGetSyncAttrib\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;I[JI)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglDestroySync](https://developer.android.com/reference/android/opengl/EGL15.html#eglDestroySync(android.opengl.EGLDisplay,%20android.opengl.EGLSync))
        ///
        /// Required features: "android-opengl-EGLDisplay", "android-opengl-EGLSync"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSync")))]
        pub fn eglDestroySync<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLSync>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglDestroySync", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglDestroySync\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglClientWaitSync](https://developer.android.com/reference/android/opengl/EGL15.html#eglClientWaitSync(android.opengl.EGLDisplay,%20android.opengl.EGLSync,%20int,%20long))
        ///
        /// Required features: "android-opengl-EGLDisplay", "android-opengl-EGLSync"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSync")))]
        pub fn eglClientWaitSync<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLSync>>, arg2: i32, arg3: i64) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglClientWaitSync", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;IJ)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglClientWaitSync\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;IJ)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglGetPlatformDisplay](https://developer.android.com/reference/android/opengl/EGL15.html#eglGetPlatformDisplay(int,%20long,%20long%5B%5D,%20int))
        ///
        /// Required features: "android-opengl-EGLDisplay"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay")))]
        pub fn eglGetPlatformDisplay<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i64, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLDisplay>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglGetPlatformDisplay", .descriptor == "(IJ[JI)Landroid/opengl/EGLDisplay;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglGetPlatformDisplay\0", "(IJ[JI)Landroid/opengl/EGLDisplay;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglCreatePlatformWindowSurface](https://developer.android.com/reference/android/opengl/EGL15.html#eglCreatePlatformWindowSurface(android.opengl.EGLDisplay,%20android.opengl.EGLConfig,%20java.nio.Buffer,%20long%5B%5D,%20int))
        ///
        /// Required features: "android-opengl-EGLConfig", "android-opengl-EGLDisplay", "android-opengl-EGLSurface", "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLConfig", feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSurface", feature = "java-nio-Buffer")))]
        pub fn eglCreatePlatformWindowSurface<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLConfig>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::Buffer>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLSurface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglCreatePlatformWindowSurface", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLConfig;Ljava/nio/Buffer;[JI)Landroid/opengl/EGLSurface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglCreatePlatformWindowSurface\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLConfig;Ljava/nio/Buffer;[JI)Landroid/opengl/EGLSurface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglCreatePlatformPixmapSurface](https://developer.android.com/reference/android/opengl/EGL15.html#eglCreatePlatformPixmapSurface(android.opengl.EGLDisplay,%20android.opengl.EGLConfig,%20java.nio.Buffer,%20long%5B%5D,%20int))
        ///
        /// Required features: "android-opengl-EGLConfig", "android-opengl-EGLDisplay", "android-opengl-EGLSurface", "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLConfig", feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSurface", feature = "java-nio-Buffer")))]
        pub fn eglCreatePlatformPixmapSurface<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLConfig>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::Buffer>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLSurface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglCreatePlatformPixmapSurface", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLConfig;Ljava/nio/Buffer;[JI)Landroid/opengl/EGLSurface;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglCreatePlatformPixmapSurface\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLConfig;Ljava/nio/Buffer;[JI)Landroid/opengl/EGLSurface;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglWaitSync](https://developer.android.com/reference/android/opengl/EGL15.html#eglWaitSync(android.opengl.EGLDisplay,%20android.opengl.EGLSync,%20int))
        ///
        /// Required features: "android-opengl-EGLDisplay", "android-opengl-EGLSync"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLSync")))]
        pub fn eglWaitSync<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLSync>>, arg2: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglWaitSync", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglWaitSync\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLSync;I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglCreateImage](https://developer.android.com/reference/android/opengl/EGL15.html#eglCreateImage(android.opengl.EGLDisplay,%20android.opengl.EGLContext,%20int,%20long,%20long%5B%5D,%20int))
        ///
        /// Required features: "android-opengl-EGLContext", "android-opengl-EGLDisplay", "android-opengl-EGLImage"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLContext", feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLImage")))]
        pub fn eglCreateImage<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLContext>>, arg2: i32, arg3: i64, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLImage>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglCreateImage", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLContext;IJ[JI)Landroid/opengl/EGLImage;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglCreateImage\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLContext;IJ[JI)Landroid/opengl/EGLImage;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [eglDestroyImage](https://developer.android.com/reference/android/opengl/EGL15.html#eglDestroyImage(android.opengl.EGLDisplay,%20android.opengl.EGLImage))
        ///
        /// Required features: "android-opengl-EGLDisplay", "android-opengl-EGLImage"
        #[cfg(any(feature = "all", all(feature = "android-opengl-EGLDisplay", feature = "android-opengl-EGLImage")))]
        pub fn eglDestroyImage<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLDisplay>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::opengl::EGLImage>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/opengl/EGL15", java.flags == PUBLIC | STATIC | NATIVE, .name == "eglDestroyImage", .descriptor == "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLImage;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/opengl/EGL15\0", "eglDestroyImage\0", "(Landroid/opengl/EGLDisplay;Landroid/opengl/EGLImage;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [EGL_CL_EVENT_HANDLE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CL_EVENT_HANDLE)
        pub const EGL_CL_EVENT_HANDLE : i32 = 12444;

        /// public static final [EGL_CONDITION_SATISFIED](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONDITION_SATISFIED)
        pub const EGL_CONDITION_SATISFIED : i32 = 12534;

        /// public static final [EGL_CONTEXT_MAJOR_VERSION](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_MAJOR_VERSION)
        pub const EGL_CONTEXT_MAJOR_VERSION : i32 = 12440;

        /// public static final [EGL_CONTEXT_MINOR_VERSION](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_MINOR_VERSION)
        pub const EGL_CONTEXT_MINOR_VERSION : i32 = 12539;

        /// public static final [EGL_CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT)
        pub const EGL_CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT : i32 = 2;

        /// public static final [EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT)
        pub const EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT : i32 = 1;

        /// public static final [EGL_CONTEXT_OPENGL_DEBUG](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_DEBUG)
        pub const EGL_CONTEXT_OPENGL_DEBUG : i32 = 12720;

        /// public static final [EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE)
        pub const EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE : i32 = 12721;

        /// public static final [EGL_CONTEXT_OPENGL_PROFILE_MASK](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_PROFILE_MASK)
        pub const EGL_CONTEXT_OPENGL_PROFILE_MASK : i32 = 12541;

        /// public static final [EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY)
        pub const EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY : i32 = 12733;

        /// public static final [EGL_CONTEXT_OPENGL_ROBUST_ACCESS](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_CONTEXT_OPENGL_ROBUST_ACCESS)
        pub const EGL_CONTEXT_OPENGL_ROBUST_ACCESS : i32 = 12722;

        /// public static final [EGL_FOREVER](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_FOREVER)
        pub const EGL_FOREVER : i64 = -1i64;

        /// public static final [EGL_GL_COLORSPACE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_COLORSPACE)
        pub const EGL_GL_COLORSPACE : i32 = 12445;

        /// public static final [EGL_GL_COLORSPACE_LINEAR](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_COLORSPACE_LINEAR)
        pub const EGL_GL_COLORSPACE_LINEAR : i32 = 12426;

        /// public static final [EGL_GL_COLORSPACE_SRGB](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_COLORSPACE_SRGB)
        pub const EGL_GL_COLORSPACE_SRGB : i32 = 12425;

        /// public static final [EGL_GL_RENDERBUFFER](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_RENDERBUFFER)
        pub const EGL_GL_RENDERBUFFER : i32 = 12473;

        /// public static final [EGL_GL_TEXTURE_2D](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_2D)
        pub const EGL_GL_TEXTURE_2D : i32 = 12465;

        /// public static final [EGL_GL_TEXTURE_3D](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_3D)
        pub const EGL_GL_TEXTURE_3D : i32 = 12466;

        /// public static final [EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_X](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_X)
        pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_X : i32 = 12468;

        /// public static final [EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Y](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Y)
        pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Y : i32 = 12470;

        /// public static final [EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Z](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Z)
        pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Z : i32 = 12472;

        /// public static final [EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_X](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_X)
        pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_X : i32 = 12467;

        /// public static final [EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Y](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Y)
        pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Y : i32 = 12469;

        /// public static final [EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Z](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Z)
        pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Z : i32 = 12471;

        /// public static final [EGL_GL_TEXTURE_LEVEL](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_LEVEL)
        pub const EGL_GL_TEXTURE_LEVEL : i32 = 12476;

        /// public static final [EGL_GL_TEXTURE_ZOFFSET](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_GL_TEXTURE_ZOFFSET)
        pub const EGL_GL_TEXTURE_ZOFFSET : i32 = 12477;

        /// public static final [EGL_IMAGE_PRESERVED](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_IMAGE_PRESERVED)
        pub const EGL_IMAGE_PRESERVED : i32 = 12498;

        /// public static final [EGL_LOSE_CONTEXT_ON_RESET](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_LOSE_CONTEXT_ON_RESET)
        pub const EGL_LOSE_CONTEXT_ON_RESET : i32 = 12735;

        /// **get** public static final [EGL_NO_CONTEXT](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_NO_CONTEXT)
        ///
        /// Required feature: android-opengl-EGLContext
        #[cfg(any(feature = "all", feature = "android-opengl-EGLContext"))]
        pub fn EGL_NO_CONTEXT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLContext>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/opengl/EGL15\0", "EGL_NO_CONTEXT\0", "Landroid/opengl/EGLContext;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EGL_NO_DISPLAY](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_NO_DISPLAY)
        ///
        /// Required feature: android-opengl-EGLDisplay
        #[cfg(any(feature = "all", feature = "android-opengl-EGLDisplay"))]
        pub fn EGL_NO_DISPLAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLDisplay>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/opengl/EGL15\0", "EGL_NO_DISPLAY\0", "Landroid/opengl/EGLDisplay;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EGL_NO_IMAGE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_NO_IMAGE)
        ///
        /// Required feature: android-opengl-EGLImage
        #[cfg(any(feature = "all", feature = "android-opengl-EGLImage"))]
        pub fn EGL_NO_IMAGE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLImage>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/opengl/EGL15\0", "EGL_NO_IMAGE\0", "Landroid/opengl/EGLImage;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [EGL_NO_RESET_NOTIFICATION](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_NO_RESET_NOTIFICATION)
        pub const EGL_NO_RESET_NOTIFICATION : i32 = 12734;

        /// **get** public static final [EGL_NO_SURFACE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_NO_SURFACE)
        ///
        /// Required feature: android-opengl-EGLSurface
        #[cfg(any(feature = "all", feature = "android-opengl-EGLSurface"))]
        pub fn EGL_NO_SURFACE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLSurface>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/opengl/EGL15\0", "EGL_NO_SURFACE\0", "Landroid/opengl/EGLSurface;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EGL_NO_SYNC](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_NO_SYNC)
        ///
        /// Required feature: android-opengl-EGLSync
        #[cfg(any(feature = "all", feature = "android-opengl-EGLSync"))]
        pub fn EGL_NO_SYNC<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::opengl::EGLSync>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/opengl/EGL15\0", "EGL_NO_SYNC\0", "Landroid/opengl/EGLSync;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [EGL_OPENGL_ES3_BIT](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_OPENGL_ES3_BIT)
        pub const EGL_OPENGL_ES3_BIT : i32 = 64;

        /// public static final [EGL_PLATFORM_ANDROID_KHR](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_PLATFORM_ANDROID_KHR)
        pub const EGL_PLATFORM_ANDROID_KHR : i32 = 12609;

        /// public static final [EGL_SIGNALED](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SIGNALED)
        pub const EGL_SIGNALED : i32 = 12530;

        /// public static final [EGL_SYNC_CL_EVENT](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_CL_EVENT)
        pub const EGL_SYNC_CL_EVENT : i32 = 12542;

        /// public static final [EGL_SYNC_CL_EVENT_COMPLETE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_CL_EVENT_COMPLETE)
        pub const EGL_SYNC_CL_EVENT_COMPLETE : i32 = 12543;

        /// public static final [EGL_SYNC_CONDITION](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_CONDITION)
        pub const EGL_SYNC_CONDITION : i32 = 12536;

        /// public static final [EGL_SYNC_FENCE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_FENCE)
        pub const EGL_SYNC_FENCE : i32 = 12537;

        /// public static final [EGL_SYNC_FLUSH_COMMANDS_BIT](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_FLUSH_COMMANDS_BIT)
        pub const EGL_SYNC_FLUSH_COMMANDS_BIT : i32 = 1;

        /// public static final [EGL_SYNC_PRIOR_COMMANDS_COMPLETE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_PRIOR_COMMANDS_COMPLETE)
        pub const EGL_SYNC_PRIOR_COMMANDS_COMPLETE : i32 = 12528;

        /// public static final [EGL_SYNC_STATUS](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_STATUS)
        pub const EGL_SYNC_STATUS : i32 = 12529;

        /// public static final [EGL_SYNC_TYPE](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_SYNC_TYPE)
        pub const EGL_SYNC_TYPE : i32 = 12535;

        /// public static final [EGL_TIMEOUT_EXPIRED](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_TIMEOUT_EXPIRED)
        pub const EGL_TIMEOUT_EXPIRED : i32 = 12533;

        /// public static final [EGL_UNSIGNALED](https://developer.android.com/reference/android/opengl/EGL15.html#EGL_UNSIGNALED)
        pub const EGL_UNSIGNALED : i32 = 12531;
    }
}
