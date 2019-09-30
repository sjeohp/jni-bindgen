// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-CalendarContract"))]
__jni_bindgen! {
    /// public final class [CalendarContract](https://developer.android.com/reference/android/provider/CalendarContract.html)
    ///
    /// Required feature: android-provider-CalendarContract
    public final class CalendarContract ("android/provider/CalendarContract") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [CalendarContract](https://developer.android.com/reference/android/provider/CalendarContract.html#CalendarContract())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::CalendarContract>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/CalendarContract", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/CalendarContract\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [startViewCalendarEventInManagedProfile](https://developer.android.com/reference/android/provider/CalendarContract.html#startViewCalendarEventInManagedProfile(android.content.Context,%20long,%20long,%20long,%20boolean,%20int))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn startViewCalendarEventInManagedProfile<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i64, arg2: i64, arg3: i64, arg4: bool, arg5: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/CalendarContract", java.flags == PUBLIC | STATIC, .name == "startViewCalendarEventInManagedProfile", .descriptor == "(Landroid/content/Context;JJJZI)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/provider/CalendarContract\0", "startViewCalendarEventInManagedProfile\0", "(Landroid/content/Context;JJJZI)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACCOUNT_TYPE_LOCAL](https://developer.android.com/reference/android/provider/CalendarContract.html#ACCOUNT_TYPE_LOCAL)
        pub const ACCOUNT_TYPE_LOCAL : &'static str = "LOCAL";

        /// public static final [ACTION_EVENT_REMINDER](https://developer.android.com/reference/android/provider/CalendarContract.html#ACTION_EVENT_REMINDER)
        pub const ACTION_EVENT_REMINDER : &'static str = "android.intent.action.EVENT_REMINDER";

        /// public static final [ACTION_HANDLE_CUSTOM_EVENT](https://developer.android.com/reference/android/provider/CalendarContract.html#ACTION_HANDLE_CUSTOM_EVENT)
        pub const ACTION_HANDLE_CUSTOM_EVENT : &'static str = "android.provider.calendar.action.HANDLE_CUSTOM_EVENT";

        /// public static final [ACTION_VIEW_MANAGED_PROFILE_CALENDAR_EVENT](https://developer.android.com/reference/android/provider/CalendarContract.html#ACTION_VIEW_MANAGED_PROFILE_CALENDAR_EVENT)
        pub const ACTION_VIEW_MANAGED_PROFILE_CALENDAR_EVENT : &'static str = "android.provider.calendar.action.VIEW_MANAGED_PROFILE_CALENDAR_EVENT";

        /// public static final [AUTHORITY](https://developer.android.com/reference/android/provider/CalendarContract.html#AUTHORITY)
        pub const AUTHORITY : &'static str = "com.android.calendar";

        /// public static final [CALLER_IS_SYNCADAPTER](https://developer.android.com/reference/android/provider/CalendarContract.html#CALLER_IS_SYNCADAPTER)
        pub const CALLER_IS_SYNCADAPTER : &'static str = "caller_is_syncadapter";

        /// **get** public static final [CONTENT_URI](https://developer.android.com/reference/android/provider/CalendarContract.html#CONTENT_URI)
        ///
        /// Required feature: android-net-Uri
        #[cfg(any(feature = "all", feature = "android-net-Uri"))]
        pub fn CONTENT_URI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/CalendarContract\0", "CONTENT_URI\0", "Landroid/net/Uri;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [EXTRA_CUSTOM_APP_URI](https://developer.android.com/reference/android/provider/CalendarContract.html#EXTRA_CUSTOM_APP_URI)
        pub const EXTRA_CUSTOM_APP_URI : &'static str = "customAppUri";

        /// public static final [EXTRA_EVENT_ALL_DAY](https://developer.android.com/reference/android/provider/CalendarContract.html#EXTRA_EVENT_ALL_DAY)
        pub const EXTRA_EVENT_ALL_DAY : &'static str = "allDay";

        /// public static final [EXTRA_EVENT_BEGIN_TIME](https://developer.android.com/reference/android/provider/CalendarContract.html#EXTRA_EVENT_BEGIN_TIME)
        pub const EXTRA_EVENT_BEGIN_TIME : &'static str = "beginTime";

        /// public static final [EXTRA_EVENT_END_TIME](https://developer.android.com/reference/android/provider/CalendarContract.html#EXTRA_EVENT_END_TIME)
        pub const EXTRA_EVENT_END_TIME : &'static str = "endTime";

        /// public static final [EXTRA_EVENT_ID](https://developer.android.com/reference/android/provider/CalendarContract.html#EXTRA_EVENT_ID)
        pub const EXTRA_EVENT_ID : &'static str = "id";
    }
}
