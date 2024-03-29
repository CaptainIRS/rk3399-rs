#[doc = "Register `ANALOG_CTL_24` reader"]
pub type R = crate::R<AnalogCtl24Spec>;
#[doc = "Register `ANALOG_CTL_24` writer"]
pub type W = crate::W<AnalogCtl24Spec>;
#[doc = "Field `R_EMP_600MV_6DB` reader - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 6db."]
pub type REmp600mv6dbR = crate::FieldReader;
#[doc = "Field `R_EMP_600MV_6DB` writer - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 6db."]
pub type REmp600mv6dbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 6db."]
    #[inline(always)]
    pub fn r_emp_600mv_6db(&self) -> REmp600mv6dbR {
        REmp600mv6dbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The lookup-table 2(for calculating \n\nchx_pre_emp_bit) value when V_diff is \n\n600mv and Pre_emphasis is 6db."]
    #[inline(always)]
    #[must_use]
    pub fn r_emp_600mv_6db(&mut self) -> REmp600mv6dbW<AnalogCtl24Spec> {
        REmp600mv6dbW::new(self, 0)
    }
}
#[doc = "EMP_600MV_6DB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl24Spec;
impl crate::RegisterSpec for AnalogCtl24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_24::R`](R) reader structure"]
impl crate::Readable for AnalogCtl24Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_24::W`](W) writer structure"]
impl crate::Writable for AnalogCtl24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CTL_24 to value 0x78"]
impl crate::Resettable for AnalogCtl24Spec {
    const RESET_VALUE: u32 = 0x78;
}
