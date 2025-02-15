// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-ExpandableListActivity"))]
__jni_bindgen! {
    /// public class [ExpandableListActivity](https://developer.android.com/reference/android/app/ExpandableListActivity.html)
    ///
    /// Required feature: android-app-ExpandableListActivity
    public class ExpandableListActivity ("android/app/ExpandableListActivity") extends crate::android::app::Activity, implements crate::android::view::View_OnCreateContextMenuListener, crate::android::widget::ExpandableListView_OnChildClickListener, crate::android::widget::ExpandableListView_OnGroupCollapseListener, crate::android::widget::ExpandableListView_OnGroupExpandListener {

        /// [ExpandableListActivity](https://developer.android.com/reference/android/app/ExpandableListActivity.html#ExpandableListActivity())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::ExpandableListActivity>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreateContextMenu](https://developer.android.com/reference/android/app/ExpandableListActivity.html#onCreateContextMenu(android.view.ContextMenu,%20android.view.View,%20android.view.ContextMenu.ContextMenuInfo))
        ///
        /// Required features: "android-view-ContextMenu", "android-view-ContextMenu_ContextMenuInfo", "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-ContextMenu", feature = "android-view-ContextMenu_ContextMenuInfo", feature = "android-view-View")))]
        pub fn onCreateContextMenu<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ContextMenu>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ContextMenu_ContextMenuInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "onCreateContextMenu", .descriptor == "(Landroid/view/ContextMenu;Landroid/view/View;Landroid/view/ContextMenu$ContextMenuInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "onCreateContextMenu\0", "(Landroid/view/ContextMenu;Landroid/view/View;Landroid/view/ContextMenu$ContextMenuInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onChildClick](https://developer.android.com/reference/android/app/ExpandableListActivity.html#onChildClick(android.widget.ExpandableListView,%20android.view.View,%20int,%20int,%20long))
        ///
        /// Required features: "android-view-View", "android-widget-ExpandableListView"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-widget-ExpandableListView")))]
        pub fn onChildClick<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::ExpandableListView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: i32, arg3: i32, arg4: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "onChildClick", .descriptor == "(Landroid/widget/ExpandableListView;Landroid/view/View;IIJ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "onChildClick\0", "(Landroid/widget/ExpandableListView;Landroid/view/View;IIJ)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGroupCollapse](https://developer.android.com/reference/android/app/ExpandableListActivity.html#onGroupCollapse(int))
        pub fn onGroupCollapse<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "onGroupCollapse", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "onGroupCollapse\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGroupExpand](https://developer.android.com/reference/android/app/ExpandableListActivity.html#onGroupExpand(int))
        pub fn onGroupExpand<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "onGroupExpand", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "onGroupExpand\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onRestoreInstanceState](https://developer.android.com/reference/android/app/ExpandableListActivity.html#onRestoreInstanceState(android.os.Bundle))
        // ///
        // /// Required features: "android-os-Bundle"
        // #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        // fn onRestoreInstanceState<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/ExpandableListActivity", java.flags == PROTECTED, .name == "onRestoreInstanceState", .descriptor == "(Landroid/os/Bundle;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "onRestoreInstanceState\0", "(Landroid/os/Bundle;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [onContentChanged](https://developer.android.com/reference/android/app/ExpandableListActivity.html#onContentChanged())
        pub fn onContentChanged<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "onContentChanged", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "onContentChanged\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setListAdapter](https://developer.android.com/reference/android/app/ExpandableListActivity.html#setListAdapter(android.widget.ExpandableListAdapter))
        ///
        /// Required features: "android-widget-ExpandableListAdapter"
        #[cfg(any(feature = "all", all(feature = "android-widget-ExpandableListAdapter")))]
        pub fn setListAdapter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::ExpandableListAdapter>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "setListAdapter", .descriptor == "(Landroid/widget/ExpandableListAdapter;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "setListAdapter\0", "(Landroid/widget/ExpandableListAdapter;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExpandableListView](https://developer.android.com/reference/android/app/ExpandableListActivity.html#getExpandableListView())
        ///
        /// Required features: "android-widget-ExpandableListView"
        #[cfg(any(feature = "all", all(feature = "android-widget-ExpandableListView")))]
        pub fn getExpandableListView<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::ExpandableListView>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "getExpandableListView", .descriptor == "()Landroid/widget/ExpandableListView;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "getExpandableListView\0", "()Landroid/widget/ExpandableListView;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExpandableListAdapter](https://developer.android.com/reference/android/app/ExpandableListActivity.html#getExpandableListAdapter())
        ///
        /// Required features: "android-widget-ExpandableListAdapter"
        #[cfg(any(feature = "all", all(feature = "android-widget-ExpandableListAdapter")))]
        pub fn getExpandableListAdapter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::widget::ExpandableListAdapter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "getExpandableListAdapter", .descriptor == "()Landroid/widget/ExpandableListAdapter;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "getExpandableListAdapter\0", "()Landroid/widget/ExpandableListAdapter;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSelectedId](https://developer.android.com/reference/android/app/ExpandableListActivity.html#getSelectedId())
        pub fn getSelectedId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "getSelectedId", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "getSelectedId\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSelectedPosition](https://developer.android.com/reference/android/app/ExpandableListActivity.html#getSelectedPosition())
        pub fn getSelectedPosition<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "getSelectedPosition", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "getSelectedPosition\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSelectedChild](https://developer.android.com/reference/android/app/ExpandableListActivity.html#setSelectedChild(int,%20int,%20boolean))
        pub fn setSelectedChild<'env>(&'env self, arg0: i32, arg1: i32, arg2: bool) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "setSelectedChild", .descriptor == "(IIZ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "setSelectedChild\0", "(IIZ)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSelectedGroup](https://developer.android.com/reference/android/app/ExpandableListActivity.html#setSelectedGroup(int))
        pub fn setSelectedGroup<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/ExpandableListActivity", java.flags == PUBLIC, .name == "setSelectedGroup", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/ExpandableListActivity\0", "setSelectedGroup\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
