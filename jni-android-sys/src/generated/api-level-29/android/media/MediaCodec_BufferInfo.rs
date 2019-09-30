// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaCodec_BufferInfo"))]
__jni_bindgen! {
    /// public final class [MediaCodec.BufferInfo](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html)
    ///
    /// Required feature: android-media-MediaCodec_BufferInfo
    public final class MediaCodec_BufferInfo ("android/media/MediaCodec$BufferInfo") extends crate::java::lang::Object {

        /// [BufferInfo](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#BufferInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaCodec_BufferInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$BufferInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$BufferInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#set(int,%20int,%20long,%20int))
        pub fn set<'env>(&'env self, arg0: i32, arg1: i32, arg2: i64, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$BufferInfo", java.flags == PUBLIC, .name == "set", .descriptor == "(IIJI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$BufferInfo\0", "set\0", "(IIJI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [flags](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#flags)
        pub fn flags<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "flags\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [flags](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#flags)
        pub fn set_flags<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "flags\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [offset](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#offset)
        pub fn offset<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "offset\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [offset](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#offset)
        pub fn set_offset<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "offset\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [presentationTimeUs](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#presentationTimeUs)
        pub fn presentationTimeUs<'env>(&'env self) -> i64 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "presentationTimeUs\0", "J\0");
                env.get_long_field(class, field)
            }
        }

        /// **set** public [presentationTimeUs](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#presentationTimeUs)
        pub fn set_presentationTimeUs<'env>(&'env self, value: i64) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "presentationTimeUs\0", "J\0");
                env.set_long_field(class, field, value)
            }
        }

        /// **get** public [size](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#size)
        pub fn size<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "size\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [size](https://developer.android.com/reference/android/media/MediaCodec.BufferInfo.html#size)
        pub fn set_size<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/media/MediaCodec$BufferInfo\0", "size\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
