#[doc = "Register `ANALOG_CTL_40` reader"]
pub type R = crate::R<AnalogCtl40Spec>;
#[doc = "Register `ANALOG_CTL_40` writer"]
pub type W = crate::W<AnalogCtl40Spec>;
#[doc = "Field `R_CH1_EMP_FORCE_VALUE` reader - The forced ch1 emp value (for \n\ncalculating ch1_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
pub type RCh1EmpForceValueR = crate::FieldReader;
#[doc = "Field `R_CH1_EMP_FORCE_VALUE` writer - The forced ch1 emp value (for \n\ncalculating ch1_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
pub type RCh1EmpForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The forced ch1 emp value (for \n\ncalculating ch1_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
    #[inline(always)]
    pub fn r_ch1_emp_force_value(&self) -> RCh1EmpForceValueR {
        RCh1EmpForceValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The forced ch1 emp value (for \n\ncalculating ch1_pre_emphasis_bit) value \n\nin specific V_diff and Pre_emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn r_ch1_emp_force_value(&mut self) -> RCh1EmpForceValueW<AnalogCtl40Spec> {
        RCh1EmpForceValueW::new(self, 0)
    }
}
#[doc = "CH1_EMP_FORCE_VALUE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl40Spec;
impl crate::RegisterSpec for AnalogCtl40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_40::R`](R) reader structure"]
impl crate::Readable for AnalogCtl40Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_40::W`](W) writer structure"]
impl crate::Writable for AnalogCtl40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_40 to value 0"]
impl crate::Resettable for AnalogCtl40Spec {
    const RESET_VALUE: u32 = 0;
}
