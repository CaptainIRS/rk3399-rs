#[doc = "Register `GP_CONF1` reader"]
pub type R = crate::R<GpConf1Spec>;
#[doc = "Register `GP_CONF1` writer"]
pub type W = crate::W<GpConf1Spec>;
#[doc = "Field `CH_IN_EN` reader - Each bit controls the enabling of the respective audio\n\nchannel. For instance, bit 1, when set (1'b1), the\n\naudio Channel 1 is enabled. When cleared, the\n\nreferred channel is disabled."]
pub type ChInEnR = crate::FieldReader;
#[doc = "Field `CH_IN_EN` writer - Each bit controls the enabling of the respective audio\n\nchannel. For instance, bit 1, when set (1'b1), the\n\naudio Channel 1 is enabled. When cleared, the\n\nreferred channel is disabled."]
pub type ChInEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Each bit controls the enabling of the respective audio\n\nchannel. For instance, bit 1, when set (1'b1), the\n\naudio Channel 1 is enabled. When cleared, the\n\nreferred channel is disabled."]
    #[inline(always)]
    pub fn ch_in_en(&self) -> ChInEnR {
        ChInEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Each bit controls the enabling of the respective audio\n\nchannel. For instance, bit 1, when set (1'b1), the\n\naudio Channel 1 is enabled. When cleared, the\n\nreferred channel is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch_in_en(&mut self) -> ChInEnW<GpConf1Spec> {
        ChInEnW::new(self, 0)
    }
}
#[doc = "Audio GPA Channel Enable Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpConf1Spec;
impl crate::RegisterSpec for GpConf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gp_conf1::R`](R) reader structure"]
impl crate::Readable for GpConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`gp_conf1::W`](W) writer structure"]
impl crate::Writable for GpConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GP_CONF1 to value 0"]
impl crate::Resettable for GpConf1Spec {
    const RESET_VALUE: u8 = 0;
}
