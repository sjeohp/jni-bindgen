// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-jar-JarEntry"))]
__jni_bindgen! {
    /// public class [JarEntry](https://developer.android.com/reference/java/util/jar/JarEntry.html)
    ///
    /// Required feature: java-util-jar-JarEntry
    public class JarEntry ("java/util/jar/JarEntry") extends crate::java::util::zip::ZipEntry {

        /// [JarEntry](https://developer.android.com/reference/java/util/jar/JarEntry.html#JarEntry(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::jar::JarEntry>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/JarEntry", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/JarEntry\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [JarEntry](https://developer.android.com/reference/java/util/jar/JarEntry.html#JarEntry(java.util.zip.ZipEntry))
        ///
        /// Required features: "java-util-zip-ZipEntry"
        #[cfg(any(feature = "all", all(feature = "java-util-zip-ZipEntry")))]
        pub fn new_ZipEntry<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::zip::ZipEntry>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::jar::JarEntry>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/JarEntry", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/zip/ZipEntry;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/JarEntry\0", "<init>\0", "(Ljava/util/zip/ZipEntry;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [JarEntry](https://developer.android.com/reference/java/util/jar/JarEntry.html#JarEntry(java.util.jar.JarEntry))
        ///
        /// Required features: "java-util-jar-JarEntry"
        #[cfg(any(feature = "all", all(feature = "java-util-jar-JarEntry")))]
        pub fn new_JarEntry<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::jar::JarEntry>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::jar::JarEntry>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/JarEntry", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/jar/JarEntry;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/JarEntry\0", "<init>\0", "(Ljava/util/jar/JarEntry;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAttributes](https://developer.android.com/reference/java/util/jar/JarEntry.html#getAttributes())
        ///
        /// Required features: "java-util-jar-Attributes"
        #[cfg(any(feature = "all", all(feature = "java-util-jar-Attributes")))]
        pub fn getAttributes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::jar::Attributes>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/JarEntry", java.flags == PUBLIC, .name == "getAttributes", .descriptor == "()Ljava/util/jar/Attributes;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/JarEntry\0", "getAttributes\0", "()Ljava/util/jar/Attributes;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertificates](https://developer.android.com/reference/java/util/jar/JarEntry.html#getCertificates())
        ///
        /// Required features: "java-security-cert-Certificate"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-Certificate")))]
        pub fn getCertificates<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::Certificate, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/JarEntry", java.flags == PUBLIC, .name == "getCertificates", .descriptor == "()[Ljava/security/cert/Certificate;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/JarEntry\0", "getCertificates\0", "()[Ljava/security/cert/Certificate;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCodeSigners](https://developer.android.com/reference/java/util/jar/JarEntry.html#getCodeSigners())
        ///
        /// Required features: "java-security-CodeSigner"
        #[cfg(any(feature = "all", all(feature = "java-security-CodeSigner")))]
        pub fn getCodeSigners<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::CodeSigner, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/jar/JarEntry", java.flags == PUBLIC, .name == "getCodeSigners", .descriptor == "()[Ljava/security/CodeSigner;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/jar/JarEntry\0", "getCodeSigners\0", "()[Ljava/security/CodeSigner;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CENATT](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENATT)
        pub const CENATT : i32 = 36;

        /// public static final [CENATX](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENATX)
        pub const CENATX : i32 = 38;

        /// public static final [CENCOM](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENCOM)
        pub const CENCOM : i32 = 32;

        /// public static final [CENCRC](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENCRC)
        pub const CENCRC : i32 = 16;

        /// public static final [CENDSK](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENDSK)
        pub const CENDSK : i32 = 34;

        /// public static final [CENEXT](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENEXT)
        pub const CENEXT : i32 = 30;

        /// public static final [CENFLG](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENFLG)
        pub const CENFLG : i32 = 8;

        /// public static final [CENHDR](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENHDR)
        pub const CENHDR : i32 = 46;

        /// public static final [CENHOW](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENHOW)
        pub const CENHOW : i32 = 10;

        /// public static final [CENLEN](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENLEN)
        pub const CENLEN : i32 = 24;

        /// public static final [CENNAM](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENNAM)
        pub const CENNAM : i32 = 28;

        /// public static final [CENOFF](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENOFF)
        pub const CENOFF : i32 = 42;

        /// public static final [CENSIG](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENSIG)
        pub const CENSIG : i64 = 33639248i64;

        /// public static final [CENSIZ](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENSIZ)
        pub const CENSIZ : i32 = 20;

        /// public static final [CENTIM](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENTIM)
        pub const CENTIM : i32 = 12;

        /// public static final [CENVEM](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENVEM)
        pub const CENVEM : i32 = 4;

        /// public static final [CENVER](https://developer.android.com/reference/java/util/jar/JarEntry.html#CENVER)
        pub const CENVER : i32 = 6;

        /// public static final [ENDCOM](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDCOM)
        pub const ENDCOM : i32 = 20;

        /// public static final [ENDHDR](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDHDR)
        pub const ENDHDR : i32 = 22;

        /// public static final [ENDOFF](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDOFF)
        pub const ENDOFF : i32 = 16;

        /// public static final [ENDSIG](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDSIG)
        pub const ENDSIG : i64 = 101010256i64;

        /// public static final [ENDSIZ](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDSIZ)
        pub const ENDSIZ : i32 = 12;

        /// public static final [ENDSUB](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDSUB)
        pub const ENDSUB : i32 = 8;

        /// public static final [ENDTOT](https://developer.android.com/reference/java/util/jar/JarEntry.html#ENDTOT)
        pub const ENDTOT : i32 = 10;

        /// public static final [EXTCRC](https://developer.android.com/reference/java/util/jar/JarEntry.html#EXTCRC)
        pub const EXTCRC : i32 = 4;

        /// public static final [EXTHDR](https://developer.android.com/reference/java/util/jar/JarEntry.html#EXTHDR)
        pub const EXTHDR : i32 = 16;

        /// public static final [EXTLEN](https://developer.android.com/reference/java/util/jar/JarEntry.html#EXTLEN)
        pub const EXTLEN : i32 = 12;

        /// public static final [EXTSIG](https://developer.android.com/reference/java/util/jar/JarEntry.html#EXTSIG)
        pub const EXTSIG : i64 = 134695760i64;

        /// public static final [EXTSIZ](https://developer.android.com/reference/java/util/jar/JarEntry.html#EXTSIZ)
        pub const EXTSIZ : i32 = 8;

        /// public static final [LOCCRC](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCCRC)
        pub const LOCCRC : i32 = 14;

        /// public static final [LOCEXT](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCEXT)
        pub const LOCEXT : i32 = 28;

        /// public static final [LOCFLG](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCFLG)
        pub const LOCFLG : i32 = 6;

        /// public static final [LOCHDR](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCHDR)
        pub const LOCHDR : i32 = 30;

        /// public static final [LOCHOW](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCHOW)
        pub const LOCHOW : i32 = 8;

        /// public static final [LOCLEN](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCLEN)
        pub const LOCLEN : i32 = 22;

        /// public static final [LOCNAM](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCNAM)
        pub const LOCNAM : i32 = 26;

        /// public static final [LOCSIG](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCSIG)
        pub const LOCSIG : i64 = 67324752i64;

        /// public static final [LOCSIZ](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCSIZ)
        pub const LOCSIZ : i32 = 18;

        /// public static final [LOCTIM](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCTIM)
        pub const LOCTIM : i32 = 10;

        /// public static final [LOCVER](https://developer.android.com/reference/java/util/jar/JarEntry.html#LOCVER)
        pub const LOCVER : i32 = 4;
    }
}
