use anyhow::Result;

use futures::StreamExt;
use ratatui::widgets::TableState;
use tokio::sync::Mutex;

use std::{collections::BTreeMap, sync::Arc};
use tokio_tungstenite::connect_async;
use url::Url;

type ProductsMap = BTreeMap<String, Product>;

#[derive(Debug)]
pub struct App {
    // UI table state
    pub state: TableState,
    pub items: Vec<Vec<String>>,
    // Raw data state
    pub products: Products,
    pub products_map: ProductsMap,
    pub streams_count: usize,
}

pub type ArcMutexApp = Arc<Mutex<App>>;

use std::{collections::HashMap, env};

use crate::types::{Product, ProductResponse, Products, SocketData};

impl App {
    pub async fn new() -> App {
        let exchange_info = get_pairs().await.unwrap();

        let mut products = exchange_info.data;
        products.sort_by(|p1, p2| p1.s.cmp(&p2.s));

        let _parent_categories = get_parent_categories(&products);

        let products_map = create_btreemap(products.clone());

        let items = convert_to_vectors(&products);

        App {
            state: TableState::default(),
            items,
            products,
            products_map,
            streams_count: 0,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_row(&mut self, index: usize) {
        self.state.select(Some(index))
    }

    pub fn set_items(&mut self, products: &Products) {
        let result = convert_to_vectors(products);
        self.items = result;
    }
}

pub async fn connect_websocket(arc_mutex_app: ArcMutexApp) -> Result<()> {
    let websocket_url: String =
        env::var("WEBSOCKET_URL").expect("WEBSOCKET_URL enviroment variable unset");

    let (mut socket, _) =
        connect_async(Url::parse(websocket_url.as_str()).expect("Can't connect")).await?;
    loop {
        let msg = socket.next().await.expect("Can't fetch case count")?;
        let message_string = msg
            .clone()
            .into_text()
            .expect("Cant convert response into string");

        let stream_data: SocketData = serde_json::from_str(&message_string)?;

        let mut app = arc_mutex_app.lock().await;

        app.streams_count = stream_data.data.len();

        for mini_ticker in stream_data.data.into_iter() {
            app.products_map.entry(mini_ticker.s).and_modify(|product| {
                product.c = mini_ticker.c;
                product.h = mini_ticker.h;
                product.l = mini_ticker.l;
                product.o = mini_ticker.o;
                product.qv = mini_ticker.q;
                product.v = mini_ticker.v;
            });
        }

        let new_products: Products = app.products_map.clone().into_values().collect();

        app.set_items(&new_products);

        // TODO: remove, because useless
        app.products = new_products;
    }
}

pub fn create_btreemap(products: Products) -> ProductsMap {
    let products_keyed = products.into_iter().fold(BTreeMap::new(), |mut acc, cur| {
        acc.insert(cur.s.to_owned(), cur);
        acc
    });
    products_keyed
}

pub fn convert_to_vectors(products: &Products) -> Vec<Vec<String>> {
    let result: Vec<_> = products
        .iter()
        .map(|asset| {
            let volume = asset.qv.clone();
            let pair = format!("{a}/{b}", a = asset.b, b = asset.q);
            let last_price = asset.c.clone();
            let opening_price: f64 = asset.o.parse().unwrap();
            let closing_price: f64 = asset.c.parse().unwrap();

            let change = (opening_price - closing_price) / opening_price;
            let change_formatted = format!("{:+.2}%", change);

            vec![pair, last_price, change_formatted, volume]
        })
        .collect();
    result
}

pub fn get_parent_categories(products: &Products) -> HashMap<&String, Vec<String>> {
    let parent_categories = products.iter().fold(HashMap::new(), |mut res, cur| {
        let parent_category_name = &cur.pn;
        let currency = &cur.q;
        let entry: &mut Vec<String> = res.entry(parent_category_name).or_insert(Vec::new());

        if !entry.contains(currency) {
            entry.push(currency.clone())
        }

        res
    });
    parent_categories
}

async fn get_pairs() -> Result<ProductResponse> {
    let url: String = env::var("REST_API_URL").expect("REST_API_URL enviroment variable unset");
    let response = reqwest::get(url).await?;
    let body: ProductResponse = response.json().await?;
    Ok(body)
}
