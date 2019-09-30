// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-graphics-drawable-ShapeDrawable"))]
__jni_bindgen! {
    /// public class [ShapeDrawable](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html)
    ///
    /// Required feature: android-graphics-drawable-ShapeDrawable
    public class ShapeDrawable ("android/graphics/drawable/ShapeDrawable") extends crate::android::graphics::drawable::Drawable {

        /// [ShapeDrawable](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#ShapeDrawable())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::ShapeDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ShapeDrawable](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#ShapeDrawable(android.graphics.drawable.shapes.Shape))
        ///
        /// Required features: "android-graphics-drawable-shapes-Shape"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-shapes-Shape")))]
        pub fn new_Shape<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::shapes::Shape>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::graphics::drawable::ShapeDrawable>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/graphics/drawable/shapes/Shape;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "<init>\0", "(Landroid/graphics/drawable/shapes/Shape;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getShape](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getShape())
        ///
        /// Required features: "android-graphics-drawable-shapes-Shape"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-shapes-Shape")))]
        pub fn getShape<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::shapes::Shape>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getShape", .descriptor == "()Landroid/graphics/drawable/shapes/Shape;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getShape\0", "()Landroid/graphics/drawable/shapes/Shape;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setShape](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setShape(android.graphics.drawable.shapes.Shape))
        ///
        /// Required features: "android-graphics-drawable-shapes-Shape"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-shapes-Shape")))]
        pub fn setShape<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::shapes::Shape>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setShape", .descriptor == "(Landroid/graphics/drawable/shapes/Shape;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setShape\0", "(Landroid/graphics/drawable/shapes/Shape;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setShaderFactory](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setShaderFactory(android.graphics.drawable.ShapeDrawable.ShaderFactory))
        ///
        /// Required features: "android-graphics-drawable-ShapeDrawable_ShaderFactory"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-ShapeDrawable_ShaderFactory")))]
        pub fn setShaderFactory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::ShapeDrawable_ShaderFactory>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setShaderFactory", .descriptor == "(Landroid/graphics/drawable/ShapeDrawable$ShaderFactory;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setShaderFactory\0", "(Landroid/graphics/drawable/ShapeDrawable$ShaderFactory;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getShaderFactory](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getShaderFactory())
        ///
        /// Required features: "android-graphics-drawable-ShapeDrawable_ShaderFactory"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-ShapeDrawable_ShaderFactory")))]
        pub fn getShaderFactory<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::ShapeDrawable_ShaderFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getShaderFactory", .descriptor == "()Landroid/graphics/drawable/ShapeDrawable$ShaderFactory;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getShaderFactory\0", "()Landroid/graphics/drawable/ShapeDrawable$ShaderFactory;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPaint](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getPaint())
        ///
        /// Required features: "android-graphics-Paint"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Paint")))]
        pub fn getPaint<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::Paint>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getPaint", .descriptor == "()Landroid/graphics/Paint;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getPaint\0", "()Landroid/graphics/Paint;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPadding](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setPadding(int,%20int,%20int,%20int))
        pub fn setPadding_int_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setPadding", .descriptor == "(IIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setPadding\0", "(IIII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPadding](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setPadding(android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn setPadding_Rect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setPadding", .descriptor == "(Landroid/graphics/Rect;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setPadding\0", "(Landroid/graphics/Rect;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIntrinsicWidth](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setIntrinsicWidth(int))
        pub fn setIntrinsicWidth<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setIntrinsicWidth", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setIntrinsicWidth\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIntrinsicHeight](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setIntrinsicHeight(int))
        pub fn setIntrinsicHeight<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setIntrinsicHeight", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setIntrinsicHeight\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntrinsicWidth](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getIntrinsicWidth())
        pub fn getIntrinsicWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getIntrinsicWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getIntrinsicWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIntrinsicHeight](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getIntrinsicHeight())
        pub fn getIntrinsicHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getIntrinsicHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getIntrinsicHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPadding](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getPadding(android.graphics.Rect))
        ///
        /// Required features: "android-graphics-Rect"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        pub fn getPadding<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getPadding", .descriptor == "(Landroid/graphics/Rect;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getPadding\0", "(Landroid/graphics/Rect;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onDraw](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#onDraw(android.graphics.drawable.shapes.Shape,%20android.graphics.Canvas,%20android.graphics.Paint))
        // ///
        // /// Required features: "android-graphics-Canvas", "android-graphics-Paint", "android-graphics-drawable-shapes-Shape"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-Canvas", feature = "android-graphics-Paint", feature = "android-graphics-drawable-shapes-Shape")))]
        // fn onDraw<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::drawable::shapes::Shape>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Canvas>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Paint>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PROTECTED, .name == "onDraw", .descriptor == "(Landroid/graphics/drawable/shapes/Shape;Landroid/graphics/Canvas;Landroid/graphics/Paint;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "onDraw\0", "(Landroid/graphics/drawable/shapes/Shape;Landroid/graphics/Canvas;Landroid/graphics/Paint;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [draw](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#draw(android.graphics.Canvas))
        ///
        /// Required features: "android-graphics-Canvas"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Canvas")))]
        pub fn draw<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Canvas>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "draw", .descriptor == "(Landroid/graphics/Canvas;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "draw\0", "(Landroid/graphics/Canvas;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChangingConfigurations](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getChangingConfigurations())
        pub fn getChangingConfigurations<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getChangingConfigurations", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getChangingConfigurations\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAlpha](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setAlpha(int))
        pub fn setAlpha<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setAlpha", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setAlpha\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlpha](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getAlpha())
        pub fn getAlpha<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getAlpha", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getAlpha\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTintList](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setTintList(android.content.res.ColorStateList))
        ///
        /// Required features: "android-content-res-ColorStateList"
        #[cfg(any(feature = "all", all(feature = "android-content-res-ColorStateList")))]
        pub fn setTintList<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::ColorStateList>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setTintList", .descriptor == "(Landroid/content/res/ColorStateList;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setTintList\0", "(Landroid/content/res/ColorStateList;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTintBlendMode](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setTintBlendMode(android.graphics.BlendMode))
        ///
        /// Required features: "android-graphics-BlendMode"
        #[cfg(any(feature = "all", all(feature = "android-graphics-BlendMode")))]
        pub fn setTintBlendMode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::BlendMode>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setTintBlendMode", .descriptor == "(Landroid/graphics/BlendMode;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setTintBlendMode\0", "(Landroid/graphics/BlendMode;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setColorFilter](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setColorFilter(android.graphics.ColorFilter))
        ///
        /// Required features: "android-graphics-ColorFilter"
        #[cfg(any(feature = "all", all(feature = "android-graphics-ColorFilter")))]
        pub fn setColorFilter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::ColorFilter>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setColorFilter", .descriptor == "(Landroid/graphics/ColorFilter;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setColorFilter\0", "(Landroid/graphics/ColorFilter;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOpacity](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getOpacity())
        pub fn getOpacity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getOpacity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getOpacity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDither](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#setDither(boolean))
        pub fn setDither<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "setDither", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "setDither\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onBoundsChange](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#onBoundsChange(android.graphics.Rect))
        // ///
        // /// Required features: "android-graphics-Rect"
        // #[cfg(any(feature = "all", all(feature = "android-graphics-Rect")))]
        // fn onBoundsChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Rect>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PROTECTED, .name == "onBoundsChange", .descriptor == "(Landroid/graphics/Rect;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "onBoundsChange\0", "(Landroid/graphics/Rect;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onStateChange](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#onStateChange(int%5B%5D))
        // fn onStateChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PROTECTED, .name == "onStateChange", .descriptor == "([I)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "onStateChange\0", "([I)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isStateful](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#isStateful())
        pub fn isStateful<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "isStateful", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "isStateful\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [inflateTag](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#inflateTag(java.lang.String,%20android.content.res.Resources,%20org.xmlpull.v1.XmlPullParser,%20android.util.AttributeSet))
        // ///
        // /// Required features: "android-content-res-Resources", "android-util-AttributeSet", "java-lang-String", "org-xmlpull-v1-XmlPullParser"
        // #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-util-AttributeSet", feature = "java-lang-String", feature = "org-xmlpull-v1-XmlPullParser")))]
        // fn inflateTag<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PROTECTED, .name == "inflateTag", .descriptor == "(Ljava/lang/String;Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "inflateTag\0", "(Ljava/lang/String;Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [inflate](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#inflate(android.content.res.Resources,%20org.xmlpull.v1.XmlPullParser,%20android.util.AttributeSet,%20android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources", "android-content-res-Resources_Theme", "android-util-AttributeSet", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-content-res-Resources_Theme", feature = "android-util-AttributeSet", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "inflate\0", "(Landroid/content/res/Resources;Lorg/xmlpull/v1/XmlPullParser;Landroid/util/AttributeSet;Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [applyTheme](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#applyTheme(android.content.res.Resources.Theme))
        ///
        /// Required features: "android-content-res-Resources_Theme"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources_Theme")))]
        pub fn applyTheme<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources_Theme>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "applyTheme", .descriptor == "(Landroid/content/res/Resources$Theme;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "applyTheme\0", "(Landroid/content/res/Resources$Theme;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOutline](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getOutline(android.graphics.Outline))
        ///
        /// Required features: "android-graphics-Outline"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Outline")))]
        pub fn getOutline<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Outline>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getOutline", .descriptor == "(Landroid/graphics/Outline;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getOutline\0", "(Landroid/graphics/Outline;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getConstantState](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#getConstantState())
        ///
        /// Required features: "android-graphics-drawable-Drawable_ConstantState"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable_ConstantState")))]
        pub fn getConstantState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable_ConstantState>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "getConstantState", .descriptor == "()Landroid/graphics/drawable/Drawable$ConstantState;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "getConstantState\0", "()Landroid/graphics/drawable/Drawable$ConstantState;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mutate](https://developer.android.com/reference/android/graphics/drawable/ShapeDrawable.html#mutate())
        ///
        /// Required features: "android-graphics-drawable-Drawable"
        #[cfg(any(feature = "all", all(feature = "android-graphics-drawable-Drawable")))]
        pub fn mutate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/graphics/drawable/ShapeDrawable", java.flags == PUBLIC, .name == "mutate", .descriptor == "()Landroid/graphics/drawable/Drawable;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/graphics/drawable/ShapeDrawable\0", "mutate\0", "()Landroid/graphics/drawable/Drawable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
