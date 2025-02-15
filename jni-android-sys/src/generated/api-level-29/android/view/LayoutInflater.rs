// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-LayoutInflater"))]
__jni_bindgen! {
    /// public class [LayoutInflater](https://developer.android.com/reference/android/view/LayoutInflater.html)
    ///
    /// Required feature: android-view-LayoutInflater
    public class LayoutInflater ("android/view/LayoutInflater") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [LayoutInflater](https://developer.android.com/reference/android/view/LayoutInflater.html#LayoutInflater(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/LayoutInflater", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [LayoutInflater](https://developer.android.com/reference/android/view/LayoutInflater.html#LayoutInflater(android.view.LayoutInflater,%20android.content.Context))
        // ///
        // /// Required features: "android-content-Context", "android-view-LayoutInflater"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-LayoutInflater")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::LayoutInflater>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/LayoutInflater", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Landroid/view/LayoutInflater;Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "<init>\0", "(Landroid/view/LayoutInflater;Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [from](https://developer.android.com/reference/android/view/LayoutInflater.html#from(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-view-LayoutInflater"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-LayoutInflater")))]
        pub fn from<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC | STATIC, .name == "from", .descriptor == "(Landroid/content/Context;)Landroid/view/LayoutInflater;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/LayoutInflater\0", "from\0", "(Landroid/content/Context;)Landroid/view/LayoutInflater;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cloneInContext](https://developer.android.com/reference/android/view/LayoutInflater.html#cloneInContext(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-view-LayoutInflater"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-view-LayoutInflater")))]
        pub fn cloneInContext<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC | ABSTRACT, .name == "cloneInContext", .descriptor == "(Landroid/content/Context;)Landroid/view/LayoutInflater;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "cloneInContext\0", "(Landroid/content/Context;)Landroid/view/LayoutInflater;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getContext](https://developer.android.com/reference/android/view/LayoutInflater.html#getContext())
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn getContext<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Context>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "getContext", .descriptor == "()Landroid/content/Context;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "getContext\0", "()Landroid/content/Context;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFactory](https://developer.android.com/reference/android/view/LayoutInflater.html#getFactory())
        ///
        /// Required features: "android-view-LayoutInflater_Factory"
        #[cfg(any(feature = "all", all(feature = "android-view-LayoutInflater_Factory")))]
        pub fn getFactory<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater_Factory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC | FINAL, .name == "getFactory", .descriptor == "()Landroid/view/LayoutInflater$Factory;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "getFactory\0", "()Landroid/view/LayoutInflater$Factory;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFactory2](https://developer.android.com/reference/android/view/LayoutInflater.html#getFactory2())
        ///
        /// Required features: "android-view-LayoutInflater_Factory2"
        #[cfg(any(feature = "all", all(feature = "android-view-LayoutInflater_Factory2")))]
        pub fn getFactory2<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater_Factory2>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC | FINAL, .name == "getFactory2", .descriptor == "()Landroid/view/LayoutInflater$Factory2;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "getFactory2\0", "()Landroid/view/LayoutInflater$Factory2;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFactory](https://developer.android.com/reference/android/view/LayoutInflater.html#setFactory(android.view.LayoutInflater.Factory))
        ///
        /// Required features: "android-view-LayoutInflater_Factory"
        #[cfg(any(feature = "all", all(feature = "android-view-LayoutInflater_Factory")))]
        pub fn setFactory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::LayoutInflater_Factory>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "setFactory", .descriptor == "(Landroid/view/LayoutInflater$Factory;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "setFactory\0", "(Landroid/view/LayoutInflater$Factory;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFactory2](https://developer.android.com/reference/android/view/LayoutInflater.html#setFactory2(android.view.LayoutInflater.Factory2))
        ///
        /// Required features: "android-view-LayoutInflater_Factory2"
        #[cfg(any(feature = "all", all(feature = "android-view-LayoutInflater_Factory2")))]
        pub fn setFactory2<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::LayoutInflater_Factory2>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "setFactory2", .descriptor == "(Landroid/view/LayoutInflater$Factory2;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "setFactory2\0", "(Landroid/view/LayoutInflater$Factory2;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFilter](https://developer.android.com/reference/android/view/LayoutInflater.html#getFilter())
        ///
        /// Required features: "android-view-LayoutInflater_Filter"
        #[cfg(any(feature = "all", all(feature = "android-view-LayoutInflater_Filter")))]
        pub fn getFilter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::LayoutInflater_Filter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "getFilter", .descriptor == "()Landroid/view/LayoutInflater$Filter;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "getFilter\0", "()Landroid/view/LayoutInflater$Filter;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFilter](https://developer.android.com/reference/android/view/LayoutInflater.html#setFilter(android.view.LayoutInflater.Filter))
        ///
        /// Required features: "android-view-LayoutInflater_Filter"
        #[cfg(any(feature = "all", all(feature = "android-view-LayoutInflater_Filter")))]
        pub fn setFilter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::LayoutInflater_Filter>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "setFilter", .descriptor == "(Landroid/view/LayoutInflater$Filter;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "setFilter\0", "(Landroid/view/LayoutInflater$Filter;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/view/LayoutInflater.html#inflate(int,%20android.view.ViewGroup))
        ///
        /// Required features: "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn inflate_int_ViewGroup<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "inflate", .descriptor == "(ILandroid/view/ViewGroup;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "inflate\0", "(ILandroid/view/ViewGroup;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/view/LayoutInflater.html#inflate(org.xmlpull.v1.XmlPullParser,%20android.view.ViewGroup))
        ///
        /// Required features: "android-view-View", "android-view-ViewGroup", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewGroup", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate_XmlPullParser_ViewGroup<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Lorg/xmlpull/v1/XmlPullParser;Landroid/view/ViewGroup;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "inflate\0", "(Lorg/xmlpull/v1/XmlPullParser;Landroid/view/ViewGroup;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/view/LayoutInflater.html#inflate(int,%20android.view.ViewGroup,%20boolean))
        ///
        /// Required features: "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn inflate_int_ViewGroup_boolean<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg2: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "inflate", .descriptor == "(ILandroid/view/ViewGroup;Z)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "inflate\0", "(ILandroid/view/ViewGroup;Z)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [inflate](https://developer.android.com/reference/android/view/LayoutInflater.html#inflate(org.xmlpull.v1.XmlPullParser,%20android.view.ViewGroup,%20boolean))
        ///
        /// Required features: "android-view-View", "android-view-ViewGroup", "org-xmlpull-v1-XmlPullParser"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewGroup", feature = "org-xmlpull-v1-XmlPullParser")))]
        pub fn inflate_XmlPullParser_ViewGroup_boolean<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::xmlpull::v1::XmlPullParser>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>, arg2: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "inflate", .descriptor == "(Lorg/xmlpull/v1/XmlPullParser;Landroid/view/ViewGroup;Z)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "inflate\0", "(Lorg/xmlpull/v1/XmlPullParser;Landroid/view/ViewGroup;Z)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createView](https://developer.android.com/reference/android/view/LayoutInflater.html#createView(java.lang.String,%20java.lang.String,%20android.util.AttributeSet))
        ///
        /// Required features: "android-util-AttributeSet", "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-view-View", feature = "java-lang-String")))]
        pub fn createView_String_String_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC | FINAL, .name == "createView", .descriptor == "(Ljava/lang/String;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "createView\0", "(Ljava/lang/String;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createView](https://developer.android.com/reference/android/view/LayoutInflater.html#createView(android.content.Context,%20java.lang.String,%20java.lang.String,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet", "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet", feature = "android-view-View", feature = "java-lang-String")))]
        pub fn createView_Context_String_String_AttributeSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC | FINAL, .name == "createView", .descriptor == "(Landroid/content/Context;Ljava/lang/String;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "createView\0", "(Landroid/content/Context;Ljava/lang/String;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onCreateView](https://developer.android.com/reference/android/view/LayoutInflater.html#onCreateView(java.lang.String,%20android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-view-View", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-view-View", feature = "java-lang-String")))]
        // fn onCreateView<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/LayoutInflater", java.flags == PROTECTED, .name == "onCreateView", .descriptor == "(Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "onCreateView\0", "(Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onCreateView](https://developer.android.com/reference/android/view/LayoutInflater.html#onCreateView(android.view.View,%20java.lang.String,%20android.util.AttributeSet))
        // ///
        // /// Required features: "android-util-AttributeSet", "android-view-View", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "android-util-AttributeSet", feature = "android-view-View", feature = "java-lang-String")))]
        // fn onCreateView<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/view/LayoutInflater", java.flags == PROTECTED, .name == "onCreateView", .descriptor == "(Landroid/view/View;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "onCreateView\0", "(Landroid/view/View;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [onCreateView](https://developer.android.com/reference/android/view/LayoutInflater.html#onCreateView(android.content.Context,%20android.view.View,%20java.lang.String,%20android.util.AttributeSet))
        ///
        /// Required features: "android-content-Context", "android-util-AttributeSet", "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-util-AttributeSet", feature = "android-view-View", feature = "java-lang-String")))]
        pub fn onCreateView<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::util::AttributeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/LayoutInflater", java.flags == PUBLIC, .name == "onCreateView", .descriptor == "(Landroid/content/Context;Landroid/view/View;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/LayoutInflater\0", "onCreateView\0", "(Landroid/content/Context;Landroid/view/View;Ljava/lang/String;Landroid/util/AttributeSet;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
