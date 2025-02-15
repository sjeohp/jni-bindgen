// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-location-GnssMeasurement"))]
__jni_bindgen! {
    /// public final class [GnssMeasurement](https://developer.android.com/reference/android/location/GnssMeasurement.html)
    ///
    /// Required feature: android-location-GnssMeasurement
    public final class GnssMeasurement ("android/location/GnssMeasurement") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [GnssMeasurement](https://developer.android.com/reference/android/location/GnssMeasurement.html#GnssMeasurement())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::location::GnssMeasurement>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/location/GnssMeasurement", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getSvid](https://developer.android.com/reference/android/location/GnssMeasurement.html#getSvid())
        pub fn getSvid<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getSvid", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getSvid\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConstellationType](https://developer.android.com/reference/android/location/GnssMeasurement.html#getConstellationType())
        pub fn getConstellationType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getConstellationType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getConstellationType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeOffsetNanos](https://developer.android.com/reference/android/location/GnssMeasurement.html#getTimeOffsetNanos())
        pub fn getTimeOffsetNanos<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getTimeOffsetNanos", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getTimeOffsetNanos\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getState](https://developer.android.com/reference/android/location/GnssMeasurement.html#getState())
        pub fn getState<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getState", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getState\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReceivedSvTimeNanos](https://developer.android.com/reference/android/location/GnssMeasurement.html#getReceivedSvTimeNanos())
        pub fn getReceivedSvTimeNanos<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getReceivedSvTimeNanos", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getReceivedSvTimeNanos\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReceivedSvTimeUncertaintyNanos](https://developer.android.com/reference/android/location/GnssMeasurement.html#getReceivedSvTimeUncertaintyNanos())
        pub fn getReceivedSvTimeUncertaintyNanos<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getReceivedSvTimeUncertaintyNanos", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getReceivedSvTimeUncertaintyNanos\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCn0DbHz](https://developer.android.com/reference/android/location/GnssMeasurement.html#getCn0DbHz())
        pub fn getCn0DbHz<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getCn0DbHz", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getCn0DbHz\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPseudorangeRateMetersPerSecond](https://developer.android.com/reference/android/location/GnssMeasurement.html#getPseudorangeRateMetersPerSecond())
        pub fn getPseudorangeRateMetersPerSecond<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getPseudorangeRateMetersPerSecond", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getPseudorangeRateMetersPerSecond\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPseudorangeRateUncertaintyMetersPerSecond](https://developer.android.com/reference/android/location/GnssMeasurement.html#getPseudorangeRateUncertaintyMetersPerSecond())
        pub fn getPseudorangeRateUncertaintyMetersPerSecond<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getPseudorangeRateUncertaintyMetersPerSecond", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getPseudorangeRateUncertaintyMetersPerSecond\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccumulatedDeltaRangeState](https://developer.android.com/reference/android/location/GnssMeasurement.html#getAccumulatedDeltaRangeState())
        pub fn getAccumulatedDeltaRangeState<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getAccumulatedDeltaRangeState", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getAccumulatedDeltaRangeState\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccumulatedDeltaRangeMeters](https://developer.android.com/reference/android/location/GnssMeasurement.html#getAccumulatedDeltaRangeMeters())
        pub fn getAccumulatedDeltaRangeMeters<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getAccumulatedDeltaRangeMeters", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getAccumulatedDeltaRangeMeters\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAccumulatedDeltaRangeUncertaintyMeters](https://developer.android.com/reference/android/location/GnssMeasurement.html#getAccumulatedDeltaRangeUncertaintyMeters())
        pub fn getAccumulatedDeltaRangeUncertaintyMeters<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getAccumulatedDeltaRangeUncertaintyMeters", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getAccumulatedDeltaRangeUncertaintyMeters\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasCarrierFrequencyHz](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasCarrierFrequencyHz())
        pub fn hasCarrierFrequencyHz<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasCarrierFrequencyHz", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasCarrierFrequencyHz\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCarrierFrequencyHz](https://developer.android.com/reference/android/location/GnssMeasurement.html#getCarrierFrequencyHz())
        pub fn getCarrierFrequencyHz<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getCarrierFrequencyHz", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getCarrierFrequencyHz\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasCarrierCycles](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasCarrierCycles())
        #[deprecated] pub fn hasCarrierCycles<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasCarrierCycles", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasCarrierCycles\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCarrierCycles](https://developer.android.com/reference/android/location/GnssMeasurement.html#getCarrierCycles())
        #[deprecated] pub fn getCarrierCycles<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getCarrierCycles", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getCarrierCycles\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasCarrierPhase](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasCarrierPhase())
        #[deprecated] pub fn hasCarrierPhase<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasCarrierPhase", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasCarrierPhase\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCarrierPhase](https://developer.android.com/reference/android/location/GnssMeasurement.html#getCarrierPhase())
        #[deprecated] pub fn getCarrierPhase<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getCarrierPhase", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getCarrierPhase\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasCarrierPhaseUncertainty](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasCarrierPhaseUncertainty())
        #[deprecated] pub fn hasCarrierPhaseUncertainty<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasCarrierPhaseUncertainty", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasCarrierPhaseUncertainty\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCarrierPhaseUncertainty](https://developer.android.com/reference/android/location/GnssMeasurement.html#getCarrierPhaseUncertainty())
        #[deprecated] pub fn getCarrierPhaseUncertainty<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getCarrierPhaseUncertainty", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getCarrierPhaseUncertainty\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMultipathIndicator](https://developer.android.com/reference/android/location/GnssMeasurement.html#getMultipathIndicator())
        pub fn getMultipathIndicator<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getMultipathIndicator", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getMultipathIndicator\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasSnrInDb](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasSnrInDb())
        pub fn hasSnrInDb<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasSnrInDb", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasSnrInDb\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSnrInDb](https://developer.android.com/reference/android/location/GnssMeasurement.html#getSnrInDb())
        pub fn getSnrInDb<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getSnrInDb", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getSnrInDb\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasAutomaticGainControlLevelDb](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasAutomaticGainControlLevelDb())
        pub fn hasAutomaticGainControlLevelDb<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasAutomaticGainControlLevelDb", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasAutomaticGainControlLevelDb\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAutomaticGainControlLevelDb](https://developer.android.com/reference/android/location/GnssMeasurement.html#getAutomaticGainControlLevelDb())
        pub fn getAutomaticGainControlLevelDb<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getAutomaticGainControlLevelDb", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getAutomaticGainControlLevelDb\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasCodeType](https://developer.android.com/reference/android/location/GnssMeasurement.html#hasCodeType())
        pub fn hasCodeType<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "hasCodeType", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "hasCodeType\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCodeType](https://developer.android.com/reference/android/location/GnssMeasurement.html#getCodeType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getCodeType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "getCodeType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "getCodeType\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/location/GnssMeasurement.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/location/GnssMeasurement.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/location/GnssMeasurement.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/location/GnssMeasurement", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/location/GnssMeasurement\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ADR_STATE_CYCLE_SLIP](https://developer.android.com/reference/android/location/GnssMeasurement.html#ADR_STATE_CYCLE_SLIP)
        pub const ADR_STATE_CYCLE_SLIP : i32 = 4;

        /// public static final [ADR_STATE_HALF_CYCLE_REPORTED](https://developer.android.com/reference/android/location/GnssMeasurement.html#ADR_STATE_HALF_CYCLE_REPORTED)
        pub const ADR_STATE_HALF_CYCLE_REPORTED : i32 = 16;

        /// public static final [ADR_STATE_HALF_CYCLE_RESOLVED](https://developer.android.com/reference/android/location/GnssMeasurement.html#ADR_STATE_HALF_CYCLE_RESOLVED)
        pub const ADR_STATE_HALF_CYCLE_RESOLVED : i32 = 8;

        /// public static final [ADR_STATE_RESET](https://developer.android.com/reference/android/location/GnssMeasurement.html#ADR_STATE_RESET)
        pub const ADR_STATE_RESET : i32 = 2;

        /// public static final [ADR_STATE_UNKNOWN](https://developer.android.com/reference/android/location/GnssMeasurement.html#ADR_STATE_UNKNOWN)
        pub const ADR_STATE_UNKNOWN : i32 = 0;

        /// public static final [ADR_STATE_VALID](https://developer.android.com/reference/android/location/GnssMeasurement.html#ADR_STATE_VALID)
        pub const ADR_STATE_VALID : i32 = 1;

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/location/GnssMeasurement.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/location/GnssMeasurement\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [MULTIPATH_INDICATOR_DETECTED](https://developer.android.com/reference/android/location/GnssMeasurement.html#MULTIPATH_INDICATOR_DETECTED)
        pub const MULTIPATH_INDICATOR_DETECTED : i32 = 1;

        /// public static final [MULTIPATH_INDICATOR_NOT_DETECTED](https://developer.android.com/reference/android/location/GnssMeasurement.html#MULTIPATH_INDICATOR_NOT_DETECTED)
        pub const MULTIPATH_INDICATOR_NOT_DETECTED : i32 = 2;

        /// public static final [MULTIPATH_INDICATOR_UNKNOWN](https://developer.android.com/reference/android/location/GnssMeasurement.html#MULTIPATH_INDICATOR_UNKNOWN)
        pub const MULTIPATH_INDICATOR_UNKNOWN : i32 = 0;

        /// public static final [STATE_2ND_CODE_LOCK](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_2ND_CODE_LOCK)
        pub const STATE_2ND_CODE_LOCK : i32 = 65536;

        /// public static final [STATE_BDS_D2_BIT_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_BDS_D2_BIT_SYNC)
        pub const STATE_BDS_D2_BIT_SYNC : i32 = 256;

        /// public static final [STATE_BDS_D2_SUBFRAME_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_BDS_D2_SUBFRAME_SYNC)
        pub const STATE_BDS_D2_SUBFRAME_SYNC : i32 = 512;

        /// public static final [STATE_BIT_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_BIT_SYNC)
        pub const STATE_BIT_SYNC : i32 = 2;

        /// public static final [STATE_CODE_LOCK](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_CODE_LOCK)
        pub const STATE_CODE_LOCK : i32 = 1;

        /// public static final [STATE_GAL_E1BC_CODE_LOCK](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_GAL_E1BC_CODE_LOCK)
        pub const STATE_GAL_E1BC_CODE_LOCK : i32 = 1024;

        /// public static final [STATE_GAL_E1B_PAGE_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_GAL_E1B_PAGE_SYNC)
        pub const STATE_GAL_E1B_PAGE_SYNC : i32 = 4096;

        /// public static final [STATE_GAL_E1C_2ND_CODE_LOCK](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_GAL_E1C_2ND_CODE_LOCK)
        pub const STATE_GAL_E1C_2ND_CODE_LOCK : i32 = 2048;

        /// public static final [STATE_GLO_STRING_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_GLO_STRING_SYNC)
        pub const STATE_GLO_STRING_SYNC : i32 = 64;

        /// public static final [STATE_GLO_TOD_DECODED](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_GLO_TOD_DECODED)
        pub const STATE_GLO_TOD_DECODED : i32 = 128;

        /// public static final [STATE_GLO_TOD_KNOWN](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_GLO_TOD_KNOWN)
        pub const STATE_GLO_TOD_KNOWN : i32 = 32768;

        /// public static final [STATE_MSEC_AMBIGUOUS](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_MSEC_AMBIGUOUS)
        pub const STATE_MSEC_AMBIGUOUS : i32 = 16;

        /// public static final [STATE_SBAS_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_SBAS_SYNC)
        pub const STATE_SBAS_SYNC : i32 = 8192;

        /// public static final [STATE_SUBFRAME_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_SUBFRAME_SYNC)
        pub const STATE_SUBFRAME_SYNC : i32 = 4;

        /// public static final [STATE_SYMBOL_SYNC](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_SYMBOL_SYNC)
        pub const STATE_SYMBOL_SYNC : i32 = 32;

        /// public static final [STATE_TOW_DECODED](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_TOW_DECODED)
        pub const STATE_TOW_DECODED : i32 = 8;

        /// public static final [STATE_TOW_KNOWN](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_TOW_KNOWN)
        pub const STATE_TOW_KNOWN : i32 = 16384;

        /// public static final [STATE_UNKNOWN](https://developer.android.com/reference/android/location/GnssMeasurement.html#STATE_UNKNOWN)
        pub const STATE_UNKNOWN : i32 = 0;
    }
}
