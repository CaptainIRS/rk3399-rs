#[doc = "Register `DDR_DENALI_PHY_931` reader"]
pub type R = crate::R<DdrDenaliPhy931Spec>;
#[doc = "Register `DDR_DENALI_PHY_931` writer"]
pub type W = crate::W<DdrDenaliPhy931Spec>;
#[doc = "Field `PHY_PAD_DATA_TERM` reader - Controls term settings for data pads."]
pub type PhyPadDataTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_DATA_TERM` writer - Controls term settings for data pads."]
pub type PhyPadDataTermW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Controls term settings for data pads."]
    #[inline(always)]
    pub fn phy_pad_data_term(&self) -> PhyPadDataTermR {
        PhyPadDataTermR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Controls term settings for data pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_data_term(&mut self) -> PhyPadDataTermW<DdrDenaliPhy931Spec> {
        PhyPadDataTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_931::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_931::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy931Spec;
impl crate::RegisterSpec for DdrDenaliPhy931Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_931::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy931Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_931::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy931Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_931 to value 0x4410"]
impl crate::Resettable for DdrDenaliPhy931Spec {
    const RESET_VALUE: u32 = 0x4410;
}