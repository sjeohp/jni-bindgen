// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-style-SubscriptSpan"))]
__jni_bindgen! {
    /// public class [SubscriptSpan](https://developer.android.com/reference/android/text/style/SubscriptSpan.html)
    ///
    /// Required feature: android-text-style-SubscriptSpan
    public class SubscriptSpan ("android/text/style/SubscriptSpan") extends crate::android::text::style::MetricAffectingSpan, implements crate::android::text::ParcelableSpan {

        /// [SubscriptSpan](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#SubscriptSpan())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::SubscriptSpan>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SubscriptSpan](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#SubscriptSpan(android.os.Parcel))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn new_Parcel<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::SubscriptSpan>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSpanTypeId](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#getSpanTypeId())
        pub fn getSpanTypeId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "getSpanTypeId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "getSpanTypeId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateDrawState](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#updateDrawState(android.text.TextPaint))
        ///
        /// Required features: "android-text-TextPaint"
        #[cfg(any(feature = "all", all(feature = "android-text-TextPaint")))]
        pub fn updateDrawState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::TextPaint>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "updateDrawState", .descriptor == "(Landroid/text/TextPaint;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "updateDrawState\0", "(Landroid/text/TextPaint;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateMeasureState](https://developer.android.com/reference/android/text/style/SubscriptSpan.html#updateMeasureState(android.text.TextPaint))
        ///
        /// Required features: "android-text-TextPaint"
        #[cfg(any(feature = "all", all(feature = "android-text-TextPaint")))]
        pub fn updateMeasureState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::TextPaint>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/SubscriptSpan", java.flags == PUBLIC, .name == "updateMeasureState", .descriptor == "(Landroid/text/TextPaint;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/SubscriptSpan\0", "updateMeasureState\0", "(Landroid/text/TextPaint;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
