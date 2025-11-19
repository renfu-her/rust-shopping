use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Pagination {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl Pagination {
    pub fn new(current_page: i64, per_page: i64, total: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Pagination {
            current_page,
            per_page,
            total,
            total_pages,
        }
    }

    pub fn offset(&self) -> i64 {
        (self.current_page - 1) * self.per_page
    }

    pub fn has_prev(&self) -> bool {
        self.current_page > 1
    }

    pub fn has_next(&self) -> bool {
        self.current_page < self.total_pages
    }
}

