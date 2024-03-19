#[doc = "Register `DDR_DENALI_PHY_934` reader"]
pub type R = crate::R<DdrDenaliPhy934Spec>;
#[doc = "Register `DDR_DENALI_PHY_934` writer"]
pub type W = crate::W<DdrDenaliPhy934Spec>;
#[doc = "Field `PHY_PAD_CLK_TERM` reader - Controls term settings for clock pads."]
pub type PhyPadClkTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CLK_TERM` writer - Controls term settings for clock pads."]
pub type PhyPadClkTermW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Controls term settings for clock pads."]
    #[inline(always)]
    pub fn phy_pad_clk_term(&self) -> PhyPadClkTermR {
        PhyPadClkTermR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Controls term settings for clock pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_clk_term(&mut self) -> PhyPadClkTermW<DdrDenaliPhy934Spec> {
        PhyPadClkTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_934::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_934::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy934Spec;
impl crate::RegisterSpec for DdrDenaliPhy934Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_934::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy934Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_934::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy934Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_934 to value 0x4410"]
impl crate::Resettable for DdrDenaliPhy934Spec {
    const RESET_VALUE: u32 = 0x4410;
}
