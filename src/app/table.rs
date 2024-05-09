use egui::TextBuffer;
use egui_extras::{Column, TableBuilder};

use super::Row;

pub struct CompanyTable<'a> {
    rows: &'a mut Vec<Row>,
}

impl<'a> CompanyTable<'a> {
    pub fn new(rows: &'a mut Vec<Row>) -> CompanyTable<'a>{
        Self {
            rows
        }
    }

    // pub fn start_adding_new_row<'a>(&'a mut self) {
    //     self.is_new_row_being_added = true;
    //     // self.new_row = Some(row);
    // }
    //
    // pub fn finish_adding_new_row(&mut self) {
    //     self.is_new_row_being_added = false;
    // }

    pub fn table_ui(&mut self, ui: &mut egui::Ui) {
        // let text_height = egui::TextStyle::Body
        //     .resolve(ui.style())
        //     .size
        //     .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();

        let mut builder = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        builder
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Код");
                });

                header.col(|ui| {
                    ui.strong("Наименование");
                });
                header.col(|ui| {
                    ui.strong("Остаток на начало месяца");
                });
                header.col(|ui| {
                    ui.strong("Оборот за месяц");
                });
                header.col(|ui| {
                    ui.strong("Остаток на конец");
                });
            })
            .body(|mut body| {
                let row_height = 18.0;
                body.rows(row_height, self.rows.len(), |mut row| {
                    let index = row.index();
                    row.col(|ui| {
                        ui.label(format!("{index}"));
                    });
                    match &mut self.rows[index] {
                        Row::Constant(company) => {
                            row.col(|ui| {
                                ui.label(&company.name);
                            });
                            row.col(|ui| {
                                ui.columns(3, |columns| {
                                    columns[0].label("salam");
                                    columns[1].add(egui::Separator::default().vertical());
                                    columns[2].label("poka");
                                });
                            });
                            row.col(|ui| {
                                ui.columns(3, |columns| {
                                    columns[0].label("siytir");
                                    columns[1].add(egui::Separator::default().vertical());
                                    columns[2].label("poka");
                                });
                            });
                            row.col(|ui| {
                                ui.label("30");
                            });
                        },
                        Row::BeingEdited(_) => todo!(),
                        Row::New(new_company) => {
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut new_company.name);
                            });


                            row.col(|ui| {
                                ui.columns(3, |columns| {
                                    if columns[0].text_edit_singleline(&mut new_company.remainder_begin_month_pos).changed(){
                                        new_company.remainder_begin_month_neg.clear();
                                    };

                                    columns[1].add(egui::Separator::default().vertical());
                                    if columns[2].text_edit_singleline(&mut new_company.remainder_begin_month_neg).changed() {
                                        new_company.remainder_begin_month_pos.clear();
                                    };
                                });
                            });

                        },
                        Row::Total => todo!(),
                    }
                });
            });
    }
}
