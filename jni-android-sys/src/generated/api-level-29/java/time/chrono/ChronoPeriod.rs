// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-chrono-ChronoPeriod"))]
__jni_bindgen! {
    /// public interface [ChronoPeriod](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html)
    ///
    /// Required feature: java-time-chrono-ChronoPeriod
    public interface ChronoPeriod ("java/time/chrono/ChronoPeriod") extends crate::java::lang::Object, implements crate::java::time::temporal::TemporalAmount {

        /// [between](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#between(java.time.chrono.ChronoLocalDate,%20java.time.chrono.ChronoLocalDate))
        ///
        /// Required features: "java-time-chrono-ChronoLocalDate", "java-time-chrono-ChronoPeriod"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-ChronoLocalDate", feature = "java-time-chrono-ChronoPeriod")))]
        pub fn between<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::chrono::ChronoLocalDate>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::chrono::ChronoLocalDate>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::ChronoPeriod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | STATIC, .name == "between", .descriptor == "(Ljava/time/chrono/ChronoLocalDate;Ljava/time/chrono/ChronoLocalDate;)Ljava/time/chrono/ChronoPeriod;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/chrono/ChronoPeriod\0", "between\0", "(Ljava/time/chrono/ChronoLocalDate;Ljava/time/chrono/ChronoLocalDate;)Ljava/time/chrono/ChronoPeriod;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#get(java.time.temporal.TemporalUnit))
        ///
        /// Required features: "java-time-temporal-TemporalUnit"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalUnit")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::TemporalUnit>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "get", .descriptor == "(Ljava/time/temporal/TemporalUnit;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "get\0", "(Ljava/time/temporal/TemporalUnit;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUnits](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#getUnits())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getUnits<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "getUnits", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "getUnits\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChronology](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#getChronology())
        ///
        /// Required features: "java-time-chrono-Chronology"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-Chronology")))]
        pub fn getChronology<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::Chronology>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "getChronology", .descriptor == "()Ljava/time/chrono/Chronology;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "getChronology\0", "()Ljava/time/chrono/Chronology;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isZero](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#isZero())
        pub fn isZero<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC, .name == "isZero", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "isZero\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isNegative](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#isNegative())
        pub fn isNegative<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC, .name == "isNegative", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "isNegative\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [plus](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#plus(java.time.temporal.TemporalAmount))
        ///
        /// Required features: "java-time-chrono-ChronoPeriod", "java-time-temporal-TemporalAmount"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-ChronoPeriod", feature = "java-time-temporal-TemporalAmount")))]
        pub fn plus<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::TemporalAmount>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::ChronoPeriod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "plus", .descriptor == "(Ljava/time/temporal/TemporalAmount;)Ljava/time/chrono/ChronoPeriod;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "plus\0", "(Ljava/time/temporal/TemporalAmount;)Ljava/time/chrono/ChronoPeriod;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [minus](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#minus(java.time.temporal.TemporalAmount))
        ///
        /// Required features: "java-time-chrono-ChronoPeriod", "java-time-temporal-TemporalAmount"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-ChronoPeriod", feature = "java-time-temporal-TemporalAmount")))]
        pub fn minus<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::TemporalAmount>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::ChronoPeriod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "minus", .descriptor == "(Ljava/time/temporal/TemporalAmount;)Ljava/time/chrono/ChronoPeriod;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "minus\0", "(Ljava/time/temporal/TemporalAmount;)Ljava/time/chrono/ChronoPeriod;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [multipliedBy](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#multipliedBy(int))
        ///
        /// Required features: "java-time-chrono-ChronoPeriod"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-ChronoPeriod")))]
        pub fn multipliedBy<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::ChronoPeriod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "multipliedBy", .descriptor == "(I)Ljava/time/chrono/ChronoPeriod;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "multipliedBy\0", "(I)Ljava/time/chrono/ChronoPeriod;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [negated](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#negated())
        ///
        /// Required features: "java-time-chrono-ChronoPeriod"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-ChronoPeriod")))]
        pub fn negated<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::ChronoPeriod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC, .name == "negated", .descriptor == "()Ljava/time/chrono/ChronoPeriod;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "negated\0", "()Ljava/time/chrono/ChronoPeriod;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [normalized](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#normalized())
        ///
        /// Required features: "java-time-chrono-ChronoPeriod"
        #[cfg(any(feature = "all", all(feature = "java-time-chrono-ChronoPeriod")))]
        pub fn normalized<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::chrono::ChronoPeriod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "normalized", .descriptor == "()Ljava/time/chrono/ChronoPeriod;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "normalized\0", "()Ljava/time/chrono/ChronoPeriod;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addTo](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#addTo(java.time.temporal.Temporal))
        ///
        /// Required features: "java-time-temporal-Temporal"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-Temporal")))]
        pub fn addTo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::Temporal>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::Temporal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "addTo", .descriptor == "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "addTo\0", "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [subtractFrom](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#subtractFrom(java.time.temporal.Temporal))
        ///
        /// Required features: "java-time-temporal-Temporal"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-Temporal")))]
        pub fn subtractFrom<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::temporal::Temporal>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::Temporal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "subtractFrom", .descriptor == "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "subtractFrom\0", "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/time/chrono/ChronoPeriod.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/chrono/ChronoPeriod", java.flags == PUBLIC | ABSTRACT, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/chrono/ChronoPeriod\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
