#[doc = "Register `ROOT_COMPLEX_BASE_ADDRESS_1` reader"]
pub type R = crate::R<RootComplexBaseAddress1Spec>;
#[doc = "Register `ROOT_COMPLEX_BASE_ADDRESS_1` writer"]
pub type W = crate::W<RootComplexBaseAddress1Spec>;
#[doc = "Field `BAMRW` reader - Base Address \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
pub type BamrwR = crate::FieldReader<u32>;
#[doc = "Field `BAMRW` writer - Base Address \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base Address \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base Address \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<RootComplexBaseAddress1Spec> {
        BamrwW::new(self, 0)
    }
}
#[doc = "Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_complex_base_address_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_complex_base_address_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootComplexBaseAddress1Spec;
impl crate::RegisterSpec for RootComplexBaseAddress1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_complex_base_address_1::R`](R) reader structure"]
impl crate::Readable for RootComplexBaseAddress1Spec {}
#[doc = "`write(|w| ..)` method takes [`root_complex_base_address_1::W`](W) writer structure"]
impl crate::Writable for RootComplexBaseAddress1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_COMPLEX_BASE_ADDRESS_1 to value 0"]
impl crate::Resettable for RootComplexBaseAddress1Spec {
    const RESET_VALUE: u32 = 0;
}