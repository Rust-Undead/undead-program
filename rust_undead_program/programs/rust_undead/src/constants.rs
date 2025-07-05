pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const CONFIG: &[u8] = b"config";
pub const BATTLE: &[u8] = b"battleroom";
pub const USER_PROFILE: &[u8] = b"user_profile";
pub const USER_ACHIEVEMENT: &[u8] =
b"user_achievements";
pub const UNDEAD_WARRIOR: &[u8] = b"undead_warrior";
pub const LEADERBOARD: &[u8] = b"leaderboard";

// images suite 
// IPFS folder hashes
pub const GUARDIAN_FOLDER_HASH: &str = "bafybeieg4s45fshekdmtqssax4c2tw3ro5z6rmv4ka5dnit7x66f4tmsby";
pub const VALIDATOR_FOLDER_HASH: &str = "bafybeibs2qb55efetumvknddcbizjwg4kvzdnyojys6jtovclfodxhec2a";
pub const ORACLE_FOLDER_HASH: &str = "bafybeia3ukkyzpqo6sjtjxj3iqdtfnpyh5szffxdo3avov2muo2wnzs6dy";
pub const DAEMON_FOLDER_HASH: &str = "bafybeicyzvtflal64zu5jrfhveuuqaoizhiaulvyax7s5c3jnoxyyendxu";

// // Gateway URL
pub const IPFS_GATEWAY: &str = "https://gateway.pinata.cloud/ipfs";

// Image counts per rarity
pub const COMMON_COUNT: u8 = 10;   // c1 to c10
pub const UNCOMMON_COUNT: u8 = 6;  // u1 to u6
pub const RARE_COUNT: u8 = 4;      // r1 to r4
