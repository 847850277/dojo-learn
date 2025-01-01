use starknet::ContractAddress;

#[derive(Drop, Serde, Debug)]
#[dojo::model]
pub struct Container {
    #[key]
    pub game_id: u64,
    // game status 0: created, 1: joined, 2: finished
    pub status: u8,
    pub creator: ContractAddress,
    pub last_move_player: ContractAddress,
    //pub grids: Array<Item>,
}
