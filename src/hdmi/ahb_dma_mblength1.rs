#[doc = "Register `AHB_DMA_MBLENGTH1` reader"]
pub type R = crate::R<AhbDmaMblength1Spec>;
#[doc = "Field `MBURSTLENGTH` reader - Requested burst length"]
pub type MburstlengthR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Requested burst length"]
    #[inline(always)]
    pub fn mburstlength(&self) -> MburstlengthR {
        MburstlengthR::new((self.bits & 1) != 0)
    }
}
#[doc = "Audio DMA Burst Length Register 1\n\nThis registers holds the length of the current burst operation. As an example, if the first\n\nburst transaction of the AHB audio DMA is a length of 8, then the second burst should start\n\nat address ohaddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 32.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mblength1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaMblength1Spec;
impl crate::RegisterSpec for AhbDmaMblength1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_mblength1::R`](R) reader structure"]
impl crate::Readable for AhbDmaMblength1Spec {}
#[doc = "`reset()` method sets AHB_DMA_MBLENGTH1 to value 0"]
impl crate::Resettable for AhbDmaMblength1Spec {
    const RESET_VALUE: u8 = 0;
}
