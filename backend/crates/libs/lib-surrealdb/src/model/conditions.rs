use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub start: Option<i64>,
    pub limit: Option<i64>,
    pub orderby: Option<Vec<OrderBy>>,
}

#[derive(Debug, Deserialize)]
pub struct OrderBy {
    pub field: String,
    pub is_desc: bool,
}

impl ListOptions {
    pub fn get_list_options(&self) -> String {
        let limit = self.gen_limit();

        if self.orderby.is_none() {
            return limit;
        }

        let order_by = self.gen_orderby().join(", ");
        format!("ORDER BY {} {}", order_by, limit)
    }

    fn gen_orderby(&self) -> Vec<String> {
        let mut orders = Vec::new();

        if let Some(order_bys) = &self.orderby {
            let _ = order_bys
                .iter()
                .map(|order_by| {
                    let order = if order_by.is_desc { "DESC" } else { "ASC" };
                    let order_con = format!("{} {}", order_by.field, order);

                    orders.push(order_con);
                })
                .count();
        }

        orders
    }

    fn gen_limit(&self) -> String {
        let mut limit_con = String::new();

        if let Some(limit) = &self.limit {
            let condition = format!("LIMIT {} ", limit);
            limit_con.push_str(&condition);
        }

        if let Some(start) = &self.start {
            let condition = format!("START {} ", start);
            limit_con.push_str(&condition);
        }

        limit_con
    }
}

pub trait Filter {
    fn gen_condition(&self) -> Vec<String>;
}

pub fn gen_all_condition<T>(filters: Option<Vec<T>>, list_options: Option<ListOptions>) -> String
where
    T: Filter + std::fmt::Debug + DeserializeOwned + Default,
{
    let filter = match filters {
        Some(filters) => {
            if filters.is_empty() {
                return String::new();
            }
            let filter = filters
                .iter()
                .map(|filter| filter.gen_condition().join(" AND "))
                .collect::<Vec<String>>()
                .join(" AND ");

            format!("WHERE {}", filter)
        }
        None => String::new(),
    };
    let list_option = match list_options {
        Some(l) => l.get_list_options(),
        None => String::new(),
    };

    format!("{} {}", filter, list_option)
}

macro_rules! create_eq_con {
    ($struct:ident ,$conditions:ident, $field:ident, $value:ident) => {
        if let Some($value) = &$struct.$value {
            let condition = format!("{} = {}", stringify!($field), $value);
            $conditions.push(condition);
        };
    };
}
pub(crate) use create_eq_con;

macro_rules! create_str_contain_con {
    ($struct:ident ,$conditions:ident, $field:ident, $value:ident) => {
        if let Some($value) = &$struct.$value {
            let condition = format!("{} CONTAINS '{}'", stringify!($field), $value);
            $conditions.push(condition);
        };
    };
}
pub(crate) use create_str_contain_con;

macro_rules! create_time_me_con {
    ($struct:ident ,$conditions:ident, $field:ident, $value:ident) => {
        if let Some($value) = &$struct.$value {
            let condition = format!("{} >= {}", stringify!($field), $value);
            $conditions.push(condition);
        };
    };
}
pub(crate) use create_time_me_con;

macro_rules! create_time_le_con {
    ($struct:ident ,$conditions:ident, $field:ident, $value:ident) => {
        if let Some($value) = &$struct.$value {
            let condition = format!("{} < {}", stringify!($field), $value);
            $conditions.push(condition);
        };
    };
}
pub(crate) use create_time_le_con;
