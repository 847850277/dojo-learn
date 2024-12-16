import {
    Component,
    ComponentValue,
    defineQuery,
    Entity,
    getComponentValue,
    Has,
    isComponentUpdate,
    Schema,
} from "@dojoengine/recs";
import { Account, CallData, ec, hash, RpcProvider, stark } from "starknet";
import { onUnmounted, reactive, ref, toRaw, watchEffect } from "vue";

export enum Direction {
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}

export function updatePositionWithDirection(
    direction: Direction,
    value: { vec: { x: number; y: number } }
) {
    switch (direction) {
        case Direction.Left:
            value.vec.x--;
            break;
        case Direction.Right:
            value.vec.x++;
            break;
        case Direction.Up:
            value.vec.y--;
            break;
        case Direction.Down:
            value.vec.y++;
            break;
        default:
            throw new Error("Invalid direction provided");
    }
    return value;
}

export const getAccount = async (nodeUrl: string) => {
    const provider = new RpcProvider({ nodeUrl });
    const localBurners: any = localStorage.getItem("burners") || "{}";
    const burners = JSON.parse(localBurners);
    let address = "";
    for (let key in burners) {
        if (burners[key].active) {
            address = key;
            break;
        }
    }
    if (!address) {
        return null;
    }
    const account = new Account(
        provider,
        address,
        burners[address].privateKey,
        "1"
    );
    return account;
};

//TODO 从已知的账户获取账户
export const createAccount = async ({
    nodeUrl,
    classHash,
}: {
    nodeUrl: string;
    classHash: string;
}) => {
    const provider = new RpcProvider({ nodeUrl: "https://starknet-sepolia.public.blastapi.io/rpc/v0_7" }); // only for starknet-devnet-rs
    console.log("provider",provider);
    //连接已经存在的账户
    const privateKey = "0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf";
    const address = "0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a";
    const account = new Account(provider, address, privateKey);
    // account get pub key
    //const pubKey = ec.keyFromPrivate(privateKey).getPublic();
    const pubKey = "Sd82dMc87BBHiDQyHRjackMbvjRzzQKTnw8eKgicSNE";


    const localBurners: any = localStorage.getItem("burners") || "{}";
    const burners = JSON.parse(localBurners);
    for (const key in burners) {
        burners[key].active = false;
    }
    burners[address] = {
        active: true,
        pubKey,
        privateKey,
    };
    localStorage.setItem("burners", JSON.stringify(burners));
    return account;
};

export function useEntityQuery(fragments: any[]) {
    const stableFragments = toRaw(fragments);

    const query = reactive(
        defineQuery(stableFragments, {
            runOnInit: true,
        })
    );

    const entitiesRef = ref<any>([]);

    const setEntities = (entities: any) => {
        entitiesRef.value = [...entities];
    };

    const subscription = query.update$.subscribe(() => {
        setEntities([...query.matching]);
    });

    watchEffect(() => {
        setEntities([...query.matching]);
    });

    onUnmounted(() => {
        subscription.unsubscribe();
    });

    return entitiesRef;
}

export function useComponentValue<S extends Schema>(
    component: Component<S>,
    entity: Entity | undefined,
    defaultValue?: ComponentValue<S>
) {
    const value = ref<any>(
        entity != null ? getComponentValue(component, entity) : undefined
    );

    value.value =
        entity != null ? getComponentValue(component, entity) : undefined;
    if (entity == null) return;
    const queryResult = defineQuery([Has(component)], { runOnInit: false });
    const subscription = queryResult.update$.subscribe((update) => {
        console.log(update);
        if (isComponentUpdate(update, component) && update.entity === entity) {
            const [nextValue] = update.value;
            value.value = nextValue;
        }
    });

    onUnmounted(() => {
        subscription.unsubscribe();
    });

    return value ?? defaultValue;
}
