#[doc = "Register `DDR_DENALI_PHY_537` reader"]
pub type R = crate::R<DdrDenaliPhy537Spec>;
#[doc = "Register `DDR_DENALI_PHY_537` writer"]
pub type W = crate::W<DdrDenaliPhy537Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_2_0` reader - CA training foreground pattern 2 for address slice 0."]
pub type PhyAdrCalvlFg2_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_2_0` writer - CA training foreground pattern 2 for address slice 0."]
pub type PhyAdrCalvlFg2_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training foreground pattern 2 for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_2_0(&self) -> PhyAdrCalvlFg2_0R {
        PhyAdrCalvlFg2_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training foreground pattern 2 for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_2_0(&mut self) -> PhyAdrCalvlFg2_0W<DdrDenaliPhy537Spec> {
        PhyAdrCalvlFg2_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_537::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_537::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy537Spec;
impl crate::RegisterSpec for DdrDenaliPhy537Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_537::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy537Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_537::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy537Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_537 to value 0"]
impl crate::Resettable for DdrDenaliPhy537Spec {
    const RESET_VALUE: u32 = 0;
}
