// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-AsyncTask"))]
__jni_bindgen! {
    /// public class [AsyncTask](https://developer.android.com/reference/android/os/AsyncTask.html)
    ///
    /// Required feature: android-os-AsyncTask
    public class AsyncTask ("android/os/AsyncTask") extends crate::java::lang::Object {

        /// [AsyncTask](https://developer.android.com/reference/android/os/AsyncTask.html#AsyncTask())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::AsyncTask>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStatus](https://developer.android.com/reference/android/os/AsyncTask.html#getStatus())
        ///
        /// Required features: "android-os-AsyncTask_Status"
        #[cfg(any(feature = "all", all(feature = "android-os-AsyncTask_Status")))]
        pub fn getStatus<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::AsyncTask_Status>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL, .name == "getStatus", .descriptor == "()Landroid/os/AsyncTask$Status;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "getStatus\0", "()Landroid/os/AsyncTask$Status;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [doInBackground](https://developer.android.com/reference/android/os/AsyncTask.html#doInBackground(java.lang.Object...))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn doInBackground<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED | VARARGS | ABSTRACT, .name == "doInBackground", .descriptor == "([Ljava/lang/Object;)Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "doInBackground\0", "([Ljava/lang/Object;)Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onPreExecute](https://developer.android.com/reference/android/os/AsyncTask.html#onPreExecute())
        // fn onPreExecute<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED, .name == "onPreExecute", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "onPreExecute\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onPostExecute](https://developer.android.com/reference/android/os/AsyncTask.html#onPostExecute(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn onPostExecute<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED, .name == "onPostExecute", .descriptor == "(Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "onPostExecute\0", "(Ljava/lang/Object;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onProgressUpdate](https://developer.android.com/reference/android/os/AsyncTask.html#onProgressUpdate(java.lang.Object...))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn onProgressUpdate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED | VARARGS, .name == "onProgressUpdate", .descriptor == "([Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "onProgressUpdate\0", "([Ljava/lang/Object;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onCancelled](https://developer.android.com/reference/android/os/AsyncTask.html#onCancelled(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn onCancelled<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED, .name == "onCancelled", .descriptor == "(Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "onCancelled\0", "(Ljava/lang/Object;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [onCancelled](https://developer.android.com/reference/android/os/AsyncTask.html#onCancelled())
        // fn onCancelled<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED, .name == "onCancelled", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "onCancelled\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isCancelled](https://developer.android.com/reference/android/os/AsyncTask.html#isCancelled())
        pub fn isCancelled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL, .name == "isCancelled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "isCancelled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cancel](https://developer.android.com/reference/android/os/AsyncTask.html#cancel(boolean))
        pub fn cancel<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL, .name == "cancel", .descriptor == "(Z)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "cancel\0", "(Z)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/android/os/AsyncTask.html#get())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL, .name == "get", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "get\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/android/os/AsyncTask.html#get(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-lang-Object", "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-concurrent-TimeUnit")))]
        pub fn get_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL, .name == "get", .descriptor == "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "get\0", "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [execute](https://developer.android.com/reference/android/os/AsyncTask.html#execute(java.lang.Object...))
        ///
        /// Required features: "android-os-AsyncTask", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-os-AsyncTask", feature = "java-lang-Object")))]
        pub fn execute_Object_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::AsyncTask>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL | VARARGS, .name == "execute", .descriptor == "([Ljava/lang/Object;)Landroid/os/AsyncTask;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "execute\0", "([Ljava/lang/Object;)Landroid/os/AsyncTask;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [executeOnExecutor](https://developer.android.com/reference/android/os/AsyncTask.html#executeOnExecutor(java.util.concurrent.Executor,%20java.lang.Object...))
        ///
        /// Required features: "android-os-AsyncTask", "java-lang-Object", "java-util-concurrent-Executor"
        #[cfg(any(feature = "all", all(feature = "android-os-AsyncTask", feature = "java-lang-Object", feature = "java-util-concurrent-Executor")))]
        pub fn executeOnExecutor<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::AsyncTask>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | FINAL | VARARGS, .name == "executeOnExecutor", .descriptor == "(Ljava/util/concurrent/Executor;[Ljava/lang/Object;)Landroid/os/AsyncTask;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "executeOnExecutor\0", "(Ljava/util/concurrent/Executor;[Ljava/lang/Object;)Landroid/os/AsyncTask;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [execute](https://developer.android.com/reference/android/os/AsyncTask.html#execute(java.lang.Runnable))
        ///
        /// Required features: "java-lang-Runnable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Runnable")))]
        pub fn execute_Runnable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/AsyncTask", java.flags == PUBLIC | STATIC, .name == "execute", .descriptor == "(Ljava/lang/Runnable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/os/AsyncTask\0", "execute\0", "(Ljava/lang/Runnable;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [publishProgress](https://developer.android.com/reference/android/os/AsyncTask.html#publishProgress(java.lang.Object...))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn publishProgress<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/AsyncTask", java.flags == PROTECTED | FINAL | VARARGS, .name == "publishProgress", .descriptor == "([Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/AsyncTask\0", "publishProgress\0", "([Ljava/lang/Object;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [SERIAL_EXECUTOR](https://developer.android.com/reference/android/os/AsyncTask.html#SERIAL_EXECUTOR)
        ///
        /// Required feature: java-util-concurrent-Executor
        #[cfg(any(feature = "all", feature = "java-util-concurrent-Executor"))]
        pub fn SERIAL_EXECUTOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::concurrent::Executor>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/os/AsyncTask\0", "SERIAL_EXECUTOR\0", "Ljava/util/concurrent/Executor;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [THREAD_POOL_EXECUTOR](https://developer.android.com/reference/android/os/AsyncTask.html#THREAD_POOL_EXECUTOR)
        ///
        /// Required feature: java-util-concurrent-Executor
        #[cfg(any(feature = "all", feature = "java-util-concurrent-Executor"))]
        pub fn THREAD_POOL_EXECUTOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::concurrent::Executor>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/os/AsyncTask\0", "THREAD_POOL_EXECUTOR\0", "Ljava/util/concurrent/Executor;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
