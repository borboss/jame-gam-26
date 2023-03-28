use bevy::prelude::{Component};

#[derive(Component, Debug, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub cost: u8,
    pub sprite_path: String,
    pub id: i8,
}
impl Default for Card {
    fn default() -> Card {
        Card {
            card_type: CardType::Projectile(ProjectileType::Fireball),
            cost: 10u8,            
            sprite_path: "sprites/cards/attacks/blank_attack.png".to_string(),   
            id: 0i8,
        }
    }
}

#[derive(Component)]
pub struct InInventory;

#[derive(Default, Clone, Copy, Debug)]
pub enum CardType {
    Projectile(ProjectileType),
    Melee(MeleeType),
    Buff(BuffType),
    #[default] Other,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum MeleeType {
    Stomp,
    #[default] Other,
}
#[derive(Default, Clone, Copy, Debug)]
pub enum ProjectileType {
    Fireball,
    NrgBall,
    #[default] Other,
}
#[derive(Default, Clone, Copy, Debug)]
pub enum BuffType {
    Heal,
    #[default] Other,
}



#[derive(Component)]
pub struct MarkedForPlaying;