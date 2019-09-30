// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-TypedValue"))]
__jni_bindgen! {
    /// public class [TypedValue](https://developer.android.com/reference/android/util/TypedValue.html)
    ///
    /// Required feature: android-util-TypedValue
    public class TypedValue ("android/util/TypedValue") extends crate::java::lang::Object {

        /// [TypedValue](https://developer.android.com/reference/android/util/TypedValue.html#TypedValue())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::TypedValue>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFloat](https://developer.android.com/reference/android/util/TypedValue.html#getFloat())
        pub fn getFloat<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | FINAL, .name == "getFloat", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "getFloat\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isColorType](https://developer.android.com/reference/android/util/TypedValue.html#isColorType())
        pub fn isColorType<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "isColorType", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "isColorType\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [complexToFloat](https://developer.android.com/reference/android/util/TypedValue.html#complexToFloat(int))
        pub fn complexToFloat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC, .name == "complexToFloat", .descriptor == "(I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "complexToFloat\0", "(I)F\0");
                __jni_env.call_static_float_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [complexToDimension](https://developer.android.com/reference/android/util/TypedValue.html#complexToDimension(int,%20android.util.DisplayMetrics))
        ///
        /// Required features: "android-util-DisplayMetrics"
        #[cfg(any(feature = "all", all(feature = "android-util-DisplayMetrics")))]
        pub fn complexToDimension<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::DisplayMetrics>>) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC, .name == "complexToDimension", .descriptor == "(ILandroid/util/DisplayMetrics;)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "complexToDimension\0", "(ILandroid/util/DisplayMetrics;)F\0");
                __jni_env.call_static_float_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [complexToDimensionPixelOffset](https://developer.android.com/reference/android/util/TypedValue.html#complexToDimensionPixelOffset(int,%20android.util.DisplayMetrics))
        ///
        /// Required features: "android-util-DisplayMetrics"
        #[cfg(any(feature = "all", all(feature = "android-util-DisplayMetrics")))]
        pub fn complexToDimensionPixelOffset<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::DisplayMetrics>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC, .name == "complexToDimensionPixelOffset", .descriptor == "(ILandroid/util/DisplayMetrics;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "complexToDimensionPixelOffset\0", "(ILandroid/util/DisplayMetrics;)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [complexToDimensionPixelSize](https://developer.android.com/reference/android/util/TypedValue.html#complexToDimensionPixelSize(int,%20android.util.DisplayMetrics))
        ///
        /// Required features: "android-util-DisplayMetrics"
        #[cfg(any(feature = "all", all(feature = "android-util-DisplayMetrics")))]
        pub fn complexToDimensionPixelSize<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::DisplayMetrics>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC, .name == "complexToDimensionPixelSize", .descriptor == "(ILandroid/util/DisplayMetrics;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "complexToDimensionPixelSize\0", "(ILandroid/util/DisplayMetrics;)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getComplexUnit](https://developer.android.com/reference/android/util/TypedValue.html#getComplexUnit())
        pub fn getComplexUnit<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "getComplexUnit", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "getComplexUnit\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyDimension](https://developer.android.com/reference/android/util/TypedValue.html#applyDimension(int,%20float,%20android.util.DisplayMetrics))
        ///
        /// Required features: "android-util-DisplayMetrics"
        #[cfg(any(feature = "all", all(feature = "android-util-DisplayMetrics")))]
        pub fn applyDimension<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: f32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::DisplayMetrics>>) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC, .name == "applyDimension", .descriptor == "(IFLandroid/util/DisplayMetrics;)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "applyDimension\0", "(IFLandroid/util/DisplayMetrics;)F\0");
                __jni_env.call_static_float_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDimension](https://developer.android.com/reference/android/util/TypedValue.html#getDimension(android.util.DisplayMetrics))
        ///
        /// Required features: "android-util-DisplayMetrics"
        #[cfg(any(feature = "all", all(feature = "android-util-DisplayMetrics")))]
        pub fn getDimension<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::DisplayMetrics>>) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "getDimension", .descriptor == "(Landroid/util/DisplayMetrics;)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "getDimension\0", "(Landroid/util/DisplayMetrics;)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [complexToFraction](https://developer.android.com/reference/android/util/TypedValue.html#complexToFraction(int,%20float,%20float))
        pub fn complexToFraction<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: f32, arg2: f32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC, .name == "complexToFraction", .descriptor == "(IFF)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "complexToFraction\0", "(IFF)F\0");
                __jni_env.call_static_float_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFraction](https://developer.android.com/reference/android/util/TypedValue.html#getFraction(float,%20float))
        pub fn getFraction<'env>(&'env self, arg0: f32, arg1: f32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "getFraction", .descriptor == "(FF)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "getFraction\0", "(FF)F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [coerceToString](https://developer.android.com/reference/android/util/TypedValue.html#coerceToString())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn coerceToString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | FINAL, .name == "coerceToString", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "coerceToString\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [coerceToString](https://developer.android.com/reference/android/util/TypedValue.html#coerceToString(int,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn coerceToString_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC | STATIC | FINAL, .name == "coerceToString", .descriptor == "(II)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/TypedValue\0", "coerceToString\0", "(II)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTo](https://developer.android.com/reference/android/util/TypedValue.html#setTo(android.util.TypedValue))
        ///
        /// Required features: "android-util-TypedValue"
        #[cfg(any(feature = "all", all(feature = "android-util-TypedValue")))]
        pub fn setTo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::TypedValue>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "setTo", .descriptor == "(Landroid/util/TypedValue;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "setTo\0", "(Landroid/util/TypedValue;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/util/TypedValue.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/TypedValue", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/TypedValue\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [COMPLEX_MANTISSA_MASK](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_MANTISSA_MASK)
        pub const COMPLEX_MANTISSA_MASK : i32 = 16777215;

        /// public static final [COMPLEX_MANTISSA_SHIFT](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_MANTISSA_SHIFT)
        pub const COMPLEX_MANTISSA_SHIFT : i32 = 8;

        /// public static final [COMPLEX_RADIX_0p23](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_RADIX_0p23)
        pub const COMPLEX_RADIX_0p23 : i32 = 3;

        /// public static final [COMPLEX_RADIX_16p7](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_RADIX_16p7)
        pub const COMPLEX_RADIX_16p7 : i32 = 1;

        /// public static final [COMPLEX_RADIX_23p0](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_RADIX_23p0)
        pub const COMPLEX_RADIX_23p0 : i32 = 0;

        /// public static final [COMPLEX_RADIX_8p15](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_RADIX_8p15)
        pub const COMPLEX_RADIX_8p15 : i32 = 2;

        /// public static final [COMPLEX_RADIX_MASK](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_RADIX_MASK)
        pub const COMPLEX_RADIX_MASK : i32 = 3;

        /// public static final [COMPLEX_RADIX_SHIFT](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_RADIX_SHIFT)
        pub const COMPLEX_RADIX_SHIFT : i32 = 4;

        /// public static final [COMPLEX_UNIT_DIP](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_DIP)
        pub const COMPLEX_UNIT_DIP : i32 = 1;

        /// public static final [COMPLEX_UNIT_FRACTION](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_FRACTION)
        pub const COMPLEX_UNIT_FRACTION : i32 = 0;

        /// public static final [COMPLEX_UNIT_FRACTION_PARENT](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_FRACTION_PARENT)
        pub const COMPLEX_UNIT_FRACTION_PARENT : i32 = 1;

        /// public static final [COMPLEX_UNIT_IN](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_IN)
        pub const COMPLEX_UNIT_IN : i32 = 4;

        /// public static final [COMPLEX_UNIT_MASK](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_MASK)
        pub const COMPLEX_UNIT_MASK : i32 = 15;

        /// public static final [COMPLEX_UNIT_MM](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_MM)
        pub const COMPLEX_UNIT_MM : i32 = 5;

        /// public static final [COMPLEX_UNIT_PT](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_PT)
        pub const COMPLEX_UNIT_PT : i32 = 3;

        /// public static final [COMPLEX_UNIT_PX](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_PX)
        pub const COMPLEX_UNIT_PX : i32 = 0;

        /// public static final [COMPLEX_UNIT_SHIFT](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_SHIFT)
        pub const COMPLEX_UNIT_SHIFT : i32 = 0;

        /// public static final [COMPLEX_UNIT_SP](https://developer.android.com/reference/android/util/TypedValue.html#COMPLEX_UNIT_SP)
        pub const COMPLEX_UNIT_SP : i32 = 2;

        /// public static final [DATA_NULL_EMPTY](https://developer.android.com/reference/android/util/TypedValue.html#DATA_NULL_EMPTY)
        pub const DATA_NULL_EMPTY : i32 = 1;

        /// public static final [DATA_NULL_UNDEFINED](https://developer.android.com/reference/android/util/TypedValue.html#DATA_NULL_UNDEFINED)
        pub const DATA_NULL_UNDEFINED : i32 = 0;

        /// public static final [DENSITY_DEFAULT](https://developer.android.com/reference/android/util/TypedValue.html#DENSITY_DEFAULT)
        pub const DENSITY_DEFAULT : i32 = 0;

        /// public static final [DENSITY_NONE](https://developer.android.com/reference/android/util/TypedValue.html#DENSITY_NONE)
        pub const DENSITY_NONE : i32 = 65535;

        /// public static final [TYPE_ATTRIBUTE](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_ATTRIBUTE)
        pub const TYPE_ATTRIBUTE : i32 = 2;

        /// public static final [TYPE_DIMENSION](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_DIMENSION)
        pub const TYPE_DIMENSION : i32 = 5;

        /// public static final [TYPE_FIRST_COLOR_INT](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_FIRST_COLOR_INT)
        pub const TYPE_FIRST_COLOR_INT : i32 = 28;

        /// public static final [TYPE_FIRST_INT](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_FIRST_INT)
        pub const TYPE_FIRST_INT : i32 = 16;

        /// public static final [TYPE_FLOAT](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_FLOAT)
        pub const TYPE_FLOAT : i32 = 4;

        /// public static final [TYPE_FRACTION](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_FRACTION)
        pub const TYPE_FRACTION : i32 = 6;

        /// public static final [TYPE_INT_BOOLEAN](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_BOOLEAN)
        pub const TYPE_INT_BOOLEAN : i32 = 18;

        /// public static final [TYPE_INT_COLOR_ARGB4](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_COLOR_ARGB4)
        pub const TYPE_INT_COLOR_ARGB4 : i32 = 30;

        /// public static final [TYPE_INT_COLOR_ARGB8](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_COLOR_ARGB8)
        pub const TYPE_INT_COLOR_ARGB8 : i32 = 28;

        /// public static final [TYPE_INT_COLOR_RGB4](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_COLOR_RGB4)
        pub const TYPE_INT_COLOR_RGB4 : i32 = 31;

        /// public static final [TYPE_INT_COLOR_RGB8](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_COLOR_RGB8)
        pub const TYPE_INT_COLOR_RGB8 : i32 = 29;

        /// public static final [TYPE_INT_DEC](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_DEC)
        pub const TYPE_INT_DEC : i32 = 16;

        /// public static final [TYPE_INT_HEX](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_INT_HEX)
        pub const TYPE_INT_HEX : i32 = 17;

        /// public static final [TYPE_LAST_COLOR_INT](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_LAST_COLOR_INT)
        pub const TYPE_LAST_COLOR_INT : i32 = 31;

        /// public static final [TYPE_LAST_INT](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_LAST_INT)
        pub const TYPE_LAST_INT : i32 = 31;

        /// public static final [TYPE_NULL](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_NULL)
        pub const TYPE_NULL : i32 = 0;

        /// public static final [TYPE_REFERENCE](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_REFERENCE)
        pub const TYPE_REFERENCE : i32 = 1;

        /// public static final [TYPE_STRING](https://developer.android.com/reference/android/util/TypedValue.html#TYPE_STRING)
        pub const TYPE_STRING : i32 = 3;

        /// **get** public [assetCookie](https://developer.android.com/reference/android/util/TypedValue.html#assetCookie)
        pub fn assetCookie<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "assetCookie\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [assetCookie](https://developer.android.com/reference/android/util/TypedValue.html#assetCookie)
        pub fn set_assetCookie<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "assetCookie\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [changingConfigurations](https://developer.android.com/reference/android/util/TypedValue.html#changingConfigurations)
        pub fn changingConfigurations<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "changingConfigurations\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [changingConfigurations](https://developer.android.com/reference/android/util/TypedValue.html#changingConfigurations)
        pub fn set_changingConfigurations<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "changingConfigurations\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [data](https://developer.android.com/reference/android/util/TypedValue.html#data)
        pub fn data<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "data\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [data](https://developer.android.com/reference/android/util/TypedValue.html#data)
        pub fn set_data<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "data\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [density](https://developer.android.com/reference/android/util/TypedValue.html#density)
        pub fn density<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "density\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [density](https://developer.android.com/reference/android/util/TypedValue.html#density)
        pub fn set_density<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "density\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [resourceId](https://developer.android.com/reference/android/util/TypedValue.html#resourceId)
        pub fn resourceId<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "resourceId\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [resourceId](https://developer.android.com/reference/android/util/TypedValue.html#resourceId)
        pub fn set_resourceId<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "resourceId\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [sourceResourceId](https://developer.android.com/reference/android/util/TypedValue.html#sourceResourceId)
        pub fn sourceResourceId<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "sourceResourceId\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [sourceResourceId](https://developer.android.com/reference/android/util/TypedValue.html#sourceResourceId)
        pub fn set_sourceResourceId<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "sourceResourceId\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [string](https://developer.android.com/reference/android/util/TypedValue.html#string)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn string<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "string\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [string](https://developer.android.com/reference/android/util/TypedValue.html#string)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        pub fn set_string<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "string\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [type](https://developer.android.com/reference/android/util/TypedValue.html#type)
        pub fn r#type<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "type\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [type](https://developer.android.com/reference/android/util/TypedValue.html#type)
        pub fn set_type<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/util/TypedValue\0", "type\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
