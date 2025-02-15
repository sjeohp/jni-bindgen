// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants"))]
__jni_bindgen! {
    /// public final class [DatatypeConstants](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html)
    ///
    /// Required feature: javax-xml-datatype-DatatypeConstants
    public final class DatatypeConstants ("javax/xml/datatype/DatatypeConstants") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [DatatypeConstants](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DatatypeConstants())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/xml/datatype/DatatypeConstants", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/datatype/DatatypeConstants\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [APRIL](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#APRIL)
        pub const APRIL : i32 = 4;

        /// public static final [AUGUST](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#AUGUST)
        pub const AUGUST : i32 = 8;

        /// **get** public static final [DATE](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DATE)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn DATE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "DATE\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DATETIME](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DATETIME)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn DATETIME<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "DATETIME\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAYS](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DAYS)
        ///
        /// Required feature: javax-xml-datatype-DatatypeConstants_Field
        #[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants_Field"))]
        pub fn DAYS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "DAYS\0", "Ljavax/xml/datatype/DatatypeConstants$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [DECEMBER](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DECEMBER)
        pub const DECEMBER : i32 = 12;

        /// **get** public static final [DURATION](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DURATION)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn DURATION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "DURATION\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DURATION_DAYTIME](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DURATION_DAYTIME)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn DURATION_DAYTIME<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "DURATION_DAYTIME\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DURATION_YEARMONTH](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#DURATION_YEARMONTH)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn DURATION_YEARMONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "DURATION_YEARMONTH\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [EQUAL](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#EQUAL)
        pub const EQUAL : i32 = 0;

        /// public static final [FEBRUARY](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#FEBRUARY)
        pub const FEBRUARY : i32 = 2;

        /// public static final [FIELD_UNDEFINED](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#FIELD_UNDEFINED)
        pub const FIELD_UNDEFINED : i32 = -2147483648;

        /// **get** public static final [GDAY](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#GDAY)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn GDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "GDAY\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [GMONTH](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#GMONTH)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn GMONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "GMONTH\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [GMONTHDAY](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#GMONTHDAY)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn GMONTHDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "GMONTHDAY\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [GREATER](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#GREATER)
        pub const GREATER : i32 = 1;

        /// **get** public static final [GYEAR](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#GYEAR)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn GYEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "GYEAR\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [GYEARMONTH](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#GYEARMONTH)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn GYEARMONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "GYEARMONTH\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOURS](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#HOURS)
        ///
        /// Required feature: javax-xml-datatype-DatatypeConstants_Field
        #[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants_Field"))]
        pub fn HOURS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "HOURS\0", "Ljavax/xml/datatype/DatatypeConstants$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [INDETERMINATE](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#INDETERMINATE)
        pub const INDETERMINATE : i32 = 2;

        /// public static final [JANUARY](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#JANUARY)
        pub const JANUARY : i32 = 1;

        /// public static final [JULY](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#JULY)
        pub const JULY : i32 = 7;

        /// public static final [JUNE](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#JUNE)
        pub const JUNE : i32 = 6;

        /// public static final [LESSER](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#LESSER)
        pub const LESSER : i32 = -1;

        /// public static final [MARCH](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#MARCH)
        pub const MARCH : i32 = 3;

        /// public static final [MAX_TIMEZONE_OFFSET](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#MAX_TIMEZONE_OFFSET)
        pub const MAX_TIMEZONE_OFFSET : i32 = -840;

        /// public static final [MAY](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#MAY)
        pub const MAY : i32 = 5;

        /// **get** public static final [MINUTES](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#MINUTES)
        ///
        /// Required feature: javax-xml-datatype-DatatypeConstants_Field
        #[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants_Field"))]
        pub fn MINUTES<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "MINUTES\0", "Ljavax/xml/datatype/DatatypeConstants$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [MIN_TIMEZONE_OFFSET](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#MIN_TIMEZONE_OFFSET)
        pub const MIN_TIMEZONE_OFFSET : i32 = 840;

        /// **get** public static final [MONTHS](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#MONTHS)
        ///
        /// Required feature: javax-xml-datatype-DatatypeConstants_Field
        #[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants_Field"))]
        pub fn MONTHS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "MONTHS\0", "Ljavax/xml/datatype/DatatypeConstants$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [NOVEMBER](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#NOVEMBER)
        pub const NOVEMBER : i32 = 11;

        /// public static final [OCTOBER](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#OCTOBER)
        pub const OCTOBER : i32 = 10;

        /// **get** public static final [SECONDS](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#SECONDS)
        ///
        /// Required feature: javax-xml-datatype-DatatypeConstants_Field
        #[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants_Field"))]
        pub fn SECONDS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "SECONDS\0", "Ljavax/xml/datatype/DatatypeConstants$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [SEPTEMBER](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#SEPTEMBER)
        pub const SEPTEMBER : i32 = 9;

        /// **get** public static final [TIME](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#TIME)
        ///
        /// Required feature: javax-xml-namespace-QName
        #[cfg(any(feature = "all", feature = "javax-xml-namespace-QName"))]
        pub fn TIME<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::namespace::QName>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "TIME\0", "Ljavax/xml/namespace/QName;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [YEARS](https://developer.android.com/reference/javax/xml/datatype/DatatypeConstants.html#YEARS)
        ///
        /// Required feature: javax-xml-datatype-DatatypeConstants_Field
        #[cfg(any(feature = "all", feature = "javax-xml-datatype-DatatypeConstants_Field"))]
        pub fn YEARS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::datatype::DatatypeConstants_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/xml/datatype/DatatypeConstants\0", "YEARS\0", "Ljavax/xml/datatype/DatatypeConstants$Field;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
