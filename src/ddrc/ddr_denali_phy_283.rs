#[doc = "Register `DDR_DENALI_PHY_283` reader"]
pub type R = crate::R<DdrDenaliPhy283Spec>;
#[doc = "Register `DDR_DENALI_PHY_283` writer"]
pub type W = crate::W<DdrDenaliPhy283Spec>;
#[doc = "Field `PHY_WDQLVL_DATADM_MASK_2` reader - Per-bit mask for write data leveling for slice 2. Set to 1 to mask any bit from the leveling process."]
pub type PhyWdqlvlDatadmMask2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DATADM_MASK_2` writer - Per-bit mask for write data leveling for slice 2. Set to 1 to mask any bit from the leveling process."]
pub type PhyWdqlvlDatadmMask2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Per-bit mask for write data leveling for slice 2. Set to 1 to mask any bit from the leveling process."]
    #[inline(always)]
    pub fn phy_wdqlvl_datadm_mask_2(&self) -> PhyWdqlvlDatadmMask2R {
        PhyWdqlvlDatadmMask2R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Per-bit mask for write data leveling for slice 2. Set to 1 to mask any bit from the leveling process."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_datadm_mask_2(&mut self) -> PhyWdqlvlDatadmMask2W<DdrDenaliPhy283Spec> {
        PhyWdqlvlDatadmMask2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_283::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_283::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy283Spec;
impl crate::RegisterSpec for DdrDenaliPhy283Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_283::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy283Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_283::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy283Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_283 to value 0"]
impl crate::Resettable for DdrDenaliPhy283Spec {
    const RESET_VALUE: u32 = 0;
}
