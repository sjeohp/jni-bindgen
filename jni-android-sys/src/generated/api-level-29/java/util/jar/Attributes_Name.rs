// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
__jni_bindgen! {
    /// public class [Attributes.Name](https://developer.android.com/reference/java/util/jar/Attributes.Name.html)
    ///
    /// Required feature: java-util-jar-Attributes_Name
    public class Attributes_Name ("java/util/jar/Attributes$Name") extends crate::java::lang::Object {

        /// [Name](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#Name(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/Attributes$Name", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/Attributes$Name\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/Attributes$Name", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/Attributes$Name\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/Attributes$Name", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/Attributes$Name\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/Attributes$Name", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/Attributes$Name\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CLASS_PATH](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#CLASS_PATH)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn CLASS_PATH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "CLASS_PATH\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CONTENT_TYPE](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#CONTENT_TYPE)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn CONTENT_TYPE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "CONTENT_TYPE\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXTENSION_INSTALLATION](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#EXTENSION_INSTALLATION)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        #[deprecated] pub fn EXTENSION_INSTALLATION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "EXTENSION_INSTALLATION\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXTENSION_LIST](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#EXTENSION_LIST)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn EXTENSION_LIST<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "EXTENSION_LIST\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXTENSION_NAME](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#EXTENSION_NAME)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn EXTENSION_NAME<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "EXTENSION_NAME\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [IMPLEMENTATION_TITLE](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#IMPLEMENTATION_TITLE)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn IMPLEMENTATION_TITLE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "IMPLEMENTATION_TITLE\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [IMPLEMENTATION_URL](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#IMPLEMENTATION_URL)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        #[deprecated] pub fn IMPLEMENTATION_URL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "IMPLEMENTATION_URL\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [IMPLEMENTATION_VENDOR](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#IMPLEMENTATION_VENDOR)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn IMPLEMENTATION_VENDOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "IMPLEMENTATION_VENDOR\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [IMPLEMENTATION_VENDOR_ID](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#IMPLEMENTATION_VENDOR_ID)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        #[deprecated] pub fn IMPLEMENTATION_VENDOR_ID<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "IMPLEMENTATION_VENDOR_ID\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [IMPLEMENTATION_VERSION](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#IMPLEMENTATION_VERSION)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn IMPLEMENTATION_VERSION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "IMPLEMENTATION_VERSION\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MAIN_CLASS](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#MAIN_CLASS)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn MAIN_CLASS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "MAIN_CLASS\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MANIFEST_VERSION](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#MANIFEST_VERSION)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn MANIFEST_VERSION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "MANIFEST_VERSION\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SEALED](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#SEALED)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn SEALED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "SEALED\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SIGNATURE_VERSION](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#SIGNATURE_VERSION)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn SIGNATURE_VERSION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "SIGNATURE_VERSION\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SPECIFICATION_TITLE](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#SPECIFICATION_TITLE)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn SPECIFICATION_TITLE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "SPECIFICATION_TITLE\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SPECIFICATION_VENDOR](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#SPECIFICATION_VENDOR)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn SPECIFICATION_VENDOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "SPECIFICATION_VENDOR\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SPECIFICATION_VERSION](https://developer.android.com/reference/java/util/jar/Attributes.Name.html#SPECIFICATION_VERSION)
        ///
        /// Required feature: java-util-jar-Attributes_Name
        #[cfg(any(feature = "all", feature = "java-util-jar-Attributes_Name"))]
        pub fn SPECIFICATION_VERSION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes_Name>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/util/jar/Attributes$Name\0", "SPECIFICATION_VERSION\0", "Ljava/util/jar/Attributes$Name;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
