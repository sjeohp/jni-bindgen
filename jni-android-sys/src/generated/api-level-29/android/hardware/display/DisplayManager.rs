// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-display-DisplayManager"))]
__jni_bindgen! {
    /// public final class [DisplayManager](https://developer.android.com/reference/android/hardware/display/DisplayManager.html)
    ///
    /// Required feature: android-hardware-display-DisplayManager
    public final class DisplayManager ("android/hardware/display/DisplayManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [DisplayManager](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#DisplayManager(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::display::DisplayManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/hardware/display/DisplayManager", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDisplay](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#getDisplay(int))
        ///
        /// Required features: "android-view-Display"
        #[cfg(any(feature = "all", all(feature = "android-view-Display")))]
        pub fn getDisplay<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::Display>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "getDisplay", .descriptor == "(I)Landroid/view/Display;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "getDisplay\0", "(I)Landroid/view/Display;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDisplays](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#getDisplays())
        ///
        /// Required features: "android-view-Display"
        #[cfg(any(feature = "all", all(feature = "android-view-Display")))]
        pub fn getDisplays<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::Display, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "getDisplays", .descriptor == "()[Landroid/view/Display;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "getDisplays\0", "()[Landroid/view/Display;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDisplays](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#getDisplays(java.lang.String))
        ///
        /// Required features: "android-view-Display", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-Display", feature = "java-lang-String")))]
        pub fn getDisplays_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::view::Display, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "getDisplays", .descriptor == "(Ljava/lang/String;)[Landroid/view/Display;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "getDisplays\0", "(Ljava/lang/String;)[Landroid/view/Display;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerDisplayListener](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#registerDisplayListener(android.hardware.display.DisplayManager.DisplayListener,%20android.os.Handler))
        ///
        /// Required features: "android-hardware-display-DisplayManager_DisplayListener", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-hardware-display-DisplayManager_DisplayListener", feature = "android-os-Handler")))]
        pub fn registerDisplayListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::display::DisplayManager_DisplayListener>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "registerDisplayListener", .descriptor == "(Landroid/hardware/display/DisplayManager$DisplayListener;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "registerDisplayListener\0", "(Landroid/hardware/display/DisplayManager$DisplayListener;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterDisplayListener](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#unregisterDisplayListener(android.hardware.display.DisplayManager.DisplayListener))
        ///
        /// Required features: "android-hardware-display-DisplayManager_DisplayListener"
        #[cfg(any(feature = "all", all(feature = "android-hardware-display-DisplayManager_DisplayListener")))]
        pub fn unregisterDisplayListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::display::DisplayManager_DisplayListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "unregisterDisplayListener", .descriptor == "(Landroid/hardware/display/DisplayManager$DisplayListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "unregisterDisplayListener\0", "(Landroid/hardware/display/DisplayManager$DisplayListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createVirtualDisplay](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#createVirtualDisplay(java.lang.String,%20int,%20int,%20int,%20android.view.Surface,%20int))
        ///
        /// Required features: "android-hardware-display-VirtualDisplay", "android-view-Surface", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-hardware-display-VirtualDisplay", feature = "android-view-Surface", feature = "java-lang-String")))]
        pub fn createVirtualDisplay_String_int_int_int_Surface_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::hardware::display::VirtualDisplay>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "createVirtualDisplay", .descriptor == "(Ljava/lang/String;IIILandroid/view/Surface;I)Landroid/hardware/display/VirtualDisplay;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "createVirtualDisplay\0", "(Ljava/lang/String;IIILandroid/view/Surface;I)Landroid/hardware/display/VirtualDisplay;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createVirtualDisplay](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#createVirtualDisplay(java.lang.String,%20int,%20int,%20int,%20android.view.Surface,%20int,%20android.hardware.display.VirtualDisplay.Callback,%20android.os.Handler))
        ///
        /// Required features: "android-hardware-display-VirtualDisplay", "android-hardware-display-VirtualDisplay_Callback", "android-os-Handler", "android-view-Surface", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-hardware-display-VirtualDisplay", feature = "android-hardware-display-VirtualDisplay_Callback", feature = "android-os-Handler", feature = "android-view-Surface", feature = "java-lang-String")))]
        pub fn createVirtualDisplay_String_int_int_int_Surface_int_Callback_Handler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>, arg5: i32, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::hardware::display::VirtualDisplay_Callback>>, arg7: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::hardware::display::VirtualDisplay>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/display/DisplayManager", java.flags == PUBLIC, .name == "createVirtualDisplay", .descriptor == "(Ljava/lang/String;IIILandroid/view/Surface;ILandroid/hardware/display/VirtualDisplay$Callback;Landroid/os/Handler;)Landroid/hardware/display/VirtualDisplay;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6.into()), __jni_bindgen::AsJValue::as_jvalue(&arg7.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/display/DisplayManager\0", "createVirtualDisplay\0", "(Ljava/lang/String;IIILandroid/view/Surface;ILandroid/hardware/display/VirtualDisplay$Callback;Landroid/os/Handler;)Landroid/hardware/display/VirtualDisplay;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DISPLAY_CATEGORY_PRESENTATION](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#DISPLAY_CATEGORY_PRESENTATION)
        pub const DISPLAY_CATEGORY_PRESENTATION : &'static str = "android.hardware.display.category.PRESENTATION";

        /// public static final [VIRTUAL_DISPLAY_FLAG_AUTO_MIRROR](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#VIRTUAL_DISPLAY_FLAG_AUTO_MIRROR)
        pub const VIRTUAL_DISPLAY_FLAG_AUTO_MIRROR : i32 = 16;

        /// public static final [VIRTUAL_DISPLAY_FLAG_OWN_CONTENT_ONLY](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#VIRTUAL_DISPLAY_FLAG_OWN_CONTENT_ONLY)
        pub const VIRTUAL_DISPLAY_FLAG_OWN_CONTENT_ONLY : i32 = 8;

        /// public static final [VIRTUAL_DISPLAY_FLAG_PRESENTATION](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#VIRTUAL_DISPLAY_FLAG_PRESENTATION)
        pub const VIRTUAL_DISPLAY_FLAG_PRESENTATION : i32 = 2;

        /// public static final [VIRTUAL_DISPLAY_FLAG_PUBLIC](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#VIRTUAL_DISPLAY_FLAG_PUBLIC)
        pub const VIRTUAL_DISPLAY_FLAG_PUBLIC : i32 = 1;

        /// public static final [VIRTUAL_DISPLAY_FLAG_SECURE](https://developer.android.com/reference/android/hardware/display/DisplayManager.html#VIRTUAL_DISPLAY_FLAG_SECURE)
        pub const VIRTUAL_DISPLAY_FLAG_SECURE : i32 = 4;
    }
}
