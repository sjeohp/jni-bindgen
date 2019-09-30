// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-MonthDisplayHelper"))]
__jni_bindgen! {
    /// public class [MonthDisplayHelper](https://developer.android.com/reference/android/util/MonthDisplayHelper.html)
    ///
    /// Required feature: android-util-MonthDisplayHelper
    public class MonthDisplayHelper ("android/util/MonthDisplayHelper") extends crate::java::lang::Object {

        /// [MonthDisplayHelper](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#MonthDisplayHelper(int,%20int,%20int))
        pub fn new_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::MonthDisplayHelper>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "<init>", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "<init>\0", "(III)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [MonthDisplayHelper](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#MonthDisplayHelper(int,%20int))
        pub fn new_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::MonthDisplayHelper>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getYear](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getYear())
        pub fn getYear<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getYear", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getYear\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMonth](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getMonth())
        pub fn getMonth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getMonth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getMonth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWeekStartDay](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getWeekStartDay())
        pub fn getWeekStartDay<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getWeekStartDay", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getWeekStartDay\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFirstDayOfMonth](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getFirstDayOfMonth())
        pub fn getFirstDayOfMonth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getFirstDayOfMonth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getFirstDayOfMonth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumberOfDaysInMonth](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getNumberOfDaysInMonth())
        pub fn getNumberOfDaysInMonth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getNumberOfDaysInMonth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getNumberOfDaysInMonth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOffset](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getOffset())
        pub fn getOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDigitsForRow](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getDigitsForRow(int))
        pub fn getDigitsForRow<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getDigitsForRow", .descriptor == "(I)[I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getDigitsForRow\0", "(I)[I\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDayAt](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getDayAt(int,%20int))
        pub fn getDayAt<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getDayAt", .descriptor == "(II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getDayAt\0", "(II)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRowOf](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getRowOf(int))
        pub fn getRowOf<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getRowOf", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getRowOf\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getColumnOf](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#getColumnOf(int))
        pub fn getColumnOf<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "getColumnOf", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "getColumnOf\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [previousMonth](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#previousMonth())
        pub fn previousMonth<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "previousMonth", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "previousMonth\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [nextMonth](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#nextMonth())
        pub fn nextMonth<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "nextMonth", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "nextMonth\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isWithinCurrentMonth](https://developer.android.com/reference/android/util/MonthDisplayHelper.html#isWithinCurrentMonth(int,%20int))
        pub fn isWithinCurrentMonth<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/MonthDisplayHelper", java.flags == PUBLIC, .name == "isWithinCurrentMonth", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/MonthDisplayHelper\0", "isWithinCurrentMonth\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
