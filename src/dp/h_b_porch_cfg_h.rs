#[doc = "Register `H_B_PORCH_CFG_H` reader"]
pub type R = crate::R<HBPorchCfgHSpec>;
#[doc = "Register `H_B_PORCH_CFG_H` writer"]
pub type W = crate::W<HBPorchCfgHSpec>;
#[doc = "Field `H_B_PORCH_CFG_H` reader - H_B_PORCH_CFG is used to specify the number \n\nof pixel in frame horizon back porch part. This \n\nregister is H_B_PORCH_CFG \\[11:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type HBPorchCfgHR = crate::FieldReader;
#[doc = "Field `H_B_PORCH_CFG_H` writer - H_B_PORCH_CFG is used to specify the number \n\nof pixel in frame horizon back porch part. This \n\nregister is H_B_PORCH_CFG \\[11:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type HBPorchCfgHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - H_B_PORCH_CFG is used to specify the number \n\nof pixel in frame horizon back porch part. This \n\nregister is H_B_PORCH_CFG \\[11:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    pub fn h_b_porch_cfg_h(&self) -> HBPorchCfgHR {
        HBPorchCfgHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - H_B_PORCH_CFG is used to specify the number \n\nof pixel in frame horizon back porch part. This \n\nregister is H_B_PORCH_CFG \\[11:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn h_b_porch_cfg_h(&mut self) -> HBPorchCfgHW<HBPorchCfgHSpec> {
        HBPorchCfgHW::new(self, 0)
    }
}
#[doc = "Horizon Back Porch High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_cfg_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_cfg_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBPorchCfgHSpec;
impl crate::RegisterSpec for HBPorchCfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_b_porch_cfg_h::R`](R) reader structure"]
impl crate::Readable for HBPorchCfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`h_b_porch_cfg_h::W`](W) writer structure"]
impl crate::Writable for HBPorchCfgHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets H_B_PORCH_CFG_H to value 0"]
impl crate::Resettable for HBPorchCfgHSpec {
    const RESET_VALUE: u32 = 0;
}
