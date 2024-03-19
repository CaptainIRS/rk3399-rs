#[doc = "Register `DDR_DENALI_PHY_322` reader"]
pub type R = crate::R<DdrDenaliPhy322Spec>;
#[doc = "Register `DDR_DENALI_PHY_322` writer"]
pub type W = crate::W<DdrDenaliPhy322Spec>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_2` reader - Read DQ4 slave delay setting for slice 2."]
pub type PhyRddq4SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_2` writer - Read DQ4 slave delay setting for slice 2."]
pub type PhyRddq4SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_2` reader - Read DQ5 slave delay setting for slice 2."]
pub type PhyRddq5SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_2` writer - Read DQ5 slave delay setting for slice 2."]
pub type PhyRddq5SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ4 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq4_slave_delay_2(&self) -> PhyRddq4SlaveDelay2R {
        PhyRddq4SlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ5 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq5_slave_delay_2(&self) -> PhyRddq5SlaveDelay2R {
        PhyRddq5SlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ4 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq4_slave_delay_2(&mut self) -> PhyRddq4SlaveDelay2W<DdrDenaliPhy322Spec> {
        PhyRddq4SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ5 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq5_slave_delay_2(&mut self) -> PhyRddq5SlaveDelay2W<DdrDenaliPhy322Spec> {
        PhyRddq5SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_322::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_322::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy322Spec;
impl crate::RegisterSpec for DdrDenaliPhy322Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_322::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy322Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_322::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy322Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_322 to value 0"]
impl crate::Resettable for DdrDenaliPhy322Spec {
    const RESET_VALUE: u32 = 0;
}
