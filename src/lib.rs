pub mod auth;
pub mod pages;
pub mod database;
pub mod middleware;

pub use auth::{
    staff_account_creation::{
        StaffAccountCredentialRequest,
        JobsRequest,
        SpecificRolesRequest,
        StaffDashboardAccessRequest,
        Permissions,
        ApiResponse,
    },
    delete_staff_request::delete_staff_detail,
    fetches_list_of_staffs::{
        fetches_list_of_staffs,
        fetches_staff_detail,
    },
};
pub use database::{
    db::db_create_pool,
    tables_creation::account_option_page,
};
pub use middleware::error_validation;
pub use pages::list_of_accessible_pages::{
    staff_account_creation_page,
    dashboard,
    render_regis_page,
    render_detail,
    delete_detail
};
