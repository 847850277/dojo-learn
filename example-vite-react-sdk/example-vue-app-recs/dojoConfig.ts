import { createDojoConfig } from "@dojoengine/core";

//import manifest from "../contract/manifest_dev.json";
import manifest from "../contract/manifest_sepolia.json";

export const dojoConfig = createDojoConfig({
    manifest,
});

dojoConfig.toriiUrl = "http://localhost:8080";
dojoConfig.rpcUrl = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7";