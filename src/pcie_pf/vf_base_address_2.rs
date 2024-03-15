#[doc = "Register `VF_BASE_ADDRESS_2` reader"]
pub type R = crate::R<VfBaseAddress2Spec>;
#[doc = "Register `VF_BASE_ADDRESS_2` writer"]
pub type W = crate::W<VfBaseAddress2Spec>;
#[doc = "Field `MSI` reader - Memory Space Indicator \\[MSI\\]
Specifies whether this BAR defines a memory address range or an I/O address range (0 = memory, 1 = I/O). The value read in this field is determined by the setting of BAR Configuration Registers of the associated Physical Function"]
pub type MsiR = crate::BitReader;
#[doc = "Field `R7` reader - Reserved \\[R7\\]
This bit is hardwired to 0 for both memory and I/O BARs."]
pub type R7R = crate::BitReader;
#[doc = "Field `S0` reader - Size \\[S0\\]
When the BAR is used to define a memory address range, this field indicates whether the address range is 32-bit or 64-bit (0 = 32- bit, 1 = 64 bit). For 64-bit address ranges, the value in BAR 1 is treated as a continuation of the base address in BAR 0. The value read in this field is determined by the setting of BAR Configuration Registers of the associated Physical Function."]
pub type S0R = crate::BitReader;
#[doc = "Field `P0` reader - Prefetchability \\[P0\\]
When the BAR is used to define a memory address range, this field declares whether data from the address range is prefetchable (0 = non- prefetchable, 1 = prefetchable). The value read in this field is determined by the setting of BAR Configuration Registers of the associated Physical Function"]
pub type P0R = crate::BitReader;
#[doc = "Field `R8` reader - Reserved \\[R8\\]
These bits are hardwired to 0"]
pub type R8R = crate::FieldReader;
#[doc = "Field `BAMR0` reader - Base Address - RO part \\[BAMR0\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub type Bamr0R = crate::FieldReader<u16>;
#[doc = "Field `BAMRW` reader - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub type BamrwR = crate::FieldReader<u16>;
#[doc = "Field `BAMRW` writer - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Memory Space Indicator \\[MSI\\]
Specifies whether this BAR defines a memory address range or an I/O address range (0 = memory, 1 = I/O). The value read in this field is determined by the setting of BAR Configuration Registers of the associated Physical Function"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved \\[R7\\]
This bit is hardwired to 0 for both memory and I/O BARs."]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Size \\[S0\\]
When the BAR is used to define a memory address range, this field indicates whether the address range is 32-bit or 64-bit (0 = 32- bit, 1 = 64 bit). For 64-bit address ranges, the value in BAR 1 is treated as a continuation of the base address in BAR 0. The value read in this field is determined by the setting of BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub fn s0(&self) -> S0R {
        S0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Prefetchability \\[P0\\]
When the BAR is used to define a memory address range, this field declares whether data from the address range is prefetchable (0 = non- prefetchable, 1 = prefetchable). The value read in this field is determined by the setting of BAR Configuration Registers of the associated Physical Function"]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved \\[R8\\]
These bits are hardwired to 0"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:21 - Base Address - RO part \\[BAMR0\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub fn bamr0(&self) -> Bamr0R {
        Bamr0R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:31 - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - Base Address - RW part \\[BAMRW\\]
This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<VfBaseAddress2Spec> {
        BamrwW::new(self, 22)
    }
}
#[doc = "VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfBaseAddress2Spec;
impl crate::RegisterSpec for VfBaseAddress2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vf_base_address_2::R`](R) reader structure"]
impl crate::Readable for VfBaseAddress2Spec {}
#[doc = "`write(|w| ..)` method takes [`vf_base_address_2::W`](W) writer structure"]
impl crate::Writable for VfBaseAddress2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VF_BASE_ADDRESS_2 to value 0x04"]
impl crate::Resettable for VfBaseAddress2Spec {
    const RESET_VALUE: u32 = 0x04;
}