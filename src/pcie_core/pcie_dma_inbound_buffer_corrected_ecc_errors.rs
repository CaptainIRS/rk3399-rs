#[doc = "Register `PCIE_DMA_INBOUND_BUFFER_CORRECTED_ECC_ERRORS` reader"]
pub type R = crate::R<PcieDmaInboundBufferCorrectedEccErrorsSpec>;
#[doc = "Field `total` reader - ECC Error Reg \\[total\\]\n\nECC Error Detection Register"]
pub type TotalR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ECC Error Reg \\[total\\]\n\nECC Error Detection Register"]
    #[inline(always)]
    pub fn total(&self) -> TotalR {
        TotalR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PCIe DMA Inbound Buffer corrected ECC Errors\n\nReserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_inbound_buffer_corrected_ecc_errors::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaInboundBufferCorrectedEccErrorsSpec;
impl crate::RegisterSpec for PcieDmaInboundBufferCorrectedEccErrorsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_inbound_buffer_corrected_ecc_errors::R`](R) reader structure"]
impl crate::Readable for PcieDmaInboundBufferCorrectedEccErrorsSpec {}
#[doc = "`reset()` method sets PCIE_DMA_INBOUND_BUFFER_CORRECTED_ECC_ERRORS to value 0"]
impl crate::Resettable for PcieDmaInboundBufferCorrectedEccErrorsSpec {
    const RESET_VALUE: u32 = 0;
}
