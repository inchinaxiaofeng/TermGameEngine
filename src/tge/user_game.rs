use crate::game::Game;

pub fn start(game: &mut Game) {
    game.entities.push(Entity {
        id:1,
        name: "Hero".to_string(),
        position: (5,5)
    })
}

pub fn update(game: &mut Game) {

}
