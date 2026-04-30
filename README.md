# SorobanProject
Trustless buyer-seller escrow for safe peer-to-peer transactions on Stellar.

---

## Project Description
is a decentralized escrow platform built on Stellar that enables safe peer-to-peer transactions between buyers and sellers in informal marketplaces. In many Southeast Asian communities, users rely on platforms like Facebook Marketplace where payments are often sent upfront, exposing buyers to scams and non-delivery risks. EscrowCart solves this by allowing buyers to lock funds in a Soroban smart contract, ensuring that payment is only released once the buyer confirms receipt of the item. By leveraging Stellar’s fast, low-cost transactions and secure on-chain logic, EscrowCart removes the need for trust between parties and provides a simple, transparent way to conduct secure online transactions.

---

## Problem
A Facebook Marketplace buyer in Manila pays a seller upfront via GCash and risks losing ₱5,000 if the item is fake or never delivered.

---

## Project Vision
SorobanProject allows buyers to lock funds in a Soroban smart contract and only release them once the item is received, eliminating the need to trust unknown sellers.

---

## Timeline
- Day 1: Smart contract (escrow logic)
- Day 2: Testing + bug fixes
- Day 3: Basic UI + demo flow

---

## Stellar Features Used
- Soroban Smart Contracts (escrow logic)
- On-chain state storage (escrow tracking)

---

## Future Scope
SorobanProject aims to make informal e-commerce in Southeast Asia safer by removing trust from transactions.  
By leveraging Stellar’s fast and low-cost infrastructure, it enables secure payments between buyers and sellers who do not know each other.

---

## Prerequisites
- Rust (latest stable)
- Soroban CLI (v20+)
- Cargo

---

## Deployed Contract Details
[1] https://stellar.expert/explorer/testnet/tx/9540138263aaf7d32b307914294e1233e2b492431813ce071a375bef5e7b02e1
[2] https://lab.stellar.org/r/testnet/contract/CD3H6DMHHFMV2AYN2XKPSXUNHEFNOO22VQFJO42MKBHBDNUYTEFL6T7M

## Build
```bash
soroban contract build


