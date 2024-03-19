#[doc = "Register `DDR_DENALI_PHY_217` reader"]
pub type R = crate::R<DdrDenaliPhy217Spec>;
#[doc = "Register `DDR_DENALI_PHY_217` writer"]
pub type W = crate::W<DdrDenaliPhy217Spec>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_1` reader - Interim backup step delay used in gate training algorithm for slice 1."]
pub type PhyGtlvlBackStep1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_1` writer - Interim backup step delay used in gate training algorithm for slice 1."]
pub type PhyGtlvlBackStep1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_1` reader - Final backup step delay used in gate training algorithm for slice 1."]
pub type PhyGtlvlFinalStep1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_1` writer - Final backup step delay used in gate training algorithm for slice 1."]
pub type PhyGtlvlFinalStep1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_back_step_1(&self) -> PhyGtlvlBackStep1R {
        PhyGtlvlBackStep1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_final_step_1(&self) -> PhyGtlvlFinalStep1R {
        PhyGtlvlFinalStep1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_back_step_1(&mut self) -> PhyGtlvlBackStep1W<DdrDenaliPhy217Spec> {
        PhyGtlvlBackStep1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_final_step_1(&mut self) -> PhyGtlvlFinalStep1W<DdrDenaliPhy217Spec> {
        PhyGtlvlFinalStep1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_217::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_217::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy217Spec;
impl crate::RegisterSpec for DdrDenaliPhy217Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_217::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy217Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_217::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy217Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_217 to value 0"]
impl crate::Resettable for DdrDenaliPhy217Spec {
    const RESET_VALUE: u32 = 0;
}
