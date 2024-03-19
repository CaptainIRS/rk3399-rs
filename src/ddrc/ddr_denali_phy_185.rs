#[doc = "Register `DDR_DENALI_PHY_185` reader"]
pub type R = crate::R<DdrDenaliPhy185Spec>;
#[doc = "Register `DDR_DENALI_PHY_185` writer"]
pub type W = crate::W<DdrDenaliPhy185Spec>;
#[doc = "Field `PHY_RX_CAL_DM_1` reader - RX Calibration codes for DM for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDm1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DM_1` writer - RX Calibration codes for DM for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDm1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQS_1` reader - RX Calibration codes for DQS for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDqs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQS_1` writer - RX Calibration codes for DQS for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDqs1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DM for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dm_1(&self) -> PhyRxCalDm1R {
        PhyRxCalDm1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQS for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dqs_1(&self) -> PhyRxCalDqs1R {
        PhyRxCalDqs1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DM for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dm_1(&mut self) -> PhyRxCalDm1W<DdrDenaliPhy185Spec> {
        PhyRxCalDm1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQS for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dqs_1(&mut self) -> PhyRxCalDqs1W<DdrDenaliPhy185Spec> {
        PhyRxCalDqs1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_185::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_185::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy185Spec;
impl crate::RegisterSpec for DdrDenaliPhy185Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_185::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy185Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_185::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy185Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_185 to value 0"]
impl crate::Resettable for DdrDenaliPhy185Spec {
    const RESET_VALUE: u32 = 0;
}
