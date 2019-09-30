// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
__jni_bindgen! {
    /// public enum [ChronoUnit](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html)
    ///
    /// Required feature: java-time-temporal-ChronoUnit
    public enum ChronoUnit ("java/time/temporal/ChronoUnit") extends crate::java::lang::Enum, implements crate::java::time::temporal::TemporalUnit {

        /// [values](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#values())
        ///
        /// Required features: "java-time-temporal-ChronoUnit"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-ChronoUnit")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::time::temporal::ChronoUnit, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/time/temporal/ChronoUnit;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/ChronoUnit\0", "values\0", "()[Ljava/time/temporal/ChronoUnit;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-time-temporal-ChronoUnit"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-time-temporal-ChronoUnit")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/time/temporal/ChronoUnit;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/ChronoUnit\0", "valueOf\0", "(Ljava/lang/String;)Ljava/time/temporal/ChronoUnit;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [ChronoUnit](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#ChronoUnit(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/temporal/ChronoUnit", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDuration](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#getDuration())
        ///
        /// Required features: "java-time-Duration"
        #[cfg(any(feature = "all", all(feature = "java-time-Duration")))]
        pub fn getDuration<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Duration>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "getDuration", .descriptor == "()Ljava/time/Duration;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "getDuration\0", "()Ljava/time/Duration;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDurationEstimated](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#isDurationEstimated())
        pub fn isDurationEstimated<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "isDurationEstimated", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "isDurationEstimated\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDateBased](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#isDateBased())
        pub fn isDateBased<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "isDateBased", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "isDateBased\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isTimeBased](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#isTimeBased())
        pub fn isTimeBased<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "isTimeBased", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "isTimeBased\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSupportedBy](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#isSupportedBy(java.time.temporal.Temporal))
        ///
        /// Required features: "java-time-temporal-Temporal"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-Temporal")))]
        pub fn isSupportedBy<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::Temporal>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "isSupportedBy", .descriptor == "(Ljava/time/temporal/Temporal;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "isSupportedBy\0", "(Ljava/time/temporal/Temporal;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addTo](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#addTo(java.time.temporal.Temporal,%20long))
        ///
        /// Required features: "java-time-temporal-Temporal"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-Temporal")))]
        pub fn addTo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::Temporal>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::Temporal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "addTo", .descriptor == "(Ljava/time/temporal/Temporal;J)Ljava/time/temporal/Temporal;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "addTo\0", "(Ljava/time/temporal/Temporal;J)Ljava/time/temporal/Temporal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [between](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#between(java.time.temporal.Temporal,%20java.time.temporal.Temporal))
        ///
        /// Required features: "java-time-temporal-Temporal"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-Temporal")))]
        pub fn between<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::Temporal>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::Temporal>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "between", .descriptor == "(Ljava/time/temporal/Temporal;Ljava/time/temporal/Temporal;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "between\0", "(Ljava/time/temporal/Temporal;Ljava/time/temporal/Temporal;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/ChronoUnit", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/ChronoUnit\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [NANOS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#NANOS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn NANOS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "NANOS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MICROS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#MICROS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn MICROS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "MICROS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MILLIS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#MILLIS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn MILLIS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "MILLIS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SECONDS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#SECONDS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn SECONDS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "SECONDS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MINUTES](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#MINUTES)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn MINUTES<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "MINUTES\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOURS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#HOURS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn HOURS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "HOURS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HALF_DAYS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#HALF_DAYS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn HALF_DAYS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "HALF_DAYS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAYS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#DAYS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn DAYS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "DAYS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [WEEKS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#WEEKS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn WEEKS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "WEEKS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MONTHS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#MONTHS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn MONTHS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "MONTHS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [YEARS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#YEARS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn YEARS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "YEARS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DECADES](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#DECADES)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn DECADES<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "DECADES\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CENTURIES](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#CENTURIES)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn CENTURIES<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "CENTURIES\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MILLENNIA](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#MILLENNIA)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn MILLENNIA<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "MILLENNIA\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ERAS](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#ERAS)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn ERAS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "ERAS\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FOREVER](https://developer.android.com/reference/java/time/temporal/ChronoUnit.html#FOREVER)
        ///
        /// Required feature: java-time-temporal-ChronoUnit
        #[cfg(any(feature = "all", feature = "java-time-temporal-ChronoUnit"))]
        pub fn FOREVER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::ChronoUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/time/temporal/ChronoUnit\0", "FOREVER\0", "Ljava/time/temporal/ChronoUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::time::temporal::ChronoUnit, crate::java::lang::Throwable>>> { ... }
    }
}
