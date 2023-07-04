// https://datatracker.ietf.org/doc/html/rfc5830
// https://en.wikipedia.org/wiki/GOST_%28block_cipher%29
// https://standartgost.ru/g/%D0%93%D0%9E%D0%A1%D0%A2_28147-89

pub struct Gost28147_89 {
    cipher_key: [u32;8],
    round_keys: [u32;32],
    substitution_box: [u8;128]
}

impl Gost28147_89 {

    const ROUND_KEY_POSITION: [u8;32] = [
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        7, 6, 5, 4, 3, 2, 1, 0
    ];
    
    const DEFAULT_SUBSTITUTION_BOX: [u8;128] = [
        0x4, 0xA, 0x9, 0x2, 0xD, 0x8, 0x0, 0xE, 0x6, 0xB, 0x1, 0xC, 0x7, 0xF, 0x5, 0x3,
        0xE, 0xB, 0x4, 0xC, 0x6, 0xD, 0xF, 0xA, 0x2, 0x3, 0x8, 0x1, 0x0, 0x7, 0x5, 0x9,
        0x5, 0x8, 0x1, 0xD, 0xA, 0x3, 0x4, 0x2, 0xE, 0xF, 0xC, 0x7, 0x6, 0x0, 0x9, 0xB,
        0x7, 0xD, 0xA, 0x1, 0x0, 0x8, 0x9, 0xF, 0xE, 0x4, 0x6, 0xC, 0xB, 0x2, 0x5, 0x3,
        0x6, 0xC, 0x7, 0x1, 0x5, 0xF, 0xD, 0x8, 0x4, 0xA, 0x9, 0xE, 0x0, 0x3, 0xB, 0x2,
        0x4, 0xB, 0xA, 0x0, 0x7, 0x2, 0x1, 0xD, 0x3, 0x6, 0x8, 0x5, 0x9, 0xC, 0xF, 0xE,
        0xD, 0xB, 0x4, 0x1, 0x3, 0xF, 0x5, 0x9, 0x0, 0xA, 0xE, 0x7, 0x6, 0x8, 0x2, 0xC,
        0x1, 0xF, 0xD, 0x0, 0x5, 0x7, 0xA, 0x4, 0x9, 0x2, 0x3, 0xE, 0x6, 0xB, 0x8, 0xC
    ];

    pub fn new() -> Gost28147_89 {
        let cipher_key = [0u32;8];
        let round_keys= [0u32;32];
        let mut substitution_box = [0u8;128];
        substitution_box.copy_from_slice(&Gost28147_89::DEFAULT_SUBSTITUTION_BOX);
        Gost28147_89 { cipher_key, round_keys, substitution_box }
    }

    /// sets the s_box
    pub fn set_substitution_box(&mut self, substitution_box: &[u8;128]) {
        self.substitution_box.copy_from_slice(substitution_box);
    }

    /// sets the cipher key 
    pub fn set_key(&mut self, cipher_key: &[u32;8]) {
        self.cipher_key.clone_from(cipher_key);
        self.prepare_round_keys();
    }

    pub fn set_key_from_u8(&mut self, cipher_key: &[u8;32])
    {
        #[repr(C)]
        union CipherKey {
            array_u8: [u8;32],
            array_u32: [u32;8]
        }

        let keys = CipherKey {array_u8: cipher_key.clone()};
        
        unsafe {
            self.cipher_key.copy_from_slice(&keys.array_u32);
        }
    }

