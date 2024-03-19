#[doc = "Register `DDR_DENALI_PHY_390` reader"]
pub type R = crate::R<DdrDenaliPhy390Spec>;
#[doc = "Register `DDR_DENALI_PHY_390` writer"]
pub type W = crate::W<DdrDenaliPhy390Spec>;
#[doc = "Field `PHY_DQ_TSEL_SELECT_3` reader - Operation type tsel select values for DQ/DM signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
pub type PhyDqTselSelect3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DQ_TSEL_SELECT_3` writer - Operation type tsel select values for DQ/DM signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
pub type PhyDqTselSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_DQS_TSEL_ENABLE_3` reader - Operation type tsel enables for DQS signals for slice 3. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqsTselEnable3R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_ENABLE_3` writer - Operation type tsel enables for DQS signals for slice 3. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqsTselEnable3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:23 - Operation type tsel select values for DQ/DM signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
    #[inline(always)]
    pub fn phy_dq_tsel_select_3(&self) -> PhyDqTselSelect3R {
        PhyDqTselSelect3R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:26 - Operation type tsel enables for DQS signals for slice 3. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_dqs_tsel_enable_3(&self) -> PhyDqsTselEnable3R {
        PhyDqsTselEnable3R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Operation type tsel select values for DQ/DM signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_select_3(&mut self) -> PhyDqTselSelect3W<DdrDenaliPhy390Spec> {
        PhyDqTselSelect3W::new(self, 0)
    }
    #[doc = "Bits 24:26 - Operation type tsel enables for DQS signals for slice 3. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_enable_3(&mut self) -> PhyDqsTselEnable3W<DdrDenaliPhy390Spec> {
        PhyDqsTselEnable3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_390::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_390::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy390Spec;
impl crate::RegisterSpec for DdrDenaliPhy390Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_390::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy390Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_390::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy390Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_390 to value 0"]
impl crate::Resettable for DdrDenaliPhy390Spec {
    const RESET_VALUE: u32 = 0;
}
