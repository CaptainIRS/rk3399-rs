#[doc = "Register `SDMMC_TCBCNT` reader"]
pub type R = crate::R<SdmmcTcbcntSpec>;
#[doc = "Field `TRANS_CARD_BYTE_COUNT` reader - Number of bytes transferred by CIU unit to card. In 32-bit or 64-bit AMBA data-bus-width modes, register should be accessed in full to avoid read-coherency problems. In 16-bit AMBA data-bus-width mode, internal 16-bit coherency register is implemented. User should first read lower 16 bits and then higher 16 bits. When reading lower 16 bits, higher 16 bits of counter are stored in temporary register. When higher 16 bits are read, data from temporary register is supplied. Both TCBCNT and TBBCNT share same coherency register. When AREA_OPTIMIZED parameter is 1, register should be read only after data transfer completes; during data transfer, register returns 0."]
pub type TransCardByteCountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card. In 32-bit or 64-bit AMBA data-bus-width modes, register should be accessed in full to avoid read-coherency problems. In 16-bit AMBA data-bus-width mode, internal 16-bit coherency register is implemented. User should first read lower 16 bits and then higher 16 bits. When reading lower 16 bits, higher 16 bits of counter are stored in temporary register. When higher 16 bits are read, data from temporary register is supplied. Both TCBCNT and TBBCNT share same coherency register. When AREA_OPTIMIZED parameter is 1, register should be read only after data transfer completes; during data transfer, register returns 0."]
    #[inline(always)]
    pub fn trans_card_byte_count(&self) -> TransCardByteCountR {
        TransCardByteCountR::new(self.bits)
    }
}
#[doc = "Transferred CIU card byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_tcbcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcTcbcntSpec;
impl crate::RegisterSpec for SdmmcTcbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_tcbcnt::R`](R) reader structure"]
impl crate::Readable for SdmmcTcbcntSpec {}
#[doc = "`reset()` method sets SDMMC_TCBCNT to value 0"]
impl crate::Resettable for SdmmcTcbcntSpec {
    const RESET_VALUE: u32 = 0;
}