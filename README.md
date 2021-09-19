1.测试结果截图
![测试结果截图](./test_result.png)

2.合约编译截图
![合约编译截图](./contract_build.png)

3.canvas-ui 合约内容截图
![canvas-ui 合约内容截图](./contract_content.png)

4.dave 授权 alice 可以从 dave 账号中抽取 100。
```shell
    allowance
        |->caller:dave,
        |->spender:alice,
        |->value:100
```
![canvas-ui 设置授权](./set_allowance.png)

5.由于超出授权金额，所以 transfer_from 失败。
```shell
    transfer_from:
        Call from Account:alice
        |->dave - 105,
        |->bob + 105
        |-->{"err":"InsufficientApproval"}
```
![canvas-ui transfer_from 失败](./transfer_from_failed_when_insufficient_approval.png)

6.由于未超出授权金额，所以 transfer_from 成功。
```shell
    transfer_from:
        Call from Account:alice
        |->dave - 100,
        |->bob + 100
        |-->{"ok":[]}
```
![canvas-ui transfer_from 成功](./transfer_from_worked.png)