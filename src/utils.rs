use ratatui::layout::{Constraint, Layout, Rect};

use crate::{
    dependency_builder::CrateToAdd,
    view::widgets::{CrateItemList, ItemListStatus},
};

pub fn toggle_status_all(dependencies: &mut [CrateItemList]) {
    for item in dependencies.iter_mut() {
        item.status = match item.status {
            ItemListStatus::Selected => ItemListStatus::Unselected,
            ItemListStatus::Unselected => ItemListStatus::Selected,
        };
    }
}

pub fn toggle_dependencies_all(crates: &[CrateItemList], dependencies_added: &mut Vec<CrateToAdd>) {
    for item in crates {
        let dependency_to_add = CrateToAdd {
            crate_name: item.name.clone(),
            features: item.features.clone(),
        };
        if dependencies_added.contains(&dependency_to_add)
            && item.status == ItemListStatus::Unselected
        {
            dependencies_added.retain(|it| *it != *dependency_to_add);
        } else if item.status == ItemListStatus::Selected {
            dependencies_added.push(dependency_to_add);
        }
    }
}

pub fn toggle_one_dependency(
    crate_selected: &mut CrateItemList,
    dependencies_added: &mut Vec<CrateToAdd>,
) {
    match crate_selected.status {
        ItemListStatus::Selected => {
            crate_selected.status = ItemListStatus::Unselected;
            dependencies_added.retain(|item| {
                *item
                    != CrateToAdd {
                        crate_name: crate_selected.name.clone(),
                        features: crate_selected.features.clone(),
                    }
            });
        }
        ItemListStatus::Unselected => {
            crate_selected.status = ItemListStatus::Selected;
            dependencies_added.push(CrateToAdd {
                crate_name: crate_selected.name.clone(),
                features: crate_selected.features.clone(),
            });
        }
    }
}

pub fn trim_ending(text: &str) -> String {
    todo!()
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
