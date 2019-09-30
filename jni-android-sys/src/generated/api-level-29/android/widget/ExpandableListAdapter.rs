// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-ExpandableListAdapter"))]
__jni_bindgen! {
    /// public interface [ExpandableListAdapter](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html)
    ///
    /// Required feature: android-widget-ExpandableListAdapter
    public interface ExpandableListAdapter ("android/widget/ExpandableListAdapter") extends crate::java::lang::Object {

        /// [registerDataSetObserver](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#registerDataSetObserver(android.database.DataSetObserver))
        ///
        /// Required features: "android-database-DataSetObserver"
        #[cfg(any(feature = "all", all(feature = "android-database-DataSetObserver")))]
        pub fn registerDataSetObserver<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::DataSetObserver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "registerDataSetObserver", .descriptor == "(Landroid/database/DataSetObserver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "registerDataSetObserver\0", "(Landroid/database/DataSetObserver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterDataSetObserver](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#unregisterDataSetObserver(android.database.DataSetObserver))
        ///
        /// Required features: "android-database-DataSetObserver"
        #[cfg(any(feature = "all", all(feature = "android-database-DataSetObserver")))]
        pub fn unregisterDataSetObserver<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::database::DataSetObserver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "unregisterDataSetObserver", .descriptor == "(Landroid/database/DataSetObserver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "unregisterDataSetObserver\0", "(Landroid/database/DataSetObserver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGroupCount](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getGroupCount())
        pub fn getGroupCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getGroupCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getGroupCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChildrenCount](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getChildrenCount(int))
        pub fn getChildrenCount<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getChildrenCount", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getChildrenCount\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGroup](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getGroup(int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getGroup<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getGroup", .descriptor == "(I)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getGroup\0", "(I)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChild](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getChild(int,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getChild<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getChild", .descriptor == "(II)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getChild\0", "(II)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGroupId](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getGroupId(int))
        pub fn getGroupId<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getGroupId", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getGroupId\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChildId](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getChildId(int,%20int))
        pub fn getChildId<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getChildId", .descriptor == "(II)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getChildId\0", "(II)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasStableIds](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#hasStableIds())
        pub fn hasStableIds<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "hasStableIds", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "hasStableIds\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGroupView](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getGroupView(int,%20boolean,%20android.view.View,%20android.view.ViewGroup))
        ///
        /// Required features: "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn getGroupView<'env>(&'env self, arg0: i32, arg1: bool, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getGroupView", .descriptor == "(IZLandroid/view/View;Landroid/view/ViewGroup;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getGroupView\0", "(IZLandroid/view/View;Landroid/view/ViewGroup;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChildView](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getChildView(int,%20int,%20boolean,%20android.view.View,%20android.view.ViewGroup))
        ///
        /// Required features: "android-view-View", "android-view-ViewGroup"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-ViewGroup")))]
        pub fn getChildView<'env>(&'env self, arg0: i32, arg1: i32, arg2: bool, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ViewGroup>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getChildView", .descriptor == "(IIZLandroid/view/View;Landroid/view/ViewGroup;)Landroid/view/View;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getChildView\0", "(IIZLandroid/view/View;Landroid/view/ViewGroup;)Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isChildSelectable](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#isChildSelectable(int,%20int))
        pub fn isChildSelectable<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "isChildSelectable", .descriptor == "(II)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "isChildSelectable\0", "(II)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [areAllItemsEnabled](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#areAllItemsEnabled())
        pub fn areAllItemsEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "areAllItemsEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "areAllItemsEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEmpty](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#isEmpty())
        pub fn isEmpty<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "isEmpty", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "isEmpty\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGroupExpanded](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#onGroupExpanded(int))
        pub fn onGroupExpanded<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "onGroupExpanded", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "onGroupExpanded\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGroupCollapsed](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#onGroupCollapsed(int))
        pub fn onGroupCollapsed<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "onGroupCollapsed", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "onGroupCollapsed\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCombinedChildId](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getCombinedChildId(long,%20long))
        pub fn getCombinedChildId<'env>(&'env self, arg0: i64, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getCombinedChildId", .descriptor == "(JJ)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getCombinedChildId\0", "(JJ)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCombinedGroupId](https://developer.android.com/reference/android/widget/ExpandableListAdapter.html#getCombinedGroupId(long))
        pub fn getCombinedGroupId<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListAdapter", java.flags == PUBLIC | ABSTRACT, .name == "getCombinedGroupId", .descriptor == "(J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListAdapter\0", "getCombinedGroupId\0", "(J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
