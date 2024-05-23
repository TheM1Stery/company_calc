use super::{
    model::{EditedCompany, NewCompany},
    EditedCompanyRow, NewCompanyRow,
};

pub fn map_to_new(
    NewCompanyRow {
        name,
        remainder_begin_month_pos,
        remainder_begin_month_neg,
        debit_turnover,
        credit_turnover,
    }: &NewCompanyRow,
) -> Result<NewCompany, Box<dyn std::error::Error>> {
    let new_remainder: f64 = match (
        remainder_begin_month_pos.as_str(),
        remainder_begin_month_neg.as_str(),
    ) {
        (first, "") => first.parse()?,
        ("", second) => {
            let parsed: f64 = second.parse()?;
            parsed * -1.0
        }
        _ => 0.0,
    };

    let parsed_debit: f64 = debit_turnover.parse()?;

    let parsed_credit: f64 = credit_turnover.parse()?;

    Ok(NewCompany {
        name: name.to_string(),
        remainder_begin_month: new_remainder,
        debit_turnover: parsed_debit,
        credit_turnover: parsed_credit,
    })
}

pub fn map_to_edited(
    EditedCompanyRow {
        id,
        name,
        remainder_begin_month_pos,
        remainder_begin_month_neg,
        debit_turnover,
        credit_turnover,
    }: &EditedCompanyRow,
) -> Result<EditedCompany, Box<dyn std::error::Error>> {
    let new_remainder: f64 = match (
        remainder_begin_month_pos.as_str(),
        remainder_begin_month_neg.as_str(),
    ) {
        (first, "") => first.parse()?,
        ("", second) => {
            let parsed: f64 = second.parse()?;
            parsed * -1.0
        }
        _ => 0.0,
    };
    if name.is_empty() {
        return Err("no empty string".into());
    }

    let parsed_debit: f64 = debit_turnover.parse()?;

    let parsed_credit: f64 = credit_turnover.parse()?;
    Ok(EditedCompany {
        id: *id,
        name: name.to_string(),
        remainder_begin_month: new_remainder,
        debit_turnover: parsed_debit,
        credit_turnover: parsed_credit,
    })
}
