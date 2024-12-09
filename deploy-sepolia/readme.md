# 先部署世界合约

- 世界合约
- 位置合约
- 移动等分别是不同的合约


sozo build --profile release

sozo migrate --profile release

世界合约的class_hash下声明的合约 https://sepolia.voyager.online/class/0x079d9ce84b97bcc2a631996c3100d57966fc2f5b061fb1ec4dfd0040976bcac6#contracts



参数为 1234，不正确的部署的世界合约地址 `0x03f8eb232e97efff1ba1e6f74d5946e6c6749b199486336fec4fd77734dcac58`
参数为 0x149397d4ee0e1895700b62c74d73a1314f53924b1f76d9cd2cf4d865c18abd0 正确的 部署的世界合约地址 `0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf`

世界的源码 https://github.com/dojoengine/dojo/blob/main/crates/dojo/core/src/world/world_contract.cairo

阅读世界的源码含义 初始化参数为世界合约的class_hash


注册命名空间




支线任务

cairo的ByteArray数据类型用法  https://github.com/cartridge-gg/cainome/blob/main/crates/cairo-serde/src/types/byte_array.rs#L130
和 byte_array.rs



