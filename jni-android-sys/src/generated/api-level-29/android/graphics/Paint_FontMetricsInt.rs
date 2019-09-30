// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-Paint_FontMetricsInt"))]
__jni_bindgen! {
    /// public class [Paint.FontMetricsInt](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html)
    ///
    /// Required feature: android-graphics-Paint_FontMetricsInt
    public class Paint_FontMetricsInt ("android/graphics/Paint$FontMetricsInt") extends crate::java::lang::Object {

        /// [FontMetricsInt](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#FontMetricsInt())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::Paint_FontMetricsInt>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Paint$FontMetricsInt", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Paint$FontMetricsInt\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/Paint$FontMetricsInt", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/Paint$FontMetricsInt\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [ascent](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#ascent)
        pub fn ascent<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "ascent\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [ascent](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#ascent)
        pub fn set_ascent<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "ascent\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [bottom](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#bottom)
        pub fn bottom<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "bottom\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [bottom](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#bottom)
        pub fn set_bottom<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "bottom\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [descent](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#descent)
        pub fn descent<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "descent\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [descent](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#descent)
        pub fn set_descent<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "descent\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [leading](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#leading)
        pub fn leading<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "leading\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [leading](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#leading)
        pub fn set_leading<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "leading\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [top](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#top)
        pub fn top<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "top\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [top](https://developer.android.com/reference/android/graphics/Paint.FontMetricsInt.html#top)
        pub fn set_top<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/graphics/Paint$FontMetricsInt\0", "top\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
