# OnionEarn 🧅
Instant Escrow for Farmers  
A Stellar-powered payment solution where Pangasinan onion farmers receive instant USDC payouts via Soroban escrow contracts, eliminating weeks of debt-inducing delays.

---

## Overview
OnionEarn is a mobile-first Web3 platform built during the Stellar Bootcamp that empowers smallholder farmers in Pangasinan to break free from exploitative payment cycles. Traders lock USDC into a Soroban escrow contract, and once delivery is confirmed, funds are instantly released to the farmer’s wallet. With Stellar’s low fees, even ₱500 onion deliveries become viable micro‑escrows — something traditional banks cannot support. OnionEarn demonstrates how blockchain can solve real agricultural cash flow problems with immediate impact.

---

## Problem
Meet Mang Jose, 52. Pangasinan.  
He harvests sibuyas (onions) and delivers them to a trader. Payment takes 2–4 weeks. In the meantime, he borrows money at high interest just to afford seeds and fertilizer for the next cycle. His ₱8,000–₱20,000 harvest shrinks under debt, and reinvestment becomes impossible.  
The real problem isn’t productivity. It’s liquidity. Farmers produce, but delayed payments trap them in debt cycles that limit growth and sustainability.

---

## Solution
OnionEarn makes Mang Jose financially secure. A trader locks USDC into a Soroban escrow contract. Once delivery is confirmed in the app, funds are instantly released to the farmer’s wallet. Farmers can cash out via their cooperative anchor to PHP, ensuring adoption without new intermediaries. OnionEarn doesn’t ask farmers to wait. It gives them immediate liquidity, trust, and reinvestment power.

---

## Stellar Features Used

| Feature | Purpose |
|---------|---------|
| Soroban Smart Contracts | Escrow logic for delivery confirmation and fund release |
| USDC Transfers | Real on-chain payments between traders and farmers |
| Trustlines | Farmers opt-in to receive USDC securely |

---

## Project Features (MVP — First Demo Flow)

| # | Feature | What It Does |
|---|---------|--------------|
| 1 | Escrow Creation | Trader taps “Pay Farmer” → USDC locked in contract |
| 2 | Delivery Confirmation | Trader taps “Confirm Delivery” → funds released instantly |
| 3 | Micro‑Escrow Support | Works even for ₱500 onion deliveries |
| 4 | Farmer Wallet | Farmer receives USDC directly in Stellar wallet |
| 5 | Cooperative Cash‑Out | Farmers cash out USDC to PHP via local cooperative anchor |

---

## Smart-Contract-Style Logic (Win-Win System)
Everyone wins:  
- **Farmers win**: Immediate liquidity, no debt cycles, reinvest faster.  
- **Traders win**: Guaranteed delivery backed by escrow, stronger trust with farmers.  
- **System wins**: Transparent, low‑fee, scalable payments recorded permanently on Stellar.

---

## Target Users
- Smallholder onion farmers in Pangasinan earning ₱8,000–₱20,000 per harvest.  
- Local onion traders / SMEs who need reliable, instant settlement.  
- Cooperatives acting as cash‑out anchors for USDC → PHP conversion.

---

## 🛠 Prerequisites
- Rust  
- Soroban SDK  
- Soroban CLI  
- Stellar Testnet account with USDC trustline

---

## Project Structure
OnionEarn/
├── src/
│   ├── lib.rs        # Escrow contract logic
│   └── test.rs       # Unit tests (5 passing)
├── Cargo.toml
├── README.md
└── target/

---

## Screens

| Screen | Description |
|--------|-------------|
| Landing | OnionEarn splash with Stellar badge and farmer payment CTA |
| Escrow Flow | Trader taps “Pay Farmer” → escrow created |
| Delivery Confirm | Trader taps “Confirm Delivery” → funds released |
| Farmer Wallet | Farmer sees USDC instantly credited |
| Cooperative Cash‑Out | Farmers cash out USDC to PHP via cooperative anchor |

---

## Smart Contract
- **Network**: Stellar Testnet  
- **SDK**: Soroban SDK 21.0 (Rust)  
- **CLI**: Soroban CLI v25.2.0  

**Contract Functions**

| Function | Description |
|----------|-------------|
| create_escrow | Trader locks USDC into contract |
| confirm_delivery | Trader confirms delivery |
| release_funds | Contract releases USDC to farmer |
| get_escrow | Read escrow details |

---

## Why This Wins
- **Real impact**: Farmers break debt cycles with instant liquidity.  
- **Transparent**: Escrow logic on Soroban ensures trust and verifiability.  
- **Micro‑escrow viability**: ₱500 onion deliveries become financially possible.  
- **Local adoption**: Cooperative anchors ensure farmers can cash out easily.

---

## Future Recommendations
- Full mobile app integration with Stellar wallets.  
- Cooperative anchor partnerships for USDC → PHP conversion.  
- Expansion to other crops (rice, garlic, mango).  
- DAO‑style governance for farmer cooperatives.

---

## License
MIT License — Built with 🧅 for Pangasinan farmers and financial inclusion.

---

## Built For
Stellar Hackathon — Build on Stellar: Smart Contract Track  
“Pay farmers instantly. Break debt cycles. Build trust with OnionEarn.”
