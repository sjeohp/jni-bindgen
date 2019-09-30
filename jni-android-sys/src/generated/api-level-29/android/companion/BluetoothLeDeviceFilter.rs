// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-companion-BluetoothLeDeviceFilter"))]
__jni_bindgen! {
    /// public final class [BluetoothLeDeviceFilter](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html)
    ///
    /// Required feature: android-companion-BluetoothLeDeviceFilter
    public final class BluetoothLeDeviceFilter ("android/companion/BluetoothLeDeviceFilter") extends crate::java::lang::Object, implements crate::android::companion::DeviceFilter {

        // // Not emitting: Non-public method
        // /// [BluetoothLeDeviceFilter](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#BluetoothLeDeviceFilter(java.util.regex.Pattern,%20android.bluetooth.le.ScanFilter,%20byte%5B%5D,%20byte%5B%5D,%20java.lang.String,%20java.lang.String,%20int,%20int,%20int,%20int,%20boolean))
        // ///
        // /// Required features: "android-bluetooth-le-ScanFilter", "java-lang-String", "java-util-regex-Pattern"
        // #[cfg(any(feature = "all", all(feature = "android-bluetooth-le-ScanFilter", feature = "java-lang-String", feature = "java-util-regex-Pattern")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::regex::Pattern>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::bluetooth::le::ScanFilter>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::companion::BluetoothLeDeviceFilter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/util/regex/Pattern;Landroid/bluetooth/le/ScanFilter;[B[BLjava/lang/String;Ljava/lang/String;IIIIZ)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6), __jni_bindgen::AsJValue::as_jvalue(&arg7), __jni_bindgen::AsJValue::as_jvalue(&arg8), __jni_bindgen::AsJValue::as_jvalue(&arg9), __jni_bindgen::AsJValue::as_jvalue(&arg10)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/companion/BluetoothLeDeviceFilter\0", "<init>\0", "(Ljava/util/regex/Pattern;Landroid/bluetooth/le/ScanFilter;[B[BLjava/lang/String;Ljava/lang/String;IIIIZ)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [equals](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/companion/BluetoothLeDeviceFilter\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/companion/BluetoothLeDeviceFilter\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/companion/BluetoothLeDeviceFilter\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/companion/BluetoothLeDeviceFilter\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/companion/BluetoothLeDeviceFilter\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRenamePrefixLengthLimit](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#getRenamePrefixLengthLimit())
        pub fn getRenamePrefixLengthLimit<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/companion/BluetoothLeDeviceFilter", java.flags == PUBLIC | STATIC, .name == "getRenamePrefixLengthLimit", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/companion/BluetoothLeDeviceFilter\0", "getRenamePrefixLengthLimit\0", "()I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/companion/BluetoothLeDeviceFilter.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/companion/BluetoothLeDeviceFilter\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
