# 先部署世界合约

- 世界合约
- 位置合约
- 移动等分别是不同的合约


sozo build --profile release

sozo migrate --profile release

世界合约的class_hash下声明的合约 https://sepolia.voyager.online/class/0x079d9ce84b97bcc2a631996c3100d57966fc2f5b061fb1ec4dfd0040976bcac6#contracts

当前链接合约有完整的交互demo https://sepolia.voyager.online/contract/0x06f08a7168b67d269b393a2f4eff772481ec260702397088d414f318621bbe8c

我当前用的世界合约报错的，可以不看 https://sepolia.voyager.online/contract/0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf

我账户2当前用的世界合约 https://sepolia.voyager.online/contract/0x024b4ad4381306818a1d4cf65320f049415fa2ed328b11fa4d4552b3f6421b52

参数为 1234，不正确的部署的世界合约地址 `0x03f8eb232e97efff1ba1e6f74d5946e6c6749b199486336fec4fd77734dcac58`

参数为 0x149397d4ee0e1895700b62c74d73a1314f53924b1f76d9cd2cf4d865c18abd0 正确的 部署的世界合约地址 `0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf`

世界的源码 https://github.com/dojoengine/dojo/blob/main/crates/dojo/core/src/world/world_contract.cairo

阅读世界的源码含义 初始化参数为世界合约的class_hash


注册命名空间

注册模型，调用注册模型方法，在model合约内部会自动部署该模型合约，自己部署和调用注册模型都能把model部署到链上。

注册事件，调用注册事件方法，在注册事件合约内部会自动部署该事件合约，自己部署和调用注册事件部署到链上。

注册合约，调用注册合约方法，在注册合约合约内部会自动部署该合约合约，自己部署和调用注册合约部署到链上。参数为合约的class_hash,已经salt


TODO

- 执行合约的spawn方法
  - 找到spawn需要的参数
  - 构建参数
  - 调用合约的spawn方法

在页面调用该合约失败

错误信息
```json
{
  "error": {
    "message": "Resource `0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf` is registered but not as `model`"
  }
}
```

pub fn resource_conflict(name: @ByteArray, expected_type: @ByteArray) -> ByteArray {
format!("Resource `{}` is registered but not as {}", name, expected_type)
}

先在页面成功调用该合约，discord去问问呢？


vue的example


调用init_contract?，好像也会报错误呢。



支线任务

cairo的ByteArray数据类型用法  https://github.com/cartridge-gg/cainome/blob/main/crates/cairo-serde/src/types/byte_array.rs#L130
和 byte_array.rs


在Rust中怎么获取链上的class_hash


client的项目调整为sepolia的项目


