#[doc = "Register `DDR_PI_REG_119` reader"]
pub type R = crate::R<DdrPiReg119Spec>;
#[doc = "Register `DDR_PI_REG_119` writer"]
pub type W = crate::W<DdrPiReg119Spec>;
#[doc = "Field `PI_WDQLVL_RESP_MASK` reader - Indicates write DQ training response mask."]
pub type PiWdqlvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_RESP_MASK` writer - Indicates write DQ training response mask."]
pub type PiWdqlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WDQLVL_ROTATE` reader - Enables write DQ training rotate for periodic training."]
pub type PiWdqlvlRotateR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_ROTATE` writer - Enables write DQ training rotate for periodic training."]
pub type PiWdqlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_CS_MAP` reader - Indicates map of chip selects that are included in write DQ training\n\nsequence."]
pub type PiWdqlvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_CS_MAP` writer - Indicates map of chip selects that are included in write DQ training\n\nsequence."]
pub type PiWdqlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_START_POINT` reader - Indicates write DQ training VREF start value."]
pub type PiWdqlvlVrefInitialStartPointR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_START_POINT` writer - Indicates write DQ training VREF start value."]
pub type PiWdqlvlVrefInitialStartPointW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:3 - Indicates write DQ training response mask."]
    #[inline(always)]
    pub fn pi_wdqlvl_resp_mask(&self) -> PiWdqlvlRespMaskR {
        PiWdqlvlRespMaskR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enables write DQ training rotate for periodic training."]
    #[inline(always)]
    pub fn pi_wdqlvl_rotate(&self) -> PiWdqlvlRotateR {
        PiWdqlvlRotateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Indicates map of chip selects that are included in write DQ training\n\nsequence."]
    #[inline(always)]
    pub fn pi_wdqlvl_cs_map(&self) -> PiWdqlvlCsMapR {
        PiWdqlvlCsMapR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:30 - Indicates write DQ training VREF start value."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_start_point(&self) -> PiWdqlvlVrefInitialStartPointR {
        PiWdqlvlVrefInitialStartPointR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates write DQ training response mask."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_resp_mask(&mut self) -> PiWdqlvlRespMaskW<DdrPiReg119Spec> {
        PiWdqlvlRespMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables write DQ training rotate for periodic training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_rotate(&mut self) -> PiWdqlvlRotateW<DdrPiReg119Spec> {
        PiWdqlvlRotateW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Indicates map of chip selects that are included in write DQ training\n\nsequence."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cs_map(&mut self) -> PiWdqlvlCsMapW<DdrPiReg119Spec> {
        PiWdqlvlCsMapW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Indicates write DQ training VREF start value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_start_point(
        &mut self,
    ) -> PiWdqlvlVrefInitialStartPointW<DdrPiReg119Spec> {
        PiWdqlvlVrefInitialStartPointW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_119::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_119::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg119Spec;
impl crate::RegisterSpec for DdrPiReg119Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_119::R`](R) reader structure"]
impl crate::Readable for DdrPiReg119Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_119::W`](W) writer structure"]
impl crate::Writable for DdrPiReg119Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_119 to value 0"]
impl crate::Resettable for DdrPiReg119Spec {
    const RESET_VALUE: u32 = 0;
}
