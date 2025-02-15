// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-gesture-Gesture"))]
__jni_bindgen! {
    /// public class [Gesture](https://developer.android.com/reference/android/gesture/Gesture.html)
    ///
    /// Required feature: android-gesture-Gesture
    public class Gesture ("android/gesture/Gesture") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        /// [Gesture](https://developer.android.com/reference/android/gesture/Gesture.html#Gesture())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::gesture::Gesture>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/android/gesture/Gesture.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStrokes](https://developer.android.com/reference/android/gesture/Gesture.html#getStrokes())
        ///
        /// Required features: "java-util-ArrayList"
        #[cfg(any(feature = "all", all(feature = "java-util-ArrayList")))]
        pub fn getStrokes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::ArrayList>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "getStrokes", .descriptor == "()Ljava/util/ArrayList;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "getStrokes\0", "()Ljava/util/ArrayList;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStrokesCount](https://developer.android.com/reference/android/gesture/Gesture.html#getStrokesCount())
        pub fn getStrokesCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "getStrokesCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "getStrokesCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addStroke](https://developer.android.com/reference/android/gesture/Gesture.html#addStroke(android.gesture.GestureStroke))
        ///
        /// Required features: "android-gesture-GestureStroke"
        #[cfg(any(feature = "all", all(feature = "android-gesture-GestureStroke")))]
        pub fn addStroke<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::gesture::GestureStroke>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "addStroke", .descriptor == "(Landroid/gesture/GestureStroke;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "addStroke\0", "(Landroid/gesture/GestureStroke;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLength](https://developer.android.com/reference/android/gesture/Gesture.html#getLength())
        pub fn getLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "getLength", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "getLength\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBoundingBox](https://developer.android.com/reference/android/gesture/Gesture.html#getBoundingBox())
        ///
        /// Required features: "android-graphics-RectF"
        #[cfg(any(feature = "all", all(feature = "android-graphics-RectF")))]
        pub fn getBoundingBox<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::RectF>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "getBoundingBox", .descriptor == "()Landroid/graphics/RectF;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "getBoundingBox\0", "()Landroid/graphics/RectF;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toPath](https://developer.android.com/reference/android/gesture/Gesture.html#toPath())
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn toPath<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "toPath", .descriptor == "()Landroid/graphics/Path;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "toPath\0", "()Landroid/graphics/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toPath](https://developer.android.com/reference/android/gesture/Gesture.html#toPath(android.graphics.Path))
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn toPath_Path<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Path>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "toPath", .descriptor == "(Landroid/graphics/Path;)Landroid/graphics/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "toPath\0", "(Landroid/graphics/Path;)Landroid/graphics/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toPath](https://developer.android.com/reference/android/gesture/Gesture.html#toPath(int,%20int,%20int,%20int))
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn toPath_int_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "toPath", .descriptor == "(IIII)Landroid/graphics/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "toPath\0", "(IIII)Landroid/graphics/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toPath](https://developer.android.com/reference/android/gesture/Gesture.html#toPath(android.graphics.Path,%20int,%20int,%20int,%20int))
        ///
        /// Required features: "android-graphics-Path"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Path")))]
        pub fn toPath_Path_int_int_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Path>>, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Path>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "toPath", .descriptor == "(Landroid/graphics/Path;IIII)Landroid/graphics/Path;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "toPath\0", "(Landroid/graphics/Path;IIII)Landroid/graphics/Path;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getID](https://developer.android.com/reference/android/gesture/Gesture.html#getID())
        pub fn getID<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "getID", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "getID\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toBitmap](https://developer.android.com/reference/android/gesture/Gesture.html#toBitmap(int,%20int,%20int,%20int,%20int))
        ///
        /// Required features: "android-graphics-Bitmap"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Bitmap")))]
        pub fn toBitmap_int_int_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Bitmap>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "toBitmap", .descriptor == "(IIIII)Landroid/graphics/Bitmap;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "toBitmap\0", "(IIIII)Landroid/graphics/Bitmap;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toBitmap](https://developer.android.com/reference/android/gesture/Gesture.html#toBitmap(int,%20int,%20int,%20int))
        ///
        /// Required features: "android-graphics-Bitmap"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Bitmap")))]
        pub fn toBitmap_int_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Bitmap>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "toBitmap", .descriptor == "(IIII)Landroid/graphics/Bitmap;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "toBitmap\0", "(IIII)Landroid/graphics/Bitmap;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/gesture/Gesture.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/gesture/Gesture.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/Gesture", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/Gesture\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/gesture/Gesture.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/gesture/Gesture\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
