// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-midi-MidiDeviceInfo"))]
__jni_bindgen! {
    /// public final class [MidiDeviceInfo](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html)
    ///
    /// Required feature: android-media-midi-MidiDeviceInfo
    public final class MidiDeviceInfo ("android/media/midi/MidiDeviceInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [MidiDeviceInfo](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#MidiDeviceInfo(int,%20int,%20int,%20int,%20java.lang.String%5B%5D,%20java.lang.String%5B%5D,%20android.os.Bundle,%20boolean))
        // ///
        // /// Required features: "android-os-Bundle", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>, arg7: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::midi::MidiDeviceInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/midi/MidiDeviceInfo", java.flags == (empty), .name == "<init>", .descriptor == "(IIII[Ljava/lang/String;[Ljava/lang/String;Landroid/os/Bundle;Z)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6.into()), __jni_bindgen::AsJValue::as_jvalue(&arg7)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "<init>\0", "(IIII[Ljava/lang/String;[Ljava/lang/String;Landroid/os/Bundle;Z)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getType](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#getType())
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "getType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "getType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getId](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#getId())
        pub fn getId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "getId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "getId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInputPortCount](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#getInputPortCount())
        pub fn getInputPortCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "getInputPortCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "getInputPortCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOutputPortCount](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#getOutputPortCount())
        pub fn getOutputPortCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "getOutputPortCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "getOutputPortCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPorts](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#getPorts())
        ///
        /// Required features: "android-media-midi-MidiDeviceInfo_PortInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiDeviceInfo_PortInfo")))]
        pub fn getPorts<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::media::midi::MidiDeviceInfo_PortInfo, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "getPorts", .descriptor == "()[Landroid/media/midi/MidiDeviceInfo$PortInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "getPorts\0", "()[Landroid/media/midi/MidiDeviceInfo$PortInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProperties](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#getProperties())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getProperties<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "getProperties", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "getProperties\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isPrivate](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#isPrivate())
        pub fn isPrivate<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "isPrivate", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "isPrivate\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiDeviceInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiDeviceInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/media/midi/MidiDeviceInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [PROPERTY_BLUETOOTH_DEVICE](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_BLUETOOTH_DEVICE)
        pub const PROPERTY_BLUETOOTH_DEVICE : &'static str = "bluetooth_device";

        /// public static final [PROPERTY_MANUFACTURER](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_MANUFACTURER)
        pub const PROPERTY_MANUFACTURER : &'static str = "manufacturer";

        /// public static final [PROPERTY_NAME](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_NAME)
        pub const PROPERTY_NAME : &'static str = "name";

        /// public static final [PROPERTY_PRODUCT](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_PRODUCT)
        pub const PROPERTY_PRODUCT : &'static str = "product";

        /// public static final [PROPERTY_SERIAL_NUMBER](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_SERIAL_NUMBER)
        pub const PROPERTY_SERIAL_NUMBER : &'static str = "serial_number";

        /// public static final [PROPERTY_USB_DEVICE](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_USB_DEVICE)
        pub const PROPERTY_USB_DEVICE : &'static str = "usb_device";

        /// public static final [PROPERTY_VERSION](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#PROPERTY_VERSION)
        pub const PROPERTY_VERSION : &'static str = "version";

        /// public static final [TYPE_BLUETOOTH](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#TYPE_BLUETOOTH)
        pub const TYPE_BLUETOOTH : i32 = 3;

        /// public static final [TYPE_USB](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#TYPE_USB)
        pub const TYPE_USB : i32 = 1;

        /// public static final [TYPE_VIRTUAL](https://developer.android.com/reference/android/media/midi/MidiDeviceInfo.html#TYPE_VIRTUAL)
        pub const TYPE_VIRTUAL : i32 = 2;
    }
}
