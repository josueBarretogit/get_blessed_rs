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

pub fn toggle_status_all(crates: &mut [CrateItemList]) {
    for item in crates {
        item.status = match item.status {
            ItemListStatus::Selected => ItemListStatus::Unselected,
            ItemListStatus::Unselected => ItemListStatus::Selected,
        };
    }
}

pub fn toggle_status_one_crate(crate_selected: &mut CrateItemList) {
    match crate_selected.status {
        ItemListStatus::Selected => {
            crate_selected.status = ItemListStatus::Unselected;
        }
        ItemListStatus::Unselected => {
            crate_selected.status = ItemListStatus::Selected;
        }
    }
}

///If the crate is selected and it is not in the `crates_to_add` list then push it, else remove
///it
///if the crate is already in the list of crates to be  added and the features selelcted are
///different, then update it
pub fn push_or_remove_crates(crates_to_add: &mut Vec<CrateToAdd>, crates: &[CrateItemList]) {
    for krate in crates {
        match krate.status {
            ItemListStatus::Selected => {
                if !crates_to_add.iter().any(|crate_to_add| {
                    let features_are_different = crate_to_add
                        .features
                        .clone()
                        .zip(krate.features.clone())
                        .map(|(features_item_list, features_crate_to_add)| {
                            features_item_list.len() != features_crate_to_add.len()
                        });

                    match features_are_different {
                        Some(are_different) => {
                            are_different && crate_to_add.crate_name == krate.name
                        }
                        None => crate_to_add.crate_name == krate.name,
                    }
                }) {
                    crates_to_add.push(CrateToAdd::from(krate));
                }
            }
            ItemListStatus::Unselected => {
                crates_to_add.retain(|crate_to_add| crate_to_add.crate_name != krate.name);
            }
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
