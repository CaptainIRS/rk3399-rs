#[doc = "Register `AUD_INPUTCLKFS` reader"]
pub type R = crate::R<AudInputclkfsSpec>;
#[doc = "Register `AUD_INPUTCLKFS` writer"]
pub type W = crate::W<AudInputclkfsSpec>;
#[doc = "Field `IFSFACTOR` reader - Fs factor configuration: ifsfactor\\[2:0\\]
| Audio Clock | Action 0 | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. | 256xFs | The input audio clock (I2S only) is divided by 2 and then used at audio packetizer to calculate the CTS value and ACR packet insertion rate. | 512xFs | The input audio clock (either I2S or SPDIF according to configuration) used divided by 4 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. Note: When the SPDIF interface is receiving an HBR audio stream (\"Support for HBR over SDPIF\" parameter must be enabled), it is required that the selected IFSFACTOR to be set at 512xFs in order to comply with the HDMI ACR requirements for HBR Bits Name Attr Description audio streams. | Reserved | 64xFs | The input audio clock (I2S only) is multiplied by 2 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. others | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. The SPDIF interface, for non HBR audio, requires that the configured oversampling value to be 128xFs when HTX_SPDIFBYPDRU is enabled and 512xFs if not. When the SPDIF interface is receiving HBR audio (HBR_ON_SPDIF must be enabled), in order to comply with the HDMI ACR requirements for HBR audio streams."]
pub type IfsfactorR = crate::FieldReader;
#[doc = "Field `IFSFACTOR` writer - Fs factor configuration: ifsfactor\\[2:0\\]
| Audio Clock | Action 0 | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. | 256xFs | The input audio clock (I2S only) is divided by 2 and then used at audio packetizer to calculate the CTS value and ACR packet insertion rate. | 512xFs | The input audio clock (either I2S or SPDIF according to configuration) used divided by 4 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. Note: When the SPDIF interface is receiving an HBR audio stream (\"Support for HBR over SDPIF\" parameter must be enabled), it is required that the selected IFSFACTOR to be set at 512xFs in order to comply with the HDMI ACR requirements for HBR Bits Name Attr Description audio streams. | Reserved | 64xFs | The input audio clock (I2S only) is multiplied by 2 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. others | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. The SPDIF interface, for non HBR audio, requires that the configured oversampling value to be 128xFs when HTX_SPDIFBYPDRU is enabled and 512xFs if not. When the SPDIF interface is receiving HBR audio (HBR_ON_SPDIF must be enabled), in order to comply with the HDMI ACR requirements for HBR audio streams."]
pub type IfsfactorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Fs factor configuration: ifsfactor\\[2:0\\]
| Audio Clock | Action 0 | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. | 256xFs | The input audio clock (I2S only) is divided by 2 and then used at audio packetizer to calculate the CTS value and ACR packet insertion rate. | 512xFs | The input audio clock (either I2S or SPDIF according to configuration) used divided by 4 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. Note: When the SPDIF interface is receiving an HBR audio stream (\"Support for HBR over SDPIF\" parameter must be enabled), it is required that the selected IFSFACTOR to be set at 512xFs in order to comply with the HDMI ACR requirements for HBR Bits Name Attr Description audio streams. | Reserved | 64xFs | The input audio clock (I2S only) is multiplied by 2 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. others | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. The SPDIF interface, for non HBR audio, requires that the configured oversampling value to be 128xFs when HTX_SPDIFBYPDRU is enabled and 512xFs if not. When the SPDIF interface is receiving HBR audio (HBR_ON_SPDIF must be enabled), in order to comply with the HDMI ACR requirements for HBR audio streams."]
    #[inline(always)]
    pub fn ifsfactor(&self) -> IfsfactorR {
        IfsfactorR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fs factor configuration: ifsfactor\\[2:0\\]
| Audio Clock | Action 0 | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. | 256xFs | The input audio clock (I2S only) is divided by 2 and then used at audio packetizer to calculate the CTS value and ACR packet insertion rate. | 512xFs | The input audio clock (either I2S or SPDIF according to configuration) used divided by 4 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. Note: When the SPDIF interface is receiving an HBR audio stream (\"Support for HBR over SDPIF\" parameter must be enabled), it is required that the selected IFSFACTOR to be set at 512xFs in order to comply with the HDMI ACR requirements for HBR Bits Name Attr Description audio streams. | Reserved | 64xFs | The input audio clock (I2S only) is multiplied by 2 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. others | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. The SPDIF interface, for non HBR audio, requires that the configured oversampling value to be 128xFs when HTX_SPDIFBYPDRU is enabled and 512xFs if not. When the SPDIF interface is receiving HBR audio (HBR_ON_SPDIF must be enabled), in order to comply with the HDMI ACR requirements for HBR audio streams."]
    #[inline(always)]
    #[must_use]
    pub fn ifsfactor(&mut self) -> IfsfactorW<AudInputclkfsSpec> {
        IfsfactorW::new(self, 0)
    }
}
#[doc = "Fs factor configuration: ifsfactor\\[2:0\\]
| Audio Clock | Action 0 | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. | 256xFs | The input audio clock (I2S only) is divided by 2 and then used at audio packetizer to calculate the CTS value and ACR packet insertion rate. | 512xFs | The input audio clock (either I2S or SPDIF according to configuration) used divided by 4 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. Note: When the SPDIF interface is receiving an HBR audio stream (\"Support for HBR over SDPIF\" parameter must be enabled), it is required that the selected IFSFACTOR to be set at 512xFs in order to comply with the HDMI ACR requirements for HBR Bits Name Attr Description audio streams. | Reserved | 64xFs | The input audio clock (I2S only) is multiplied by 2 and then used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. others | 128xFs | If you select the Bypass SPDIF DRU unit in coreConsultant, the input audio clock (either I2S or SPDIF according to configuration) is used at the audio packetizer to calculate the CTS value and ACR packet insertion rate. The SPDIF interface, for non HBR audio, requires that the configured oversampling value to be 128xFs when HTX_SPDIFBYPDRU is enabled and 512xFs if not. When the SPDIF interface is receiving HBR audio (HBR_ON_SPDIF must be enabled), in order to comply with the HDMI ACR requirements for HBR audio streams.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_inputclkfs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_inputclkfs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudInputclkfsSpec;
impl crate::RegisterSpec for AudInputclkfsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_inputclkfs::R`](R) reader structure"]
impl crate::Readable for AudInputclkfsSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_inputclkfs::W`](W) writer structure"]
impl crate::Writable for AudInputclkfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_INPUTCLKFS to value 0"]
impl crate::Resettable for AudInputclkfsSpec {
    const RESET_VALUE: u8 = 0;
}