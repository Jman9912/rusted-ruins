
use array2d::*;
use objholder::*;
use gamedata::item::Inventory;
use gamedata::chara::CharaId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub w: u32,
    pub h: u32,
    pub tile: Array2d<TileInfo>,
    pub player_pos: Vec2d,
    /// Characters on this map
    charaid: Vec<CharaId>,
}

/// This represents special objects on a tile. For example, stairs, doors, traps.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum SpecialTileKind {
    None, DownStairs, UpStairs,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileInfo {
    pub tile: TileIdx, // Basic tile type
    pub wall: Option<WallIdx>, // If wall is presented, the tile is no walkable
    pub items: Option<Inventory>,
    pub chara: Option<CharaId>,
    pub special: SpecialTileKind,
}

impl Default for TileInfo {
    fn default() -> TileInfo {
        TileInfo {
            tile: TileIdx(0),
            wall: None,
            items: None,
            chara: None,
            special: SpecialTileKind::None,
        }
    }
}

impl Map {
    pub fn new(w: u32, h: u32) -> Map {
        Map {
            w: w, h: h, tile: Array2d::new(w, h, TileInfo::default()),
            player_pos: Vec2d::new(0, 0), charaid: Vec::new()
        }
    }

    pub fn get_chara<T: Into<Vec2d>>(&self, pos: T) -> Option<CharaId> {
        self.tile[pos.into()].chara.clone()
    }

    pub fn iter_charaid(&self) -> ::std::slice::Iter<CharaId> {
        self.charaid.iter()
    }

    pub fn is_movable(&self, pos: Vec2d) -> bool {
        if !self.is_inside(pos) {
            return false;
        }

        if self.tile[pos].wall.is_some() {
            false
        }else{
            true
        }
    }

    /// Return given pos is inside map or not
    #[inline]
    pub fn is_inside(&self, pos: Vec2d) -> bool {
        if pos.0 >= 0 && pos.1 >= 0 && (pos.0 as u32) < self.w && (pos.1 as u32) < self.h {
            true
        }else{
            false
        }
    }

    /// Add one character on this map.
    pub(crate) fn add_chara(&mut self, pos: Vec2d, id: CharaId) {
        self.charaid.push(id);
        self.tile[pos].chara = Some(id);
    }

    /// Get character position
    pub fn chara_pos(&self, cid: CharaId) -> Option<Vec2d> {
        for p in self.tile.iter_idx() {
            if self.tile[p].chara.as_ref() == Some(&cid) {
                return Some(p);
            }
        }
        None
    }

    pub fn move_chara(&mut self, cid: CharaId, dir: Direction) -> bool {
        use std::mem::replace;
        if let Some(p) = self.chara_pos(cid) {
            let new_p = p + dir.as_vec();
            if self.is_movable(p + dir.as_vec()) { // Swap charas of the two tiles
                let temp0 = replace(&mut self.tile[p].chara,     None);
                let temp1 = replace(&mut self.tile[new_p].chara, None);
                replace(&mut self.tile[p].chara,     temp1);
                replace(&mut self.tile[new_p].chara, temp0);
                true
            }else{
                false
            }
        }else{
            false
        }
    }

    pub(crate) fn search_empty_onmap_charaid_n(&self) -> u32 {
        'i_loop:
        for i in 0.. {
            for cid in self.charaid.iter() {
                if let CharaId::OnMap { n, .. } = *cid {
                    if i == n { continue 'i_loop; }
                }
            }
            return i;
        }
        panic!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct MapId {
    pub sid: super::site::SiteId,
    pub floor: u32,
}

pub const STARTING_MAP_ID: MapId = MapId { sid: super::site::SiteId::Start, floor: 0 };