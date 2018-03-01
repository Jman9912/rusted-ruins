
use gamedata::item::ItemList;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ShopKind {
    /// Sells weapons and armors
    Equipment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shop {
    kind: ShopKind,
    items: ItemList,
}
