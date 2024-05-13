use crate::view::widgets::{CrateItemList, ItemListStatus};

pub fn toggle_status_all(dependencies: &mut Vec<CrateItemList>) {
    dependencies.iter_mut().for_each(|item| {
        item.status = match item.status {
            ItemListStatus::Selected => ItemListStatus::Unselected,
            ItemListStatus::Unselected => ItemListStatus::Selected,
        };
    });
}

pub fn toggle_dependencies_all(crates: &Vec<CrateItemList>, dependencies_added: &mut Vec<String>) {
    crates.iter().for_each(|item| {
        let dependency_to_add = item.name.to_owned();
        if dependencies_added.contains(&dependency_to_add)
            && item.status == ItemListStatus::Unselected
        {
            dependencies_added.retain(|it| *it != *dependency_to_add);
        } else if item.status == ItemListStatus::Selected {
            dependencies_added.push(dependency_to_add);
        }
    })
}

pub fn toggle_one_dependency(
    crate_selected: &mut CrateItemList,
    dependencies_added: &mut Vec<String>,
) {
    match crate_selected.status {
        ItemListStatus::Selected => {
            crate_selected.status = ItemListStatus::Unselected;
            dependencies_added.retain(|item| *item != crate_selected.name);
        }
        ItemListStatus::Unselected => {
            crate_selected.status = ItemListStatus::Selected;
            dependencies_added.push(crate_selected.name.to_owned());
        }
    }
}
