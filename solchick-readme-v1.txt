ts-node ./metaplex/js/packages/cli/src/candy-machine-v1-cli.ts help upload

> Step1 upload
ts-node ./metaplex/js/packages/cli/src/candy-machine-v1-cli.ts upload ./metaplex/solchick-assets --env devnet --keypair D:\BlockChain\Solana\projects\keys\alice.json

>output
candy machine with publickey: 89CBeAbyoiFg9gDRcptsKE16MYmnfHnuuFGCpJFS2PTx

> Step2 check
ts-node ./metaplex/js/packages/cli/src/candy-machine-v1-cli.ts verify --keypair D:\BlockChain\Solana\projects\keys\alice.json

> Step3 Create
ts-node ./metaplex/js/packages/cli/src/candy-machine-v1-cli.ts create_candy_machine --env devnet --keypair D:\BlockChain\Solana\projects\keys\alice.json -p 1

> Step4: Mint
ts-node ./metaplex/js/packages/cli/src/candy-machine-v1-cli.ts mint_multiple_tokens -k --env devnet --keypair ./keys/keypair.json --number 3

mint_one_token 

ts-node ./metaplex/js/packages/cli/src/candy-machine-v1-cli.ts mint_one_token -k ./keys/keypair.json --env devnet

CUq8DRyvu9KTiRLj9s5HZNKvAbA9Zx2XjXKCdSnZBBPN

https://solscan.io/token/FGTSxd5R5VgN7gPNCjkXRVSHKAwNjgZhTEfjiVbBtQ4k?cluster=devnet

https://solscan.io/token/63L6tE72vcUoUnEXpgbEwVr7kBVm5dmoajsPT9BUMXBy?cluster=devnet

https://solscan.io/token/FPd8wTykwZiEyuu7EZZVcijPS4xcsUZKBs8DKaAYX1Ra?cluster=devnet

https://solscan.io/token/ENT1madCompWMc2P8rmTQ7QEhFg4GonEZk1hML9hD4UN

ts-node ./metaplex/js/packages/cli/src/candy-machine-cli.ts verify --keypair ./keys/keypair.json --env mainnet-beta