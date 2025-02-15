// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-inputmethodservice-Keyboard_Key"))]
__jni_bindgen! {
    /// public class [Keyboard.Key](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html)
    ///
    /// Required feature: android-inputmethodservice-Keyboard_Key
    #[deprecated] public class Keyboard_Key ("android/inputmethodservice/Keyboard$Key") extends crate::java::lang::Object {

        /// [Key](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#Key(android.inputmethodservice.Keyboard.Row))
        ///
        /// Required features: "android-inputmethodservice-Keyboard_Row"
        #[cfg(any(feature = "all", all(feature = "android-inputmethodservice-Keyboard_Row")))]
        #[deprecated] pub fn new_Row<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::inputmethodservice::Keyboard_Row>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::inputmethodservice::Keyboard_Key>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/inputmethodservice/Keyboard$Row;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "<init>\0", "(Landroid/inputmethodservice/Keyboard$Row;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Key](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#Key(android.content.res.Resources,%20android.inputmethodservice.Keyboard.Row,%20int,%20int,%20android.content.res.XmlResourceParser))
        ///
        /// Required features: "android-content-res-Resources", "android-content-res-XmlResourceParser", "android-inputmethodservice-Keyboard_Row"
        #[cfg(any(feature = "all", all(feature = "android-content-res-Resources", feature = "android-content-res-XmlResourceParser", feature = "android-inputmethodservice-Keyboard_Row")))]
        #[deprecated] pub fn new_Resources_Row_int_int_XmlResourceParser<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::Resources>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::inputmethodservice::Keyboard_Row>>, arg2: i32, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::res::XmlResourceParser>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::inputmethodservice::Keyboard_Key>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/res/Resources;Landroid/inputmethodservice/Keyboard$Row;IILandroid/content/res/XmlResourceParser;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "<init>\0", "(Landroid/content/res/Resources;Landroid/inputmethodservice/Keyboard$Row;IILandroid/content/res/XmlResourceParser;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPressed](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#onPressed())
        #[deprecated] pub fn onPressed<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "onPressed", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "onPressed\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onReleased](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#onReleased(boolean))
        #[deprecated] pub fn onReleased<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "onReleased", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "onReleased\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isInside](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#isInside(int,%20int))
        #[deprecated] pub fn isInside<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "isInside", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "isInside\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [squaredDistanceFrom](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#squaredDistanceFrom(int,%20int))
        #[deprecated] pub fn squaredDistanceFrom<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "squaredDistanceFrom", .descriptor == "(II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "squaredDistanceFrom\0", "(II)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentDrawableState](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#getCurrentDrawableState())
        #[deprecated] pub fn getCurrentDrawableState<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/inputmethodservice/Keyboard$Key", java.flags == PUBLIC, .name == "getCurrentDrawableState", .descriptor == "()[I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/inputmethodservice/Keyboard$Key\0", "getCurrentDrawableState\0", "()[I\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public [codes](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#codes)
        #[deprecated] pub fn codes<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "codes\0", "[I\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [codes](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#codes)
        #[deprecated] pub fn set_codes<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::IntArray>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "codes\0", "[I\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [edgeFlags](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#edgeFlags)
        #[deprecated] pub fn edgeFlags<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "edgeFlags\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [edgeFlags](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#edgeFlags)
        #[deprecated] pub fn set_edgeFlags<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "edgeFlags\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [gap](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#gap)
        #[deprecated] pub fn gap<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "gap\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [gap](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#gap)
        #[deprecated] pub fn set_gap<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "gap\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [height](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#height)
        #[deprecated] pub fn height<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "height\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [height](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#height)
        #[deprecated] pub fn set_height<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "height\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [icon](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#icon)
        ///
        /// Required feature: android-graphics-drawable-Drawable
        #[cfg(any(feature = "all", feature = "android-graphics-drawable-Drawable"))]
        #[deprecated] pub fn icon<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "icon\0", "Landroid/graphics/drawable/Drawable;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [icon](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#icon)
        ///
        /// Required feature: android-graphics-drawable-Drawable
        #[cfg(any(feature = "all", feature = "android-graphics-drawable-Drawable"))]
        #[deprecated] pub fn set_icon<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::graphics::drawable::Drawable>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "icon\0", "Landroid/graphics/drawable/Drawable;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [iconPreview](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#iconPreview)
        ///
        /// Required feature: android-graphics-drawable-Drawable
        #[cfg(any(feature = "all", feature = "android-graphics-drawable-Drawable"))]
        #[deprecated] pub fn iconPreview<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::graphics::drawable::Drawable>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "iconPreview\0", "Landroid/graphics/drawable/Drawable;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [iconPreview](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#iconPreview)
        ///
        /// Required feature: android-graphics-drawable-Drawable
        #[cfg(any(feature = "all", feature = "android-graphics-drawable-Drawable"))]
        #[deprecated] pub fn set_iconPreview<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::android::graphics::drawable::Drawable>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "iconPreview\0", "Landroid/graphics/drawable/Drawable;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [label](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#label)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn label<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "label\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [label](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#label)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn set_label<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "label\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [modifier](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#modifier)
        #[deprecated] pub fn modifier<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "modifier\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [modifier](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#modifier)
        #[deprecated] pub fn set_modifier<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "modifier\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [on](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#on)
        #[deprecated] pub fn on<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "on\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [on](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#on)
        #[deprecated] pub fn set_on<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "on\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [popupCharacters](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#popupCharacters)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn popupCharacters<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "popupCharacters\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [popupCharacters](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#popupCharacters)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn set_popupCharacters<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "popupCharacters\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [popupResId](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#popupResId)
        #[deprecated] pub fn popupResId<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "popupResId\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [popupResId](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#popupResId)
        #[deprecated] pub fn set_popupResId<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "popupResId\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [pressed](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#pressed)
        #[deprecated] pub fn pressed<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "pressed\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [pressed](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#pressed)
        #[deprecated] pub fn set_pressed<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "pressed\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [repeatable](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#repeatable)
        #[deprecated] pub fn repeatable<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "repeatable\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [repeatable](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#repeatable)
        #[deprecated] pub fn set_repeatable<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "repeatable\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [sticky](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#sticky)
        #[deprecated] pub fn sticky<'env>(&'env self) -> bool {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "sticky\0", "Z\0");
                env.get_boolean_field(class, field)
            }
        }

        /// **set** public [sticky](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#sticky)
        #[deprecated] pub fn set_sticky<'env>(&'env self, value: bool) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "sticky\0", "Z\0");
                env.set_boolean_field(class, field, value)
            }
        }

        /// **get** public [text](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#text)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn text<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "text\0", "Ljava/lang/CharSequence;\0");
                env.get_object_field(class, field)
            }
        }

        /// **set** public [text](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#text)
        ///
        /// Required feature: java-lang-CharSequence
        #[cfg(any(feature = "all", feature = "java-lang-CharSequence"))]
        #[deprecated] pub fn set_text<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::lang::CharSequence>>) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "text\0", "Ljava/lang/CharSequence;\0");
                env.set_object_field(class, field, value)
            }
        }

        /// **get** public [width](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#width)
        #[deprecated] pub fn width<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "width\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [width](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#width)
        #[deprecated] pub fn set_width<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "width\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [x](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#x)
        #[deprecated] pub fn x<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "x\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [x](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#x)
        #[deprecated] pub fn set_x<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "x\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }

        /// **get** public [y](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#y)
        #[deprecated] pub fn y<'env>(&'env self) -> i32 {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "y\0", "I\0");
                env.get_int_field(class, field)
            }
        }

        /// **set** public [y](https://developer.android.com/reference/android/inputmethodservice/Keyboard.Key.html#y)
        #[deprecated] pub fn set_y<'env>(&'env self, value: i32) {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/inputmethodservice/Keyboard$Key\0", "y\0", "I\0");
                env.set_int_field(class, field, value)
            }
        }
    }
}
