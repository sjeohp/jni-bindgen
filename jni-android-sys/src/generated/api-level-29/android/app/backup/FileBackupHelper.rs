// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-backup-FileBackupHelper"))]
__jni_bindgen! {
    /// public class [FileBackupHelper](https://developer.android.com/reference/android/app/backup/FileBackupHelper.html)
    ///
    /// Required feature: android-app-backup-FileBackupHelper
    public class FileBackupHelper ("android/app/backup/FileBackupHelper") extends crate::java::lang::Object, implements crate::android::app::backup::BackupHelper {

        /// [FileBackupHelper](https://developer.android.com/reference/android/app/backup/FileBackupHelper.html#FileBackupHelper(android.content.Context,%20java.lang.String...))
        ///
        /// Required features: "android-content-Context", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::backup::FileBackupHelper>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/FileBackupHelper", java.flags == PUBLIC | VARARGS, .name == "<init>", .descriptor == "(Landroid/content/Context;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/FileBackupHelper\0", "<init>\0", "(Landroid/content/Context;[Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [performBackup](https://developer.android.com/reference/android/app/backup/FileBackupHelper.html#performBackup(android.os.ParcelFileDescriptor,%20android.app.backup.BackupDataOutput,%20android.os.ParcelFileDescriptor))
        ///
        /// Required features: "android-app-backup-BackupDataOutput", "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-app-backup-BackupDataOutput", feature = "android-os-ParcelFileDescriptor")))]
        pub fn performBackup<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::ParcelFileDescriptor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::backup::BackupDataOutput>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::ParcelFileDescriptor>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/FileBackupHelper", java.flags == PUBLIC, .name == "performBackup", .descriptor == "(Landroid/os/ParcelFileDescriptor;Landroid/app/backup/BackupDataOutput;Landroid/os/ParcelFileDescriptor;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/FileBackupHelper\0", "performBackup\0", "(Landroid/os/ParcelFileDescriptor;Landroid/app/backup/BackupDataOutput;Landroid/os/ParcelFileDescriptor;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [restoreEntity](https://developer.android.com/reference/android/app/backup/FileBackupHelper.html#restoreEntity(android.app.backup.BackupDataInputStream))
        ///
        /// Required features: "android-app-backup-BackupDataInputStream"
        #[cfg(any(feature = "all", all(feature = "android-app-backup-BackupDataInputStream")))]
        pub fn restoreEntity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::backup::BackupDataInputStream>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/FileBackupHelper", java.flags == PUBLIC, .name == "restoreEntity", .descriptor == "(Landroid/app/backup/BackupDataInputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/FileBackupHelper\0", "restoreEntity\0", "(Landroid/app/backup/BackupDataInputStream;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeNewStateDescription](https://developer.android.com/reference/android/app/backup/FileBackupHelper.html#writeNewStateDescription(android.os.ParcelFileDescriptor))
        ///
        /// Required features: "android-os-ParcelFileDescriptor"
        #[cfg(any(feature = "all", all(feature = "android-os-ParcelFileDescriptor")))]
        pub fn writeNewStateDescription<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::ParcelFileDescriptor>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/backup/FileBackupHelper", java.flags == PUBLIC, .name == "writeNewStateDescription", .descriptor == "(Landroid/os/ParcelFileDescriptor;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/backup/FileBackupHelper\0", "writeNewStateDescription\0", "(Landroid/os/ParcelFileDescriptor;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
