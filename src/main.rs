use clap::Parser;
use openssl::encrypt::{Decrypter, Encrypter};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::{Padding, Rsa};
use std::fs::create_dir;
use std::process::exit;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short = 'e', long = "encrypt", conflicts_with = "decrypt")]
    encrypt: bool,

    #[arg(short = 'd', long = "decrypt", conflicts_with = "encrypt")]
    decrypt: bool,

    #[arg(long)]
    ram: bool,

    #[arg(short = 'p', long = "path")]
    path: Option<PathBuf>,

    #[arg(short = 'g', long = "generate-new-keys", conflicts_with = "public_key")]
    generate_new_keys: bool,

    #[arg(long = "public-key", default_value = "storage/keys/public.pem")]
    public_key: PathBuf,

    #[arg(long = "private-key", default_value = "storage/keys/private.pem")]
    private_key: PathBuf,
}
fn main() -> io::Result<()> {
    let path = Path::new("storage");
    if !path.exists() {
        init().expect("We were not able to create the directories.");
        exit(0)
    }
    let args = Args::parse();

    if args.generate_new_keys {
        let rsa = Rsa::generate(3072).expect("openssl rsa gen failed");
        let pkey = PKey::from_rsa(rsa).expect("to PKey failed");

        let sk_pem = pkey.private_key_to_pem_pkcs8().expect("pem pkcs8 failed");
        let pk_pem = pkey.public_key_to_pem().expect("pub pem failed");

        std::fs::write("storage/keys/private.pem", &sk_pem).expect("write private failed");
        std::fs::write("storage/keys/public.pem", &pk_pem).expect("write public failed");
        return Ok(());
    }
    if args.encrypt {
        let mut content = String::new();
        println!("enter your text below:");
        io::stdin().read_line(&mut content)?;
        let mut pk_pem: String = String::new();
        let out = args.path.as_deref().unwrap_or(Path::new("test.msg"));

        let mut clef = File::open(args.public_key).unwrap();
        clef.read_to_string(&mut pk_pem).unwrap();

        let cipher_text = encrypting(content.trim_end(), &pk_pem).expect("encrypt failed");
        saving(out, &cipher_text)?;
    } else if args.decrypt {
        let mut private_pem = String::new();
        let mut file = File::open(args.private_key).unwrap();
        file.read_to_string(&mut private_pem).unwrap();

        let mut file = File::open(args.path.unwrap()).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();

        let plaintext = decrypting(&content, &private_pem).unwrap();
        println!("{:?}", String::from_utf8_lossy(&plaintext));
    }

    Ok(())
}

fn saving(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)
}

fn encrypting(content: &str, pem: &str) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let pkey = if pem.contains("BEGIN PUBLIC KEY") {
        PKey::public_key_from_pem(pem.as_bytes())?
    } else if pem.contains("BEGIN RSA PUBLIC KEY") {
        let rsa = Rsa::public_key_from_pem_pkcs1(pem.as_bytes())?;
        PKey::from_rsa(rsa)?
    } else {
        PKey::public_key_from_pem(pem.as_bytes())?
    };

    let mut encrypter = Encrypter::new(&pkey)?;
    encrypter.set_rsa_padding(Padding::PKCS1_OAEP)?;
    encrypter.set_rsa_oaep_md(MessageDigest::sha256())?;
    encrypter.set_rsa_mgf1_md(MessageDigest::sha256())?;

    let mut out = vec![0; encrypter.encrypt_len(content.as_bytes())?];
    let n = encrypter.encrypt(content.as_bytes(), &mut out)?;
    out.truncate(n);
    Ok(out)
}

fn decrypting(ciphertext: &[u8], private_pem: &str) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let pkey = if private_pem.contains("BEGIN PRIVATE KEY") {
        PKey::private_key_from_pem(private_pem.as_bytes())?
    } else if private_pem.contains("BEGIN RSA PRIVATE KEY") {
        let rsa = Rsa::private_key_from_pem(private_pem.as_bytes())?;
        PKey::from_rsa(rsa)?
    } else {
        PKey::private_key_from_pem(private_pem.as_bytes())?
    };

    let mut decrypter = Decrypter::new(&pkey)?;
    decrypter.set_rsa_padding(Padding::PKCS1_OAEP)?;
    decrypter.set_rsa_oaep_md(MessageDigest::sha256())?;
    decrypter.set_rsa_mgf1_md(MessageDigest::sha256())?;

    let mut out = vec![0; decrypter.decrypt_len(ciphertext)?];
    let n = decrypter.decrypt(ciphertext, &mut out)?;
    out.truncate(n);
    Ok(out)
}
fn opening(path: &Path) {}
fn create_persistent_storage() -> std::io::Result<()> {
    create_dir("./storage")?;
    create_dir("./storage/keys/")?;
    create_dir("./storage/index/")?; // Not implemented yet.
    create_dir("./storage/messages/")
}
fn init() -> std::io::Result<()> {
    println!("Thank you for using Oympi. This is the initialization of the program; it creates 4 directories. The first one is storage: this is where the program stores everything. If you already have a pair of cryptographic keys and you want to use them, please put them in storage/keys/ under the names 'private.pem' and 'public.pem'. Please note that we do NOT support encrypted keys yet. You can see the arguments to use by running ./oympi -h");
    create_persistent_storage()
}
