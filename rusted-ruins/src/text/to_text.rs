use crate::text::{self, misc_txt, obj_txt, ui_txt, ToText, ToTextId};
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use std::borrow::Cow;

impl<T: ToTextId> ToText for T {
    fn to_text(&self) -> Cow<str> {
        text::to_txt(self).into()
    }
}

impl ToText for Site {
    fn to_text(&self) -> Cow<str> {
        if let Some(ref name) = self.name {
            let name: &str = &*name;
            return name.into();
        }

        match self.content {
            SiteContent::AutoGenDungeon { dungeon_kind } => text::to_txt(&dungeon_kind).into(),
            SiteContent::Town { ref town } => text::obj_txt(town.id()).into(),
            SiteContent::Other => {
                warn!("Unnamed other kind site");
                "".into()
            }
        }
    }
}

impl ToText for Item {
    fn to_text(&self) -> Cow<str> {
        use crate::game::item::ItemEx;
        let mut text: Cow<str> = obj_txt(gobj::idx_to_id(self.idx)).into();

        if let Some(n) = self.charge() {
            text = format!("{} ({} : {})", text, ui_txt("item.charges"), n).into();
        }
        text
    }
}

impl ToText for CharaTemplateIdx {
    fn to_text(&self) -> Cow<str> {
        obj_txt(gobj::idx_to_id(*self)).into()
    }
}

impl ToText for Chara {
    fn to_text(&self) -> Cow<str> {
        if let Some(ref name) = self.name {
            name.into()
        } else {
            obj_txt(gobj::idx_to_id(self.template)).into()
        }
    }
}

impl ToText for MedicalEffect {
    fn to_text(&self) -> Cow<str> {
        use MedicalEffect::*;
        match self {
            None => misc_txt("!medical_effect.none"),
            Heal => misc_txt("!medical_effect.heal"),
            Sleep => misc_txt("!medical_effect.sleep"),
            Poison => misc_txt("!medical_effect.poison"),
        }
        .into()
    }
}

impl ToText for Quest {
    fn to_text(&self) -> Cow<str> {
        match self {
            Quest::SlayMonsters { idx, .. } => {
                replace_str!(text::misc_txt("!quest.slay_monsters"); monster=idx).into()
            }
        }
    }
}

/// Implement ToText for primitive types
macro_rules! impl_to_text {
    ( $($t:ty),* ) => {
        $(
            impl ToText for $t {
                fn to_text(&self) -> Cow<str> {
                    self.to_string().into()
                }
            }
        )*
    }
}

impl_to_text!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, String);

impl<'a> ToText for &'a str {
    fn to_text(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl<'a> ToText for Cow<'a, str> {
    fn to_text(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}
