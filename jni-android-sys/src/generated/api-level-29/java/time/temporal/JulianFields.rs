// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-temporal-JulianFields"))]
__jni_bindgen! {
    /// public final class [JulianFields](https://developer.android.com/reference/java/time/temporal/JulianFields.html)
    ///
    /// Required feature: java-time-temporal-JulianFields
    public final class JulianFields ("java/time/temporal/JulianFields") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [JulianFields](https://developer.android.com/reference/java/time/temporal/JulianFields.html#JulianFields())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::temporal::JulianFields>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/temporal/JulianFields", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/JulianFields\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [JULIAN_DAY](https://developer.android.com/reference/java/time/temporal/JulianFields.html#JULIAN_DAY)
        ///
        /// Required feature: java-time-temporal-TemporalField
        #[cfg(any(feature = "all", feature = "java-time-temporal-TemporalField"))]
        pub fn JULIAN_DAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalField>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/JulianFields\0", "JULIAN_DAY\0", "Ljava/time/temporal/TemporalField;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MODIFIED_JULIAN_DAY](https://developer.android.com/reference/java/time/temporal/JulianFields.html#MODIFIED_JULIAN_DAY)
        ///
        /// Required feature: java-time-temporal-TemporalField
        #[cfg(any(feature = "all", feature = "java-time-temporal-TemporalField"))]
        pub fn MODIFIED_JULIAN_DAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalField>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/JulianFields\0", "MODIFIED_JULIAN_DAY\0", "Ljava/time/temporal/TemporalField;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [RATA_DIE](https://developer.android.com/reference/java/time/temporal/JulianFields.html#RATA_DIE)
        ///
        /// Required feature: java-time-temporal-TemporalField
        #[cfg(any(feature = "all", feature = "java-time-temporal-TemporalField"))]
        pub fn RATA_DIE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalField>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/JulianFields\0", "RATA_DIE\0", "Ljava/time/temporal/TemporalField;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
