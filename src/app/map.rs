use egui::TextBuffer;

use super::{model::NewCompany, NewCompanyRow};

pub fn map_to_new(
    NewCompanyRow {
        name,
        remainder_begin_month_pos,
        remainder_begin_month_neg,
        debit_turnover,
        credit_turnover,
    }: &NewCompanyRow,
) -> Result<NewCompany, Box<dyn std::error::Error>> {
    let remainder = match (
        remainder_begin_month_pos.as_str(),
        remainder_begin_month_neg.as_str(),
    ) {
        ("", second) => second,
        (first, "") => first,
        _ => "",
    };
    let parsed_remainder: f64 = remainder.parse()?;

    let parsed_debit: f64 = debit_turnover.parse()?;

    let parsed_credit: f64 = credit_turnover.parse()?;

    Ok(NewCompany {
        name: name.to_string(),
        remainder_begin_month: parsed_remainder,
        debit_turnover: parsed_debit,
        credit_turnover: parsed_credit,
    })
}
