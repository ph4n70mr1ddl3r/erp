use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

pub struct TOTP {
    secret: Vec<u8>,
    digits: u32,
    time_step: u64,
}

impl TOTP {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            secret: secret.to_vec(),
            digits: 6,
            time_step: 30,
        }
    }

    pub fn generate_secret() -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..20).map(|_| rng.gen::<u8>()).collect()
    }

    pub fn secret_to_base32(secret: &[u8]) -> String {
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, secret)
    }

    pub fn base32_to_secret(encoded: &str) -> Result<Vec<u8>, String> {
        base32::decode(base32::Alphabet::RFC4648 { padding: false }, encoded)
            .ok_or_else(|| "Invalid base32 encoding".to_string())
    }

    pub fn generate_code(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let counter = timestamp / self.time_step;
        self.generate_code_at_counter(counter)
    }

    fn generate_code_at_counter(&self, counter: u64) -> String {
        let counter_bytes = counter.to_be_bytes();

        let mut mac = match HmacSha1::new_from_slice(&self.secret) {
            Ok(m) => m,
            Err(_) => return "000000".to_string(),
        };
        mac.update(&counter_bytes);
        let result = mac.finalize().into_bytes();

        let offset = (result[19] & 0x0f) as usize;
        let binary = ((result[offset] as u32 & 0x7f) << 24)
            | ((result[offset + 1] as u32) << 16)
            | ((result[offset + 2] as u32) << 8)
            | (result[offset + 3] as u32);

        let otp = binary % 10u32.pow(self.digits);
        format!("{:0width$}", otp, width = self.digits as usize)
    }

    pub fn verify(&self, code: &str, allowed_drift: i32) -> bool {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let counter = timestamp / self.time_step;

        for drift in -allowed_drift..=allowed_drift {
            let test_counter = (counter as i64 + drift as i64) as u64;
            if self.generate_code_at_counter(test_counter) == code {
                return true;
            }
        }
        false
    }

    pub fn generate_qr_code_url(secret: &[u8], email: &str, issuer: &str) -> String {
        let secret_base32 = Self::secret_to_base32(secret);
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30",
            urlencoding::encode(issuer),
            urlencoding::encode(email),
            secret_base32,
            urlencoding::encode(issuer)
        )
    }

    pub fn generate_backup_codes(count: usize) -> Vec<String> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..count)
            .map(|_| {
                let code: u64 = rng.gen();
                format!("{:016x}", code)
            })
            .collect()
    }
}

mod base32 {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    pub enum Alphabet {
        RFC4648 { padding: bool },
    }

    pub fn encode(alphabet: Alphabet, data: &[u8]) -> String {
        let padding = match alphabet {
            Alphabet::RFC4648 { padding } => padding,
        };

        let mut result = String::new();
        let mut buffer: u64 = 0;
        let mut bits = 0;

        for &byte in data {
            buffer = (buffer << 8) | (byte as u64);
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                let index = ((buffer >> bits) & 0x1f) as usize;
                result.push(ALPHABET[index] as char);
            }
        }

        if bits > 0 {
            let index = ((buffer << (5 - bits)) & 0x1f) as usize;
            result.push(ALPHABET[index] as char);
        }

        if padding {
            #[allow(clippy::manual_is_multiple_of)]
            while result.len() % 8 != 0 {
                result.push('=');
            }
        }

        result
    }

    pub fn decode(alphabet: Alphabet, encoded: &str) -> Option<Vec<u8>> {
        let padding = match alphabet {
            Alphabet::RFC4648 { padding } => padding,
        };

        let decoded_map: std::collections::HashMap<char, u8> = ALPHABET
            .iter()
            .enumerate()
            .map(|(i, &c)| (c as char, i as u8))
            .collect();

        let mut result = Vec::new();
        let mut buffer: u64 = 0;
        let mut bits = 0;

        for c in encoded.chars() {
            if c == '=' && padding {
                continue;
            }
            let &val = decoded_map.get(&c.to_ascii_uppercase())?;
            buffer = (buffer << 5) | (val as u64);
            bits += 5;
            while bits >= 8 {
                bits -= 8;
                result.push(((buffer >> bits) & 0xff) as u8);
            }
        }

        Some(result)
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        fn is_safe(c: char) -> bool {
            c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~'
        }
        s.chars()
            .map(|c| {
                if is_safe(c) {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u32)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totp_generate_and_verify() {
        let secret = TOTP::generate_secret();
        let totp = TOTP::new(&secret);
        let code = totp.generate_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
        assert!(totp.verify(&code, 1));
    }

    #[test]
    fn test_backup_codes() {
        let codes = TOTP::generate_backup_codes(10);
        assert_eq!(codes.len(), 10);
        for code in &codes {
            assert_eq!(code.len(), 16);
        }
    }

    #[test]
    fn test_qr_code_url() {
        let secret = TOTP::generate_secret();
        let url = TOTP::generate_qr_code_url(&secret, "user@example.com", "ERP System");
        assert!(url.starts_with("otpauth://totp/"));
        assert!(url.contains("secret="));
        assert!(url.contains("issuer="));
    }
}
