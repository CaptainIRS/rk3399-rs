#[doc = "Register `DDR_DENALI_PHY_671` reader"]
pub type R = crate::R<DdrDenaliPhy671Spec>;
#[doc = "Register `DDR_DENALI_PHY_671` writer"]
pub type W = crate::W<DdrDenaliPhy671Spec>;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_1` reader - Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in training."]
pub type PhyAdrCalvlTrainMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_1` writer - Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in training."]
pub type PhyAdrCalvlTrainMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in training."]
    #[inline(always)]
    pub fn phy_adr_calvl_train_mask_1(&self) -> PhyAdrCalvlTrainMask1R {
        PhyAdrCalvlTrainMask1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_train_mask_1(&mut self) -> PhyAdrCalvlTrainMask1W<DdrDenaliPhy671Spec> {
        PhyAdrCalvlTrainMask1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_671::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_671::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy671Spec;
impl crate::RegisterSpec for DdrDenaliPhy671Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_671::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy671Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_671::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy671Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_671 to value 0"]
impl crate::Resettable for DdrDenaliPhy671Spec {
    const RESET_VALUE: u32 = 0;
}
