// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-print-PrintJobInfo"))]
__jni_bindgen! {
    /// public final class [PrintJobInfo](https://developer.android.com/reference/android/print/PrintJobInfo.html)
    ///
    /// Required feature: android-print-PrintJobInfo
    public final class PrintJobInfo ("android/print/PrintJobInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [PrintJobInfo](https://developer.android.com/reference/android/print/PrintJobInfo.html#PrintJobInfo())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::print::PrintJobInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/print/PrintJobInfo", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getId](https://developer.android.com/reference/android/print/PrintJobInfo.html#getId())
        ///
        /// Required features: "android-print-PrintJobId"
        #[cfg(any(feature = "all", all(feature = "android-print-PrintJobId")))]
        pub fn getId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrintJobId>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getId", .descriptor == "()Landroid/print/PrintJobId;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getId\0", "()Landroid/print/PrintJobId;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLabel](https://developer.android.com/reference/android/print/PrintJobInfo.html#getLabel())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getLabel<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getLabel", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getLabel\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrinterId](https://developer.android.com/reference/android/print/PrintJobInfo.html#getPrinterId())
        ///
        /// Required features: "android-print-PrinterId"
        #[cfg(any(feature = "all", all(feature = "android-print-PrinterId")))]
        pub fn getPrinterId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrinterId>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getPrinterId", .descriptor == "()Landroid/print/PrinterId;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getPrinterId\0", "()Landroid/print/PrinterId;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getState](https://developer.android.com/reference/android/print/PrintJobInfo.html#getState())
        pub fn getState<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getState", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getState\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCreationTime](https://developer.android.com/reference/android/print/PrintJobInfo.html#getCreationTime())
        pub fn getCreationTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getCreationTime", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getCreationTime\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCopies](https://developer.android.com/reference/android/print/PrintJobInfo.html#getCopies())
        pub fn getCopies<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getCopies", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getCopies\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPages](https://developer.android.com/reference/android/print/PrintJobInfo.html#getPages())
        ///
        /// Required features: "android-print-PageRange"
        #[cfg(any(feature = "all", all(feature = "android-print-PageRange")))]
        pub fn getPages<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::print::PageRange, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getPages", .descriptor == "()[Landroid/print/PageRange;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getPages\0", "()[Landroid/print/PageRange;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAttributes](https://developer.android.com/reference/android/print/PrintJobInfo.html#getAttributes())
        ///
        /// Required features: "android-print-PrintAttributes"
        #[cfg(any(feature = "all", all(feature = "android-print-PrintAttributes")))]
        pub fn getAttributes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::print::PrintAttributes>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getAttributes", .descriptor == "()Landroid/print/PrintAttributes;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getAttributes\0", "()Landroid/print/PrintAttributes;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasAdvancedOption](https://developer.android.com/reference/android/print/PrintJobInfo.html#hasAdvancedOption(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn hasAdvancedOption<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "hasAdvancedOption", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "hasAdvancedOption\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAdvancedStringOption](https://developer.android.com/reference/android/print/PrintJobInfo.html#getAdvancedStringOption(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAdvancedStringOption<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getAdvancedStringOption", .descriptor == "(Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getAdvancedStringOption\0", "(Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAdvancedIntOption](https://developer.android.com/reference/android/print/PrintJobInfo.html#getAdvancedIntOption(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAdvancedIntOption<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "getAdvancedIntOption", .descriptor == "(Ljava/lang/String;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "getAdvancedIntOption\0", "(Ljava/lang/String;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/print/PrintJobInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/print/PrintJobInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/print/PrintJobInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/print/PrintJobInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/print/PrintJobInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/print/PrintJobInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/print/PrintJobInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [STATE_BLOCKED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_BLOCKED)
        pub const STATE_BLOCKED : i32 = 4;

        /// public static final [STATE_CANCELED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_CANCELED)
        pub const STATE_CANCELED : i32 = 7;

        /// public static final [STATE_COMPLETED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_COMPLETED)
        pub const STATE_COMPLETED : i32 = 5;

        /// public static final [STATE_CREATED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_CREATED)
        pub const STATE_CREATED : i32 = 1;

        /// public static final [STATE_FAILED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_FAILED)
        pub const STATE_FAILED : i32 = 6;

        /// public static final [STATE_QUEUED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_QUEUED)
        pub const STATE_QUEUED : i32 = 2;

        /// public static final [STATE_STARTED](https://developer.android.com/reference/android/print/PrintJobInfo.html#STATE_STARTED)
        pub const STATE_STARTED : i32 = 3;
    }
}
