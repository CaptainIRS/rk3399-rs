#[doc = "Register `DDR_DENALI_PHY_81` reader"]
pub type R = crate::R<DdrDenaliPhy81Spec>;
#[doc = "Register `DDR_DENALI_PHY_81` writer"]
pub type W = crate::W<DdrDenaliPhy81Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_0` reader - Initial DQ/DM slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_0` writer - Initial DQ/DM slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyStart0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_0` reader - Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
pub type PhyRdlvlRddqsDqSlvDlyStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_0` writer - Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
pub type PhyRdlvlRddqsDqSlvDlyStart0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Initial DQ/DM slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_0(&self) -> PhyWdqlvlDqdmSlvDlyStart0R {
        PhyWdqlvlDqdmSlvDlyStart0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_0(&self) -> PhyRdlvlRddqsDqSlvDlyStart0R {
        PhyRdlvlRddqsDqSlvDlyStart0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Initial DQ/DM slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_0(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyStart0W<DdrDenaliPhy81Spec> {
        PhyWdqlvlDqdmSlvDlyStart0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_0(
        &mut self,
    ) -> PhyRdlvlRddqsDqSlvDlyStart0W<DdrDenaliPhy81Spec> {
        PhyRdlvlRddqsDqSlvDlyStart0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy81Spec;
impl crate::RegisterSpec for DdrDenaliPhy81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_81::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy81Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_81::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_81 to value 0"]
impl crate::Resettable for DdrDenaliPhy81Spec {
    const RESET_VALUE: u32 = 0;
}
