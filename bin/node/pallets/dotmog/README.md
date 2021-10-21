DOTMog module for the game www.dotmog.com

DOT Mog, a Substrate based Unity3D Game - Immerse yourself in the futuristic universe of the Mogwais. Lead them to glory and honor. Experience epic and thrilling adventures & duels. Explore the infinite universe & mysteries.

As probably the first substrate based Unity3D game using our open-sourced SubstrateNetApi, we bring the functionalities and benefits of this promising technology to a broad audience in a playful and immersive way.

License: APGL 3.0

Current Types needed:

{
  "Address": "MultiAddress",
  "LookupSource": "MultiAddress",
  "GameEventType": {
    "_enum": [
      "Default",
      "Hatch"
    ]
  },
  "GameEvent": {
    "id": "H256",
    "begin": "BlockNumber",
    "duration": "u16",
    "event_type": "GameEventType",
    "hashes": "Vec<H256>",
    "value": "u64"
  },
  "RarityType": {
    "_enum": [
      "Minor",
      "Normal",
      "Rare",
      "Epic",
      "Legendary"
    ]
  },
  "MogwaiStruct": {
    "id": "H256",
    "dna": "H256",
    "genesis": "BlockNumber",
    "price": "Balance",
    "gen": "u32",
    "rarity":"RarityType"
  },
  "MogwaiBios": {
    "mogwai_id": "Hash",
    "state": "u32",
    "metaxy": "Vec<[u8;16]>",
    "intrinsic": "Balance",
    "level": "u8",
    "phases": "Vec<BlockNumber>",
    "adaptations": "Vec<Hash>"
  },
  "MogwaicoinAddress": {
    "address": "Vec<u8>",
    "account": "AccountId",
    "signature": "Vec<u8>",
    "state": "ClaimState",
    "balance": "Balance"
  },
  "ClaimState": {
    "_enum": [
      "None",
      "Registred",
      "Verified",
      "Secured",
      "Processed",
      "Holded",
      "Failed",
      "Cancelled"
    ]
  }
}



