Example of generating Merkle root and proof for a [list of uint256](./vals.txt)

```shell
cargo run [file containing values to hash] [index of value to generate proof]

# example
cargo run vals.txt 1

# output
root 0x70fb6688449391615accc7470ee615c3cbf90ce815cab88b76ccee446eb24e5c
leaf 0x405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace
proof 0xb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6
proof 0x58dedfa8c8510aa7a44a262de0df204bc81f3b437741b6b63212d1173a876672
proof 0x1ae795286c5dd0cad1dd38aef9a3015558e9d303ec7474e5e59abc16a43abbd6
proof 0xe25aed4b6d352e943f1603e731916abe40d8986f0d71a855e57ddfde0ff24b8a
proof 0x2ff5fc6d0fcc5b8cd467ff2c66773d8fe79bb1c0cbe7f8c708b44678abf42289
true
```
