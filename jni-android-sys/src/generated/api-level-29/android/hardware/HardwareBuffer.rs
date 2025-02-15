// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-hardware-HardwareBuffer"))]
__jni_bindgen! {
    /// public final class [HardwareBuffer](https://developer.android.com/reference/android/hardware/HardwareBuffer.html)
    ///
    /// Required feature: android-hardware-HardwareBuffer
    public final class HardwareBuffer ("android/hardware/HardwareBuffer") extends crate::java::lang::Object, implements crate::android::os::Parcelable, crate::java::lang::AutoCloseable {

        // // Not emitting: Non-public method
        // /// [HardwareBuffer](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#HardwareBuffer(long))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::hardware::HardwareBuffer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/hardware/HardwareBuffer", java.flags == (empty), .name == "<init>", .descriptor == "(J)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "<init>\0", "(J)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [create](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#create(int,%20int,%20int,%20int,%20long))
        ///
        /// Required features: "android-hardware-HardwareBuffer"
        #[cfg(any(feature = "all", all(feature = "android-hardware-HardwareBuffer")))]
        pub fn create<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::hardware::HardwareBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC | STATIC, .name == "create", .descriptor == "(IIIIJ)Landroid/hardware/HardwareBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/hardware/HardwareBuffer\0", "create\0", "(IIIIJ)Landroid/hardware/HardwareBuffer;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSupported](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#isSupported(int,%20int,%20int,%20int,%20long))
        pub fn isSupported<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC | STATIC, .name == "isSupported", .descriptor == "(IIIIJ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/hardware/HardwareBuffer\0", "isSupported\0", "(IIIIJ)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/hardware/HardwareBuffer", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getWidth](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#getWidth())
        pub fn getWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "getWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "getWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHeight](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#getHeight())
        pub fn getHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "getHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "getHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFormat](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#getFormat())
        pub fn getFormat<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "getFormat", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "getFormat\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLayers](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#getLayers())
        pub fn getLayers<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "getLayers", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "getLayers\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUsage](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#getUsage())
        pub fn getUsage<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "getUsage", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "getUsage\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isClosed](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#isClosed())
        pub fn isClosed<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "isClosed", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "isClosed\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/hardware/HardwareBuffer", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/hardware/HardwareBuffer\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [BLOB](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#BLOB)
        pub const BLOB : i32 = 33;

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/hardware/HardwareBuffer\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [DS_24UI8](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#DS_24UI8)
        pub const DS_24UI8 : i32 = 50;

        /// public static final [DS_FP32UI8](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#DS_FP32UI8)
        pub const DS_FP32UI8 : i32 = 52;

        /// public static final [D_16](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#D_16)
        pub const D_16 : i32 = 48;

        /// public static final [D_24](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#D_24)
        pub const D_24 : i32 = 49;

        /// public static final [D_FP32](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#D_FP32)
        pub const D_FP32 : i32 = 51;

        /// public static final [RGBA_1010102](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#RGBA_1010102)
        pub const RGBA_1010102 : i32 = 43;

        /// public static final [RGBA_8888](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#RGBA_8888)
        pub const RGBA_8888 : i32 = 1;

        /// public static final [RGBA_FP16](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#RGBA_FP16)
        pub const RGBA_FP16 : i32 = 22;

        /// public static final [RGBX_8888](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#RGBX_8888)
        pub const RGBX_8888 : i32 = 2;

        /// public static final [RGB_565](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#RGB_565)
        pub const RGB_565 : i32 = 4;

        /// public static final [RGB_888](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#RGB_888)
        pub const RGB_888 : i32 = 3;

        /// public static final [S_UI8](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#S_UI8)
        pub const S_UI8 : i32 = 53;

        /// public static final [USAGE_CPU_READ_OFTEN](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_CPU_READ_OFTEN)
        pub const USAGE_CPU_READ_OFTEN : i64 = 3i64;

        /// public static final [USAGE_CPU_READ_RARELY](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_CPU_READ_RARELY)
        pub const USAGE_CPU_READ_RARELY : i64 = 2i64;

        /// public static final [USAGE_CPU_WRITE_OFTEN](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_CPU_WRITE_OFTEN)
        pub const USAGE_CPU_WRITE_OFTEN : i64 = 48i64;

        /// public static final [USAGE_CPU_WRITE_RARELY](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_CPU_WRITE_RARELY)
        pub const USAGE_CPU_WRITE_RARELY : i64 = 32i64;

        /// public static final [USAGE_GPU_COLOR_OUTPUT](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_GPU_COLOR_OUTPUT)
        pub const USAGE_GPU_COLOR_OUTPUT : i64 = 512i64;

        /// public static final [USAGE_GPU_CUBE_MAP](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_GPU_CUBE_MAP)
        pub const USAGE_GPU_CUBE_MAP : i64 = 33554432i64;

        /// public static final [USAGE_GPU_DATA_BUFFER](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_GPU_DATA_BUFFER)
        pub const USAGE_GPU_DATA_BUFFER : i64 = 16777216i64;

        /// public static final [USAGE_GPU_MIPMAP_COMPLETE](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_GPU_MIPMAP_COMPLETE)
        pub const USAGE_GPU_MIPMAP_COMPLETE : i64 = 67108864i64;

        /// public static final [USAGE_GPU_SAMPLED_IMAGE](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_GPU_SAMPLED_IMAGE)
        pub const USAGE_GPU_SAMPLED_IMAGE : i64 = 256i64;

        /// public static final [USAGE_PROTECTED_CONTENT](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_PROTECTED_CONTENT)
        pub const USAGE_PROTECTED_CONTENT : i64 = 16384i64;

        /// public static final [USAGE_SENSOR_DIRECT_DATA](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_SENSOR_DIRECT_DATA)
        pub const USAGE_SENSOR_DIRECT_DATA : i64 = 8388608i64;

        /// public static final [USAGE_VIDEO_ENCODE](https://developer.android.com/reference/android/hardware/HardwareBuffer.html#USAGE_VIDEO_ENCODE)
        pub const USAGE_VIDEO_ENCODE : i64 = 65536i64;
    }
}
