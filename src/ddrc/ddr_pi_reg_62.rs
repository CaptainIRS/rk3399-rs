#[doc = "Register `DDR_PI_REG_62` reader"]
pub type R = crate::R<DdrPiReg62Spec>;
#[doc = "Register `DDR_PI_REG_62` writer"]
pub type W = crate::W<DdrPiReg62Spec>;
#[doc = "Field `PI_WRLVL_CS_MAP` reader - Defines the chip select map for write leveling operations.\n\nBit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable chip\n\nfor write leveling."]
pub type PiWrlvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_CS_MAP` writer - Defines the chip select map for write leveling operations.\n\nBit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable chip\n\nfor write leveling."]
pub type PiWrlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WRLVL_ERROR_STATUS` reader - Holds the error that is associated with the write level error\n\ninterrupt. Bit0 set indicates a PI_REG_65.pi_tdfi_wrlvl_max\n\nparameter violation and bit1 set indicates a\n\nPI_REG_64.pi_tdfi_wrlvl_resp parameter violation."]
pub type PiWrlvlErrorStatusR = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRLVL_EN` reader - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles from a dfi_wrlvl_en assertion to the first\n\ndfi_wrlvl_strobe assertion."]
pub type PiTdfiWrlvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRLVL_EN` writer - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles from a dfi_wrlvl_en assertion to the first\n\ndfi_wrlvl_strobe assertion."]
pub type PiTdfiWrlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Defines the chip select map for write leveling operations.\n\nBit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable chip\n\nfor write leveling."]
    #[inline(always)]
    pub fn pi_wrlvl_cs_map(&self) -> PiWrlvlCsMapR {
        PiWrlvlCsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Holds the error that is associated with the write level error\n\ninterrupt. Bit0 set indicates a PI_REG_65.pi_tdfi_wrlvl_max\n\nparameter violation and bit1 set indicates a\n\nPI_REG_64.pi_tdfi_wrlvl_resp parameter violation."]
    #[inline(always)]
    pub fn pi_wrlvl_error_status(&self) -> PiWrlvlErrorStatusR {
        PiWrlvlErrorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles from a dfi_wrlvl_en assertion to the first\n\ndfi_wrlvl_strobe assertion."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_en(&self) -> PiTdfiWrlvlEnR {
        PiTdfiWrlvlEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the chip select map for write leveling operations.\n\nBit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable chip\n\nfor write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_cs_map(&mut self) -> PiWrlvlCsMapW<DdrPiReg62Spec> {
        PiWrlvlCsMapW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Defines the DFI tWRLVL_EN timing parameter (in DFI clocks), the\n\nminimum cycles from a dfi_wrlvl_en assertion to the first\n\ndfi_wrlvl_strobe assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_en(&mut self) -> PiTdfiWrlvlEnW<DdrPiReg62Spec> {
        PiTdfiWrlvlEnW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg62Spec;
impl crate::RegisterSpec for DdrPiReg62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_62::R`](R) reader structure"]
impl crate::Readable for DdrPiReg62Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_62::W`](W) writer structure"]
impl crate::Writable for DdrPiReg62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_62 to value 0"]
impl crate::Resettable for DdrPiReg62Spec {
    const RESET_VALUE: u32 = 0;
}
