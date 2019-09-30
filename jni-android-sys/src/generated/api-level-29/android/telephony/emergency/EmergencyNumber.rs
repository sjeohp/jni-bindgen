// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-emergency-EmergencyNumber"))]
__jni_bindgen! {
    /// public final class [EmergencyNumber](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html)
    ///
    /// Required feature: android-telephony-emergency-EmergencyNumber
    public final class EmergencyNumber ("android/telephony/emergency/EmergencyNumber") extends crate::java::lang::Object, implements crate::android::os::Parcelable, crate::java::lang::Comparable {

        // // Not emitting: Non-public method
        // /// [EmergencyNumber](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EmergencyNumber(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::emergency::EmergencyNumber>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [writeToParcel](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumber](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getNumber())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getNumber", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getNumber\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCountryIso](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getCountryIso())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getCountryIso<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getCountryIso", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getCountryIso\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMnc](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getMnc())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMnc<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getMnc", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getMnc\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEmergencyServiceCategories](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getEmergencyServiceCategories())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getEmergencyServiceCategories<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getEmergencyServiceCategories", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getEmergencyServiceCategories\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEmergencyUrns](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getEmergencyUrns())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getEmergencyUrns<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getEmergencyUrns", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getEmergencyUrns\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isInEmergencyServiceCategories](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#isInEmergencyServiceCategories(int))
        pub fn isInEmergencyServiceCategories<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "isInEmergencyServiceCategories", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "isInEmergencyServiceCategories\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEmergencyNumberSources](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getEmergencyNumberSources())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getEmergencyNumberSources<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getEmergencyNumberSources", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getEmergencyNumberSources\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isFromSources](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#isFromSources(int))
        pub fn isFromSources<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "isFromSources", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "isFromSources\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEmergencyCallRouting](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#getEmergencyCallRouting())
        pub fn getEmergencyCallRouting<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "getEmergencyCallRouting", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "getEmergencyCallRouting\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareTo](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#compareTo(android.telephony.emergency.EmergencyNumber))
        ///
        /// Required features: "android-telephony-emergency-EmergencyNumber"
        #[cfg(any(feature = "all", all(feature = "android-telephony-emergency-EmergencyNumber")))]
        pub fn compareTo_EmergencyNumber<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telephony::emergency::EmergencyNumber>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC, .name == "compareTo", .descriptor == "(Landroid/telephony/emergency/EmergencyNumber;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "compareTo\0", "(Landroid/telephony/emergency/EmergencyNumber;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [compareTo](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#compareTo(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn compareTo_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/emergency/EmergencyNumber", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "compareTo", .descriptor == "(Ljava/lang/Object;)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/emergency/EmergencyNumber\0", "compareTo\0", "(Ljava/lang/Object;)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/telephony/emergency/EmergencyNumber\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [EMERGENCY_CALL_ROUTING_EMERGENCY](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_CALL_ROUTING_EMERGENCY)
        pub const EMERGENCY_CALL_ROUTING_EMERGENCY : i32 = 1;

        /// public static final [EMERGENCY_CALL_ROUTING_NORMAL](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_CALL_ROUTING_NORMAL)
        pub const EMERGENCY_CALL_ROUTING_NORMAL : i32 = 2;

        /// public static final [EMERGENCY_CALL_ROUTING_UNKNOWN](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_CALL_ROUTING_UNKNOWN)
        pub const EMERGENCY_CALL_ROUTING_UNKNOWN : i32 = 0;

        /// public static final [EMERGENCY_NUMBER_SOURCE_DATABASE](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_NUMBER_SOURCE_DATABASE)
        pub const EMERGENCY_NUMBER_SOURCE_DATABASE : i32 = 16;

        /// public static final [EMERGENCY_NUMBER_SOURCE_DEFAULT](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_NUMBER_SOURCE_DEFAULT)
        pub const EMERGENCY_NUMBER_SOURCE_DEFAULT : i32 = 8;

        /// public static final [EMERGENCY_NUMBER_SOURCE_MODEM_CONFIG](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_NUMBER_SOURCE_MODEM_CONFIG)
        pub const EMERGENCY_NUMBER_SOURCE_MODEM_CONFIG : i32 = 4;

        /// public static final [EMERGENCY_NUMBER_SOURCE_NETWORK_SIGNALING](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_NUMBER_SOURCE_NETWORK_SIGNALING)
        pub const EMERGENCY_NUMBER_SOURCE_NETWORK_SIGNALING : i32 = 1;

        /// public static final [EMERGENCY_NUMBER_SOURCE_SIM](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_NUMBER_SOURCE_SIM)
        pub const EMERGENCY_NUMBER_SOURCE_SIM : i32 = 2;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_AIEC](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_AIEC)
        pub const EMERGENCY_SERVICE_CATEGORY_AIEC : i32 = 64;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_AMBULANCE](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_AMBULANCE)
        pub const EMERGENCY_SERVICE_CATEGORY_AMBULANCE : i32 = 2;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_FIRE_BRIGADE](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_FIRE_BRIGADE)
        pub const EMERGENCY_SERVICE_CATEGORY_FIRE_BRIGADE : i32 = 4;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_MARINE_GUARD](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_MARINE_GUARD)
        pub const EMERGENCY_SERVICE_CATEGORY_MARINE_GUARD : i32 = 8;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_MIEC](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_MIEC)
        pub const EMERGENCY_SERVICE_CATEGORY_MIEC : i32 = 32;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_MOUNTAIN_RESCUE](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_MOUNTAIN_RESCUE)
        pub const EMERGENCY_SERVICE_CATEGORY_MOUNTAIN_RESCUE : i32 = 16;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_POLICE](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_POLICE)
        pub const EMERGENCY_SERVICE_CATEGORY_POLICE : i32 = 1;

        /// public static final [EMERGENCY_SERVICE_CATEGORY_UNSPECIFIED](https://developer.android.com/reference/android/telephony/emergency/EmergencyNumber.html#EMERGENCY_SERVICE_CATEGORY_UNSPECIFIED)
        pub const EMERGENCY_SERVICE_CATEGORY_UNSPECIFIED : i32 = 0;
    }
}
