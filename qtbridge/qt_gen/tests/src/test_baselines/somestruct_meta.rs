impl qtbridge::bridge::QMetaInfo for SomeStruct {
    fn register_meta(mut meta_obj: std::pin::Pin<&mut qtbridge::bridge::DynamicMetaObjectBuilder>) {
        use qt_type_lib::get_meta_type_id_of_fn_return_value;
        use qt_type_lib::QMetaTypeGet;
        use qt_type_lib::{QMetaType, QMetaTypeId};
        use qtbridge::bridge::metacallbacks::{
            property_read_callback_for, property_write_callback_for, slot_callback_for,
        };
        use qtbridge::qt_type_lib;
        meta_obj
            .as_mut()
            .register_signal("thisValueChanged", &[qt_type_lib::QString::get_qmetatype()]);
        meta_obj
            .as_mut()
            .register_signal("otherValueChanged", &[f32::get_qmetatype()]);
        meta_obj.as_mut().register_signal(
            "thisStringValueChangedByRef",
            &[qt_type_lib::QString::get_qmetatype()],
        );
        meta_obj.as_mut().register_signal(
            "thatStringValueChangedByValue",
            &[qt_type_lib::QString::get_qmetatype()],
        );
        meta_obj.as_mut().register_slot(
            "onThatValueChanged",
            &[qt_type_lib::QString::get_qmetatype()],
            &QMetaType::default(),
            slot_callback_for::<SomeStruct>(|this, inputs, _outputs| {
                let arg_0_ref = unsafe { inputs[0usize].cast::<qt_type_lib::QString>().as_ref() }
                    .expect("Argument reference is null");
                let arg_0_var: <String as ToOwned>::Owned = arg_0_ref.into();
                this.on_that_value_changed(&arg_0_var)
            }),
        );
        meta_obj.as_mut().register_property(
            "this_value",
            &(qt_type_lib::QString::get_qmetatype()),
            property_read_callback_for::<SomeStruct>(|this| this.get_value().into()),
            property_write_callback_for::<SomeStruct>(|this, value| {
                let Ok(value) = TryInto::try_into(value) else {
                    panic!(
                        "Failed to convert value '{}' to type '{}' in qproperty '{}'",
                        value.to_string(),
                        "String",
                        "this_value"
                    );
                };
                this.set_value(&value);
            }),
            "thisValueChanged",
        );
        meta_obj.as_mut().register_property(
            "otherValue",
            &(QMetaType::new(
                get_meta_type_id_of_fn_return_value(|this: &Self| &this.otherValueVar) as i32,
            )),
            property_read_callback_for::<SomeStruct>(|this| (&this.otherValueVar).into()),
            property_write_callback_for::<SomeStruct>(|this, value| {
                let Ok(value) = value.try_into() else {
                    panic!(
                        "Failed to convert value '{}' to type '{}' in qproperty '{}'",
                        value.to_string(),
                        std::any::type_name_of_val(&this.otherValueVar),
                        "otherValue"
                    );
                };
                if this.otherValueVar != value {
                    this.otherValueVar = value;
                    this.other_value_changed(this.otherValueVar.clone());
                }
            }),
            "otherValueChanged",
        );
        meta_obj
            .as_mut()
            .add_class_info("DefaultProperty", "this_value");
        meta_obj.as_mut().end_meta_registration();
    }
    fn get_shared_dynamic_meta_object() -> &'static qtbridge::bridge::DynamicMetaObjectBuilder {
        use std::any::TypeId;
        use std::cell::RefCell;
        use std::collections::HashMap;
        thread_local ! (static DYNAMIC_META_MAP : RefCell < HashMap < TypeId , * const qtbridge :: bridge :: DynamicMetaObjectBuilder >> = RefCell :: new (HashMap :: new ()));
        let type_id = TypeId::of::<SomeStruct>();
        {
            let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_data_map| {
                dynamic_meta_data_map
                    .get(&type_id)
                    .copied()
                    .unwrap_or_default()
            });
            if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
                return meta_data_ref;
            }
        }
        let meta_data_ptr =
            qtbridge::bridge::create_dynamic_meta_object_builder_for_type::<SomeStruct>();
        let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
        DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_data_map| {
            dynamic_meta_data_map.insert(type_id, meta_data_ptr);
        });
        meta_data_ref
    }
}
