#[doc = "Register `PI_REG_92` reader"]
pub type R = crate::R<PiReg92Spec>;
#[doc = "Register `PI_REG_92` writer"]
pub type W = crate::W<PiReg92Spec>;
#[doc = "Field `PI_TDFI_WRCSLAT_F2` reader - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiWrcslatF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRCSLAT_F2` writer - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiWrcslatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_PHY_WRDATA` reader - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataR = crate::FieldReader;
#[doc = "Field `PI_TDFI_PHY_WRDATA` writer - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_CALVL_REQ` writer - Indicates user request to initiate CA training. Set to 1 to trigger."]
pub type PiCalvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_CS` reader - Specifies the target chip select for the CA training operation that is initiated through the PI_REG_92.pi_calvl_req parameter."]
pub type PiCalvlCsR = crate::FieldReader;
#[doc = "Field `PI_CALVL_CS` writer - Specifies the target chip select for the CA training operation that is initiated through the PI_REG_92.pi_calvl_req parameter."]
pub type PiCalvlCsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_wrcslat_f2(&self) -> PiTdfiWrcslatF2R {
        PiTdfiWrcslatF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    pub fn pi_tdfi_phy_wrdata(&self) -> PiTdfiPhyWrdataR {
        PiTdfiPhyWrdataR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Specifies the target chip select for the CA training operation that is initiated through the PI_REG_92.pi_calvl_req parameter."]
    #[inline(always)]
    pub fn pi_calvl_cs(&self) -> PiCalvlCsR {
        PiCalvlCsR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrcslat_f2(&mut self) -> PiTdfiWrcslatF2W<PiReg92Spec> {
        PiTdfiWrcslatF2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phy_wrdata(&mut self) -> PiTdfiPhyWrdataW<PiReg92Spec> {
        PiTdfiPhyWrdataW::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates user request to initiate CA training. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_req(&mut self) -> PiCalvlReqW<PiReg92Spec> {
        PiCalvlReqW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Specifies the target chip select for the CA training operation that is initiated through the PI_REG_92.pi_calvl_req parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_cs(&mut self) -> PiCalvlCsW<PiReg92Spec> {
        PiCalvlCsW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg92Spec;
impl crate::RegisterSpec for PiReg92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_92::R`](R) reader structure"]
impl crate::Readable for PiReg92Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_92::W`](W) writer structure"]
impl crate::Writable for PiReg92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_92 to value 0"]
impl crate::Resettable for PiReg92Spec {
    const RESET_VALUE: u32 = 0;
}