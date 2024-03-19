#[doc = "Register `PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0` reader"]
pub type R = crate::R<PcieLmTransmitCreditUpdateIntervalConfiguration0Spec>;
#[doc = "Register `PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0` writer"]
pub type W = crate::W<PcieLmTransmitCreditUpdateIntervalConfiguration0Spec>;
#[doc = "Field `MPUI` reader - Minimum Posted Update Interval \\[MPUI\\]
Minimum credit update interval for posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub type MpuiR = crate::FieldReader<u16>;
#[doc = "Field `MPUI` writer - Minimum Posted Update Interval \\[MPUI\\]
Minimum credit update interval for posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub type MpuiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MNUI` reader - Minimum Non- Posted Update Interval \\[MNUI\\]
Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub type MnuiR = crate::FieldReader<u16>;
#[doc = "Field `MNUI` writer - Minimum Non- Posted Update Interval \\[MNUI\\]
Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub type MnuiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum Posted Update Interval \\[MPUI\\]
Minimum credit update interval for posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub fn mpui(&self) -> MpuiR {
        MpuiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Minimum Non- Posted Update Interval \\[MNUI\\]
Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub fn mnui(&self) -> MnuiR {
        MnuiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum Posted Update Interval \\[MPUI\\]
Minimum credit update interval for posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    #[must_use]
    pub fn mpui(&mut self) -> MpuiW<PcieLmTransmitCreditUpdateIntervalConfiguration0Spec> {
        MpuiW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Minimum Non- Posted Update Interval \\[MNUI\\]
Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    #[must_use]
    pub fn mnui(&mut self) -> MnuiW<PcieLmTransmitCreditUpdateIntervalConfiguration0Spec> {
        MnuiW::new(self, 16)
    }
}
#[doc = "Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_update_interval_configuration_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_credit_update_interval_configuration_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmTransmitCreditUpdateIntervalConfiguration0Spec;
impl crate::RegisterSpec for PcieLmTransmitCreditUpdateIntervalConfiguration0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_transmit_credit_update_interval_configuration_0::R`](R) reader structure"]
impl crate::Readable for PcieLmTransmitCreditUpdateIntervalConfiguration0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_transmit_credit_update_interval_configuration_0::W`](W) writer structure"]
impl crate::Writable for PcieLmTransmitCreditUpdateIntervalConfiguration0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0 to value 0x0010_0010"]
impl crate::Resettable for PcieLmTransmitCreditUpdateIntervalConfiguration0Spec {
    const RESET_VALUE: u32 = 0x0010_0010;
}
