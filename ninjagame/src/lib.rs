
// Core Turbo functionality
use turbo::*;

// Links the driver file to the main lib file
mod ecs_gamestate;

// Initial TurbECS import
mod turbecs;

// Manager imports
use turbecs::{managers};

use managers::entity_manager::EntityManager;
use managers::component_manager::ComponentManager;
use managers::scene_manager::{SceneManager, Scenes};

use managers::time_manager::TimeManager;
use managers::input_manager::InputManager;

use managers::collision_manager::CollisionManager;

// Community work/manager imports
use managers::particlemanager::ParticleManager;

// Game specific elements that need to be in the GameState

mod assets;
use assets::game_state::{run_data::RunData, score_manager::ScoreManager, online_manager::OnlineManager};


#[turbo::game]
struct GameState {
    
    // Core managers for turbecs

    pub scene_manager : SceneManager,
    pub entity_manager : EntityManager,
    pub component_manager : ComponentManager,
    pub render_manager : Vec<Vec<usize>>,

    pub time_manager : TimeManager,
    pub input_manager : InputManager,

    // Community integrated work/managers

    pub particle_manager : ParticleManager,

    // Project specific elements

    pub collision_manager : CollisionManager,
    pub score_manager : ScoreManager,
    pub run_data : RunData,
    pub online_manager : OnlineManager,
    
    pub can_interact : bool,
    
}

impl GameState {
    fn new() -> Self {

        camera::set_xy(0, 0);

        Self {
            
            // Core managers for turbecs

            scene_manager : SceneManager::new(),
            entity_manager : EntityManager::new(),
            component_manager : ComponentManager::new(),
            render_manager : Vec::with_capacity(10),
            time_manager : TimeManager::new(),
            input_manager : InputManager::new(),
            
            // Community integrated work/managers

            particle_manager : ParticleManager::new(),

            // Project specific elements

            collision_manager : CollisionManager::new(),
            score_manager : ScoreManager::new(),
            run_data : RunData::new(),
            online_manager : OnlineManager::new(),

            can_interact : true, 

        }
    
    }

    fn update(&mut self) {

        // Checks the scene state before continuing

        self.check_scene_state();

        self.input_manager.update();

        self.time_manager.update();

        self.particle_manager.update();

        self.online_manager.update();

        // From here TurbECS will run it's lifetime functions!

        self.run_lifetime();

        self.handle_collisions();

        self.render_colliders();

        // Moved to within the render system
        // self.particle_manager.draw();
        
        self.input_manager.render();
        self.time_manager.render();

    }
}