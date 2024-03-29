#[doc = "Register `PCIE_RC_L1_PM_SUBSTATES_CONTROL_2` reader"]
pub type R = crate::R<PcieRcL1PmSubstatesControl2Spec>;
#[doc = "Register `PCIE_RC_L1_PM_SUBSTATES_CONTROL_2` writer"]
pub type W = crate::W<PcieRcL1PmSubstatesControl2Spec>;
#[doc = "Field `L1PwrOnSc` reader - T_POWER_ON Scale \\[L1PwrOnSc\\]\n\n(no description)"]
pub type L1pwrOnScR = crate::FieldReader;
#[doc = "Field `L1PwrOnSc` writer - T_POWER_ON Scale \\[L1PwrOnSc\\]\n\n(no description)"]
pub type L1pwrOnScW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `L1PwrOnVal` reader - T_POWER_ON Value \\[L1PwrOnVal\\]\n\n(no description)"]
pub type L1pwrOnValR = crate::FieldReader;
#[doc = "Field `L1PwrOnVal` writer - T_POWER_ON Value \\[L1PwrOnVal\\]\n\n(no description)"]
pub type L1pwrOnValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - T_POWER_ON Scale \\[L1PwrOnSc\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1pwr_on_sc(&self) -> L1pwrOnScR {
        L1pwrOnScR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:7 - T_POWER_ON Value \\[L1PwrOnVal\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1pwr_on_val(&self) -> L1pwrOnValR {
        L1pwrOnValR::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - T_POWER_ON Scale \\[L1PwrOnSc\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1pwr_on_sc(&mut self) -> L1pwrOnScW<PcieRcL1PmSubstatesControl2Spec> {
        L1pwrOnScW::new(self, 0)
    }
    #[doc = "Bits 3:7 - T_POWER_ON Value \\[L1PwrOnVal\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1pwr_on_val(&mut self) -> L1pwrOnValW<PcieRcL1PmSubstatesControl2Spec> {
        L1pwrOnValW::new(self, 3)
    }
}
#[doc = "L1 PM Substates Control 2 Register\n\nRSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_l1_pm_substates_control_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_l1_pm_substates_control_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcL1PmSubstatesControl2Spec;
impl crate::RegisterSpec for PcieRcL1PmSubstatesControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_l1_pm_substates_control_2::R`](R) reader structure"]
impl crate::Readable for PcieRcL1PmSubstatesControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_l1_pm_substates_control_2::W`](W) writer structure"]
impl crate::Writable for PcieRcL1PmSubstatesControl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_L1_PM_SUBSTATES_CONTROL_2 to value 0x28"]
impl crate::Resettable for PcieRcL1PmSubstatesControl2Spec {
    const RESET_VALUE: u32 = 0x28;
}
