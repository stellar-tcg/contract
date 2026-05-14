On-Chain Trading Card Game (TCG) on Stellar

An on-chain Trading Card Game (TCG) on Stellar is a blockchain-native card ecosystem where every card exists as a tokenized asset on-chain, players own their cards directly in wallets, and cards can be traded instantly using Stellar’s built-in decentralized exchange (DEX).

Think of it as a mix of:

Hearthstone
Magic: The Gathering Arena
NFT ownership
and a built-in decentralized marketplace powered by Stellar.
Core Idea

Each trading card is:

A Stellar asset
Issued by a trusted issuer account
Stored in users’ wallets
Tradable on Stellar’s native DEX
Usable inside gameplay

Instead of storing cards in a centralized database:

Ownership = blockchain wallet ownership
Trading = Stellar DEX trades
Scarcity = controlled token issuance
Marketplace = native blockchain infrastructure
Why Stellar Is Perfect for This
1. Native Asset Issuance

Stellar allows anyone to issue assets directly on-chain.

Example:

DRAGON_CARD
FIRE_WIZARD
LEGENDARY_KNIGHT

Each becomes a Stellar asset.

2. Built-In DEX

Unlike many blockchains, Stellar already has:

an integrated decentralized exchange
orderbooks
swaps
liquidity functionality

This means:

no need to build a marketplace from scratch
cards can trade peer-to-peer instantly
3. Extremely Low Fees

Typical Stellar fees:

fractions of a cent

Perfect for:

frequent card trading
pack openings
micro-transactions
tournaments
4. Fast Transactions

Settlement time:

~3–5 seconds

This is critical for gaming UX.

5. Easy Wallet Integration

Users can connect:

Freighter Wallet
LOBSTR Wallet
Albedo Wallet

This gives players direct ownership.

High-Level Architecture
                    ┌──────────────────────┐
                    │     Frontend App     │
                    │ React / Next.js      │
                    └──────────┬───────────┘
                               │
                               │ Stellar SDK
                               ▼
                ┌───────────────────────────┐
                │      Stellar Network      │
                │                           │
                │  - Asset Issuer Accounts  │
                │  - Native DEX             │
                │  - Trustlines             │
                │  - Payments               │
                │  - Soroban Smart Contracts│
                └──────────┬────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌─────────────┐   ┌────────────────┐   ┌─────────────┐
│ Game Server │   │ Metadata Store │   │ Match Engine│
│ Matchmaking │   │ IPFS/Arweave   │   │ Battle Logic│
└─────────────┘   └────────────────┘   └─────────────┘
How Cards Work on Stellar
Card = Stellar Asset

Example:

Card	Asset Code
Fire Dragon	FIRDRAGON
Shadow Mage	SHADMAGE
Ice Titan	ICETITAN

Each asset has:

asset code
issuer address
metadata
rarity
stats
Asset Model

Stellar assets use:

ASSET_CODE:ISSUER_ADDRESS

Example:

FIRDRAGON:GABCD12345...
Card Metadata Structure

The blockchain stores ownership.

Detailed card data lives off-chain:

{
  "name": "Fire Dragon",
  "rarity": "Legendary",
  "attack": 95,
  "defense": 70,
  "mana": 8,
  "description": "Burns enemy rows",
  "image": "ipfs://...",
  "animation": "ipfs://..."
}

Usually stored on:

IPFS
Arweave
centralized CDN
Trustline System

On Stellar, users must trust assets before holding them.

Workflow:

User creates wallet
User adds trustline to card issuer
Cards can now be received

Example:

Trust FIRECARD asset

This prevents spam assets.

Marketplace / DEX Trading

This is where Stellar becomes extremely powerful.

Native DEX Flow

Player A:

sells FIRE_DRAGON

Player B:

buys FIRE_DRAGON using XLM or USDC

All handled on-chain.

Trading Flow
Player A creates sell order
        ↓
Stellar DEX orderbook
        ↓
Player B matches order
        ↓
Ownership transferred instantly
Card Packs

Booster packs can be implemented using:

random minting
probability tables
smart contracts
loot systems

Example:

Common: 70%
Rare: 20%
Epic: 8%
Legendary: 2%
Soroban Smart Contracts

Soroban enables advanced game mechanics.

Use Soroban for:

crafting
tournaments
staking
rewards
pack openings
quests
PvP logic
seasonal events
Smart Contract Examples
Pack Opening Contract
pub fn open_pack(user: Address) {
    let random = pseudo_random();

    if random < 2 {
        mint_legendary(user);
    } else if random < 10 {
        mint_epic(user);
    }
}
Battle Mechanics

There are two approaches.

Option 1: Off-Chain Gameplay (Recommended)

Blockchain handles:

ownership
rewards
trading

Game server handles:

battles
animations
matchmaking

Pros:

fast
scalable
cheap

Most realistic approach.

Option 2: Fully On-Chain Battles

Every move becomes blockchain state.

Pros:

maximum transparency

Cons:

expensive
slower
harder UX
Recommended Hybrid Architecture

Best approach:

Feature	On-Chain	Off-Chain
Card ownership	✅	
Trading	✅	
Rewards	✅	
Packs	✅	
Battles		✅
Matchmaking		✅
Animations		✅
Economy Design
Currency System

Possible currencies:

XLM
in-game stablecoin
GOLD token
tournament token
Revenue Model
1. Pack Sales

Sell:

starter packs
seasonal packs
legendary packs
2. Marketplace Fees

Take:

1–3% trading fee
3. Tournament Entry Fees

Players pay:

XLM
USDC
native token
4. Cosmetic NFTs

Sell:

skins
emotes
animated cards
boards
Game Modes
PvP Ranked

Competitive ranked ladder.

Casual

Unranked play.

