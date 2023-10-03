//! A tycoon game that you managing a Software Company.
//!
//! # About
//! This game is entire developed using Rust and [Bevy](https://bevyengine.org/) and using a
//! [ECS](https://en.wikipedia.org/wiki/Entity_component_system) design pattern.
//!
//! # Development flow
//! To develop anything in this project you will need to follow simple steps:
//!
//! ### 2 - Pull in branch main
//! ``` bash
//! git checkout main
//! git pull
//! ```
//! ### 2 - Create a new branch.
//! Create a new branch following the [**Semantic Messages**](https://gist.github.com/joshbuchea/6f47e86d2510bce28f8e7f42ae84c716),
//! **lowercase** and separate by **hifen** ( "-" ).
//!
//! In this project we are using essentially three:
//! 1. feat - a new feature
//! 2. fix - solve a bug or broken funcionallity
//! 3. refactor - refact some code, change names, organiza de structure..
//!
//! Example:
//! ``` bash
//! git checkout -b "feat/square-function"
//! ```
//! ### 3 - Develop your changes.
//! All the project design and rules to follow will be explain later in this guide.
//! We recommended to follow next two steps while are devolping your new code: **Documentation** and **Unit testing**.
//! ``` rust
//! fn square(x: f32) -> f32 {
//!     return x * x;
//! }
//! ```
//!
//! Try to create small **commits** following the [**Semantic Messages**](https://gist.github.com/joshbuchea/6f47e86d2510bce28f8e7f42ae84c716)
//! as well.
//!
//! ``` bash
//! git add *
//! git commit -m "feat/square-function"
//! ```
//!
//! ### 4 - Create documentation.
//! Use the [Rust docs](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) to create  the documentation to your
//! new code. Try to explain all of your **structs**, **modules**, **methods** and etc, being practical with one or two lines.
//!
//! Remember, the first paragraph has to be a simple phrase that explain the code. This phrase will be use to search about your new
//! code. After that you can explain with more details.
//!
//! ### 5 - Create unit testing
//! Testing is a very important on a big project like that, because if any new feature changes some old code ( that are being used in
//! parts of the project ), the tests will validate if this code continues to serve the correct output for the rest of the project.
//!
//! Create unit tests for all functions, methods and everything it is possible.
//!
//! ### 6 - Run the tests
//! Before push your changes remember to run the tests, to validate the old tests and the new ones.
//! ``` bash
//! cargo test
//! ```
//!
//! When all test has passed you can continue.
//!
//! ### 7 - Create the Pull Request for main
//! Try to explain what your new code do in the description of the PR.
//!
//! # Project Design
//! As mentioned before this project use a [ECS](https://en.wikipedia.org/wiki/Entity_component_system) design pattern.
//! You have to know how the ECS works before coding for the game. Try the official docs of [Bevy](https://bevyengine.org/).
//!
//! * We try to separate the modules in this project to big funcionalities of the game. This big module will contain all the
//! components, systems, resources and entities to support that logic and make it works.
//!
//! * Use cammel case in **Structs**, and snake case for everything else.
//!
//! ``` rust
//! #[derive(Component)]
//! pub struct LikeThisExample {
//!     foo_bar: f32
//! }
//! ```
//!
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

mod player_interaction;
use player_interaction::PlayerInteractionPlugin;

mod world;
use world::WorldPlugin;

mod npc;
use npc::NpcPlugin;

mod scene;
use scene::ScenePlugin;

fn main() {
    App::new()
        //-- Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        //-- Fps
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        //-- Plugins
        .add_plugins(PlayerInteractionPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(NpcPlugin)
        .add_plugins(ScenePlugin)
        .run();
}
