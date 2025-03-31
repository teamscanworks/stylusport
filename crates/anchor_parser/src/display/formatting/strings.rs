use crate::display::constants::formatting;

pub fn format_module_name(name: &str) -> String {
    format!("{} {}", formatting::LABEL_MODULE, name)
}

pub fn format_function_name(name: &str) -> String {
    format!("{} {}", formatting::LABEL_FUNCTION, name)
}

pub fn format_struct_name(name: &str) -> String {
    format!("{} {}", formatting::LABEL_STRUCT, name)
}

pub fn format_items_count(count: usize) -> String {
    format!("{}{}", formatting::LABEL_ITEMS, count)
}

pub fn format_item_reference(parent_index: usize, item_index: usize, item_type: &str) -> String {
    format!(
        "{}{}.{}: {}",
        formatting::LABEL_ITEM,
        parent_index,
        item_index,
        item_type
    )
}

pub fn format_nested_module_name(name: &str) -> String {
    format!("{}{}", formatting::LABEL_NESTED_MODULE, name)
}

pub fn format_attributes_count(count: usize) -> String {
    format!("{}{}", formatting::LABEL_ATTR, count)
}
