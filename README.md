# Defluencer Project Website

The official project website.

Visit defluencer.eth on Ropsten testnet for the latest dev build.

## Requirements
- [Trunk](https://trunkrs.dev/#install)
- [IPFS](https://docs.ipfs.io/install/command-line/#command-line)

## Deloy

- ```trunk clean```
- ```trunk build --release```
- ```ipfs add --cid-version=1 -r www``` 
- (Optional) ```ipfs pin add <HASH_FROM_PREVIOUS_STEP>```
- (Optional) Update your [Ethereum Name Service](https://ens.domains/) domain's content hash with ipfs://<<HASH_FROM_PREVIOUS_STEP>>
- (Optional) Visit a gateway. https://<HASH_FROM_PREVIOUS_STEP>.ipfs.dweb.link
- Profit!

## Testing
### Setup 1
IPFS natively in Brave. (live streams won't work, cannot enable pubsub and IPNS is slow)
- Install [Brave](https://brave.com/) browser
- Go to brave://settings
- Enable IPFS companion then when asked enable IPFS
- Click companion extension icon then click My Node
- Go to settings and replace
```
"API": {
  "HTTPHeaders": {}
},
```
with this
```
"API": {
  "HTTPHeaders": {
    "Access-Control-Allow-Methods": [
      "PUT",
      "POST",
      "GET"
    ],
    "Access-Control-Allow-Origin": [
      "http://localhost:45005",
      "http://127.0.0.1:45005",
      "https://webui.ipfs.io",
      "http://<INSERT_CID_HERE>.ipfs.localhost:48084"
    ]
  }
},
```
- Replace <INSERT_CID_HERE> with the root CID.
- Restart browser

### Setup 2
IPFS + any browser
- [Install IPFS Desktop](https://docs.ipfs.io/install/ipfs-desktop/#ipfs-desktop)
- Right click on IPFS tray icon, under settings, check both Enable PubSub & Enable IPNS over PubSub.
- Allow CORS with these commands. (Replace <INSERT_CID_HERE> with root CID)
    - ```ipfs config --json API.HTTPHeaders.Access-Control-Allow-Methods '["GET", "POST", "PUT"]'```
    - ```ipfs config --json API.HTTPHeaders.Access-Control-Allow-Origin '["http://localhost:5001", "http://127.0.0.1:5001", "https://webui.ipfs.io", "http://<INSERT_CID_HERE>.ipfs.localhost:8080"]'```

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
