#[doc = "Register `PCIE_PF_BASE_ADDRESS_1` reader"]
pub type R = crate::R<PciePfBaseAddress1Spec>;
#[doc = "Register `PCIE_PF_BASE_ADDRESS_1` writer"]
pub type W = crate::W<PciePfBaseAddress1Spec>;
#[doc = "Field `BAMRW` reader - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address of\n\nthe memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nsetting of BAR Configuration\n\nRegisters of the associated\n\nPhysical Function. All other bits are\n\nnot writeable, and are read as 0's."]
pub type BamrwR = crate::FieldReader<u32>;
#[doc = "Field `BAMRW` writer - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address of\n\nthe memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nsetting of BAR Configuration\n\nRegisters of the associated\n\nPhysical Function. All other bits are\n\nnot writeable, and are read as 0's."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address of\n\nthe memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nsetting of BAR Configuration\n\nRegisters of the associated\n\nPhysical Function. All other bits are\n\nnot writeable, and are read as 0's."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address of\n\nthe memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nsetting of BAR Configuration\n\nRegisters of the associated\n\nPhysical Function. All other bits are\n\nnot writeable, and are read as 0's."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<PciePfBaseAddress1Spec> {
        BamrwW::new(self, 0)
    }
}
#[doc = "Base Address Register 1\n\nThis field defines the base address of\n\nthe memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nsetting of BAR Configuration\n\nRegisters of the associated\n\nPhysical Function. All other bits are\n\nnot writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfBaseAddress1Spec;
impl crate::RegisterSpec for PciePfBaseAddress1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_base_address_1::R`](R) reader structure"]
impl crate::Readable for PciePfBaseAddress1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_base_address_1::W`](W) writer structure"]
impl crate::Writable for PciePfBaseAddress1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_BASE_ADDRESS_1 to value 0"]
impl crate::Resettable for PciePfBaseAddress1Spec {
    const RESET_VALUE: u32 = 0;
}
