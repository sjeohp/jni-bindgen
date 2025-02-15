// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-Patterns"))]
__jni_bindgen! {
    /// public class [Patterns](https://developer.android.com/reference/android/util/Patterns.html)
    ///
    /// Required feature: android-util-Patterns
    public class Patterns ("android/util/Patterns") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Patterns](https://developer.android.com/reference/android/util/Patterns.html#Patterns())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::Patterns>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/util/Patterns", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/Patterns\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [concatGroups](https://developer.android.com/reference/android/util/Patterns.html#concatGroups(java.util.regex.Matcher))
        ///
        /// Required features: "java-lang-String", "java-util-regex-Matcher"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-regex-Matcher")))]
        pub fn concatGroups<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::regex::Matcher>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Patterns", java.flags == PUBLIC | STATIC | FINAL, .name == "concatGroups", .descriptor == "(Ljava/util/regex/Matcher;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/Patterns\0", "concatGroups\0", "(Ljava/util/regex/Matcher;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [digitsAndPlusOnly](https://developer.android.com/reference/android/util/Patterns.html#digitsAndPlusOnly(java.util.regex.Matcher))
        ///
        /// Required features: "java-lang-String", "java-util-regex-Matcher"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-regex-Matcher")))]
        pub fn digitsAndPlusOnly<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::regex::Matcher>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/Patterns", java.flags == PUBLIC | STATIC | FINAL, .name == "digitsAndPlusOnly", .descriptor == "(Ljava/util/regex/Matcher;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/Patterns\0", "digitsAndPlusOnly\0", "(Ljava/util/regex/Matcher;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [DOMAIN_NAME](https://developer.android.com/reference/android/util/Patterns.html#DOMAIN_NAME)
        ///
        /// Required feature: java-util-regex-Pattern
        #[cfg(any(feature = "all", feature = "java-util-regex-Pattern"))]
        pub fn DOMAIN_NAME<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::regex::Pattern>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/Patterns\0", "DOMAIN_NAME\0", "Ljava/util/regex/Pattern;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EMAIL_ADDRESS](https://developer.android.com/reference/android/util/Patterns.html#EMAIL_ADDRESS)
        ///
        /// Required feature: java-util-regex-Pattern
        #[cfg(any(feature = "all", feature = "java-util-regex-Pattern"))]
        pub fn EMAIL_ADDRESS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::regex::Pattern>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/Patterns\0", "EMAIL_ADDRESS\0", "Ljava/util/regex/Pattern;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [GOOD_IRI_CHAR](https://developer.android.com/reference/android/util/Patterns.html#GOOD_IRI_CHAR)
        #[deprecated] pub const GOOD_IRI_CHAR : &'static str = "a-zA-Z0-9\u{a0}-\u{d7ff}豈-\u{fdcf}ﷰ-\u{ffef}";

        /// **get** public static final [IP_ADDRESS](https://developer.android.com/reference/android/util/Patterns.html#IP_ADDRESS)
        ///
        /// Required feature: java-util-regex-Pattern
        #[cfg(any(feature = "all", feature = "java-util-regex-Pattern"))]
        pub fn IP_ADDRESS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::regex::Pattern>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/Patterns\0", "IP_ADDRESS\0", "Ljava/util/regex/Pattern;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PHONE](https://developer.android.com/reference/android/util/Patterns.html#PHONE)
        ///
        /// Required feature: java-util-regex-Pattern
        #[cfg(any(feature = "all", feature = "java-util-regex-Pattern"))]
        pub fn PHONE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::regex::Pattern>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/Patterns\0", "PHONE\0", "Ljava/util/regex/Pattern;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [TOP_LEVEL_DOMAIN](https://developer.android.com/reference/android/util/Patterns.html#TOP_LEVEL_DOMAIN)
        ///
        /// Required feature: java-util-regex-Pattern
        #[cfg(any(feature = "all", feature = "java-util-regex-Pattern"))]
        #[deprecated] pub fn TOP_LEVEL_DOMAIN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::regex::Pattern>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/Patterns\0", "TOP_LEVEL_DOMAIN\0", "Ljava/util/regex/Pattern;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TOP_LEVEL_DOMAIN_STR](https://developer.android.com/reference/android/util/Patterns.html#TOP_LEVEL_DOMAIN_STR)
        #[deprecated] pub const TOP_LEVEL_DOMAIN_STR : &'static str = "((aero|arpa|asia|a[cdefgilmnoqrstuwxz])|(biz|b[abdefghijmnorstvwyz])|(cat|com|coop|c[acdfghiklmnoruvxyz])|d[ejkmoz]|(edu|e[cegrstu])|f[ijkmor]|(gov|g[abdefghilmnpqrstuwy])|h[kmnrtu]|(info|int|i[delmnoqrst])|(jobs|j[emop])|k[eghimnprwyz]|l[abcikrstuvy]|(mil|mobi|museum|m[acdeghklmnopqrstuvwxyz])|(name|net|n[acefgilopruz])|(org|om)|(pro|p[aefghklmnrstwy])|qa|r[eosuw]|s[abcdeghijklmnortuvyz]|(tel|travel|t[cdfghjklmnoprtvwz])|u[agksyz]|v[aceginu]|w[fs]|(δοκιμή|испытание|рф|срб|טעסט|آزمایشی|إختبار|الاردن|الجزائر|السعودية|المغرب|امارات|بھارت|تونس|سورية|فلسطين|قطر|مصر|परीक\u{94d}षा|भारत|ভ\u{9be}রত|ਭਾਰਤ|ભારત|இந\u{bcd}திய\u{bbe}|இலங\u{bcd}கை|சிங\u{bcd}கப\u{bcd}பூர\u{bcd}|பரிட\u{bcd}சை|భ\u{c3e}రత\u{c4d}|ලංක\u{dcf}|ไทย|テスト|中国|中國|台湾|台灣|新加坡|测试|測試|香港|테스트|한국|xn\\-\\-0zwm56d|xn\\-\\-11b5bs3a9aj6g|xn\\-\\-3e0b707e|xn\\-\\-45brj9c|xn\\-\\-80akhbyknj4f|xn\\-\\-90a3ac|xn\\-\\-9t4b11yi5a|xn\\-\\-clchc0ea0b2g2a9gcd|xn\\-\\-deba0ad|xn\\-\\-fiqs8s|xn\\-\\-fiqz9s|xn\\-\\-fpcrj9c3d|xn\\-\\-fzc2c9e2c|xn\\-\\-g6w251d|xn\\-\\-gecrj9c|xn\\-\\-h2brj9c|xn\\-\\-hgbk6aj7f53bba|xn\\-\\-hlcj6aya9esc7a|xn\\-\\-j6w193g|xn\\-\\-jxalpdlp|xn\\-\\-kgbechtv|xn\\-\\-kprw13d|xn\\-\\-kpry57d|xn\\-\\-lgbbat1ad8j|xn\\-\\-mgbaam7a8h|xn\\-\\-mgbayh7gpa|xn\\-\\-mgbbh1a71e|xn\\-\\-mgbc0a9azcg|xn\\-\\-mgberp4a5d4ar|xn\\-\\-o3cw4h|xn\\-\\-ogbpf8fl|xn\\-\\-p1ai|xn\\-\\-pgbs0dh|xn\\-\\-s9brj9c|xn\\-\\-wgbh1c|xn\\-\\-wgbl6a|xn\\-\\-xkc2al3hye2a|xn\\-\\-xkc2dl3a5ee0h|xn\\-\\-yfro4i67o|xn\\-\\-ygbi2ammx|xn\\-\\-zckzah|xxx)|y[et]|z[amw])";

        /// public static final [TOP_LEVEL_DOMAIN_STR_FOR_WEB_URL](https://developer.android.com/reference/android/util/Patterns.html#TOP_LEVEL_DOMAIN_STR_FOR_WEB_URL)
        #[deprecated] pub const TOP_LEVEL_DOMAIN_STR_FOR_WEB_URL : &'static str = "(?:(?:aero|arpa|asia|a[cdefgilmnoqrstuwxz])|(?:biz|b[abdefghijmnorstvwyz])|(?:cat|com|coop|c[acdfghiklmnoruvxyz])|d[ejkmoz]|(?:edu|e[cegrstu])|f[ijkmor]|(?:gov|g[abdefghilmnpqrstuwy])|h[kmnrtu]|(?:info|int|i[delmnoqrst])|(?:jobs|j[emop])|k[eghimnprwyz]|l[abcikrstuvy]|(?:mil|mobi|museum|m[acdeghklmnopqrstuvwxyz])|(?:name|net|n[acefgilopruz])|(?:org|om)|(?:pro|p[aefghklmnrstwy])|qa|r[eosuw]|s[abcdeghijklmnortuvyz]|(?:tel|travel|t[cdfghjklmnoprtvwz])|u[agksyz]|v[aceginu]|w[fs]|(?:δοκιμή|испытание|рф|срб|טעסט|آزمایشی|إختبار|الاردن|الجزائر|السعودية|المغرب|امارات|بھارت|تونس|سورية|فلسطين|قطر|مصر|परीक\u{94d}षा|भारत|ভ\u{9be}রত|ਭਾਰਤ|ભારત|இந\u{bcd}திய\u{bbe}|இலங\u{bcd}கை|சிங\u{bcd}கப\u{bcd}பூர\u{bcd}|பரிட\u{bcd}சை|భ\u{c3e}రత\u{c4d}|ලංක\u{dcf}|ไทย|テスト|中国|中國|台湾|台灣|新加坡|测试|測試|香港|테스트|한국|xn\\-\\-0zwm56d|xn\\-\\-11b5bs3a9aj6g|xn\\-\\-3e0b707e|xn\\-\\-45brj9c|xn\\-\\-80akhbyknj4f|xn\\-\\-90a3ac|xn\\-\\-9t4b11yi5a|xn\\-\\-clchc0ea0b2g2a9gcd|xn\\-\\-deba0ad|xn\\-\\-fiqs8s|xn\\-\\-fiqz9s|xn\\-\\-fpcrj9c3d|xn\\-\\-fzc2c9e2c|xn\\-\\-g6w251d|xn\\-\\-gecrj9c|xn\\-\\-h2brj9c|xn\\-\\-hgbk6aj7f53bba|xn\\-\\-hlcj6aya9esc7a|xn\\-\\-j6w193g|xn\\-\\-jxalpdlp|xn\\-\\-kgbechtv|xn\\-\\-kprw13d|xn\\-\\-kpry57d|xn\\-\\-lgbbat1ad8j|xn\\-\\-mgbaam7a8h|xn\\-\\-mgbayh7gpa|xn\\-\\-mgbbh1a71e|xn\\-\\-mgbc0a9azcg|xn\\-\\-mgberp4a5d4ar|xn\\-\\-o3cw4h|xn\\-\\-ogbpf8fl|xn\\-\\-p1ai|xn\\-\\-pgbs0dh|xn\\-\\-s9brj9c|xn\\-\\-wgbh1c|xn\\-\\-wgbl6a|xn\\-\\-xkc2al3hye2a|xn\\-\\-xkc2dl3a5ee0h|xn\\-\\-yfro4i67o|xn\\-\\-ygbi2ammx|xn\\-\\-zckzah|xxx)|y[et]|z[amw]))";

        /// **get** public static final [WEB_URL](https://developer.android.com/reference/android/util/Patterns.html#WEB_URL)
        ///
        /// Required feature: java-util-regex-Pattern
        #[cfg(any(feature = "all", feature = "java-util-regex-Pattern"))]
        pub fn WEB_URL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::regex::Pattern>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/Patterns\0", "WEB_URL\0", "Ljava/util/regex/Pattern;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
