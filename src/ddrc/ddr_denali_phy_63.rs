#[doc = "Register `DDR_DENALI_PHY_63` reader"]
pub type R = crate::R<DdrDenaliPhy63Spec>;
#[doc = "Register `DDR_DENALI_PHY_63` writer"]
pub type W = crate::W<DdrDenaliPhy63Spec>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_0` reader - Write clock slave delay setting for DM for slice 0."]
pub type PhyClkWrdmSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_0` writer - Write clock slave delay setting for DM for slice 0."]
pub type PhyClkWrdmSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_0` reader - Write clock slave delay setting for DQS for slice 0."]
pub type PhyClkWrdqsSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_0` writer - Write clock slave delay setting for DQS for slice 0."]
pub type PhyClkWrdqsSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DM for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdm_slave_delay_0(&self) -> PhyClkWrdmSlaveDelay0R {
        PhyClkWrdmSlaveDelay0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Write clock slave delay setting for DQS for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdqs_slave_delay_0(&self) -> PhyClkWrdqsSlaveDelay0R {
        PhyClkWrdqsSlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DM for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdm_slave_delay_0(&mut self) -> PhyClkWrdmSlaveDelay0W<DdrDenaliPhy63Spec> {
        PhyClkWrdmSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Write clock slave delay setting for DQS for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdqs_slave_delay_0(&mut self) -> PhyClkWrdqsSlaveDelay0W<DdrDenaliPhy63Spec> {
        PhyClkWrdqsSlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy63Spec;
impl crate::RegisterSpec for DdrDenaliPhy63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_63::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy63Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_63::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_63 to value 0"]
impl crate::Resettable for DdrDenaliPhy63Spec {
    const RESET_VALUE: u32 = 0;
}
