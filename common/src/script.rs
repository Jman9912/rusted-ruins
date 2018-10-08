
use std::ops::Index;
use std::fmt;
use hashmap::HashMap;

/// Instructions are executed in Game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Instruction {
    /// Jump to given section
    Jump(String),
    /// Jump if given expr is true
    JumpIf(String, Expr),
    /// Talk instruction (textid, Vec<choice's textid, destination section>)
    Talk(String, Vec<(String, String)>),
    /// Player recieve money
    RecieveMoney(Expr),
    /// Remove item form player's inventory
    RemoveItem(String),
    /// Special instruction to start buying at a shop
    ShopBuy,
    /// Special instruction to start selling at a shop
    ShopSell,
    /// Special instruction to get locations of dungeons
    GetDungeonLocation,
}

/// Instructions are executed in Game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Expr {
    Value(Value),
    HasItem(String),
}

/// Instructions are executed in Game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Error,
}

/// Script consists of one or more sections.
/// One section includes one or more instructions.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Script(HashMap<String, Vec<Instruction>>);

impl Script {
    pub fn from_map(map: HashMap<String, Vec<Instruction>>) -> Script {
        Script(map)
    }
    
    pub fn get(&self, pos: &ScriptPos) -> Option<&Instruction> {
        self.0[&pos.section].get(pos.i)
    }
    
    pub fn section(&self, s: &str) -> &[Instruction] {
        self.0[s].as_ref()
    }
}

pub const QUIT_SECTION: &'static str = "quit";

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ScriptPos {
    pub section: String,
    pub i: usize,
}

impl ScriptPos {
    pub fn advance(&mut self) {
        self.i += 1;
    }

    pub fn set_section<S: ToString>(&mut self, section: S) {
        let section = section.to_string();

        assert_ne!(section, QUIT_SECTION);
        
        self.i = 0;
        self.section = section;
    }
}

impl<'a> Index<&'a ScriptPos> for Script {
    type Output = Instruction;

    fn index(&self, pos: &ScriptPos) -> &Instruction {
        &self.section(&pos.section)[pos.i]
    }
}

/// Object that include script data.
#[derive(Serialize, Deserialize)]
pub struct ScriptObject {
    pub id: String,
    pub script: Script,
}

#[derive(Clone, Debug)]
pub struct ScriptParseError {
    description: String,
}

impl ::std::error::Error for ScriptParseError {}

impl fmt::Display for ScriptParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "script parse error : {}", self.description)
    }
}
