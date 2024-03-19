#[doc = "Register `DDR_DENALI_PHY_460` reader"]
pub type R = crate::R<DdrDenaliPhy460Spec>;
#[doc = "Register `DDR_DENALI_PHY_460` writer"]
pub type W = crate::W<DdrDenaliPhy460Spec>;
#[doc = "Field `PHY_RDDQS_DQ7_FALL_SLAVE_DELAY_3` reader - Falling edge read DQS slave delay setting for DQ7 for slice 3."]
pub type PhyRddqsDq7FallSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ7_FALL_SLAVE_DELAY_3` writer - Falling edge read DQS slave delay setting for DQ7 for slice 3."]
pub type PhyRddqsDq7FallSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DM_RISE_SLAVE_DELAY_3` reader - Rising edge read DQS slave delay setting for DM for slice 3."]
pub type PhyRddqsDmRiseSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DM_RISE_SLAVE_DELAY_3` writer - Rising edge read DQS slave delay setting for DM for slice 3."]
pub type PhyRddqsDmRiseSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ7 for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq7_fall_slave_delay_3(&self) -> PhyRddqsDq7FallSlaveDelay3R {
        PhyRddqsDq7FallSlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DM for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dm_rise_slave_delay_3(&self) -> PhyRddqsDmRiseSlaveDelay3R {
        PhyRddqsDmRiseSlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ7 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq7_fall_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDq7FallSlaveDelay3W<DdrDenaliPhy460Spec> {
        PhyRddqsDq7FallSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DM for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dm_rise_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDmRiseSlaveDelay3W<DdrDenaliPhy460Spec> {
        PhyRddqsDmRiseSlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_460::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_460::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy460Spec;
impl crate::RegisterSpec for DdrDenaliPhy460Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_460::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy460Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_460::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy460Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_460 to value 0"]
impl crate::Resettable for DdrDenaliPhy460Spec {
    const RESET_VALUE: u32 = 0;
}
