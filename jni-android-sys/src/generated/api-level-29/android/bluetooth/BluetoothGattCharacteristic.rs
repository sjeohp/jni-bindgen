// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-bluetooth-BluetoothGattCharacteristic"))]
__jni_bindgen! {
    /// public class [BluetoothGattCharacteristic](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html)
    ///
    /// Required feature: android-bluetooth-BluetoothGattCharacteristic
    public class BluetoothGattCharacteristic ("android/bluetooth/BluetoothGattCharacteristic") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [BluetoothGattCharacteristic](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#BluetoothGattCharacteristic(java.util.UUID,%20int,%20int))
        ///
        /// Required features: "java-util-UUID"
        #[cfg(any(feature = "all", all(feature = "java-util-UUID")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::UUID>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::bluetooth::BluetoothGattCharacteristic>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/UUID;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "<init>\0", "(Ljava/util/UUID;II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addDescriptor](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#addDescriptor(android.bluetooth.BluetoothGattDescriptor))
        ///
        /// Required features: "android-bluetooth-BluetoothGattDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothGattDescriptor")))]
        pub fn addDescriptor<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::bluetooth::BluetoothGattDescriptor>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "addDescriptor", .descriptor == "(Landroid/bluetooth/BluetoothGattDescriptor;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "addDescriptor\0", "(Landroid/bluetooth/BluetoothGattDescriptor;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getService](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getService())
        ///
        /// Required features: "android-bluetooth-BluetoothGattService"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothGattService")))]
        pub fn getService<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::bluetooth::BluetoothGattService>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getService", .descriptor == "()Landroid/bluetooth/BluetoothGattService;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getService\0", "()Landroid/bluetooth/BluetoothGattService;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUuid](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getUuid())
        ///
        /// Required features: "java-util-UUID"
        #[cfg(any(feature = "all", all(feature = "java-util-UUID")))]
        pub fn getUuid<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::UUID>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getUuid", .descriptor == "()Ljava/util/UUID;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getUuid\0", "()Ljava/util/UUID;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstanceId](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getInstanceId())
        pub fn getInstanceId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getInstanceId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getInstanceId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProperties](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getProperties())
        pub fn getProperties<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getProperties", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getProperties\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPermissions](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getPermissions())
        pub fn getPermissions<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getPermissions", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getPermissions\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWriteType](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getWriteType())
        pub fn getWriteType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getWriteType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getWriteType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWriteType](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#setWriteType(int))
        pub fn setWriteType<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "setWriteType", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "setWriteType\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDescriptors](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getDescriptors())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getDescriptors<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getDescriptors", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getDescriptors\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDescriptor](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getDescriptor(java.util.UUID))
        ///
        /// Required features: "android-bluetooth-BluetoothGattDescriptor", "java-util-UUID"
        #[cfg(any(feature = "all", all(feature = "android-bluetooth-BluetoothGattDescriptor", feature = "java-util-UUID")))]
        pub fn getDescriptor<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::UUID>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::bluetooth::BluetoothGattDescriptor>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getDescriptor", .descriptor == "(Ljava/util/UUID;)Landroid/bluetooth/BluetoothGattDescriptor;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getDescriptor\0", "(Ljava/util/UUID;)Landroid/bluetooth/BluetoothGattDescriptor;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getValue())
        pub fn getValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getValue", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getValue\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getIntValue(int,%20int))
        ///
        /// Required features: "java-lang-Integer"
        #[cfg(any(feature = "all", all(feature = "java-lang-Integer")))]
        pub fn getIntValue<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Integer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getIntValue", .descriptor == "(II)Ljava/lang/Integer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getIntValue\0", "(II)Ljava/lang/Integer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFloatValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getFloatValue(int,%20int))
        ///
        /// Required features: "java-lang-Float"
        #[cfg(any(feature = "all", all(feature = "java-lang-Float")))]
        pub fn getFloatValue<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Float>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getFloatValue", .descriptor == "(II)Ljava/lang/Float;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getFloatValue\0", "(II)Ljava/lang/Float;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStringValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#getStringValue(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getStringValue<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "getStringValue", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "getStringValue\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#setValue(byte%5B%5D))
        pub fn setValue_byte_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "setValue", .descriptor == "([B)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "setValue\0", "([B)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#setValue(int,%20int,%20int))
        pub fn setValue_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "setValue", .descriptor == "(III)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "setValue\0", "(III)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#setValue(int,%20int,%20int,%20int))
        pub fn setValue_int_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "setValue", .descriptor == "(IIII)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "setValue\0", "(IIII)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setValue](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#setValue(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setValue_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/bluetooth/BluetoothGattCharacteristic", java.flags == PUBLIC, .name == "setValue", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/bluetooth/BluetoothGattCharacteristic\0", "setValue\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/bluetooth/BluetoothGattCharacteristic\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [FORMAT_FLOAT](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_FLOAT)
        pub const FORMAT_FLOAT : i32 = 52;

        /// public static final [FORMAT_SFLOAT](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_SFLOAT)
        pub const FORMAT_SFLOAT : i32 = 50;

        /// public static final [FORMAT_SINT16](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_SINT16)
        pub const FORMAT_SINT16 : i32 = 34;

        /// public static final [FORMAT_SINT32](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_SINT32)
        pub const FORMAT_SINT32 : i32 = 36;

        /// public static final [FORMAT_SINT8](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_SINT8)
        pub const FORMAT_SINT8 : i32 = 33;

        /// public static final [FORMAT_UINT16](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_UINT16)
        pub const FORMAT_UINT16 : i32 = 18;

        /// public static final [FORMAT_UINT32](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_UINT32)
        pub const FORMAT_UINT32 : i32 = 20;

        /// public static final [FORMAT_UINT8](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#FORMAT_UINT8)
        pub const FORMAT_UINT8 : i32 = 17;

        /// public static final [PERMISSION_READ](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_READ)
        pub const PERMISSION_READ : i32 = 1;

        /// public static final [PERMISSION_READ_ENCRYPTED](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_READ_ENCRYPTED)
        pub const PERMISSION_READ_ENCRYPTED : i32 = 2;

        /// public static final [PERMISSION_READ_ENCRYPTED_MITM](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_READ_ENCRYPTED_MITM)
        pub const PERMISSION_READ_ENCRYPTED_MITM : i32 = 4;

        /// public static final [PERMISSION_WRITE](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_WRITE)
        pub const PERMISSION_WRITE : i32 = 16;

        /// public static final [PERMISSION_WRITE_ENCRYPTED](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_WRITE_ENCRYPTED)
        pub const PERMISSION_WRITE_ENCRYPTED : i32 = 32;

        /// public static final [PERMISSION_WRITE_ENCRYPTED_MITM](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_WRITE_ENCRYPTED_MITM)
        pub const PERMISSION_WRITE_ENCRYPTED_MITM : i32 = 64;

        /// public static final [PERMISSION_WRITE_SIGNED](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_WRITE_SIGNED)
        pub const PERMISSION_WRITE_SIGNED : i32 = 128;

        /// public static final [PERMISSION_WRITE_SIGNED_MITM](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PERMISSION_WRITE_SIGNED_MITM)
        pub const PERMISSION_WRITE_SIGNED_MITM : i32 = 256;

        /// public static final [PROPERTY_BROADCAST](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_BROADCAST)
        pub const PROPERTY_BROADCAST : i32 = 1;

        /// public static final [PROPERTY_EXTENDED_PROPS](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_EXTENDED_PROPS)
        pub const PROPERTY_EXTENDED_PROPS : i32 = 128;

        /// public static final [PROPERTY_INDICATE](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_INDICATE)
        pub const PROPERTY_INDICATE : i32 = 32;

        /// public static final [PROPERTY_NOTIFY](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_NOTIFY)
        pub const PROPERTY_NOTIFY : i32 = 16;

        /// public static final [PROPERTY_READ](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_READ)
        pub const PROPERTY_READ : i32 = 2;

        /// public static final [PROPERTY_SIGNED_WRITE](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_SIGNED_WRITE)
        pub const PROPERTY_SIGNED_WRITE : i32 = 64;

        /// public static final [PROPERTY_WRITE](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_WRITE)
        pub const PROPERTY_WRITE : i32 = 8;

        /// public static final [PROPERTY_WRITE_NO_RESPONSE](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#PROPERTY_WRITE_NO_RESPONSE)
        pub const PROPERTY_WRITE_NO_RESPONSE : i32 = 4;

        /// public static final [WRITE_TYPE_DEFAULT](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#WRITE_TYPE_DEFAULT)
        pub const WRITE_TYPE_DEFAULT : i32 = 2;

        /// public static final [WRITE_TYPE_NO_RESPONSE](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#WRITE_TYPE_NO_RESPONSE)
        pub const WRITE_TYPE_NO_RESPONSE : i32 = 1;

        /// public static final [WRITE_TYPE_SIGNED](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#WRITE_TYPE_SIGNED)
        pub const WRITE_TYPE_SIGNED : i32 = 4;

        // // Not emitting: Non-public field
        // /// **get** protected [mDescriptors](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#mDescriptors)
        // ///
        // /// Required feature: java-util-List
        // #[cfg(any(feature = "all", feature = "java-util-List"))]
        // pub fn mDescriptors<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/bluetooth/BluetoothGattCharacteristic\0", "mDescriptors\0", "Ljava/util/List;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [mDescriptors](https://developer.android.com/reference/android/bluetooth/BluetoothGattCharacteristic.html#mDescriptors)
        // ///
        // /// Required feature: java-util-List
        // #[cfg(any(feature = "all", feature = "java-util-List"))]
        // pub fn set_mDescriptors<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::util::List>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("android/bluetooth/BluetoothGattCharacteristic\0", "mDescriptors\0", "Ljava/util/List;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }
    }
}
