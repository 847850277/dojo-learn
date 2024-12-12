# Create Burner

创建账户

源码地址为
https://github.com/dojoengine/dojo.js/blob/main/packages/create-burner/src/manager/burnerManager.ts#L313



+ 按钮为调用合约的spawn方法


上、下、坐、有为调用合约的move方法

源码地址为
https://github.com/dojoengine/dojo.js/blob/main/packages/core/src/provider/DojoProvider.ts#L185C18-L185C25
逻辑为，转化为合约的参数，然后调用合约的方法（不同命名空间下，可能合约地址不一样）


entity_id 表示的是什么呢？