    pub fn set_key_from_u8_slice(&mut self, cipher_key: &[u8]) {
        assert!(cipher_key.len() == 32);
        let src_ptr = cipher_key.as_ptr() as *const u32;
        let dst_ptr =  self.cipher_key.as_mut_ptr();
        let count = self.cipher_key.len();
        unsafe {std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, count)};
        self.prepare_round_keys();
    }

    fn prepare_round_keys(&mut self) {
        for index in 0..32 {
            let round_key_position = Gost28147_89::ROUND_KEY_POSITION[index] as usize;
            self.round_keys[index]= self.cipher_key[round_key_position];
        }
    }

    pub fn encrypt(&self) -> u64 {
        0
    }
    
    pub fn decrypt(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let gost = Gost28147_89::new();
        assert_eq!(gost.cipher_key, [0u32;8]);
        assert_eq!(gost.round_keys, [0u32;32]);
        assert_eq!(gost.substitution_box, Gost28147_89::DEFAULT_SUBSTITUTION_BOX);
    }

    #[test]
    fn set_keys() {

        let cipher_key_u32: [u32;8] = [
            0x733D2C20,
            0x65686573,
            0x74746769,
            0x79676120,
            0x626E7373,
            0x20657369,
            0x326C6568,
            0x33206D54
        ];

        let cipher_key_u8: [u8;32] = [
            0x20, 0x2C, 0x3D, 0x73,   
            0x73, 0x65, 0x68, 0x65,   
            0x69, 0x67, 0x74, 0x74,   
            0x20, 0x61, 0x67, 0x79,   
            0x73, 0x73, 0x6E, 0x62,   
            0x69, 0x73, 0x65, 0x20,   
            0x68, 0x65, 0x6C, 0x32,   
            0x54, 0x6D, 0x20, 0x33  
        ];

        let mut gost = Gost28147_89::new();
        gost.set_key(&cipher_key_u32);
        assert_eq!(gost.cipher_key, cipher_key_u32);

        gost.set_key_from_u8(&cipher_key_u8);
        assert_eq!(gost.cipher_key, cipher_key_u32);

        gost.set_key_from_u8_slice(&cipher_key_u8);
        assert_eq!(gost.cipher_key, cipher_key_u32);
    }

    #[test]
    fn distribution_round_keys() {
        let mut gost = Gost28147_89::new();
        let cipher_key = [0x733D2C20, 0x65686573, 0x74746769, 0x79676120, 0x626E7373, 0x20657369, 0x326C6568, 0x33206D54];
        gost.set_key(&cipher_key);

        let round_keys = [
            0x733D2C20, 0x65686573, 0x74746769, 0x79676120, 0x626E7373, 0x20657369, 0x326C6568, 0x33206D54,
            0x733D2C20, 0x65686573, 0x74746769, 0x79676120, 0x626E7373, 0x20657369, 0x326C6568, 0x33206D54,
            0x733D2C20, 0x65686573, 0x74746769, 0x79676120, 0x626E7373, 0x20657369, 0x326C6568, 0x33206D54,
            0x33206D54, 0x326C6568, 0x20657369, 0x626E7373, 0x79676120, 0x74746769, 0x65686573, 0x733D2C20
        ];
        assert_eq!(gost.round_keys, round_keys);
    }
}

/*
Test vectors:
S-Box = [
    0x4, 0xA, 0x9, 0x2, 0xD, 0x8, 0x0, 0xE, 0x6, 0xB, 0x1, 0xC, 0x7, 0xF, 0x5, 0x3,
    0xE, 0xB, 0x4, 0xC, 0x6, 0xD, 0xF, 0xA, 0x2, 0x3, 0x8, 0x1, 0x0, 0x7, 0x5, 0x9,
    0x5, 0x8, 0x1, 0xD, 0xA, 0x3, 0x4, 0x2, 0xE, 0xF, 0xC, 0x7, 0x6, 0x0, 0x9, 0xB,
    0x7, 0xD, 0xA, 0x1, 0x0, 0x8, 0x9, 0xF, 0xE, 0x4, 0x6, 0xC, 0xB, 0x2, 0x5, 0x3,
    0x6, 0xC, 0x7, 0x1, 0x5, 0xF, 0xD, 0x8, 0x4, 0xA, 0x9, 0xE, 0x0, 0x3, 0xB, 0x2,
    0x4, 0xB, 0xA, 0x0, 0x7, 0x2, 0x1, 0xD, 0x3, 0x6, 0x8, 0x5, 0x9, 0xC, 0xF, 0xE,
    0xD, 0xB, 0x4, 0x1, 0x3, 0xF, 0x5, 0x9, 0x0, 0xA, 0xE, 0x7, 0x6, 0x8, 0x2, 0xC,
    0x1, 0xF, 0xD, 0x0, 0x5, 0x7, 0xA, 0x4, 0x9, 0x2, 0x3, 0xE, 0x6, 0xB, 0x8, 0xC
]

Input = 0x00000000 0x00000000
K1 = 0x733D2C20 0x65686573 0x74746769 0x79676120 0x626E7373 0x20657369 0x326C6568 0x33206D54
K2 = 0x110C733D 0x0D166568 0x130E7474 0x06417967 0x1D00626E 0x161A2065 0x090D326C 0x4D393320
K3 = 0x80B111F3 0x730DF216 0x850013F1 0xC7E1F941 0x620C1DFF 0x3ABAE91A 0x3FA109F2 0xF513B239
K4 = 0xA0E2804E 0xFF1B73F2 0xECE27A00 0xE7B8C7E1 0xEE1D620C 0xAC0CC5BA 0xA804C05E 0xA18B0AEC
Outputs:
R1 = 0x42ABBCCE 0x32BC0B1B
R2 = 0x5203EBC8 0x5D9BCFFD
R3 = 0x8D345899 0x00FF0E28
R4 = 0xE7860419 0x0D2A562D
*/