#[doc = "Register `DDR_PI_REG_94` reader"]
pub type R = crate::R<DdrPiReg94Spec>;
#[doc = "Register `DDR_PI_REG_94` writer"]
pub type W = crate::W<DdrPiReg94Spec>;
#[doc = "Field `PI_CALVL_ON_SREF_EXIT` reader - Enables automatic CA training on a self-refresh exit. Set to 1 to\n\nenable."]
pub type PiCalvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_CALVL_ON_SREF_EXIT` writer - Enables automatic CA training on a self-refresh exit. Set to 1 to\n\nenable."]
pub type PiCalvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_ROTATE` reader - Enables rotational chip select for interval CA training. Set to 1 for\n\nrotating CS."]
pub type PiCalvlRotateR = crate::BitReader;
#[doc = "Field `PI_CALVL_ROTATE` writer - Enables rotational chip select for interval CA training. Set to 1 for\n\nrotating CS."]
pub type PiCalvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_CS_MAP` reader - Defines the chip select map for CA training operations. Bit0 controls\n\ncs0, bit1 controls cs1. Set each bit to 1 to enable chip for CA\n\ntraining."]
pub type PiCalvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_CALVL_CS_MAP` writer - Defines the chip select map for CA training operations. Bit0 controls\n\ncs0, bit1 controls cs1. Set each bit to 1 to enable chip for CA\n\ntraining."]
pub type PiCalvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TDFI_CALVL_EN` reader - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles between a dfi_calvl_en assertion and a dfi_cke\n\nde-assertion."]
pub type PiTdfiCalvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_EN` writer - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles between a dfi_calvl_en assertion and a dfi_cke\n\nde-assertion."]
pub type PiTdfiCalvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enables automatic CA training on a self-refresh exit. Set to 1 to\n\nenable."]
    #[inline(always)]
    pub fn pi_calvl_on_sref_exit(&self) -> PiCalvlOnSrefExitR {
        PiCalvlOnSrefExitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables rotational chip select for interval CA training. Set to 1 for\n\nrotating CS."]
    #[inline(always)]
    pub fn pi_calvl_rotate(&self) -> PiCalvlRotateR {
        PiCalvlRotateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Defines the chip select map for CA training operations. Bit0 controls\n\ncs0, bit1 controls cs1. Set each bit to 1 to enable chip for CA\n\ntraining."]
    #[inline(always)]
    pub fn pi_calvl_cs_map(&self) -> PiCalvlCsMapR {
        PiCalvlCsMapR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles between a dfi_calvl_en assertion and a dfi_cke\n\nde-assertion."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_en(&self) -> PiTdfiCalvlEnR {
        PiTdfiCalvlEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables automatic CA training on a self-refresh exit. Set to 1 to\n\nenable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_on_sref_exit(&mut self) -> PiCalvlOnSrefExitW<DdrPiReg94Spec> {
        PiCalvlOnSrefExitW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables rotational chip select for interval CA training. Set to 1 for\n\nrotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_rotate(&mut self) -> PiCalvlRotateW<DdrPiReg94Spec> {
        PiCalvlRotateW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Defines the chip select map for CA training operations. Bit0 controls\n\ncs0, bit1 controls cs1. Set each bit to 1 to enable chip for CA\n\ntraining."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_cs_map(&mut self) -> PiCalvlCsMapW<DdrPiReg94Spec> {
        PiCalvlCsMapW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles between a dfi_calvl_en assertion and a dfi_cke\n\nde-assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_en(&mut self) -> PiTdfiCalvlEnW<DdrPiReg94Spec> {
        PiTdfiCalvlEnW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_94::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_94::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg94Spec;
impl crate::RegisterSpec for DdrPiReg94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_94::R`](R) reader structure"]
impl crate::Readable for DdrPiReg94Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_94::W`](W) writer structure"]
impl crate::Writable for DdrPiReg94Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_94 to value 0"]
impl crate::Resettable for DdrPiReg94Spec {
    const RESET_VALUE: u32 = 0;
}
