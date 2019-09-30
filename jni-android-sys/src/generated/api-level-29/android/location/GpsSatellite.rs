// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-location-GpsSatellite"))]
__jni_bindgen! {
    /// public final class [GpsSatellite](https://developer.android.com/reference/android/location/GpsSatellite.html)
    ///
    /// Required feature: android-location-GpsSatellite
    #[deprecated] public final class GpsSatellite ("android/location/GpsSatellite") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [GpsSatellite](https://developer.android.com/reference/android/location/GpsSatellite.html#GpsSatellite(int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::location::GpsSatellite>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/location/GpsSatellite", java.flags == (empty), .name == "<init>", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "<init>\0", "(I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getPrn](https://developer.android.com/reference/android/location/GpsSatellite.html#getPrn())
        #[deprecated] pub fn getPrn<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "getPrn", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "getPrn\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSnr](https://developer.android.com/reference/android/location/GpsSatellite.html#getSnr())
        #[deprecated] pub fn getSnr<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "getSnr", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "getSnr\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getElevation](https://developer.android.com/reference/android/location/GpsSatellite.html#getElevation())
        #[deprecated] pub fn getElevation<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "getElevation", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "getElevation\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAzimuth](https://developer.android.com/reference/android/location/GpsSatellite.html#getAzimuth())
        #[deprecated] pub fn getAzimuth<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "getAzimuth", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "getAzimuth\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasEphemeris](https://developer.android.com/reference/android/location/GpsSatellite.html#hasEphemeris())
        #[deprecated] pub fn hasEphemeris<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "hasEphemeris", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "hasEphemeris\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasAlmanac](https://developer.android.com/reference/android/location/GpsSatellite.html#hasAlmanac())
        #[deprecated] pub fn hasAlmanac<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "hasAlmanac", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "hasAlmanac\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [usedInFix](https://developer.android.com/reference/android/location/GpsSatellite.html#usedInFix())
        #[deprecated] pub fn usedInFix<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GpsSatellite", java.flags == PUBLIC, .name == "usedInFix", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GpsSatellite\0", "usedInFix\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
