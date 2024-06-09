use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::ListState,
};

use crate::{
    dependency_builder::CrateToAdd,
    view::{
        app::App,
        widgets::{CategoriesWidget, CrateItemList, FeatureItemList, ItemListStatus},
    },
};

pub fn toggle_status_all(dependencies: &mut [CrateItemList]) {
    for item in dependencies {
        item.status = match item.status {
            ItemListStatus::Selected => ItemListStatus::Unselected,
            ItemListStatus::Unselected => ItemListStatus::Selected,
        };
    }
}

pub fn toggle_dependencies_all(crates: &[CrateItemList], dependencies_added: &mut Vec<CrateToAdd>) {
    for item in crates {
        let dependency_to_add = CrateToAdd::from(item.clone());

        if dependencies_added
            .iter()
            .any(|dependency| dependency.crate_name == dependency_to_add.crate_name)
            && item.status == ItemListStatus::Unselected
        {
            dependencies_added.retain(|it| *it.crate_name != *dependency_to_add.crate_name);
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
            dependencies_added.retain(|item| *item.crate_name != crate_selected.name);
        }
        ItemListStatus::Unselected => {
            crate_selected.status = ItemListStatus::Selected;
            dependencies_added.push(CrateToAdd::from(crate_selected.clone()));
        }
    }
}

pub fn toggle_one_feature(current_crate: &mut CrateItemList, features_list_state: &ListState) {
    if let Some((index, current_crate_features)) = features_list_state
        .selected()
        .zip(current_crate.features.as_mut())
    {
        if current_crate_features.is_empty() {
            return;
        }

        let current_feature_selected = &mut current_crate_features[index];

        match current_feature_selected.status {
            ItemListStatus::Selected => {
                current_feature_selected.status = ItemListStatus::Unselected;
            }
            ItemListStatus::Unselected => {
                current_feature_selected.status = ItemListStatus::Selected;
            }
        };
    };
}

pub fn select_crate_if_features_are_selected(app: &mut App) {
    if let Some((crate_selected, index_current_crate)) = app.get_current_crate_selected() {
        let current_crate_is_selected = app
            .crates_to_add
            .widget
            .crates
            .iter()
            .any(|crate_to_add| crate_to_add.crate_name == crate_selected.name);

        if crate_selected.features.as_ref().is_some_and(|features| {
            features
                .iter()
                .any(|feature| feature.status == ItemListStatus::Selected)
        }) && !current_crate_is_selected
        {
            app.crates_to_add
                .widget
                .crates
                .push(CrateToAdd::from(crate_selected));
            match app.crate_categories.widget {
                CategoriesWidget::General => {
                    app.general_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Common => {
                    app.common_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::FFI => {
                    app.ffi_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Math => {
                    app.math_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Clis => {
                    app.clis_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Graphics => {
                    app.graphics_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Databases => {
                    app.database_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Networking => {
                    app.networking_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Concurrency => {
                    app.concurrency_crates[index_current_crate].status = ItemListStatus::Selected;
                }
                CategoriesWidget::Cryptography => {
                    app.cryptography_crates[index_current_crate].status = ItemListStatus::Selected;
                }
            }
        }
    }
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

pub fn load_features(
    crates_list: &mut [CrateItemList],
    index_crate_to_update: usize,
    features: Option<Vec<FeatureItemList>>,
) {
    crates_list[index_crate_to_update].is_loading = false;
    if let Some(feat) = features {
        crates_list[index_crate_to_update].features = Some(feat);
    }
}
