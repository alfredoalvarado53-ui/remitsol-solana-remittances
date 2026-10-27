# 📄 RemitSol Pitch Deck (Two-Page Institutional Brief)

---

## PAGE 1: The Broken Web2 Model & The Solana Revolution

### 1. The Market Opportunity & Pain Points
- **Addressable Market:** $65.4 Billion USD annually flowing through the US-to-Mexico remittance corridor (the single largest bilateral remittance corridor in the world).
- **The Web2 Status Quo:**
  - **Exorbitant Rake:** Western Union, MoneyGram, and traditional wire providers extract an average of **6.2%** across wire fees and skewed FX rate spreads.
  - **Latency:** 1–3 business days for funds to clear international correspondent banks.
  - **Physical Insecurity:** Recipients must physically visit retail agent kiosks to collect cash, exposing them to targeted extortion.
  - **Economic Drain:** More than $3.9 Billion is siphoned away annually from families living near the poverty line.

### 2. The Solution: RemitSol on Solana
RemitSol transforms cross-border liquidity using Solana's high-throughput, low-fee architecture:
- **Instant USDC Settlement:** 400ms block finality replaces multi-day banking delays.
- **Zero Surcharge Core Rails:** Network cost is fractions of a cent ($0.00025), allowing RemitSol to charge an ultra-competitive flat 0.2% fee (saving families >95% in fees).
- **Programmable Streaming Remittances:** Rather than sending lump sums that risk misappropriation, senders can initiate automated continuous or scheduled streaming payouts (e.g. $50/week) directly from an onchain smart contract escrow.

---

## PAGE 2: Architecture, Tokenomics & Go-To-Market

### 3. Technical Architecture
```
[US Sender (USD)]
       │
       ▼ (Fiat On-Ramp / Solana Pay)
[RemitSol Escrow Contract (Solana)] ◄─── Time-Locked / Streaming Logic
       │
       ▼ (400ms Instant Settlement)
[Recipient Wallet (USDC SPL Token)]
       │
       ▼ (Regulated Local Off-Ramp API)
[Mexican SPEI Bank Transfer (MXN)] ➔ [Family Bank Account / Debit Card]
```
- **Non-Custodial Escrow:** Funds are deposited into program-derived addresses (PDAs) with multi-signature or emergency timelock recovery.
- **Micro-Allowances:** Program logic handles automated linear unlock schedules without ongoing gas overhead.

### 4. Business Model & Unit Economics
- **Revenue Stream:** 0.20% flat platform fee on settled volume + 0.10% spread on automated fiat off-ramping.
- **Conservative Year 1 Target:** Capturing just 0.15% market share of the corridor ($100M annual volume) yields **$300,000 USD** in gross recurring software revenue with near-zero infrastructure overhead.

### 5. Regulatory & Compliance Roadmap (Mexico & US)
- **Non-Custodial Protocol:** Smart contracts do not take custody of funds, operating strictly as open-source liquidity routing logic.
- **US FinCEN & Mexican FinTech Law Conformance:** Gateway partners handle BSA/AML and KYC verification at the fiat on/off-ramp boundary (Bitso/STP in Mexico, Circle in the US).
