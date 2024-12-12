import { createDojoConfig } from "@dojoengine/core";

//import manifest from "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/example-vite-react-sdk/contract/manifest_sepolia.json";
import manifest from "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/example-vite-react-sdk/contract/manifest_dev.json";

export const dojoConfig = createDojoConfig({
    manifest,
});
dojoConfig.toriiUrl = "http://localhost:8080";
// dojoConfig.rpcUrl = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7";

// 主账户私钥和地址
// dojoConfig.masterAddress="0x0156c66218B0836d8d49096529BBA0E750Eb36377E5a98F99A70ee997296D36a";
// dojoConfig.masterPrivateKey="0x025e5b9982a2c8e04cb477d7c71aec25e2043e4d52cb61604208e1939acfb8bf";