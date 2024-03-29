#[doc = "Register `AUD_N3` reader"]
pub type R = crate::R<AudN3Spec>;
#[doc = "Register `AUD_N3` writer"]
pub type W = crate::W<AudN3Spec>;
#[doc = "Field `AUDN` reader - HDMI Audio Clock Regenerator N value"]
pub type AudnR = crate::FieldReader;
#[doc = "Field `AUDN` writer - HDMI Audio Clock Regenerator N value"]
pub type AudnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NCTS_ATOMIC_WRITE` reader - When set, the new N and CTS values are only used\n\nwhen aud_n1 register is written. If clear, N and CTS\n\ndata is updated each time a new N or CTS byte is\n\nwritten.\n\nThe following write sequence is recommended:\n\naud_n3 (set bit ncts_atomic_write if desired)\n\naud_cts3 (set CTS_manual and CTS value if\n\ndesired/enabled)\n\naud_cts2 (required in CTS_manual)\n\naud_cts1 (required in CTS_manual)\n\naud_n3 (bit ncts_atomic_write with same value as in\n\nstep 1.)\n\naud_n2\n\naud_n1\n\nFor dynamic N/CTS changes, perform only steps\n\nfrom 2-7 or 5-7 depending on the state of\n\nCTS_manual."]
pub type NctsAtomicWriteR = crate::BitReader;
#[doc = "Field `NCTS_ATOMIC_WRITE` writer - When set, the new N and CTS values are only used\n\nwhen aud_n1 register is written. If clear, N and CTS\n\ndata is updated each time a new N or CTS byte is\n\nwritten.\n\nThe following write sequence is recommended:\n\naud_n3 (set bit ncts_atomic_write if desired)\n\naud_cts3 (set CTS_manual and CTS value if\n\ndesired/enabled)\n\naud_cts2 (required in CTS_manual)\n\naud_cts1 (required in CTS_manual)\n\naud_n3 (bit ncts_atomic_write with same value as in\n\nstep 1.)\n\naud_n2\n\naud_n1\n\nFor dynamic N/CTS changes, perform only steps\n\nfrom 2-7 or 5-7 depending on the state of\n\nCTS_manual."]
pub type NctsAtomicWriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - HDMI Audio Clock Regenerator N value"]
    #[inline(always)]
    pub fn audn(&self) -> AudnR {
        AudnR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - When set, the new N and CTS values are only used\n\nwhen aud_n1 register is written. If clear, N and CTS\n\ndata is updated each time a new N or CTS byte is\n\nwritten.\n\nThe following write sequence is recommended:\n\naud_n3 (set bit ncts_atomic_write if desired)\n\naud_cts3 (set CTS_manual and CTS value if\n\ndesired/enabled)\n\naud_cts2 (required in CTS_manual)\n\naud_cts1 (required in CTS_manual)\n\naud_n3 (bit ncts_atomic_write with same value as in\n\nstep 1.)\n\naud_n2\n\naud_n1\n\nFor dynamic N/CTS changes, perform only steps\n\nfrom 2-7 or 5-7 depending on the state of\n\nCTS_manual."]
    #[inline(always)]
    pub fn ncts_atomic_write(&self) -> NctsAtomicWriteR {
        NctsAtomicWriteR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HDMI Audio Clock Regenerator N value"]
    #[inline(always)]
    #[must_use]
    pub fn audn(&mut self) -> AudnW<AudN3Spec> {
        AudnW::new(self, 0)
    }
    #[doc = "Bit 7 - When set, the new N and CTS values are only used\n\nwhen aud_n1 register is written. If clear, N and CTS\n\ndata is updated each time a new N or CTS byte is\n\nwritten.\n\nThe following write sequence is recommended:\n\naud_n3 (set bit ncts_atomic_write if desired)\n\naud_cts3 (set CTS_manual and CTS value if\n\ndesired/enabled)\n\naud_cts2 (required in CTS_manual)\n\naud_cts1 (required in CTS_manual)\n\naud_n3 (bit ncts_atomic_write with same value as in\n\nstep 1.)\n\naud_n2\n\naud_n1\n\nFor dynamic N/CTS changes, perform only steps\n\nfrom 2-7 or 5-7 depending on the state of\n\nCTS_manual."]
    #[inline(always)]
    #[must_use]
    pub fn ncts_atomic_write(&mut self) -> NctsAtomicWriteW<AudN3Spec> {
        NctsAtomicWriteW::new(self, 7)
    }
}
#[doc = "Audio Clock Regenerator N Value Register 3 For N expected values, refer to\n\nthe HDMI 1.4b specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudN3Spec;
impl crate::RegisterSpec for AudN3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_n3::R`](R) reader structure"]
impl crate::Readable for AudN3Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_n3::W`](W) writer structure"]
impl crate::Writable for AudN3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_N3 to value 0"]
impl crate::Resettable for AudN3Spec {
    const RESET_VALUE: u8 = 0;
}
