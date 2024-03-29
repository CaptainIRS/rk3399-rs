#[doc = "Register `ACTIVE_LINE_CFG_L` reader"]
pub type R = crate::R<ActiveLineCfgLSpec>;
#[doc = "Register `ACTIVE_LINE_CFG_L` writer"]
pub type W = crate::W<ActiveLineCfgLSpec>;
#[doc = "Field `ACTIVE_LINE_CFG_L` reader - ACTIVE_LINE_CFG is used to specify the \n\nnumber of active lines in each frame. This \n\nregister is ACTIVE_LINE_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type ActiveLineCfgLR = crate::FieldReader;
#[doc = "Field `ACTIVE_LINE_CFG_L` writer - ACTIVE_LINE_CFG is used to specify the \n\nnumber of active lines in each frame. This \n\nregister is ACTIVE_LINE_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type ActiveLineCfgLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ACTIVE_LINE_CFG is used to specify the \n\nnumber of active lines in each frame. This \n\nregister is ACTIVE_LINE_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    pub fn active_line_cfg_l(&self) -> ActiveLineCfgLR {
        ActiveLineCfgLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ACTIVE_LINE_CFG is used to specify the \n\nnumber of active lines in each frame. This \n\nregister is ACTIVE_LINE_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn active_line_cfg_l(&mut self) -> ActiveLineCfgLW<ActiveLineCfgLSpec> {
        ActiveLineCfgLW::new(self, 0)
    }
}
#[doc = "Active Line Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_cfg_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_cfg_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveLineCfgLSpec;
impl crate::RegisterSpec for ActiveLineCfgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_line_cfg_l::R`](R) reader structure"]
impl crate::Readable for ActiveLineCfgLSpec {}
#[doc = "`write(|w| ..)` method takes [`active_line_cfg_l::W`](W) writer structure"]
impl crate::Writable for ActiveLineCfgLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ACTIVE_LINE_CFG_L to value 0"]
impl crate::Resettable for ActiveLineCfgLSpec {
    const RESET_VALUE: u32 = 0;
}
