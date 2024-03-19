#[doc = "Register `PCIE_PF_VF_BASE_ADDRESS_5` reader"]
pub type R = crate::R<PciePfVfBaseAddress5Spec>;
#[doc = "Register `PCIE_PF_VF_BASE_ADDRESS_5` writer"]
pub type W = crate::W<PciePfVfBaseAddress5Spec>;
#[doc = "Field `BAMRW` reader - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub type BamrwR = crate::FieldReader<u32>;
#[doc = "Field `BAMRW` writer - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<PciePfVfBaseAddress5Spec> {
        BamrwW::new(self, 0)
    }
}
#[doc = "VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfVfBaseAddress5Spec;
impl crate::RegisterSpec for PciePfVfBaseAddress5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_vf_base_address_5::R`](R) reader structure"]
impl crate::Readable for PciePfVfBaseAddress5Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_vf_base_address_5::W`](W) writer structure"]
impl crate::Writable for PciePfVfBaseAddress5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_VF_BASE_ADDRESS_5 to value 0"]
impl crate::Resettable for PciePfVfBaseAddress5Spec {
    const RESET_VALUE: u32 = 0;
}