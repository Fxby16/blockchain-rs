use eframe::egui::Ui;
use eframe::egui;

use crate::blockchain::digital_signature::gen_keys;
use crate::ui::state::AppState;

pub fn update_ui(state: &mut AppState, ui: &mut Ui) {
    ui.heading("Blockchain Wallet Manager");

    ui.separator();
    ui.label("Create New Wallet:");
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new("Create")).clicked() {
            state.wallets.push(gen_keys());
        }
    });

    ui.separator();
    let display_text = state.selected_wallet
                    .as_ref()
                    .unwrap_or(&"None".to_string())
                    .clone();
        
    egui::ComboBox::from_label("Select wallet")
        .selected_text(display_text)
        .show_ui(ui, |ui| {
            // Opzione "None"
            if ui.selectable_label(state.selected_wallet.is_none(), "None").clicked() {
                state.selected_wallet = None;
            }
            
            // Opzioni dalle stringhe
            for option in &state.wallets {
                let wallet_id = hex::encode(option.1.as_bytes());
                let is_selected = state.selected_wallet.as_ref() == Some(&wallet_id);
                if ui.selectable_label(is_selected, &wallet_id).clicked() {
                    state.selected_wallet = Some(wallet_id.clone());
                }
            }
        });
}
