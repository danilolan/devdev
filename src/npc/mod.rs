use bevy::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(PathfindingPlugin);
    }
}

//components

//employee component and methods
//needs is the basic needs of the npc
//Atributtes is the rpg atributtes for the npc that will use to calc your performance
//experience grind is calculated using the formula: (level/xp_factor) ^ xp_power
#[derive(Component)]
pub struct Employee {
    pub experience: u32,
    pub level: u32,
    pub role: Role,
    pub needs: Needs,
    pub atributtes: Atributtes,

    xp_power: f32,
    xp_factor: f32,
}

const XP_FACTOR: f32 = 0.1;
const XP_POWER: f32 = 2.0;

impl Default for Employee {
    fn default() -> Self {
        Employee {
            experience: 0,
            level: 0,
            role: Role::None,
            xp_factor: XP_FACTOR,
            xp_power: XP_POWER,
            needs: Needs {
                hungry: 100,
                thirst: 100,
                relief: 100,
                energy: 100,
                focus: 100,
                creativity: 100,
            },
            atributtes: Atributtes {
                logic: 50,
                linguistic: 50,
                social: 50,
                practical: 50,
                artistic: 50,
            },
        }
    }
}
impl Employee {
    pub fn add_experience(&mut self, amount: u32) {
        self.experience += amount;

        while self.experience >= self.experience_to_next_level() {
            self.experience -= self.experience_to_next_level();
            self.level += 1;
        }
    }

    pub fn experience_to_next_level(&self) -> u32 {
        ((self.level as f32 / self.xp_factor).powf(self.xp_power)) as u32
    }
}
pub enum Role {
    Marketing,
    CustomerAcquisition,
    Design,
    Tech,
    Finance,
    HumanResources,
    SupportServices,
    None,
}
#[derive(Component)]
pub struct Needs {
    pub hungry: u32,
    pub thirst: u32,
    pub relief: u32,
    pub energy: u32,
    pub focus: u32,
    pub creativity: u32,
}

#[derive(Component)]
pub struct Atributtes {
    pub logic: u32,
    pub linguistic: u32,
    pub social: u32,
    pub practical: u32,
    pub artistic: u32,
}
