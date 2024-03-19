#[doc = "Register `DDR_DENALI_PHY_89` reader"]
pub type R = crate::R<DdrDenaliPhy89Spec>;
#[doc = "Register `DDR_DENALI_PHY_89` writer"]
pub type W = crate::W<DdrDenaliPhy89Spec>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_0` reader - Interim backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlBackStep0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_0` writer - Interim backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlBackStep0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_0` reader - Final backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlFinalStep0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_0` writer - Final backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlFinalStep0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_back_step_0(&self) -> PhyGtlvlBackStep0R {
        PhyGtlvlBackStep0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_final_step_0(&self) -> PhyGtlvlFinalStep0R {
        PhyGtlvlFinalStep0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_back_step_0(&mut self) -> PhyGtlvlBackStep0W<DdrDenaliPhy89Spec> {
        PhyGtlvlBackStep0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_final_step_0(&mut self) -> PhyGtlvlFinalStep0W<DdrDenaliPhy89Spec> {
        PhyGtlvlFinalStep0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_89::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_89::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy89Spec;
impl crate::RegisterSpec for DdrDenaliPhy89Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_89::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy89Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_89::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy89Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_89 to value 0"]
impl crate::Resettable for DdrDenaliPhy89Spec {
    const RESET_VALUE: u32 = 0;
}
