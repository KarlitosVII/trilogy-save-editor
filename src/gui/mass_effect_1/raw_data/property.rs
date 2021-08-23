use std::cell::Ref;
use std::cmp::Ordering;

use yew::{prelude::*, utils::NeqAssign};

use crate::gui::{
    components::{raw_ui::RawUiStruct, CallbackType, InputText},
    raw_ui::RawUi,
    RcUi,
};
use crate::save_data::{
    mass_effect_1::{
        data::{ArrayType, Property as DataProperty, StructType},
        player::Player,
    },
    List,
};

pub enum Msg {
    DuplicateName(RcUi<u32>, CallbackType),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub player: RcUi<Player>,
    pub property: RcUi<DataProperty>,
    pub label: Option<String>,
}

impl Props {
    fn player(&self) -> Ref<'_, Player> {
        self.player.borrow()
    }

    fn property(&self) -> Ref<'_, DataProperty> {
        self.property.borrow()
    }
}

pub struct Property {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Property {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Property { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DuplicateName(mut value_name_id, CallbackType::String(new_value)) => {
                let mut player = self.props.player.borrow_mut();
                let mut names = player.names.borrow_mut();

                // Duplicate
                let idx = *value_name_id.borrow() as usize;
                let mut dupe = names[idx].clone();
                dupe.string = RcUi::new(new_value);
                dupe.is_duplicate = true;
                names.push(dupe);

                // Change value name id
                *value_name_id.borrow_mut() = (names.len() - 1) as u32;

                true
            }
            _ => unreachable!(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let player = self.props.player();

        let get_name = |name_id: &u32| -> String {
            // Label override
            if let Some(ref label) = self.props.label {
                return label.clone();
            }

            let name = player.get_name(*name_id);
            name.trim_start_matches("m_a") // Array
                .trim_start_matches("m_b") // Bool
                .trim_start_matches("m_e") // Byte
                .trim_start_matches("m_f") // Float
                .trim_start_matches("m_n") // Int
                .trim_start_matches("m_o") // Object
                .trim_start_matches("m_s") // String
                .trim_start_matches("m_")
                .to_owned()
        };

        fn view_text(text: String, label: String) -> Html {
            html! {
                <div class="flex-auto flex items-center gap-1">
                    <span class="w-2/3">{ text }</span>
                    { label }
                </div>
            }
        }

        match &*self.props.property() {
            DataProperty::Array { name_id, array, .. } => {
                let label = get_name(name_id);

                let items = array.iter().enumerate().map(|(idx, item)| match item {
                    ArrayType::Int(int) => int.view(&idx.to_string()),
                    ArrayType::Object(object_id) => {
                        if *object_id != 0 {
                            // Object
                            let object = player.get_object(*object_id);
                            let object_name = player.get_name(object.object_name_id);

                            let label = format!("{} : {}", object_name, idx);
                            let properties = &player.get_data(*object_id).properties;
                            self.view_properties(label, properties)
                        } else {
                            // Null
                            html! { "Null" }
                        }
                    }
                    ArrayType::Vector(vector) => vector.view(&idx.to_string()),
                    ArrayType::String(string) => string.view(&idx.to_string()),
                    ArrayType::Properties(properties) => {
                        self.view_properties(idx.to_string(), properties)
                    }
                });

                html! {
                    <RawUiStruct {label}>
                        { for items }
                    </RawUiStruct>
                }
            }
            DataProperty::Bool { name_id, value, .. } => {
                let label = get_name(name_id);
                value.view(&label)
            }
            DataProperty::Byte { name_id, value, .. } => {
                let label = get_name(name_id);
                value.view(&label)
            }
            DataProperty::Float { name_id, value, .. } => {
                let label = get_name(name_id);
                value.view(&label)
            }
            DataProperty::Int { name_id, value, .. } => {
                let label = get_name(name_id);
                value.view(&label)
            }
            DataProperty::Name { name_id, value_name_id, .. } => {
                let label = get_name(name_id);
                let name = &player.names.borrow()[*value_name_id.borrow() as usize];

                if name.is_duplicate {
                    html! {
                        <InputText {label} value={RcUi::clone(&name.string)} />
                    }
                } else {
                    let value_name_id = RcUi::clone(value_name_id);
                    let oninput = self.link.callback(move |callback| {
                        Msg::DuplicateName(RcUi::clone(&value_name_id), callback)
                    });
                    html! {
                        <InputText {label}
                            value={RcUi::new(name.string.borrow().clone())}
                            {oninput}
                        />
                    }
                }
            }
            DataProperty::Object { name_id, object_id, .. } => {
                let label = get_name(name_id);
                match object_id.cmp(&0) {
                    Ordering::Greater => {
                        // Object
                        let object = player.get_object(*object_id);
                        let object_name = player.get_name(object.object_name_id);

                        let label = format!("{} : {}", object_name, label);
                        let properties = &player.get_data(*object_id).properties;
                        self.view_properties(label, properties)
                    }
                    Ordering::Less => {
                        // Class
                        let class = player.get_class(*object_id);
                        let class_name = get_name(&class.class_name_id);
                        view_text(class_name, label)
                    }
                    Ordering::Equal => {
                        // Null => Default class name
                        view_text(String::from("Class"), label)
                    }
                }
            }
            DataProperty::Str { name_id, string, .. } => {
                let label = get_name(name_id);
                string.view(&label)
            }
            DataProperty::StringRef { name_id, value, .. } => {
                let label = get_name(name_id);
                value.view(&label)
            }
            DataProperty::Struct { name_id, struct_name_id, struct_type, .. } => {
                let name = get_name(name_id);
                let struct_name = get_name(struct_name_id);
                let label = format!("{} : {}", struct_name, name);

                match struct_type {
                    StructType::LinearColor(color) => color.view(&label),
                    StructType::Vector(vector) => vector.view(&label),
                    StructType::Rotator(rotator) => rotator.view(&label),
                    StructType::Properties(properties) => self.view_properties(label, properties),
                }
            }
            DataProperty::None { .. } => unreachable!(),
        }
    }
}

impl Property {
    fn view_properties(&self, label: String, properties: &List<RcUi<DataProperty>>) -> Html {
        let len = properties.len();
        let take = if len > 0 { len - 1 } else { 0 };
        let properties = properties.iter().take(take).map(|property| {
            html! {
                <Property
                    player={RcUi::clone(&self.props.player)}
                    property={RcUi::clone(property)}
                />
            }
        });

        html! {
            <RawUiStruct {label}>
                { for properties }
            </RawUiStruct>
        }
    }
}
