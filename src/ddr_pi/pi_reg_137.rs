#[doc = "Register `PI_REG_137` reader"]
pub type R = crate::R<PiReg137Spec>;
#[doc = "Register `PI_REG_137` writer"]
pub type W = crate::W<PiReg137Spec>;
#[doc = "Field `PI_MR11_DATA_F1_1` reader - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr11DataF1_1R = crate::FieldReader;
#[doc = "Field `PI_MR11_DATA_F1_1` writer - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr11DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR12_DATA_F1_1` reader - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr12DataF1_1R = crate::FieldReader;
#[doc = "Field `PI_MR12_DATA_F1_1` writer - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr12DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR14_DATA_F1_1` reader - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF1_1R = crate::FieldReader;
#[doc = "Field `PI_MR14_DATA_F1_1` writer - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr11_data_f1_1(&self) -> PiMr11DataF1_1R {
        PiMr11DataF1_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr12_data_f1_1(&self) -> PiMr12DataF1_1R {
        PiMr12DataF1_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr14_data_f1_1(&self) -> PiMr14DataF1_1R {
        PiMr14DataF1_1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr11_data_f1_1(&mut self) -> PiMr11DataF1_1W<PiReg137Spec> {
        PiMr11DataF1_1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr12_data_f1_1(&mut self) -> PiMr12DataF1_1W<PiReg137Spec> {
        PiMr12DataF1_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr14_data_f1_1(&mut self) -> PiMr14DataF1_1W<PiReg137Spec> {
        PiMr14DataF1_1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 137\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_137::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_137::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg137Spec;
impl crate::RegisterSpec for PiReg137Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_137::R`](R) reader structure"]
impl crate::Readable for PiReg137Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_137::W`](W) writer structure"]
impl crate::Writable for PiReg137Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_137 to value 0"]
impl crate::Resettable for PiReg137Spec {
    const RESET_VALUE: u32 = 0;
}