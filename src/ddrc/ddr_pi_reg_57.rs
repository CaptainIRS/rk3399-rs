#[doc = "Register `DDR_PI_REG_57` reader"]
pub type R = crate::R<DdrPiReg57Spec>;
#[doc = "Register `DDR_PI_REG_57` writer"]
pub type W = crate::W<DdrPiReg57Spec>;
#[doc = "Field `PI_SWLVL_SM2_RD` writer - Indicates software leveling read command for stage 2."]
pub type PiSwlvlSm2RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SEQUENTIAL_LVL_REQ` writer - Indicates user request to initiate the leveling sequence. Set to 1 to\n\ntrigger."]
pub type PiSequentialLvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DFS_PERIOD_EN` reader - Enables the DFS-triggered periodic leveling."]
pub type PiDfsPeriodEnR = crate::BitReader;
#[doc = "Field `PI_DFS_PERIOD_EN` writer - Enables the DFS-triggered periodic leveling."]
pub type PiDfsPeriodEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SRE_PERIOD_EN` reader - Enables the selfrefresh exit-triggered periodic leveling."]
pub type PiSrePeriodEnR = crate::BitReader;
#[doc = "Field `PI_SRE_PERIOD_EN` writer - Enables the selfrefresh exit-triggered periodic leveling."]
pub type PiSrePeriodEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Enables the DFS-triggered periodic leveling."]
    #[inline(always)]
    pub fn pi_dfs_period_en(&self) -> PiDfsPeriodEnR {
        PiDfsPeriodEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the selfrefresh exit-triggered periodic leveling."]
    #[inline(always)]
    pub fn pi_sre_period_en(&self) -> PiSrePeriodEnR {
        PiSrePeriodEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates software leveling read command for stage 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_rd(&mut self) -> PiSwlvlSm2RdW<DdrPiReg57Spec> {
        PiSwlvlSm2RdW::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates user request to initiate the leveling sequence. Set to 1 to\n\ntrigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sequential_lvl_req(&mut self) -> PiSequentialLvlReqW<DdrPiReg57Spec> {
        PiSequentialLvlReqW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the DFS-triggered periodic leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfs_period_en(&mut self) -> PiDfsPeriodEnW<DdrPiReg57Spec> {
        PiDfsPeriodEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables the selfrefresh exit-triggered periodic leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sre_period_en(&mut self) -> PiSrePeriodEnW<DdrPiReg57Spec> {
        PiSrePeriodEnW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg57Spec;
impl crate::RegisterSpec for DdrPiReg57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_57::R`](R) reader structure"]
impl crate::Readable for DdrPiReg57Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_57::W`](W) writer structure"]
impl crate::Writable for DdrPiReg57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_57 to value 0"]
impl crate::Resettable for DdrPiReg57Spec {
    const RESET_VALUE: u32 = 0;
}
