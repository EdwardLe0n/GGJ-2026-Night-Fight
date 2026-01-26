#[turbo::serialize]
#[derive(PartialEq)]
pub enum ComponentTypes {
    Camera,
    Rectangle,
    Sprite,
    Text,
    TextBox,
    Button,
    Particle,

    // User made structs

    Resizer,

    // Game Specific

    PlayerController,
    SpriteSheetRenderer,

    CodeContainer,
    Number,

    RectangleCollider,

    // Other

    Other

}