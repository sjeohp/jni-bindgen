// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-ViewDebug"))]
__jni_bindgen! {
    /// public class [ViewDebug](https://developer.android.com/reference/android/view/ViewDebug.html)
    ///
    /// Required feature: android-view-ViewDebug
    public class ViewDebug ("android/view/ViewDebug") extends crate::java::lang::Object {

        /// [ViewDebug](https://developer.android.com/reference/android/view/ViewDebug.html#ViewDebug())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::ViewDebug>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/ViewDebug\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [trace](https://developer.android.com/reference/android/view/ViewDebug.html#trace(android.view.View,%20android.view.ViewDebug.RecyclerTraceType,%20int...))
        ///
        /// Required features: "android-view-View", "android-view-ViewDebug_RecyclerTraceType"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewDebug_RecyclerTraceType")))]
        #[deprecated] pub fn trace_View_RecyclerTraceType_int_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewDebug_RecyclerTraceType>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC | VARARGS, .name == "trace", .descriptor == "(Landroid/view/View;Landroid/view/ViewDebug$RecyclerTraceType;[I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "trace\0", "(Landroid/view/View;Landroid/view/ViewDebug$RecyclerTraceType;[I)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startRecyclerTracing](https://developer.android.com/reference/android/view/ViewDebug.html#startRecyclerTracing(java.lang.String,%20android.view.View))
        ///
        /// Required features: "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "java-lang-String")))]
        #[deprecated] pub fn startRecyclerTracing<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC, .name == "startRecyclerTracing", .descriptor == "(Ljava/lang/String;Landroid/view/View;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "startRecyclerTracing\0", "(Ljava/lang/String;Landroid/view/View;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stopRecyclerTracing](https://developer.android.com/reference/android/view/ViewDebug.html#stopRecyclerTracing())
        #[deprecated] pub fn stopRecyclerTracing<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC, .name == "stopRecyclerTracing", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "stopRecyclerTracing\0", "()V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [trace](https://developer.android.com/reference/android/view/ViewDebug.html#trace(android.view.View,%20android.view.ViewDebug.HierarchyTraceType))
        ///
        /// Required features: "android-view-View", "android-view-ViewDebug_HierarchyTraceType"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewDebug_HierarchyTraceType")))]
        #[deprecated] pub fn trace_View_HierarchyTraceType<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewDebug_HierarchyTraceType>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC, .name == "trace", .descriptor == "(Landroid/view/View;Landroid/view/ViewDebug$HierarchyTraceType;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "trace\0", "(Landroid/view/View;Landroid/view/ViewDebug$HierarchyTraceType;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startHierarchyTracing](https://developer.android.com/reference/android/view/ViewDebug.html#startHierarchyTracing(java.lang.String,%20android.view.View))
        ///
        /// Required features: "android-view-View", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "java-lang-String")))]
        #[deprecated] pub fn startHierarchyTracing<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC, .name == "startHierarchyTracing", .descriptor == "(Ljava/lang/String;Landroid/view/View;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "startHierarchyTracing\0", "(Ljava/lang/String;Landroid/view/View;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stopHierarchyTracing](https://developer.android.com/reference/android/view/ViewDebug.html#stopHierarchyTracing())
        #[deprecated] pub fn stopHierarchyTracing<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC, .name == "stopHierarchyTracing", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "stopHierarchyTracing\0", "()V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dumpCapturedView](https://developer.android.com/reference/android/view/ViewDebug.html#dumpCapturedView(java.lang.String,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        pub fn dumpCapturedView<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/ViewDebug", java.flags == PUBLIC | STATIC, .name == "dumpCapturedView", .descriptor == "(Ljava/lang/String;Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/view/ViewDebug\0", "dumpCapturedView\0", "(Ljava/lang/String;Ljava/lang/Object;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [TRACE_HIERARCHY](https://developer.android.com/reference/android/view/ViewDebug.html#TRACE_HIERARCHY)
        #[deprecated] pub const TRACE_HIERARCHY : bool = false;

        /// public static final [TRACE_RECYCLER](https://developer.android.com/reference/android/view/ViewDebug.html#TRACE_RECYCLER)
        #[deprecated] pub const TRACE_RECYCLER : bool = false;
    }
}
