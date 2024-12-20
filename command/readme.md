# command notes


The migration module contains the logic for migrating the world.

A migration is a sequence of steps that are executed in a specific order, based on the [`WorldDiff`] that is computed from the local and remote world.

Migrating a world can be sequenced as follows:

1. First the namespaces are synced.
2. Then, all the resources (Contract, Models, Events) are synced, which can consist of:
    - Declaring the classes.
    - Registering the resources.
    - Upgrading the resources.
3. Once resources are synced, the permissions are synced. Permissions can be in different states:
    - For newly registered resources, the permissions are applied.
    - For existing resources, the permissions are compared to the onchain state and the necessary changes are applied.
4. All contracts that are not initialized are initialized, since permissions are applied, initialization of contracts can mutate resources.