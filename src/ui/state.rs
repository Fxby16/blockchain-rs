use ed25519_dalek::{SigningKey, VerifyingKey};
use eframe::egui;

use crate::{blockchain::transactions::Transaction, ui::ui::update_ui};

pub struct AppState {
    pub wallets: Vec<(SigningKey, VerifyingKey)>,
    pub selected_wallet: Option<String>,
    pub transaction: Transaction,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            wallets: Vec::new(),
            selected_wallet: None,
            transaction: Transaction::default()
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            update_ui(self, ui);
        });
    }
}