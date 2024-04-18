# Rust for Blockchain Application Development

<a href="https://www.packtpub.com/product/rust-for-blockchain-application-development/9781837634644"><img src="https://content.packt.com/_/image/original/B19505/cover_image_large.jpg" alt="no-image" height="256px" align="right"></a>

This is the code repository for [Rust for Blockchain Application Development](https://www.packtpub.com/product/rust-for-blockchain-application-development/9781837634644), published by Packt.

**Learn to build decentralized applications on popular blockchain technologies using Rust**

## What is this book about?
This book helps you build your own blockchains and production-grade decentralized apps on blockchains like Ethereum, Solana, NEAR, and Polkadot. You’ll explore best practices, code, and assets that can be used for scaffolding multiple projects.

This book covers the following exciting features:
* Understand essential Rust concepts required to build blockchain
* Apply blockchain features such as nodes and p2 communication using Rust
* Understand and implement consensus in blockchain
* Build and deploy a dApp on Ethereum with the Foundry framework in Rust
* Develop and deploy a dApp on Solana and the NEAR protocol
* Build a custom blockchain using the Substrate framework by Polkadot

If you feel this book is for you, get your [copy](https://www.amazon.com/Rust-Blockchain-Application-Development-decentralized/dp/1837634645/ref=sr_1_1?crid=2C0TV7WOZK24V&dib=eyJ2IjoiMSJ9.VLeHyaoxnARCyNXrIwAZNwjCinJ6S_W4IBTPGuSYu8vny5FWRzKryHAN05m9vasHdvTixbNwvnjl6u3Z_GO4Q62EwWfe2K6VEP1hQQ7x5-5ZFRF85qNn-vhrjMJLMPYPtcUcjoVpn4WkSBdQXBkt2PFg2i1OPbqtsJguhKnj0xfWD9JsV9EYUYYoL27K5WdtmHwnpdVD1-frNBsMsB94ORqMoT7PBhzJzKihiRspj_s.kgXK_G44g3O5KxkRwwqaK11vGctCCQqqMwbVSHSeXR8&dib_tag=se&keywords=Rust+for+Blockchain+Application+Development&qid=1713187625&sprefix=rust+for+blockchain+application+development%2Caps%2C422&sr=8-1) today!
<a href="https://www.packtpub.com/?utm_source=github&utm_medium=banner&utm_campaign=GitHubBanner"><img src="https://raw.githubusercontent.com/PacktPublishing/GitHub/master/GitHub.png" 
alt="https://www.packtpub.com/" border="5" /></a>
## Instructions and Navigations
All of the code is organized into folders. For example, Chapter02.

The code will look like the following:
```
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}
```

**Following is what you need for this book:**
This Rust programming book is for blockchain developers interested in building dApps on popular blockchains using Rust. Blockchain architects wanting to save time required to go through documentation and understand each technology can also use this book as a quick-start guide. Experience in building applications on blockchain is required, and familiarity with Rust will be helpful but not necessary.

With the following software and hardware list you can run all code files present in the book (Chapter 1-11).
## Software and Hardware List
| Chapter | Software required | OS required |
| -------- | ------------------------------------ | ----------------------------------- |
| 1-11 | Rust 1.74.0 or higher | Windows, macOS, or Linux |
| 1-11 | Cargo | Windows, macOS, or Linux |

## Related products
* Blockchain Development for Finance Projects [[Packt]](https://www.packtpub.com/product/blockchain-development-for-finance-projects/9781838829094) [[Amazon]](https://www.amazon.com/Blockchain-Development-Finance-Projects-next-generation/dp/1838829091/ref=sr_1_1?dib=eyJ2IjoiMSJ9.eNf1c3_cfBN-Duwo7lGnHVOsZBPbYWQPejWA4cZlQOVakpAeRuCWiglOHEVqwnFFy29ex3QauzX5aS0UzHtwEpl2uzuEkwbxCqa03gTNf3aglKGHp2H-9TLMUUm7TuWuXmrhAuS_JwRQ47L-g0mWH3GWgh3HzAf8l-e-Y9nQKnPUFNUEHAZotIswjke5X4jwaqlJGPM6TPDm5tXcgBt8si1nAIBLQKySNOcm5MX8cPo.6dXUzgjUJbhmEKZUqMnXRFKRzmRn7zxkgoz5ue_f1bw&dib_tag=se&keywords=Blockchain+Development+for+Finance+Projects&qid=1713417756&sr=8-1)

* Securing Blockchain Networks like Ethereum and Hyperledger Fabric [[Packt]](https://www.packtpub.com/product/securing-blockchain-networks-like-ethereum-and-hyperledger-fabric/9781838646486) [[Amazon]](https://www.amazon.com/Securing-Blockchain-advanced-configurations-Hyperledger/dp/1838646485/ref=sr_1_1?dib=eyJ2IjoiMSJ9.KXbWqUeboIcuUC50Keis4-Fh8EHDdnTKIlRH7zVexmwWKnIxNQakYOF1ZDL1UG4RXGBRVVXn2SDwEJj3M7tyR223bXb1-f1sDw2_V33-TxgM1XiEde5Q6VtM1tf_LdTErVXvGP-iybgR5sFsvQMZipjVmvRHDq96jAkfvUAzghtQCbqZ-_MXTvnAmnAzQBDqQW0muj3ND3vEnyNJnkNtsvZvuT6U-HmHCUc2V1vHVuA.sUksLXECxbJ2z-NjRfTtHuDwukno1Pf3gEclDlRsnOk&dib_tag=se&keywords=Securing+Blockchain+Networks+like+Ethereum+and+Hyperledger+Fabric&qid=1713417830&sr=8-1)

## Get to Know the Author
**Akhil Sharma**
 is the Founder at Armur AI, a cybersecurity company that is backed by Techstars, Outlier Ventures, Aptos and is part of the Google AI startups cloud program.
Akhil teaches advanced engineering topics (Rust, GO, Blockchain, AI) on his Youtube channel and has mentored more than 200K engineers across platforms like Linkedin Learning, Udemy and Packt.
Being deeply involved with multiple Rust-based blockchain communities like Aptos, Solana and Polkadot inspired Akhil to write this book.
In his free time, Akhil likes to train in Jiu Jitsu, play the guitar or surf.
