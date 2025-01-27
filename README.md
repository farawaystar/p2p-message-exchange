**Simple P2P Network for Message Exchange using rust-libp2p**

This is an implementation of a private, decentralized topic subscriber/chat network using libp2p (the same tech behind IPFS). Use this to spin multiple nodes, and make the nodes subscribe to a chosen topic, and listen & exchange info between them. The project has elements of a secure group chat, or a blockchain transaction exchanger or simply a network diagnostics tool! Here’s the vibe:

### **What It Does**
- **Chat with peers**: Type messages in a terminal, and it get's broadcasted to all peers in the network.
- **Identify sender/receiver**: tracks the peer_id of the sender and receiver, as well which node has "dialled" into which node.
- **Monitor network**: pings to see if peers are online and measures connection speeds.
- 🔒 **Private networks**: (in-progress) Use a `swarm.key` file (like a secret password) to lock down who can join.
- 📡 **Auto-discovery**: (in-progress) Nodes find each other using IPFS-style addresses (`/ip4/.../tcp/...`).

- **Configurable Params**: Key settings such as subscriber topic, ping frequeny, some cli formatting are configurable via `config.toml`. So no recompiling needed!

### **What’s “IPFS” About It?**
- It borrows **IPFS’s private network setup** (the `swarm.key` file and `IPFS_PATH` detection).
- Uses IPFS-compatible addresses for peers (though it’s *not actually IPFS*—no file sharing/storage implemented, yet..).

---

### **Quick Start**
1. **Clone & Build**  

### Build & Run
```bash
# Clone repo
git clone https://github.com/farawaystar/p2p-message-exchange.git
cd p2p-message-exchange
 ```

```bash
# Build repo
cargo build
```

2. **Run Nodes**  
   - First node: `cargo run`  
   - Second node: `cargo run /ip4/127.0.0.1/tcp/<PORT_FROM_FIRST_NODE>`
   - Open as many nodes as you want, one terminal per node.

3. **Chat Away**  
   Type messages in any terminal. They’ll show up in all!

4. **Shut down nodes**
   - Press ctrl+c

5. **handle node dependency**
   - Just make sure to handle which node is listening to which. If say node 3 is listening to node 2, then if node 2 shuts down, then node 3 goes out of order.
   - If all the nodes are listening to the genesis node, then any interruption in other nodes, will not affect any other node. But if genesis node shuts down, no other node will work. 

---

### Example output
```text
Got Message [14:32:45.782] 📬
Peer: 12D3KooWRSEux5mRkpUbpLv6E21RjAm9wzW3aVxLKJ17FND6ZQhX
Message: Hello from node 1!
ID: 1220a8a4c
▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
```

### Example Node startup log
```text
Using chat topic: transaction
Subscribing to Topic { topic: "transaction", phantom_data: PhantomData<libp2p_gossipsub::topic::IdentityHash> }
Dialed "/ip4/127.0.0.1/tcp/58173"
Listening on /ip4/127.0.0.1/tcp/58271
Listening on /ip4/192.xxx.xxx.xx/tcp/58271
```

### Example event listening log
```text
identify: Sent { peer_id: PeerId("12D3KooWDvKVDrrQ...") }
identify: Received { peer_id: PeerId("12D3KooWDvKVDrrQ..."), info: Info { public_key: PublicKey { publickey: Ed25519(PublicKey(compressed): 3cf5335f954525bbd1...) }, protocol_version: "/ipfs/0.1.0", agent_version: "rust-libp2p/0.44.2", listen_addrs: [/ip4/192.xxx.xxx.xx/tcp/58173, /ip4/127.0.0.1/tcp/58173], protocols: ["/meshsub/1.1.0", "/ipfs/id/1.0.0", "/ipfs/ping/1.0.0", "/ipfs/id/push/1.0.0", "/meshsub/1.0.0"], observed_addr: /ip4/127.0.0.1/tcp/58272 } }
ping: rtt to 12D3KooWDvKVDrrQ... is 0 ms
```

For a **private network**, just add a `swarm.key` to `~/.ipfs/` and restart the nodes. Boom—locked down.

---

## License

MIT License - See [LICENSE](https://github.com/libp2p/rust-libp2p/blob/master/LICENSE) for details

## Acknowledgments

- [libp2p](https://github.com/libp2p/rust-libp2p/tree/master) core team
- Parity Technologies
- Protocol Labs
- Tokio maintainers
- [Turbin3](https://turbin3.com) Advanced SVM cohort (Q1 2025) deep dive into Solana Validator clients.


## More Info 
More info @ [rust-libp2p](https://github.com/libp2p/rust-libp2p/tree/master/examples/ipfs-private)

Code away!
