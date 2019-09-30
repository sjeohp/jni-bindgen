// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-zip-ZipOutputStream"))]
__jni_bindgen! {
    /// public class [ZipOutputStream](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html)
    ///
    /// Required feature: java-util-zip-ZipOutputStream
    public class ZipOutputStream ("java/util/zip/ZipOutputStream") extends crate::java::util::zip::DeflaterOutputStream {

        /// [ZipOutputStream](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ZipOutputStream(java.io.OutputStream))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn new_OutputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::ZipOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "<init>\0", "(Ljava/io/OutputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ZipOutputStream](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ZipOutputStream(java.io.OutputStream,%20java.nio.charset.Charset))
        ///
        /// Required features: "java-io-OutputStream", "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream", feature = "java-nio-charset-Charset")))]
        pub fn new_OutputStream_Charset<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::Charset>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::ZipOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "<init>\0", "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setComment](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#setComment(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setComment<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "setComment", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "setComment\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMethod](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#setMethod(int))
        pub fn setMethod<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "setMethod", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "setMethod\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLevel](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#setLevel(int))
        pub fn setLevel<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "setLevel", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "setLevel\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putNextEntry](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#putNextEntry(java.util.zip.ZipEntry))
        ///
        /// Required features: "java-util-zip-ZipEntry"
        #[cfg(any(feature = "all", all(feature = "java-util-zip-ZipEntry")))]
        pub fn putNextEntry<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::zip::ZipEntry>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "putNextEntry", .descriptor == "(Ljava/util/zip/ZipEntry;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "putNextEntry\0", "(Ljava/util/zip/ZipEntry;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [closeEntry](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#closeEntry())
        pub fn closeEntry<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "closeEntry", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "closeEntry\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#write(byte%5B%5D,%20int,%20int))
        pub fn write<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "write", .descriptor == "([BII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "write\0", "([BII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [finish](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#finish())
        pub fn finish<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "finish", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "finish\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/ZipOutputStream", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/ZipOutputStream\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CENATT](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENATT)
        pub const CENATT : i32 = 36;

        /// public static final [CENATX](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENATX)
        pub const CENATX : i32 = 38;

        /// public static final [CENCOM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENCOM)
        pub const CENCOM : i32 = 32;

        /// public static final [CENCRC](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENCRC)
        pub const CENCRC : i32 = 16;

        /// public static final [CENDSK](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENDSK)
        pub const CENDSK : i32 = 34;

        /// public static final [CENEXT](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENEXT)
        pub const CENEXT : i32 = 30;

        /// public static final [CENFLG](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENFLG)
        pub const CENFLG : i32 = 8;

        /// public static final [CENHDR](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENHDR)
        pub const CENHDR : i32 = 46;

        /// public static final [CENHOW](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENHOW)
        pub const CENHOW : i32 = 10;

        /// public static final [CENLEN](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENLEN)
        pub const CENLEN : i32 = 24;

        /// public static final [CENNAM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENNAM)
        pub const CENNAM : i32 = 28;

        /// public static final [CENOFF](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENOFF)
        pub const CENOFF : i32 = 42;

        /// public static final [CENSIG](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENSIG)
        pub const CENSIG : i64 = 33639248i64;

        /// public static final [CENSIZ](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENSIZ)
        pub const CENSIZ : i32 = 20;

        /// public static final [CENTIM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENTIM)
        pub const CENTIM : i32 = 12;

        /// public static final [CENVEM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENVEM)
        pub const CENVEM : i32 = 4;

        /// public static final [CENVER](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#CENVER)
        pub const CENVER : i32 = 6;

        /// public static final [DEFLATED](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#DEFLATED)
        pub const DEFLATED : i32 = 8;

        /// public static final [ENDCOM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDCOM)
        pub const ENDCOM : i32 = 20;

        /// public static final [ENDHDR](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDHDR)
        pub const ENDHDR : i32 = 22;

        /// public static final [ENDOFF](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDOFF)
        pub const ENDOFF : i32 = 16;

        /// public static final [ENDSIG](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDSIG)
        pub const ENDSIG : i64 = 101010256i64;

        /// public static final [ENDSIZ](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDSIZ)
        pub const ENDSIZ : i32 = 12;

        /// public static final [ENDSUB](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDSUB)
        pub const ENDSUB : i32 = 8;

        /// public static final [ENDTOT](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#ENDTOT)
        pub const ENDTOT : i32 = 10;

        /// public static final [EXTCRC](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#EXTCRC)
        pub const EXTCRC : i32 = 4;

        /// public static final [EXTHDR](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#EXTHDR)
        pub const EXTHDR : i32 = 16;

        /// public static final [EXTLEN](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#EXTLEN)
        pub const EXTLEN : i32 = 12;

        /// public static final [EXTSIG](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#EXTSIG)
        pub const EXTSIG : i64 = 134695760i64;

        /// public static final [EXTSIZ](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#EXTSIZ)
        pub const EXTSIZ : i32 = 8;

        /// public static final [LOCCRC](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCCRC)
        pub const LOCCRC : i32 = 14;

        /// public static final [LOCEXT](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCEXT)
        pub const LOCEXT : i32 = 28;

        /// public static final [LOCFLG](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCFLG)
        pub const LOCFLG : i32 = 6;

        /// public static final [LOCHDR](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCHDR)
        pub const LOCHDR : i32 = 30;

        /// public static final [LOCHOW](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCHOW)
        pub const LOCHOW : i32 = 8;

        /// public static final [LOCLEN](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCLEN)
        pub const LOCLEN : i32 = 22;

        /// public static final [LOCNAM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCNAM)
        pub const LOCNAM : i32 = 26;

        /// public static final [LOCSIG](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCSIG)
        pub const LOCSIG : i64 = 67324752i64;

        /// public static final [LOCSIZ](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCSIZ)
        pub const LOCSIZ : i32 = 18;

        /// public static final [LOCTIM](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCTIM)
        pub const LOCTIM : i32 = 10;

        /// public static final [LOCVER](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#LOCVER)
        pub const LOCVER : i32 = 4;

        /// public static final [STORED](https://developer.android.com/reference/java/util/zip/ZipOutputStream.html#STORED)
        pub const STORED : i32 = 0;
    }
}
