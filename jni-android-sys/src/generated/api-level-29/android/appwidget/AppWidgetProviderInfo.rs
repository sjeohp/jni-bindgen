// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-appwidget-AppWidgetProviderInfo"))]
__jni_bindgen! {
    /// public class [AppWidgetProviderInfo](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html)
    ///
    /// Required feature: android-appwidget-AppWidgetProviderInfo
    public class AppWidgetProviderInfo ("android/appwidget/AppWidgetProviderInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [AppWidgetProviderInfo](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#AppWidgetProviderInfo())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetProviderInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AppWidgetProviderInfo](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#AppWidgetProviderInfo(android.os.Parcel))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn new_Parcel<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetProviderInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadLabel](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#loadLabel(android.content.pm.PackageManager))
        ///
        /// Required features: "android-content-pm-PackageManager", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-pm-PackageManager", feature = "java-lang-String")))]
        pub fn loadLabel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::pm::PackageManager>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC | FINAL, .name == "loadLabel", .descriptor == "(Landroid/content/pm/PackageManager;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "loadLabel\0", "(Landroid/content/pm/PackageManager;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadIcon](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#loadIcon(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-graphics-drawable-Drawable")))]
        pub fn loadIcon<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC | FINAL, .name == "loadIcon", .descriptor == "(Landroid/content/Context;I)Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "loadIcon\0", "(Landroid/content/Context;I)Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [loadPreviewImage](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#loadPreviewImage(android.content.Context,%20int))
        ///
        /// Required features: "android-content-Context", "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-graphics-drawable-Drawable")))]
        pub fn loadPreviewImage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC | FINAL, .name == "loadPreviewImage", .descriptor == "(Landroid/content/Context;I)Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "loadPreviewImage\0", "(Landroid/content/Context;I)Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProfile](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#getProfile())
        ///
        /// Required features: "android-os-UserHandle"
        #[cfg(any(feature = "all", all(feature = "android-os-UserHandle")))]
        pub fn getProfile<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::UserHandle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC | FINAL, .name == "getProfile", .descriptor == "()Landroid/os/UserHandle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "getProfile\0", "()Landroid/os/UserHandle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#clone())
        ///
        /// Required features: "android-appwidget-AppWidgetProviderInfo"
        #[cfg(any(feature = "all", all(feature = "android-appwidget-AppWidgetProviderInfo")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::appwidget::AppWidgetProviderInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC, .name == "clone", .descriptor == "()Landroid/appwidget/AppWidgetProviderInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "clone\0", "()Landroid/appwidget/AppWidgetProviderInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [clone](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#clone())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/appwidget/AppWidgetProviderInfo", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/appwidget/AppWidgetProviderInfo\0", "clone\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/appwidget/AppWidgetProviderInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [RESIZE_BOTH](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#RESIZE_BOTH)
        pub const RESIZE_BOTH : i32 = 3;

        /// public static final [RESIZE_HORIZONTAL](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#RESIZE_HORIZONTAL)
        pub const RESIZE_HORIZONTAL : i32 = 1;

        /// public static final [RESIZE_NONE](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#RESIZE_NONE)
        pub const RESIZE_NONE : i32 = 0;

        /// public static final [RESIZE_VERTICAL](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#RESIZE_VERTICAL)
        pub const RESIZE_VERTICAL : i32 = 2;

        /// public static final [WIDGET_CATEGORY_HOME_SCREEN](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#WIDGET_CATEGORY_HOME_SCREEN)
        pub const WIDGET_CATEGORY_HOME_SCREEN : i32 = 1;

        /// public static final [WIDGET_CATEGORY_KEYGUARD](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#WIDGET_CATEGORY_KEYGUARD)
        pub const WIDGET_CATEGORY_KEYGUARD : i32 = 2;

        /// public static final [WIDGET_CATEGORY_SEARCHBOX](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#WIDGET_CATEGORY_SEARCHBOX)
        pub const WIDGET_CATEGORY_SEARCHBOX : i32 = 4;

        /// public static final [WIDGET_FEATURE_HIDE_FROM_PICKER](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#WIDGET_FEATURE_HIDE_FROM_PICKER)
        pub const WIDGET_FEATURE_HIDE_FROM_PICKER : i32 = 2;

        /// public static final [WIDGET_FEATURE_RECONFIGURABLE](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#WIDGET_FEATURE_RECONFIGURABLE)
        pub const WIDGET_FEATURE_RECONFIGURABLE : i32 = 1;

        /// **get** public [autoAdvanceViewId](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#autoAdvanceViewId)
        pub fn autoAdvanceViewId<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "autoAdvanceViewId\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [autoAdvanceViewId](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#autoAdvanceViewId)
        pub fn set_autoAdvanceViewId<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "autoAdvanceViewId\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [configure](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#configure)
        ///
        /// Required feature: android-content-ComponentName
        #[cfg(any(feature = "all", feature = "android-content-ComponentName"))]
        pub fn configure<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "configure\0", "Landroid/content/ComponentName;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [configure](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#configure)
        ///
        /// Required feature: android-content-ComponentName
        #[cfg(any(feature = "all", feature = "android-content-ComponentName"))]
        pub fn set_configure<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::content::ComponentName>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "configure\0", "Landroid/content/ComponentName;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [icon](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#icon)
        pub fn icon<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "icon\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [icon](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#icon)
        pub fn set_icon<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "icon\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [initialKeyguardLayout](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#initialKeyguardLayout)
        pub fn initialKeyguardLayout<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "initialKeyguardLayout\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [initialKeyguardLayout](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#initialKeyguardLayout)
        pub fn set_initialKeyguardLayout<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "initialKeyguardLayout\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [initialLayout](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#initialLayout)
        pub fn initialLayout<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "initialLayout\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [initialLayout](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#initialLayout)
        pub fn set_initialLayout<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "initialLayout\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [label](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#label)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        #[deprecated] pub fn label<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "label\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [label](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#label)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        #[deprecated] pub fn set_label<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::String>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "label\0", "Ljava/lang/String;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [minHeight](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minHeight)
        pub fn minHeight<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minHeight\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [minHeight](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minHeight)
        pub fn set_minHeight<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minHeight\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [minResizeHeight](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minResizeHeight)
        pub fn minResizeHeight<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minResizeHeight\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [minResizeHeight](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minResizeHeight)
        pub fn set_minResizeHeight<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minResizeHeight\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [minResizeWidth](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minResizeWidth)
        pub fn minResizeWidth<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minResizeWidth\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [minResizeWidth](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minResizeWidth)
        pub fn set_minResizeWidth<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minResizeWidth\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [minWidth](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minWidth)
        pub fn minWidth<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minWidth\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [minWidth](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#minWidth)
        pub fn set_minWidth<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "minWidth\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [previewImage](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#previewImage)
        pub fn previewImage<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "previewImage\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [previewImage](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#previewImage)
        pub fn set_previewImage<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "previewImage\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [provider](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#provider)
        ///
        /// Required feature: android-content-ComponentName
        #[cfg(any(feature = "all", feature = "android-content-ComponentName"))]
        pub fn provider<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "provider\0", "Landroid/content/ComponentName;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [provider](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#provider)
        ///
        /// Required feature: android-content-ComponentName
        #[cfg(any(feature = "all", feature = "android-content-ComponentName"))]
        pub fn set_provider<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::content::ComponentName>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "provider\0", "Landroid/content/ComponentName;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [resizeMode](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#resizeMode)
        pub fn resizeMode<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "resizeMode\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [resizeMode](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#resizeMode)
        pub fn set_resizeMode<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "resizeMode\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [updatePeriodMillis](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#updatePeriodMillis)
        pub fn updatePeriodMillis<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "updatePeriodMillis\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [updatePeriodMillis](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#updatePeriodMillis)
        pub fn set_updatePeriodMillis<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "updatePeriodMillis\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [widgetCategory](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#widgetCategory)
        pub fn widgetCategory<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "widgetCategory\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [widgetCategory](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#widgetCategory)
        pub fn set_widgetCategory<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "widgetCategory\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [widgetFeatures](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#widgetFeatures)
        pub fn widgetFeatures<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "widgetFeatures\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [widgetFeatures](https://developer.android.com/reference/android/appwidget/AppWidgetProviderInfo.html#widgetFeatures)
        pub fn set_widgetFeatures<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/appwidget/AppWidgetProviderInfo\0", "widgetFeatures\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
