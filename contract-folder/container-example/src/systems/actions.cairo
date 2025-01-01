use dojo_starter::models::{Container};

// define the interface
#[starknet::interface]
trait IActions<T> {
    fn test(ref self: T) -> u64;
    fn test_1(ref self: T, game_id: u64);
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions};
    use starknet::{ContractAddress, get_caller_address};
    use dojo_starter::models::{Container};

    use dojo::model::{ModelStorage, ModelValueStorage};
    use dojo::event::EventStorage;


    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        fn test(ref self: ContractState) -> u64{
            // Get the default world.
            let mut world = self.world_default();
            // Get the address of the current caller, possibly the player's address.
            let player = get_caller_address();
            let game_id = 1;
            let status = 1;
            let container = Container {
                game_id,
                status,
                creator: player,
                last_move_player: player,
            };
            // init player
            world.write_model(@container);
            game_id
        }

        // Implementation of the move function for the ContractState struct.
        fn test_1(ref self: ContractState, game_id: u64) {
            // Get the default world.
            let mut world = self.world_default();
            // Get the address of the current caller, possibly the player's address.
            //let player = get_caller_address();
            let mut container: Container = world.read_model(game_id);
            assert(container.creator.is_non_zero(), 'is zero address');
            world.write_model(@container);


        }
    }

    #[generate_trait]
    impl InternalImpl of InternalTrait {
        /// Use the default namespace "dojo_starter". This function is handy since the ByteArray
        /// can't be const.
        fn world_default(self: @ContractState) -> dojo::world::WorldStorage {
            self.world(@"dojo_starter")
        }
    }
}

