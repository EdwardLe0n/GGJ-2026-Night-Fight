use turbo::*;
use turbo::os::server::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum PlayerGameRequestEnum {
    Offense,
    Agile,
    Wait,
    Mixed,

    TankMask,
    NimbleMask,
    AggroMask,

    Parry
}

#[turbo::serialize]
pub struct PlayerGameRequest {

    pub some_id : String,
    pub request_enum : PlayerGameRequestEnum,
    pub some_time : u64

}
impl PlayerGameRequest {

    pub fn new(some_id : String, some_req_enum : PlayerGameRequestEnum, some_time : u64) -> Self {

        return Self {
            some_id : some_id,
            request_enum : some_req_enum,
            some_time : some_time,
        };

    }

}

#[turbo::os::channel(program = "gameChannel", name = "playerRequest")]
pub struct PlayerGameRequestNotice;
impl ChannelHandler for PlayerGameRequestNotice {
    type Recv = PlayerGameRequest; // incoming from client
    type Send = PlayerGameRequest; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {
        Self::send(&data.some_id, PlayerGameRequest::new(user_id.to_string(), data.request_enum, data.some_time))
    }
}