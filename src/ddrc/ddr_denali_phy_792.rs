#[doc = "Register `DDR_DENALI_PHY_792` reader"]
pub type R = crate::R<DdrDenaliPhy792Spec>;
#[doc = "Register `DDR_DENALI_PHY_792` writer"]
pub type W = crate::W<DdrDenaliPhy792Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_1_2` reader - CA training background pattern 1 for address slice 2."]
pub type PhyAdrCalvlBg1_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_1_2` writer - CA training background pattern 1 for address slice 2."]
pub type PhyAdrCalvlBg1_2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 1 for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_1_2(&self) -> PhyAdrCalvlBg1_2R {
        PhyAdrCalvlBg1_2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 1 for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_1_2(&mut self) -> PhyAdrCalvlBg1_2W<DdrDenaliPhy792Spec> {
        PhyAdrCalvlBg1_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_792::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_792::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy792Spec;
impl crate::RegisterSpec for DdrDenaliPhy792Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_792::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy792Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_792::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy792Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_792 to value 0"]
impl crate::Resettable for DdrDenaliPhy792Spec {
    const RESET_VALUE: u32 = 0;
}
