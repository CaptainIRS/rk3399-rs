#[doc = "Register `DDR_DENALI_PHY_19` reader"]
pub type R = crate::R<DdrDenaliPhy19Spec>;
#[doc = "Register `DDR_DENALI_PHY_19` writer"]
pub type W = crate::W<DdrDenaliPhy19Spec>;
#[doc = "Field `PHY_LP4_RDLVL_PATT11_0` reader - LPDDR4 read leveling pattern 11 data for slice 0."]
pub type PhyLp4RdlvlPatt11_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_LP4_RDLVL_PATT11_0` writer - LPDDR4 read leveling pattern 11 data for slice 0."]
pub type PhyLp4RdlvlPatt11_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 11 data for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_rdlvl_patt11_0(&self) -> PhyLp4RdlvlPatt11_0R {
        PhyLp4RdlvlPatt11_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 11 data for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_rdlvl_patt11_0(&mut self) -> PhyLp4RdlvlPatt11_0W<DdrDenaliPhy19Spec> {
        PhyLp4RdlvlPatt11_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy19Spec;
impl crate::RegisterSpec for DdrDenaliPhy19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_19::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy19Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_19::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_19 to value 0"]
impl crate::Resettable for DdrDenaliPhy19Spec {
    const RESET_VALUE: u32 = 0;
}
