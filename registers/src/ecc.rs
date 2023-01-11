// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with rtl-caliptra repo at f1feedff601b55715ccaed60ebfcd83543617752
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
#[derive(Clone, Copy)]
pub struct RegisterBlock(*mut u32);
impl RegisterBlock {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self(ptr)
    }
    pub fn ecc_reg() -> Self {
        unsafe { Self::new(0x10008000 as *mut u32) }
    }
    /// Two 32-bit read-only registers repereseting of the name
    /// of ECC component. These registers are located at
    /// ECC_base_address + 0x0000_0000 and 0x0000_0004 addresses.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn name(&self) -> ureg::Array<2, ureg::RegRef<crate::ecc::meta::Name>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0 / core::mem::size_of::<u32>())) }
    }
    /// Two 32-bit read-only registers repereseting of the version
    /// of ECC component. These registers are located at
    /// ECC_base_address + 0x0000_0008 and 0x0000_000C addresses.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn version(&self) -> ureg::Array<2, ureg::RegRef<crate::ecc::meta::Version>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(8 / core::mem::size_of::<u32>())) }
    }
    /// One 2-bit register to set the type of ECC operations.
    /// This can be:
    ///    00 for NONE
    ///    01 for KEYGEN
    ///    10 for SIGNING
    ///    11 for VERIFYING
    /// This register is located at ECC_base_address + 0x0000_0010
    /// After each software write, hardware will erase the register.
    ///
    /// Read value: [`ecc::regs::CtrlReadVal`]; Write value: [`ecc::regs::CtrlWriteVal`]
    pub fn ctrl(&self) -> ureg::RegRef<crate::ecc::meta::Ctrl> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x10 / core::mem::size_of::<u32>())) }
    }
    /// One 2-bit register including the following flags:
    /// bit #0: READY : ​Indicates if the core is ready to take
    ///                a control command and process the block.  
    /// bit #1: VALID : ​Indicates if the process is done and the
    ///                hash value stored in DIGEST registers is valid.
    /// This register is located at ECC_base_address + 0x0000_0018.
    ///
    /// Read value: [`ecc::regs::StatusReadVal`]; Write value: [`ecc::regs::StatusWriteVal`]
    pub fn status(&self) -> ureg::RegRef<crate::ecc::meta::Status> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x18 / core::mem::size_of::<u32>())) }
    }
    /// One 4-bit register to enable/disable the ECC SCA countermeasures.
    /// bit #0: POINT_RND_EN : ​Indicates if the base point randomization
    ///                       countermeasure is enabled.
    /// bit #1: MASK_SIGN_EN: ​Indicates if the mask countermeasure in SIGNING
    ///                      is enabled.
    /// bit #2: SCALAR_RND_EN: ​Indicates if the scalar ransomization countermeasure
    ///                      is enabled.
    /// bit #3: OPENSSL_EN: ​Indicates if the openssl test is enabled.
    ///                    bit 4 should be deleted after openssl keygen test.
    ///                                
    /// This register is located at ECC_base_address + 0x0000_0020
    /// After each software write, hardware will erase the register.
    ///
    /// Read value: [`ecc::regs::ScaconfigReadVal`]; Write value: [`ecc::regs::ScaconfigWriteVal`]
    pub fn scaconfig(&self) -> ureg::RegRef<crate::ecc::meta::Scaconfig> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x20 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the 384-bit seed for keygen.
    /// The seed can be any 384-bit value in [0 : 2^384-1].
    /// These registers are located at ECC_base_address +
    /// 0x0000_0080 to 0x0000_00AC in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn seed(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::Seed>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x80 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the hash of the message respect
    /// to SHA384 algorithm.
    /// The hashed message can be any 384-bit value in [0 : 2^384-1].
    /// These registers are located at ECC_base_address +
    /// 0x0000_0100 to 0x0000_012C in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn msg(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::Msg>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x100 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the private key. These registers
    /// is read by ECC user after keygen operation, or be set before
    /// signing operation.
    /// The private key should be in [1 : q-1] while q is the group
    /// order of the Secp384r1 curve.
    /// These registers are located at ECC_base_address +
    /// 0x0000_0180 to 0x0000_01AC in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn privkey(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::Privkey>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x180 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the x coordinate of public key.
    /// These registers is read by ECC user after keygen operation,
    /// or be set before verifying operation.
    /// The public key x should be in [1 : p-1] while p is the prime
    /// number of the Secp384r1 curve.
    /// These registers are located at ECC_base_address +
    /// 0x0000_0200 to 0x0000_022C in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn pubkey_x(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::PubkeyX>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x200 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the y coordinate of public key.
    /// These registers is read by ECC user after keygen operation,
    /// or be set before verifying operation.
    /// The public key y should be in [1 : p-1] while p is the prime
    /// number of the Secp384r1 curve.
    /// These registers are located at ECC_base_address +
    /// 0x0000_0280 to 0x0000_02AC in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn pubkey_y(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::PubkeyY>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x280 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the signature R of the message.
    /// These registers is read by ECC user after signing operation,
    /// or be set before verifying operation.
    /// The signature R should be in [1 : q-1] while q is the group
    /// order of the Secp384r1 curve.
    /// Based on RFC6979, If R turns out to be zero, a new nonce (by changing
    /// the private key or the message) should be selected and R computed
    /// again (this is an utterly improbable occurrence).
    /// These registers are located at ECC_base_address +
    /// 0x0000_0300 to 0x0000_032C in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn sign_r(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::SignR>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x300 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the signature S of the message.
    /// These registers is read by ECC user after signing operation,
    /// or be set before verifying operation.
    /// The signature S should be in [1 : q-1] while q is the group
    /// order of the Secp384r1 curve.
    /// These registers are located at ECC_base_address +
    /// 0x0000_0380 to 0x0000_03AC in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn sign_s(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::SignS>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x380 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the result of verifying operation.
    /// Firmware is responsible for comparing the computed result with
    /// the signature R, and if they are equal the signature is valid.
    /// The verify R result should be in [1 : q-1] while q is the group
    /// order of the Secp384r1 curve.
    /// These registers are located at ECC_base_address +
    /// 0x0000_0400 to 0x0000_042C in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn verify_r(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::VerifyR>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x400 / core::mem::size_of::<u32>())) }
    }
    /// 12 32-bit registers storing the 384-bit IV required
    /// for SCA countermeasures to randomize the inputs with no change
    /// on the ECC outputs.
    /// The IV can be any 384-bit value in [0 : 2^384-1].
    /// These registers are located at ECC_base_address +
    /// 0x0000_0480 to 0x0000_04AC in big-endian representation.
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn iv(&self) -> ureg::Array<12, ureg::RegRef<crate::ecc::meta::Iv>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x480 / core::mem::size_of::<u32>())) }
    }
    /// Controls the Key Vault read access for this engine
    ///
    /// Read value: [`ecc::regs::KvReadCtrlRegReadVal`]; Write value: [`ecc::regs::KvReadCtrlRegWriteVal`]
    pub fn kv_rd_pkey_ctrl(&self) -> ureg::RegRef<crate::ecc::meta::KvRdPkeyCtrl> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x600 / core::mem::size_of::<u32>())) }
    }
    /// Controls the Key Vault read access for this engine
    ///
    /// Read value: [`ecc::regs::KvReadCtrlRegReadVal`]; Write value: [`ecc::regs::KvReadCtrlRegWriteVal`]
    pub fn kv_rd_seed_ctrl(&self) -> ureg::RegRef<crate::ecc::meta::KvRdSeedCtrl> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x604 / core::mem::size_of::<u32>())) }
    }
    /// Controls the Key Vault read access for this engine
    ///
    /// Read value: [`ecc::regs::KvReadCtrlRegReadVal`]; Write value: [`ecc::regs::KvReadCtrlRegWriteVal`]
    pub fn kv_rd_msg_ctrl(&self) -> ureg::RegRef<crate::ecc::meta::KvRdMsgCtrl> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x608 / core::mem::size_of::<u32>())) }
    }
    /// Controls the Key Vault write access for this engine
    ///
    /// Read value: [`ecc::regs::KvWriteCtrlRegReadVal`]; Write value: [`ecc::regs::KvWriteCtrlRegWriteVal`]
    pub fn kv_wr_pkey_ctrl(&self) -> ureg::RegRef<crate::ecc::meta::KvWrPkeyCtrl> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x60c / core::mem::size_of::<u32>())) }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct CtrlWriteVal(u32);
    impl CtrlWriteVal {
        /// Control command field
        #[inline(always)]
        pub fn ctrl(
            self,
            f: impl FnOnce(super::enums::selector::CtrlSelector) -> super::enums::Ctrl,
        ) -> Self {
            Self((self.0 & !(3 << 0)) | (u32::from(f(super::enums::selector::CtrlSelector())) << 0))
        }
    }
    impl From<u32> for CtrlWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<CtrlWriteVal> for u32 {
        fn from(val: CtrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ScaconfigWriteVal(u32);
    impl ScaconfigWriteVal {
        /// SCA point randomization field
        #[inline(always)]
        pub fn point_rnd_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// SCA mask signing field
        #[inline(always)]
        pub fn mask_sign_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// SCA scalar randomization field
        #[inline(always)]
        pub fn scalar_rnd_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
        /// OPENSSL configuration field
        #[inline(always)]
        pub fn openssl_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 3)) | (u32::from(val) << 3))
        }
    }
    impl From<u32> for ScaconfigWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ScaconfigWriteVal> for u32 {
        fn from(val: ScaconfigWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StatusReadVal(u32);
    impl StatusReadVal {
        /// Status ready bit
        #[inline(always)]
        pub fn ready(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Status valid bit
        #[inline(always)]
        pub fn valid(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
    }
    impl From<u32> for StatusReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StatusReadVal> for u32 {
        fn from(val: StatusReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KvReadCtrlRegReadVal(u32);
    impl KvReadCtrlRegReadVal {
        /// Indicates that the read data is to come from the key vault.
        /// Setting this bit to 1 initiates copying of data from the key vault.
        #[inline(always)]
        pub fn read_en(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Key Vault entry to retrieve the read data from for the engine
        #[inline(always)]
        pub fn read_entry(&self) -> u32 {
            (self.0 >> 1) & 7
        }
        /// Entry selected is a PCR slot
        #[inline(always)]
        pub fn entry_is_pcr(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// Size of the source data for SHA512 and HMAC384 Block only.
        /// This field is ignored for all other reads.
        /// Size is encoded as N-1 dwords.
        /// KV flow will pad the 1024 Block data and append the length for values 0-26.
        /// All 0 data and Length must be appended in the next Block for values 27-31.
        ///                       
        /// 5'd7 - 256b of data
        ///                       
        /// 5'd11 - 384b of data
        ///                       
        /// 5'd15 - 512b of data
        #[inline(always)]
        pub fn entry_data_size(&self) -> u32 {
            (self.0 >> 5) & 0x1f
        }
        /// Reserved field
        #[inline(always)]
        pub fn rsvd(&self) -> u32 {
            (self.0 >> 10) & 0x1fffff
        }
        /// Result has been copied from the key vault
        #[inline(always)]
        pub fn read_done(&self) -> bool {
            ((self.0 >> 31) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> KvReadCtrlRegWriteVal {
            KvReadCtrlRegWriteVal(self.0)
        }
    }
    impl From<u32> for KvReadCtrlRegReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KvReadCtrlRegReadVal> for u32 {
        fn from(val: KvReadCtrlRegReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KvReadCtrlRegWriteVal(u32);
    impl KvReadCtrlRegWriteVal {
        /// Indicates that the read data is to come from the key vault.
        /// Setting this bit to 1 initiates copying of data from the key vault.
        #[inline(always)]
        pub fn read_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Key Vault entry to retrieve the read data from for the engine
        #[inline(always)]
        pub fn read_entry(self, val: u32) -> Self {
            Self((self.0 & !(7 << 1)) | ((val & 7) << 1))
        }
        /// Entry selected is a PCR slot
        #[inline(always)]
        pub fn entry_is_pcr(self, val: bool) -> Self {
            Self((self.0 & !(1 << 4)) | (u32::from(val) << 4))
        }
        /// Size of the source data for SHA512 and HMAC384 Block only.
        /// This field is ignored for all other reads.
        /// Size is encoded as N-1 dwords.
        /// KV flow will pad the 1024 Block data and append the length for values 0-26.
        /// All 0 data and Length must be appended in the next Block for values 27-31.
        ///                       
        /// 5'd7 - 256b of data
        ///                       
        /// 5'd11 - 384b of data
        ///                       
        /// 5'd15 - 512b of data
        #[inline(always)]
        pub fn entry_data_size(self, val: u32) -> Self {
            Self((self.0 & !(0x1f << 5)) | ((val & 0x1f) << 5))
        }
        /// Reserved field
        #[inline(always)]
        pub fn rsvd(self, val: u32) -> Self {
            Self((self.0 & !(0x1fffff << 10)) | ((val & 0x1fffff) << 10))
        }
    }
    impl From<u32> for KvReadCtrlRegWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KvReadCtrlRegWriteVal> for u32 {
        fn from(val: KvReadCtrlRegWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KvWriteCtrlRegReadVal(u32);
    impl KvWriteCtrlRegReadVal {
        /// Indicates that the result is to be stored in the key vault.
        /// Setting this bit to 1 will copy the result to the keyvault when it is ready.
        #[inline(always)]
        pub fn write_en(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Key Vault entry to store the result
        #[inline(always)]
        pub fn write_entry(&self) -> u32 {
            (self.0 >> 1) & 7
        }
        /// Destination selected is a PCR slot
        #[inline(always)]
        pub fn entry_is_pcr(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// HMAC KEY is a valid destination
        #[inline(always)]
        pub fn hmac_key_dest_valid(&self) -> bool {
            ((self.0 >> 5) & 1) != 0
        }
        /// HMAC BLOCK is a valid destination
        #[inline(always)]
        pub fn hmac_block_dest_valid(&self) -> bool {
            ((self.0 >> 6) & 1) != 0
        }
        /// SHA BLOCK is a valid destination
        #[inline(always)]
        pub fn sha_block_dest_valid(&self) -> bool {
            ((self.0 >> 7) & 1) != 0
        }
        /// ECC PKEY is a valid destination
        #[inline(always)]
        pub fn ecc_pkey_dest_valid(&self) -> bool {
            ((self.0 >> 8) & 1) != 0
        }
        /// ECC SEED is a valid destination
        #[inline(always)]
        pub fn ecc_seed_dest_valid(&self) -> bool {
            ((self.0 >> 9) & 1) != 0
        }
        /// ECC MSG is a valid destination
        #[inline(always)]
        pub fn ecc_msg_dest_valid(&self) -> bool {
            ((self.0 >> 10) & 1) != 0
        }
        /// Reserved field
        #[inline(always)]
        pub fn rsvd(&self) -> u32 {
            (self.0 >> 11) & 0xfffff
        }
        /// Result has been copied to key vault
        #[inline(always)]
        pub fn write_done(&self) -> bool {
            ((self.0 >> 31) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> KvWriteCtrlRegWriteVal {
            KvWriteCtrlRegWriteVal(self.0)
        }
    }
    impl From<u32> for KvWriteCtrlRegReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KvWriteCtrlRegReadVal> for u32 {
        fn from(val: KvWriteCtrlRegReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KvWriteCtrlRegWriteVal(u32);
    impl KvWriteCtrlRegWriteVal {
        /// Indicates that the result is to be stored in the key vault.
        /// Setting this bit to 1 will copy the result to the keyvault when it is ready.
        #[inline(always)]
        pub fn write_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Key Vault entry to store the result
        #[inline(always)]
        pub fn write_entry(self, val: u32) -> Self {
            Self((self.0 & !(7 << 1)) | ((val & 7) << 1))
        }
        /// Destination selected is a PCR slot
        #[inline(always)]
        pub fn entry_is_pcr(self, val: bool) -> Self {
            Self((self.0 & !(1 << 4)) | (u32::from(val) << 4))
        }
        /// HMAC KEY is a valid destination
        #[inline(always)]
        pub fn hmac_key_dest_valid(self, val: bool) -> Self {
            Self((self.0 & !(1 << 5)) | (u32::from(val) << 5))
        }
        /// HMAC BLOCK is a valid destination
        #[inline(always)]
        pub fn hmac_block_dest_valid(self, val: bool) -> Self {
            Self((self.0 & !(1 << 6)) | (u32::from(val) << 6))
        }
        /// SHA BLOCK is a valid destination
        #[inline(always)]
        pub fn sha_block_dest_valid(self, val: bool) -> Self {
            Self((self.0 & !(1 << 7)) | (u32::from(val) << 7))
        }
        /// ECC PKEY is a valid destination
        #[inline(always)]
        pub fn ecc_pkey_dest_valid(self, val: bool) -> Self {
            Self((self.0 & !(1 << 8)) | (u32::from(val) << 8))
        }
        /// ECC SEED is a valid destination
        #[inline(always)]
        pub fn ecc_seed_dest_valid(self, val: bool) -> Self {
            Self((self.0 & !(1 << 9)) | (u32::from(val) << 9))
        }
        /// ECC MSG is a valid destination
        #[inline(always)]
        pub fn ecc_msg_dest_valid(self, val: bool) -> Self {
            Self((self.0 & !(1 << 10)) | (u32::from(val) << 10))
        }
        /// Reserved field
        #[inline(always)]
        pub fn rsvd(self, val: u32) -> Self {
            Self((self.0 & !(0xfffff << 11)) | ((val & 0xfffff) << 11))
        }
    }
    impl From<u32> for KvWriteCtrlRegWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KvWriteCtrlRegWriteVal> for u32 {
        fn from(val: KvWriteCtrlRegWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(u32)]
    pub enum Ctrl {
        None = 0,
        Keygen = 1,
        Signing = 2,
        Verifying = 3,
    }
    impl Ctrl {
        #[inline(always)]
        pub fn none(&self) -> bool {
            *self == Self::None
        }
        #[inline(always)]
        pub fn keygen(&self) -> bool {
            *self == Self::Keygen
        }
        #[inline(always)]
        pub fn signing(&self) -> bool {
            *self == Self::Signing
        }
        #[inline(always)]
        pub fn verifying(&self) -> bool {
            *self == Self::Verifying
        }
    }
    impl TryFrom<u32> for Ctrl {
        type Error = ();
        #[inline(always)]
        fn try_from(val: u32) -> Result<Ctrl, ()> {
            match val {
                0 => Ok(Self::None),
                1 => Ok(Self::Keygen),
                2 => Ok(Self::Signing),
                3 => Ok(Self::Verifying),
                _ => Err(()),
            }
        }
    }
    impl From<Ctrl> for u32 {
        fn from(val: Ctrl) -> Self {
            val as u32
        }
    }
    pub mod selector {
        pub struct CtrlSelector();
        impl CtrlSelector {
            #[inline(always)]
            pub fn none(&self) -> super::Ctrl {
                super::Ctrl::None
            }
            #[inline(always)]
            pub fn keygen(&self) -> super::Ctrl {
                super::Ctrl::Keygen
            }
            #[inline(always)]
            pub fn signing(&self) -> super::Ctrl {
                super::Ctrl::Signing
            }
            #[inline(always)]
            pub fn verifying(&self) -> super::Ctrl {
                super::Ctrl::Verifying
            }
        }
    }
}
pub mod meta {
    //! Additional metadata needed by ureg.
    #[derive(Clone, Copy)]
    pub struct Name();
    impl ureg::RegType for Name {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Name {
        type ReadVal = u32;
    }
    #[derive(Clone, Copy)]
    pub struct Version();
    impl ureg::RegType for Version {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Version {
        type ReadVal = u32;
    }
    #[derive(Clone, Copy)]
    pub struct Ctrl();
    impl ureg::RegType for Ctrl {
        type Raw = u32;
    }
    impl ureg::WritableReg for Ctrl {
        type WriteVal = crate::ecc::regs::CtrlWriteVal;
    }
    impl ureg::ResettableReg for Ctrl {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Status();
    impl ureg::RegType for Status {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Status {
        type ReadVal = crate::ecc::regs::StatusReadVal;
    }
    #[derive(Clone, Copy)]
    pub struct Scaconfig();
    impl ureg::RegType for Scaconfig {
        type Raw = u32;
    }
    impl ureg::WritableReg for Scaconfig {
        type WriteVal = crate::ecc::regs::ScaconfigWriteVal;
    }
    impl ureg::ResettableReg for Scaconfig {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Seed();
    impl ureg::RegType for Seed {
        type Raw = u32;
    }
    impl ureg::WritableReg for Seed {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Seed {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Msg();
    impl ureg::RegType for Msg {
        type Raw = u32;
    }
    impl ureg::WritableReg for Msg {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Msg {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Privkey();
    impl ureg::RegType for Privkey {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Privkey {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for Privkey {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Privkey {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct PubkeyX();
    impl ureg::RegType for PubkeyX {
        type Raw = u32;
    }
    impl ureg::ReadableReg for PubkeyX {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for PubkeyX {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for PubkeyX {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct PubkeyY();
    impl ureg::RegType for PubkeyY {
        type Raw = u32;
    }
    impl ureg::ReadableReg for PubkeyY {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for PubkeyY {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for PubkeyY {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct SignR();
    impl ureg::RegType for SignR {
        type Raw = u32;
    }
    impl ureg::ReadableReg for SignR {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for SignR {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for SignR {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct SignS();
    impl ureg::RegType for SignS {
        type Raw = u32;
    }
    impl ureg::ReadableReg for SignS {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for SignS {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for SignS {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct VerifyR();
    impl ureg::RegType for VerifyR {
        type Raw = u32;
    }
    impl ureg::ReadableReg for VerifyR {
        type ReadVal = u32;
    }
    #[derive(Clone, Copy)]
    pub struct Iv();
    impl ureg::RegType for Iv {
        type Raw = u32;
    }
    impl ureg::WritableReg for Iv {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Iv {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KvRdPkeyCtrl();
    impl ureg::RegType for KvRdPkeyCtrl {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KvRdPkeyCtrl {
        type ReadVal = crate::ecc::regs::KvReadCtrlRegReadVal;
    }
    impl ureg::WritableReg for KvRdPkeyCtrl {
        type WriteVal = crate::ecc::regs::KvReadCtrlRegWriteVal;
    }
    impl ureg::ResettableReg for KvRdPkeyCtrl {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KvRdSeedCtrl();
    impl ureg::RegType for KvRdSeedCtrl {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KvRdSeedCtrl {
        type ReadVal = crate::ecc::regs::KvReadCtrlRegReadVal;
    }
    impl ureg::WritableReg for KvRdSeedCtrl {
        type WriteVal = crate::ecc::regs::KvReadCtrlRegWriteVal;
    }
    impl ureg::ResettableReg for KvRdSeedCtrl {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KvRdMsgCtrl();
    impl ureg::RegType for KvRdMsgCtrl {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KvRdMsgCtrl {
        type ReadVal = crate::ecc::regs::KvReadCtrlRegReadVal;
    }
    impl ureg::WritableReg for KvRdMsgCtrl {
        type WriteVal = crate::ecc::regs::KvReadCtrlRegWriteVal;
    }
    impl ureg::ResettableReg for KvRdMsgCtrl {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KvWrPkeyCtrl();
    impl ureg::RegType for KvWrPkeyCtrl {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KvWrPkeyCtrl {
        type ReadVal = crate::ecc::regs::KvWriteCtrlRegReadVal;
    }
    impl ureg::WritableReg for KvWrPkeyCtrl {
        type WriteVal = crate::ecc::regs::KvWriteCtrlRegWriteVal;
    }
    impl ureg::ResettableReg for KvWrPkeyCtrl {
        const RESET_VAL: Self::Raw = 0;
    }
}
