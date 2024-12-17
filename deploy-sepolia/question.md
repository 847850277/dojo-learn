I referred to dojo starter, deployed the world contract, and used the world contract's registrant to extract to sepolia
However, it is currently not possible to properly call registering and extract to generate a spawn for the contract address.
My world contract is:
```
https://sepolia.voyager.online/contract/0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf
```
The address after registering the contract is:
```
https://sepolia.voyager.online/contract/0x009486b97b51beb5d31909b2e07bcadce5edf3d248f39b88f627fb5a78337eb5#writeContract
```
The error message for calling spawn is:
```
Transaction execution has failed:
0 Error in the called contract (contract address: 0x0156c66218b0836d8d49096529bba0e750eb36377e5a98f99a70ee997296d36a, class hash: 0x036078334509b514626504edc9fb252328d1a240e4e948bef8d0c08dff45927f, selector: 0x015d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad):
Error at pc=0:29965:
Cairo traceback (most recent call last):
Unknown location (pc=0:351)
Unknown location (pc=0:9587)
1:  Error in the called contract (contract address: 0x009486b97b51beb5d31909b2e07bcadce5edf3d248f39b88f627fb5a78337eb5, class hash: 0x07ca3f23baf5a81c10d7ffafc5a3925eddc099aed69f0165d853aee84da7b9f2, selector: 0x0217c73ea9ef26581623f20edd45571c1d024612b70d0af3e0842c5b0dc253cd):
Error at pc=0:6004:
Cairo traceback (most recent call last):
Unknown location (pc=0:177)
Unknown location (pc=0:1184)
Unknown location (pc=0:2862)
2:  Error in the called contract (contract address: 0x033558685a3ca425fe6ec072efe425d172533927f6dacaa6865f93a383d9ffdf, class hash: 0x079d9ce84b97bcc2a631996c3100d57966fc2f5b061fb1ec4dfd0040976bcac6, selector: 0x00d065c837ba98927ca43d2e15d8e840ca8e67646b8e00108f1b55d18a75d80f):
Execution failed. Failure reason: "Resource `1209484786645723696253228666661161140119639292882531632261437728265869909339` is registered but not as model".