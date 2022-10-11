use once_cell::sync::OnceCell;
use reqwest::{Client, ClientBuilder};

static CELL: OnceCell<Client> = OnceCell::new();

pub fn get_client<'a>() -> &'a Client {
    CELL.get_or_init(|| ClientBuilder::new().gzip(true).build().unwrap())
}
