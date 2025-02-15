// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-midi-MidiDeviceStatus"))]
__jni_bindgen! {
    /// public final class [MidiDeviceStatus](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html)
    ///
    /// Required feature: android-media-midi-MidiDeviceStatus
    public final class MidiDeviceStatus ("android/media/midi/MidiDeviceStatus") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [MidiDeviceStatus](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#MidiDeviceStatus(android.media.midi.MidiDeviceInfo))
        // ///
        // /// Required features: "android-media-midi-MidiDeviceInfo"
        // #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiDeviceInfo")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::midi::MidiDeviceInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::midi::MidiDeviceStatus>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/midi/MidiDeviceStatus", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/media/midi/MidiDeviceInfo;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "<init>\0", "(Landroid/media/midi/MidiDeviceInfo;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDeviceInfo](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#getDeviceInfo())
        ///
        /// Required features: "android-media-midi-MidiDeviceInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiDeviceInfo")))]
        pub fn getDeviceInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::midi::MidiDeviceInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceStatus", java.flags == PUBLIC, .name == "getDeviceInfo", .descriptor == "()Landroid/media/midi/MidiDeviceInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "getDeviceInfo\0", "()Landroid/media/midi/MidiDeviceInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isInputPortOpen](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#isInputPortOpen(int))
        pub fn isInputPortOpen<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceStatus", java.flags == PUBLIC, .name == "isInputPortOpen", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "isInputPortOpen\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOutputPortOpenCount](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#getOutputPortOpenCount(int))
        pub fn getOutputPortOpenCount<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceStatus", java.flags == PUBLIC, .name == "getOutputPortOpenCount", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "getOutputPortOpenCount\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceStatus", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceStatus", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceStatus", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceStatus\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/media/midi/MidiDeviceStatus.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/media/midi/MidiDeviceStatus\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