Tournament Mode

Bracket-based events.

Adventure Mode

PvE campaigns.

Guild Wars

Clan-based battles.

Full Technical Stack
Frontend
Recommended
Technology	Purpose
React / Next.js	Web app
Tailwind CSS	Styling
Phaser.js	2D game engine
Unity WebGL	Advanced gameplay
Stellar SDK	Blockchain interaction
Backend
Technology	Purpose
Node.js	API
NestJS	Scalable backend
PostgreSQL	Game data
Redis	Matchmaking/cache
WebSocket	Real-time battles
Blockchain Layer
Component	Purpose
Stellar SDK	Transactions
Soroban	Smart contracts
Horizon API	Blockchain indexing
RPC Server	Soroban interaction
Storage Layer
Technology	Purpose
IPFS	Metadata
Arweave	Permanent assets
Cloudflare R2	Images/videos
Suggested Project Structure
Monorepo Structure
stellar-tcg/
│
├── apps/
│   ├── web-client/
│   ├── game-client/
│   ├── admin-dashboard/
│   └── backend-api/
│
├── packages/
│   ├── stellar-sdk-wrapper/
│   ├── shared-types/
│   ├── ui-components/
│   ├── gameplay-engine/
│   └── blockchain-utils/
│
├── contracts/
│   ├── pack-opening/
│   ├── rewards/
│   ├── tournament/
│   └── crafting/
│
├── metadata/
│   ├── cards/
│   ├── animations/
│   └── seasons/
│
├── infrastructure/
│   ├── docker/
│   ├── kubernetes/
│   └── nginx/
│
└── docs/
Detailed Folder Breakdown
apps/web-client

Handles:

marketplace
inventory
wallet connection
leaderboards

Tech:

Next.js
Tailwind
Stellar SDK
apps/game-client

Handles:

gameplay
animations
battles

Tech:

Phaser.js or Unity
apps/backend-api

Handles:

matchmaking
ranking
user profiles
analytics

Tech:

NestJS
PostgreSQL
Redis
contracts/

Soroban smart contracts.

Example:

card minting
rewards
tournaments
staking
packages/gameplay-engine

Core gameplay logic:

battle system
mana system
turn engine
AI logic
Database Design
Players Table
CREATE TABLE players (
    id UUID PRIMARY KEY,
    wallet_address TEXT UNIQUE,
    username TEXT,
    elo_rating INT,
    created_at TIMESTAMP
);
Cards Table
CREATE TABLE cards (
    id UUID PRIMARY KEY,
    asset_code TEXT,
    issuer TEXT,
    rarity TEXT,
    metadata_url TEXT
);
Matches Table
CREATE TABLE matches (
    id UUID PRIMARY KEY,
    player1 UUID,
    player2 UUID,
    winner UUID,
    replay_url TEXT
);
Gameplay Logic
Turn System
Start Turn
   ↓
Draw Card
   ↓
Gain Mana
   ↓
Play Card
   ↓
Attack
   ↓
End Turn
Deck Building

Players:

own cards on-chain
create off-chain decks

Deck references:

blockchain ownership verification
Anti-Cheat Design
Important

Never trust frontend clients.

Server validates:

moves
mana costs
battle outcomes
Scalability Design
Use Stellar for Settlement Only

Do NOT put:

animations
battle logic
real-time state

fully on-chain.

Hybrid systems scale much better.

Seasonal Model

Each season introduces:

new cards
balance patches
tournaments
battle passes

Example:

Season 1: Elemental Wars
Season 2: Shadow Rebellion
NFT vs Stellar Asset
Recommended:

Use Stellar assets rather than traditional NFTs.

Why?

cheaper
more liquid
native DEX support
easier trading
Possible Advanced Features
1. Card Fusion

Combine cards to create stronger variants.

2. DAO Governance

Use governance tokens for:

voting
balancing
tournaments
3. AI Opponents

AI-generated NPCs.

4. Cross-Game Assets

Cards usable across multiple games.

Security Considerations
Protect Issuer Keys

Critical:

use multisig
hardware wallets
cold storage
Prevent Infinite Minting

Mint authority controls are essential.

Validate Marketplace Trades

Prevent:

fake assets
scam collections
Development Phases
Phase 1 — MVP

Build:

wallet login
card issuance
inventory
DEX trading
basic gameplay
Phase 2 — Multiplayer

Add:

matchmaking
ranking
tournaments
Phase 3 — Advanced Economy

Add:

staking
crafting
guilds
Phase 4 — Mobile + Scaling

Add:

mobile app
advanced analytics
cross-chain support
Example User Flow
User connects wallet
        ↓
Receives starter cards
        ↓
Builds deck
        ↓
Plays battles
        ↓
Wins rewards
        ↓
Trades cards on Stellar DEX
        ↓
Buys/sells rare cards
Why This Project Has Huge Potential
Web2 TCGs have a major weakness:

Players do NOT truly own cards.

With Stellar:

users own assets directly
open economy
instant liquidity
real marketplace

This creates:

player-driven economies
real digital ownership
interoperable assets
Recommended Tech Stack Summary
Layer	Recommendation
Blockchain	Stellar + Soroban
Frontend	Next.js
Gameplay	Phaser.js
Backend	NestJS
Database	PostgreSQL
Cache	Redis
Smart Contracts	Rust (Soroban)
Storage	IPFS + Arweave
Realtime	WebSockets
Recommended MVP Features
Must-Have
Wallet authentication
Card issuance
Inventory system
Deck builder
Marketplace
PvP battles
Rewards
Leaderboards
Future Expansion Ideas
Mobile app
AI-generated cards
eSports tournaments
Creator-made cards
Cross-chain bridging
Streaming integrations
Guild economies
Card rentals
Scholarship systems