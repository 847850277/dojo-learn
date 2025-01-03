import { createDojoConfig } from "@dojoengine/core";

import manifest from "/Users/zhengpeng/Work/Source/Code/rust-code/dojo-learn/example-vite-react-sdk/contract/manifest_dev.json";
//import manifest from "/Users/zhengpeng/Source/Code/Rust-Code/Github/dojo-learn/example-vite-react-sdk/contract/manifest_dev.json";

export const dojoConfig = createDojoConfig({
    manifest,
});
dojoConfig.toriiUrl = "http://localhost:8080";
