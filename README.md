![Loyalty Tokens Banner](https://raw.githubusercontent.com/ashifaeram/my-stellar-project-Loyalty-Tokens/5cf93abeb4cf66f5ee785fdbad3537d3898ae648/stellar-explorer)

# ⭐ Loyalty Tokens (Soroban Smart Contract)

## 📌 Project Description

Loyalty Tokens is a simple smart contract built using Soroban on the Stellar network. It enables businesses, apps, or platforms to reward users with digital loyalty points based on engagement, purchases, or participation.

These tokens are stored on-chain and can be transferred securely between users.

---

## ⚙️ What It Does

* Initializes a contract with an admin account
* Allows the admin to mint loyalty tokens
* Enables users to transfer tokens to others
* Provides a way to check token balances

---

## 🚀 Features

* 🔐 Admin-controlled token minting
* 💸 Peer-to-peer token transfers
* 📊 On-chain balance tracking
* ⚡ Lightweight and efficient design
* 🧩 Built with Soroban SDK (v25+)

---

## 🧪 Example Use Cases

* E-commerce reward programs
* Gaming point systems
* Community engagement rewards
* Event participation incentives

---

## 🛠️ Tech Stack

* **Rust**
* **Soroban SDK (v25+)**
* **Stellar Blockchain**

---

## 📜 Smart Contract Functions

### `initialize(admin: Address)`

Initializes the contract with an admin. Can only be called once.

### `mint(to: Address, amount: i128)`

Allows the admin to mint loyalty tokens to a user.

### `transfer(from: Address, to: Address, amount: i128)`

Transfers tokens from one user to another.

### `balance(user: Address) -> i128`

Returns the token balance of a user.

---

## 🔗 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CB7H5ME72LGQX4GQVLLEUU43Y2WACLQYL4GJ4SRS62AFBUKELA4KS3SL

---

## 📦 Future Improvements

* ⏳ Token expiration logic
* 🎁 Reward redemption system
* 🏆 Tier-based loyalty levels
* 🪙 Standardized token interface (like ERC20 equivalent)

---

MIT License
