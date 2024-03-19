#[doc = "Register `DDR_DENALI_PHY_789` reader"]
pub type R = crate::R<DdrDenaliPhy789Spec>;
#[doc = "Register `DDR_DENALI_PHY_789` writer"]
pub type W = crate::W<DdrDenaliPhy789Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_0_2` reader - CA training foreground pattern 0 for address slice 2."]
pub type PhyAdrCalvlFg0_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_0_2` writer - CA training foreground pattern 0 for address slice 2."]
pub type PhyAdrCalvlFg0_2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training foreground pattern 0 for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_0_2(&self) -> PhyAdrCalvlFg0_2R {
        PhyAdrCalvlFg0_2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training foreground pattern 0 for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_0_2(&mut self) -> PhyAdrCalvlFg0_2W<DdrDenaliPhy789Spec> {
        PhyAdrCalvlFg0_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_789::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_789::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy789Spec;
impl crate::RegisterSpec for DdrDenaliPhy789Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_789::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy789Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_789::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy789Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_789 to value 0"]
impl crate::Resettable for DdrDenaliPhy789Spec {
    const RESET_VALUE: u32 = 0;
}
