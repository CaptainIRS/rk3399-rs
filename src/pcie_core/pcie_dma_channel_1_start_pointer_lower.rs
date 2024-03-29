#[doc = "Register `PCIE_DMA_CHANNEL_1_START_POINTER_LOWER` reader"]
pub type R = crate::R<PcieDmaChannel1StartPointerLowerSpec>;
#[doc = "Register `PCIE_DMA_CHANNEL_1_START_POINTER_LOWER` writer"]
pub type W = crate::W<PcieDmaChannel1StartPointerLowerSpec>;
#[doc = "Field `ptr` reader - Start pointer Lower DWORD \\[ptr\\]\n\nLower 32-bits Pointer Address\n\nRegisters"]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `ptr` writer - Start pointer Lower DWORD \\[ptr\\]\n\nLower 32-bits Pointer Address\n\nRegisters"]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start pointer Lower DWORD \\[ptr\\]\n\nLower 32-bits Pointer Address\n\nRegisters"]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start pointer Lower DWORD \\[ptr\\]\n\nLower 32-bits Pointer Address\n\nRegisters"]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<PcieDmaChannel1StartPointerLowerSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "PCIe DMA Channel 1 Start Pointer Lower Register\n\nLower 32-bits Pointer Address\n\nRegisters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_start_pointer_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_start_pointer_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaChannel1StartPointerLowerSpec;
impl crate::RegisterSpec for PcieDmaChannel1StartPointerLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_channel_1_start_pointer_lower::R`](R) reader structure"]
impl crate::Readable for PcieDmaChannel1StartPointerLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_channel_1_start_pointer_lower::W`](W) writer structure"]
impl crate::Writable for PcieDmaChannel1StartPointerLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_DMA_CHANNEL_1_START_POINTER_LOWER to value 0"]
impl crate::Resettable for PcieDmaChannel1StartPointerLowerSpec {
    const RESET_VALUE: u32 = 0;
}
