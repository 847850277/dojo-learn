use dojo_starter::models::{Players, Position};

// define the interface
#[starknet::interface]
trait IActions<T> {
    fn test_1(ref self: T);
    fn test_2(ref self: T);
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions};
    use starknet::{ContractAddress, get_caller_address};
    use dojo_starter::models::{Players,Position};

    use dojo::model::{ModelStorage, ModelValueStorage};
    use dojo::event::EventStorage;

    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {

        fn test_1(ref self: ContractState) {
            // Get the default world.
            let mut world = self.world_default();
            // Get the address of the current caller, possibly the player's address.
            let player = get_caller_address();
            let position_one = Position { player, x: -1, y: 1, name: 'A' };
            let position_two = Position { player, x: 1, y: 1, name: 'D' };
            // init player
            let players_one = Players {
                player,
                position_one,
                position_two,
                can_move: false,
            };
            world.write_model(@players_one);

        }

        fn test_2(ref self: ContractState) {

            // Get the default world.
            let mut world = self.world_default();
            // Get the address of the current caller, possibly the player's address.
            let player = get_caller_address();
            let players_one: Players = world.read_model(player);
            // init player
            let players_two= Players {
                player,
                position_one: players_one.position_one,
                position_two: players_one.position_two,
                can_move: false,
            };
            world.write_model(@players_two);

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

