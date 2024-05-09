#[derive(Default)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub remainder_begin_month: f64,
    pub debit_turnover: f64,
    pub credit_turnover: f64,
    pub remainder_end_month: f64
}

pub struct EditedCompany {
    pub id: i64,
    pub name: String,
    pub remainder_begin_month: f64,
    pub debit_turnover: f64,
    pub credit_turnover : f64,
}

#[derive(Default)]
pub struct NewCompany {
    pub name: String,
    pub remainder_begin_month: f64,
    pub debit_turnover: f64,
    pub credit_turnover: f64
}

pub fn merge_edit(company: &mut Company,edited: EditedCompany){
    company.id = edited.id;
    company.name = edited.name;
    company.remainder_begin_month = edited.remainder_begin_month;
    company.debit_turnover = edited.debit_turnover;
    company.credit_turnover = edited.credit_turnover;
}

