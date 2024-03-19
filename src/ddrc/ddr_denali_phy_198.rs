#[doc = "Register `DDR_DENALI_PHY_198` reader"]
pub type R = crate::R<DdrDenaliPhy198Spec>;
#[doc = "Register `DDR_DENALI_PHY_198` writer"]
pub type W = crate::W<DdrDenaliPhy198Spec>;
#[doc = "Field `PHY_RDDQS_DQ1_FALL_SLAVE_DELAY_1` reader - Falling edge read DQS slave delay setting for DQ1 for slice 1."]
pub type PhyRddqsDq1FallSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ1_FALL_SLAVE_DELAY_1` writer - Falling edge read DQS slave delay setting for DQ1 for slice 1."]
pub type PhyRddqsDq1FallSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ2_RISE_SLAVE_DELAY_1` reader - Rising edge read DQS slave delay setting for DQ2 for slice 1."]
pub type PhyRddqsDq2RiseSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ2_RISE_SLAVE_DELAY_1` writer - Rising edge read DQS slave delay setting for DQ2 for slice 1."]
pub type PhyRddqsDq2RiseSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq1_fall_slave_delay_1(&self) -> PhyRddqsDq1FallSlaveDelay1R {
        PhyRddqsDq1FallSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ2 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq2_rise_slave_delay_1(&self) -> PhyRddqsDq2RiseSlaveDelay1R {
        PhyRddqsDq2RiseSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq1_fall_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq1FallSlaveDelay1W<DdrDenaliPhy198Spec> {
        PhyRddqsDq1FallSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ2 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq2_rise_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq2RiseSlaveDelay1W<DdrDenaliPhy198Spec> {
        PhyRddqsDq2RiseSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_198::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_198::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy198Spec;
impl crate::RegisterSpec for DdrDenaliPhy198Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_198::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy198Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_198::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy198Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_198 to value 0"]
impl crate::Resettable for DdrDenaliPhy198Spec {
    const RESET_VALUE: u32 = 0;
}
