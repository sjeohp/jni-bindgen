// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-style-AlignmentSpan_Standard"))]
__jni_bindgen! {
    /// public class [AlignmentSpan.Standard](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html)
    ///
    /// Required feature: android-text-style-AlignmentSpan_Standard
    public class AlignmentSpan_Standard ("android/text/style/AlignmentSpan$Standard") extends crate::java::lang::Object, implements crate::android::text::style::AlignmentSpan, crate::android::text::ParcelableSpan {

        /// [Standard](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html#Standard(android.text.Layout.Alignment))
        ///
        /// Required features: "android-text-Layout_Alignment"
        #[cfg(any(feature = "all", all(feature = "android-text-Layout_Alignment")))]
        pub fn new_Alignment<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Layout_Alignment>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::AlignmentSpan_Standard>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/AlignmentSpan$Standard", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/text/Layout$Alignment;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/AlignmentSpan$Standard\0", "<init>\0", "(Landroid/text/Layout$Alignment;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Standard](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html#Standard(android.os.Parcel))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn new_Parcel<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::AlignmentSpan_Standard>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/AlignmentSpan$Standard", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/AlignmentSpan$Standard\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSpanTypeId](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html#getSpanTypeId())
        pub fn getSpanTypeId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/AlignmentSpan$Standard", java.flags == PUBLIC, .name == "getSpanTypeId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/AlignmentSpan$Standard\0", "getSpanTypeId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/AlignmentSpan$Standard", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/AlignmentSpan$Standard\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/AlignmentSpan$Standard", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/AlignmentSpan$Standard\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlignment](https://developer.android.com/reference/android/text/style/AlignmentSpan.Standard.html#getAlignment())
        ///
        /// Required features: "android-text-Layout_Alignment"
        #[cfg(any(feature = "all", all(feature = "android-text-Layout_Alignment")))]
        pub fn getAlignment<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::Layout_Alignment>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/AlignmentSpan$Standard", java.flags == PUBLIC, .name == "getAlignment", .descriptor == "()Landroid/text/Layout$Alignment;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/AlignmentSpan$Standard\0", "getAlignment\0", "()Landroid/text/Layout$Alignment;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
