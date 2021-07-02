use include_crypt::{include_crypt, EncryptedFile};

#[derive(PartialEq)]
pub enum UserDir {
    LocalAppdata,
    Root,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum AuthType {
    Exchange,
    // Just the username
    Username,
    // Both Username & Password
    Password
}

// if you've used c++ and header files
// this is pretty much my equivalent

// Enable/Disable certain features
pub const LAUNCHER: bool = true;
pub const BACKEND: bool = true;

// Used for random string generation
pub const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// CURRENTLY UNUSED AND DOESN'T WORK
// Use FortniteLauncher.exe to get the fltoken
pub const ABUSE_LAUNCHER: bool = true;

// What Auth system to use
pub const AUTH_TYPE: AuthType = AuthType::Exchange;

// CURRENTLY UNUSED AND DOESN'T WORK
// Whether to make a Ruten directory for binaries or just use the standard Win64 directory
pub const RUTEN_DIR_LAUNCHER: bool = true;

// AC executables to replace
pub const AC_EXECUTABLES: [&'static str; 2] = [
    "FortniteClient-Win64-Shipping_BE.exe",
    "FortniteClient-Win64-Shipping_EAC.exe",
];

// Dummy AC executable
pub static DUMMY_AC: EncryptedFile = include_crypt!("assets/DummyAC.exe");

// Where should profiles/cloudstorage/settings be stored
pub const USER_DIR: UserDir = UserDir::Root;

// Where should the server should be hosted
pub const HOST_URL: &'static str = "127.0.0.1:60101";

// Cosmetics & Profiles
pub const COSMETICS: bool = true;

// Whether to use CloudStorage from user/cloudstorage or not
pub const CUSTOM_CLOUDSTORAGE: bool = false;

// Default CloudStorage. Only used when the setting above is true.
pub static CLOUDSTORAGE: [(&str, EncryptedFile); 2] = [
    (
        "DefaultEngine.ini",
        include_crypt!("assets/cloudstorage/DefaultEngine.ini"),
    ),
    (
        "DefaultGame.ini",
        include_crypt!("assets/cloudstorage/DefaultGame.ini"),
    ),
];

// DLLs that will be injected
pub static INJECTABLES: [EncryptedFile; 1] = [include_crypt!("assets/CSM-Hybrid.dll")];

// Message that gets printed when opening Ruten
pub static WELCOME: EncryptedFile = include_crypt!("assets/welcome.txt");
