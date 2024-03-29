#[doc = "Register `PCIE_RC_SLOT_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PcieRcSlotControlAndStatusSpec>;
#[doc = "Register `PCIE_RC_SLOT_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PcieRcSlotControlAndStatusSpec>;
#[doc = "Field `ABPE` reader - Attention Button Pressed Enable \\[ABPE\\]\n\nWhen Set to 1b, this bit enables\n\nsoftware notification on an\n\nattention button pressed event. If\n\nthe Attention Button Present bit in\n\nthe Slot Capabilities register is 0b,\n\nthis bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
pub type AbpeR = crate::BitReader;
#[doc = "Field `ABPE` writer - Attention Button Pressed Enable \\[ABPE\\]\n\nWhen Set to 1b, this bit enables\n\nsoftware notification on an\n\nattention button pressed event. If\n\nthe Attention Button Present bit in\n\nthe Slot Capabilities register is 0b,\n\nthis bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
pub type AbpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFDE` reader - Power Fault Detected Enable \\[PFDE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a power\n\nfault event If a Power Controller\n\nthat supports power fault detection\n\nis not implemented, this bit is\n\npermitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
pub type PfdeR = crate::BitReader;
#[doc = "Field `PFDE` writer - Power Fault Detected Enable \\[PFDE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a power\n\nfault event If a Power Controller\n\nthat supports power fault detection\n\nis not implemented, this bit is\n\npermitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
pub type PfdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSCE` reader - MRL Sensor Changed Enable \\[MSCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a MRL\n\nsensor changed event If the MRL\n\nSensor Present bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
pub type MsceR = crate::BitReader;
#[doc = "Field `MSCE` writer - MRL Sensor Changed Enable \\[MSCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a MRL\n\nsensor changed event If the MRL\n\nSensor Present bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
pub type MsceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCE` reader - Presence Detect Changed Enable \\[PDCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a presence\n\ndetect changed event. If the Hot-\n\nPlug Capable bit in the Slot\n\nCapabilities register is 0b, this bit\n\nis permitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
pub type PdceR = crate::BitReader;
#[doc = "Field `PDCE` writer - Presence Detect Changed Enable \\[PDCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a presence\n\ndetect changed event. If the Hot-\n\nPlug Capable bit in the Slot\n\nCapabilities register is 0b, this bit\n\nis permitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
pub type PdceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIE` reader - Command Completed Interrupt Enable \\[CCIE\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), when Set, this bit enables\n\nsoftware notification when a hot-\n\nplug command is completed by the\n\nHot-Plug Controller. If Command\n\nCompleted notification is not\n\nsupported, this bit must be\n\nhardwired to 0b. Default value of\n\nthis bit is 0b."]
pub type CcieR = crate::BitReader;
#[doc = "Field `CCIE` writer - Command Completed Interrupt Enable \\[CCIE\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), when Set, this bit enables\n\nsoftware notification when a hot-\n\nplug command is completed by the\n\nHot-Plug Controller. If Command\n\nCompleted notification is not\n\nsupported, this bit must be\n\nhardwired to 0b. Default value of\n\nthis bit is 0b."]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPIE` reader - Hot-Plug Interrupt Enable \\[HPIE\\]\n\nWhen Set, this bit enables\n\ngeneration of an interrupt on\n\nenabled hot-plug events. If the Hot\n\nPlug Capable bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
pub type HpieR = crate::BitReader;
#[doc = "Field `HPIE` writer - Hot-Plug Interrupt Enable \\[HPIE\\]\n\nWhen Set, this bit enables\n\ngeneration of an interrupt on\n\nenabled hot-plug events. If the Hot\n\nPlug Capable bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
pub type HpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIC` reader - Attention Indicator Control \\[AIC\\]\n\nIf an Attention Indicator is\n\nimplemented, writes to this field\n\nset the Attention Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
pub type AicR = crate::FieldReader;
#[doc = "Field `AIC` writer - Attention Indicator Control \\[AIC\\]\n\nIf an Attention Indicator is\n\nimplemented, writes to this field\n\nset the Attention Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
pub type AicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIC` reader - Power Indicator Control \\[PIC\\]\n\nIf a Power Indicator is\n\nimplemented, writes to this field\n\nset the Power Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
pub type PicR = crate::FieldReader;
#[doc = "Field `PIC` writer - Power Indicator Control \\[PIC\\]\n\nIf a Power Indicator is\n\nimplemented, writes to this field\n\nset the Power Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
pub type PicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCC` reader - Power Controller Control \\[PCC\\]\n\nIf a Power Controller is\n\nimplemented, this bit when written\n\nsets the power state of the slot per\n\nthe defined encodings. Reads of\n\nthis bit must reflect the value from\n\nthe latest write, even if the\n\ncorresponding hot-plug command\n\nis not complete, unless software\n\nissues a write, if required to,\n\nwithout waiting for the previous\n\ncommand to complete in which\n\ncase the read value is undefined.\n\nThe defined encodings are: 0b\n\nPower On 1b Power Off"]
pub type PccR = crate::BitReader;
#[doc = "Field `PCC` writer - Power Controller Control \\[PCC\\]\n\nIf a Power Controller is\n\nimplemented, this bit when written\n\nsets the power state of the slot per\n\nthe defined encodings. Reads of\n\nthis bit must reflect the value from\n\nthe latest write, even if the\n\ncorresponding hot-plug command\n\nis not complete, unless software\n\nissues a write, if required to,\n\nwithout waiting for the previous\n\ncommand to complete in which\n\ncase the read value is undefined.\n\nThe defined encodings are: 0b\n\nPower On 1b Power Off"]
pub type PccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMIC` reader - Electromechanical Interlock Control \\[EMIC\\]\n\nIf an Electromechanical Interlock is\n\nimplemented, a write of 1b to this\n\nbit causes the state of the interlock\n\nto toggle. A write of 0b to this bit\n\nhas no effect. A read of this bit\n\nalways returns a 0b."]
pub type EmicR = crate::BitReader;
#[doc = "Field `EMIC` writer - Electromechanical Interlock Control \\[EMIC\\]\n\nIf an Electromechanical Interlock is\n\nimplemented, a write of 1b to this\n\nbit causes the state of the interlock\n\nto toggle. A write of 0b to this bit\n\nhas no effect. A read of this bit\n\nalways returns a 0b."]
pub type EmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLSCE` reader - Data Link Layer State Changed Enable \\[DLLSCE\\]\n\nIf the Data Link Layer Link Active\n\nReporting capability is 1b, this bit\n\nenables software notification when\n\nData Link Layer Link Active bit is\n\nchanged. If the Data Link Layer\n\nLink Active Reporting Capable bit is\n\n0b, this bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
pub type DllsceR = crate::BitReader;
#[doc = "Field `DLLSCE` writer - Data Link Layer State Changed Enable \\[DLLSCE\\]\n\nIf the Data Link Layer Link Active\n\nReporting capability is 1b, this bit\n\nenables software notification when\n\nData Link Layer Link Active bit is\n\nchanged. If the Data Link Layer\n\nLink Active Reporting Capable bit is\n\n0b, this bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
pub type DllsceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSCS1` reader - Reserved \\[RSCS1\\]\n\nReserved"]
pub type Rscs1R = crate::FieldReader;
#[doc = "Field `ABPRSD` reader - Attention Button Pressed \\[ABPRSD\\]\n\nIf an Attention Button is\n\nimplemented, this bit is Set when\n\nthe attention button is pressed. If\n\nan Attention Button is not\n\nsupported, this bit must not be\n\nSet."]
pub type AbprsdR = crate::BitReader;
#[doc = "Field `ABPRSD` writer - Attention Button Pressed \\[ABPRSD\\]\n\nIf an Attention Button is\n\nimplemented, this bit is Set when\n\nthe attention button is pressed. If\n\nan Attention Button is not\n\nsupported, this bit must not be\n\nSet."]
pub type AbprsdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PFD` reader - Power Fault Detected \\[PFD\\]\n\nIf a Power Controller that supports\n\npower fault detection is\n\nimplemented, this bit is Set when\n\nthe Power Controller detects a\n\npower fault at this slot. Note that,\n\ndepending on hardware capability,\n\nit is possible that a power fault can\n\nbe detected at any time,\n\nindependent of the Power\n\nController Control setting or the\n\noccupancy of the slot. If power\n\nfault detection is not supported,\n\nthis bit must not be Set."]
pub type PfdR = crate::BitReader;
#[doc = "Field `PFD` writer - Power Fault Detected \\[PFD\\]\n\nIf a Power Controller that supports\n\npower fault detection is\n\nimplemented, this bit is Set when\n\nthe Power Controller detects a\n\npower fault at this slot. Note that,\n\ndepending on hardware capability,\n\nit is possible that a power fault can\n\nbe detected at any time,\n\nindependent of the Power\n\nController Control setting or the\n\noccupancy of the slot. If power\n\nfault detection is not supported,\n\nthis bit must not be Set."]
pub type PfdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MRLSC` reader - MRL Sensor Changed \\[MRLSC\\]\n\nIf an MRL sensor is implemented,\n\nthis bit is Set when a MRL Sensor\n\nstate change is detected. If an MRL\n\nsensor is not implemented, this bit\n\nmust not be Set."]
pub type MrlscR = crate::BitReader;
#[doc = "Field `MRLSC` writer - MRL Sensor Changed \\[MRLSC\\]\n\nIf an MRL sensor is implemented,\n\nthis bit is Set when a MRL Sensor\n\nstate change is detected. If an MRL\n\nsensor is not implemented, this bit\n\nmust not be Set."]
pub type MrlscW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PDC` reader - Presence Detect Changed \\[PDC\\]\n\nThis bit is set when the value\n\nreported in the Presence Detect\n\nState bit is changed."]
pub type PdcR = crate::BitReader;
#[doc = "Field `PDC` writer - Presence Detect Changed \\[PDC\\]\n\nThis bit is set when the value\n\nreported in the Presence Detect\n\nState bit is changed."]
pub type PdcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMDCMPL` reader - Command Completed \\[CMDCMPL\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), this bit is Set when a hot-plug\n\ncommand has completed and the\n\nHot-Plug Controller is ready to\n\naccept a subsequent command.\n\nThe Command Completed status\n\nbit is Set as an indication to host\n\nsoftware that the Hot- Plug\n\nController has processed the\n\nprevious command and is ready to\n\nreceive the next command; it\n\nprovides no guarantee that the\n\naction corresponding to the\n\ncommand is complete. If\n\nCommand Completed notification\n\nis not supported, this bit must be\n\nhardwired to 0b."]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - Command Completed \\[CMDCMPL\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), this bit is Set when a hot-plug\n\ncommand has completed and the\n\nHot-Plug Controller is ready to\n\naccept a subsequent command.\n\nThe Command Completed status\n\nbit is Set as an indication to host\n\nsoftware that the Hot- Plug\n\nController has processed the\n\nprevious command and is ready to\n\nreceive the next command; it\n\nprovides no guarantee that the\n\naction corresponding to the\n\ncommand is complete. If\n\nCommand Completed notification\n\nis not supported, this bit must be\n\nhardwired to 0b."]
pub type CmdcmplW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MRLSS` reader - MRL Sensor State \\[MRLSS\\]\n\nThis bit reports the status of the\n\nMRL sensor if implemented.\n\nDefined encodings are: 0b MRL\n\nClosed 1b MRL Open"]
pub type MrlssR = crate::BitReader;
#[doc = "Field `PDS` reader - Presence Detect State \\[PDS\\]\n\nThis bit indicates the presence of\n\nan adapter in the slot, reflected by\n\nthe logical “OR” of the Physical\n\nLayer in-band presence detect\n\nmechanism and, if present, any\n\nout-of-band presence detect\n\nmechanism defined for the slot’s\n\ncorresponding form factor. Note\n\nthat the in-band presence detect\n\nmechanism requires that power be\n\napplied to an adapter for its\n\npresence to be detected.\n\nConsequently, form factors that\n\nrequire a power controller for hot-\n\nplug must implement a physical\n\npin presence detect mechanism.\n\nDefined encodings are: 0b Slot\n\nEmpty 1b Card Present in slot."]
pub type PdsR = crate::BitReader;
#[doc = "Field `EMIS` reader - Electromechanical Interlock Status \\[EMIS\\]\n\nIf an Electromechanical Interlock is\n\nimplemented, this bit indicates the\n\nstatus of the Electromechanical\n\nInterlock. Defined encodings\n\nare: 0b Electromechanical\n\nInterlock Disengaged 1b\n\nElectromechanical Interlock\n\nEngaged"]
pub type EmisR = crate::BitReader;
#[doc = "Field `DLLSC` reader - Data Link Layer State Changed \\[DLLSC\\]\n\nThis bit is Set when the value\n\nreported in the Data Link Layer\n\nLink Active bit of the Link Status\n\nregister is changed. In response to\n\na Data Link Layer State Changed\n\nevent, software must read the\n\nData Link Layer Link Active bit of\n\nthe Link Status register to\n\ndetermine if the Link is active\n\nbefore initiating configuration\n\ncycles to the hot plugged device."]
pub type DllscR = crate::BitReader;
#[doc = "Field `DLLSC` writer - Data Link Layer State Changed \\[DLLSC\\]\n\nThis bit is Set when the value\n\nreported in the Data Link Layer\n\nLink Active bit of the Link Status\n\nregister is changed. In response to\n\na Data Link Layer State Changed\n\nevent, software must read the\n\nData Link Layer Link Active bit of\n\nthe Link Status register to\n\ndetermine if the Link is active\n\nbefore initiating configuration\n\ncycles to the hot plugged device."]
pub type DllscW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RSCS2` reader - Reserved \\[RSCS2\\]\n\n(no description)"]
pub type Rscs2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Attention Button Pressed Enable \\[ABPE\\]\n\nWhen Set to 1b, this bit enables\n\nsoftware notification on an\n\nattention button pressed event. If\n\nthe Attention Button Present bit in\n\nthe Slot Capabilities register is 0b,\n\nthis bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
    #[inline(always)]
    pub fn abpe(&self) -> AbpeR {
        AbpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Fault Detected Enable \\[PFDE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a power\n\nfault event If a Power Controller\n\nthat supports power fault detection\n\nis not implemented, this bit is\n\npermitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
    #[inline(always)]
    pub fn pfde(&self) -> PfdeR {
        PfdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MRL Sensor Changed Enable \\[MSCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a MRL\n\nsensor changed event If the MRL\n\nSensor Present bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
    #[inline(always)]
    pub fn msce(&self) -> MsceR {
        MsceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Presence Detect Changed Enable \\[PDCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a presence\n\ndetect changed event. If the Hot-\n\nPlug Capable bit in the Slot\n\nCapabilities register is 0b, this bit\n\nis permitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
    #[inline(always)]
    pub fn pdce(&self) -> PdceR {
        PdceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Completed Interrupt Enable \\[CCIE\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), when Set, this bit enables\n\nsoftware notification when a hot-\n\nplug command is completed by the\n\nHot-Plug Controller. If Command\n\nCompleted notification is not\n\nsupported, this bit must be\n\nhardwired to 0b. Default value of\n\nthis bit is 0b."]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hot-Plug Interrupt Enable \\[HPIE\\]\n\nWhen Set, this bit enables\n\ngeneration of an interrupt on\n\nenabled hot-plug events. If the Hot\n\nPlug Capable bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
    #[inline(always)]
    pub fn hpie(&self) -> HpieR {
        HpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Attention Indicator Control \\[AIC\\]\n\nIf an Attention Indicator is\n\nimplemented, writes to this field\n\nset the Attention Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
    #[inline(always)]
    pub fn aic(&self) -> AicR {
        AicR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Power Indicator Control \\[PIC\\]\n\nIf a Power Indicator is\n\nimplemented, writes to this field\n\nset the Power Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
    #[inline(always)]
    pub fn pic(&self) -> PicR {
        PicR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Power Controller Control \\[PCC\\]\n\nIf a Power Controller is\n\nimplemented, this bit when written\n\nsets the power state of the slot per\n\nthe defined encodings. Reads of\n\nthis bit must reflect the value from\n\nthe latest write, even if the\n\ncorresponding hot-plug command\n\nis not complete, unless software\n\nissues a write, if required to,\n\nwithout waiting for the previous\n\ncommand to complete in which\n\ncase the read value is undefined.\n\nThe defined encodings are: 0b\n\nPower On 1b Power Off"]
    #[inline(always)]
    pub fn pcc(&self) -> PccR {
        PccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Electromechanical Interlock Control \\[EMIC\\]\n\nIf an Electromechanical Interlock is\n\nimplemented, a write of 1b to this\n\nbit causes the state of the interlock\n\nto toggle. A write of 0b to this bit\n\nhas no effect. A read of this bit\n\nalways returns a 0b."]
    #[inline(always)]
    pub fn emic(&self) -> EmicR {
        EmicR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Link Layer State Changed Enable \\[DLLSCE\\]\n\nIf the Data Link Layer Link Active\n\nReporting capability is 1b, this bit\n\nenables software notification when\n\nData Link Layer Link Active bit is\n\nchanged. If the Data Link Layer\n\nLink Active Reporting Capable bit is\n\n0b, this bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
    #[inline(always)]
    pub fn dllsce(&self) -> DllsceR {
        DllsceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Reserved \\[RSCS1\\]\n\nReserved"]
    #[inline(always)]
    pub fn rscs1(&self) -> Rscs1R {
        Rscs1R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Attention Button Pressed \\[ABPRSD\\]\n\nIf an Attention Button is\n\nimplemented, this bit is Set when\n\nthe attention button is pressed. If\n\nan Attention Button is not\n\nsupported, this bit must not be\n\nSet."]
    #[inline(always)]
    pub fn abprsd(&self) -> AbprsdR {
        AbprsdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power Fault Detected \\[PFD\\]\n\nIf a Power Controller that supports\n\npower fault detection is\n\nimplemented, this bit is Set when\n\nthe Power Controller detects a\n\npower fault at this slot. Note that,\n\ndepending on hardware capability,\n\nit is possible that a power fault can\n\nbe detected at any time,\n\nindependent of the Power\n\nController Control setting or the\n\noccupancy of the slot. If power\n\nfault detection is not supported,\n\nthis bit must not be Set."]
    #[inline(always)]
    pub fn pfd(&self) -> PfdR {
        PfdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MRL Sensor Changed \\[MRLSC\\]\n\nIf an MRL sensor is implemented,\n\nthis bit is Set when a MRL Sensor\n\nstate change is detected. If an MRL\n\nsensor is not implemented, this bit\n\nmust not be Set."]
    #[inline(always)]
    pub fn mrlsc(&self) -> MrlscR {
        MrlscR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Presence Detect Changed \\[PDC\\]\n\nThis bit is set when the value\n\nreported in the Presence Detect\n\nState bit is changed."]
    #[inline(always)]
    pub fn pdc(&self) -> PdcR {
        PdcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Completed \\[CMDCMPL\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), this bit is Set when a hot-plug\n\ncommand has completed and the\n\nHot-Plug Controller is ready to\n\naccept a subsequent command.\n\nThe Command Completed status\n\nbit is Set as an indication to host\n\nsoftware that the Hot- Plug\n\nController has processed the\n\nprevious command and is ready to\n\nreceive the next command; it\n\nprovides no guarantee that the\n\naction corresponding to the\n\ncommand is complete. If\n\nCommand Completed notification\n\nis not supported, this bit must be\n\nhardwired to 0b."]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MRL Sensor State \\[MRLSS\\]\n\nThis bit reports the status of the\n\nMRL sensor if implemented.\n\nDefined encodings are: 0b MRL\n\nClosed 1b MRL Open"]
    #[inline(always)]
    pub fn mrlss(&self) -> MrlssR {
        MrlssR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Presence Detect State \\[PDS\\]\n\nThis bit indicates the presence of\n\nan adapter in the slot, reflected by\n\nthe logical “OR” of the Physical\n\nLayer in-band presence detect\n\nmechanism and, if present, any\n\nout-of-band presence detect\n\nmechanism defined for the slot’s\n\ncorresponding form factor. Note\n\nthat the in-band presence detect\n\nmechanism requires that power be\n\napplied to an adapter for its\n\npresence to be detected.\n\nConsequently, form factors that\n\nrequire a power controller for hot-\n\nplug must implement a physical\n\npin presence detect mechanism.\n\nDefined encodings are: 0b Slot\n\nEmpty 1b Card Present in slot."]
    #[inline(always)]
    pub fn pds(&self) -> PdsR {
        PdsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Electromechanical Interlock Status \\[EMIS\\]\n\nIf an Electromechanical Interlock is\n\nimplemented, this bit indicates the\n\nstatus of the Electromechanical\n\nInterlock. Defined encodings\n\nare: 0b Electromechanical\n\nInterlock Disengaged 1b\n\nElectromechanical Interlock\n\nEngaged"]
    #[inline(always)]
    pub fn emis(&self) -> EmisR {
        EmisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Link Layer State Changed \\[DLLSC\\]\n\nThis bit is Set when the value\n\nreported in the Data Link Layer\n\nLink Active bit of the Link Status\n\nregister is changed. In response to\n\na Data Link Layer State Changed\n\nevent, software must read the\n\nData Link Layer Link Active bit of\n\nthe Link Status register to\n\ndetermine if the Link is active\n\nbefore initiating configuration\n\ncycles to the hot plugged device."]
    #[inline(always)]
    pub fn dllsc(&self) -> DllscR {
        DllscR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Reserved \\[RSCS2\\]\n\n(no description)"]
    #[inline(always)]
    pub fn rscs2(&self) -> Rscs2R {
        Rscs2R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Attention Button Pressed Enable \\[ABPE\\]\n\nWhen Set to 1b, this bit enables\n\nsoftware notification on an\n\nattention button pressed event. If\n\nthe Attention Button Present bit in\n\nthe Slot Capabilities register is 0b,\n\nthis bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn abpe(&mut self) -> AbpeW<PcieRcSlotControlAndStatusSpec> {
        AbpeW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Fault Detected Enable \\[PFDE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a power\n\nfault event If a Power Controller\n\nthat supports power fault detection\n\nis not implemented, this bit is\n\npermitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn pfde(&mut self) -> PfdeW<PcieRcSlotControlAndStatusSpec> {
        PfdeW::new(self, 1)
    }
    #[doc = "Bit 2 - MRL Sensor Changed Enable \\[MSCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a MRL\n\nsensor changed event If the MRL\n\nSensor Present bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn msce(&mut self) -> MsceW<PcieRcSlotControlAndStatusSpec> {
        MsceW::new(self, 2)
    }
    #[doc = "Bit 3 - Presence Detect Changed Enable \\[PDCE\\]\n\nWhen Set, this bit enables\n\nsoftware notification on a presence\n\ndetect changed event. If the Hot-\n\nPlug Capable bit in the Slot\n\nCapabilities register is 0b, this bit\n\nis permitted to be read-only with a\n\nvalue of 0b. Default value of this\n\nbit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn pdce(&mut self) -> PdceW<PcieRcSlotControlAndStatusSpec> {
        PdceW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Completed Interrupt Enable \\[CCIE\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), when Set, this bit enables\n\nsoftware notification when a hot-\n\nplug command is completed by the\n\nHot-Plug Controller. If Command\n\nCompleted notification is not\n\nsupported, this bit must be\n\nhardwired to 0b. Default value of\n\nthis bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CcieW<PcieRcSlotControlAndStatusSpec> {
        CcieW::new(self, 4)
    }
    #[doc = "Bit 5 - Hot-Plug Interrupt Enable \\[HPIE\\]\n\nWhen Set, this bit enables\n\ngeneration of an interrupt on\n\nenabled hot-plug events. If the Hot\n\nPlug Capable bit in the Slot\n\nCapabilities register is Clear, this\n\nbit is permitted to be read-only\n\nwith a value of 0b. Default value of\n\nthis bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn hpie(&mut self) -> HpieW<PcieRcSlotControlAndStatusSpec> {
        HpieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Attention Indicator Control \\[AIC\\]\n\nIf an Attention Indicator is\n\nimplemented, writes to this field\n\nset the Attention Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
    #[inline(always)]
    #[must_use]
    pub fn aic(&mut self) -> AicW<PcieRcSlotControlAndStatusSpec> {
        AicW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Power Indicator Control \\[PIC\\]\n\nIf a Power Indicator is\n\nimplemented, writes to this field\n\nset the Power Indicator to the\n\nwritten state. Reads of this field\n\nmust reflect the value from the\n\nlatest write, Defined encodings\n\nare: 00b Reserved 01b On 10b\n\nBlink 11b Off"]
    #[inline(always)]
    #[must_use]
    pub fn pic(&mut self) -> PicW<PcieRcSlotControlAndStatusSpec> {
        PicW::new(self, 8)
    }
    #[doc = "Bit 10 - Power Controller Control \\[PCC\\]\n\nIf a Power Controller is\n\nimplemented, this bit when written\n\nsets the power state of the slot per\n\nthe defined encodings. Reads of\n\nthis bit must reflect the value from\n\nthe latest write, even if the\n\ncorresponding hot-plug command\n\nis not complete, unless software\n\nissues a write, if required to,\n\nwithout waiting for the previous\n\ncommand to complete in which\n\ncase the read value is undefined.\n\nThe defined encodings are: 0b\n\nPower On 1b Power Off"]
    #[inline(always)]
    #[must_use]
    pub fn pcc(&mut self) -> PccW<PcieRcSlotControlAndStatusSpec> {
        PccW::new(self, 10)
    }
    #[doc = "Bit 11 - Electromechanical Interlock Control \\[EMIC\\]\n\nIf an Electromechanical Interlock is\n\nimplemented, a write of 1b to this\n\nbit causes the state of the interlock\n\nto toggle. A write of 0b to this bit\n\nhas no effect. A read of this bit\n\nalways returns a 0b."]
    #[inline(always)]
    #[must_use]
    pub fn emic(&mut self) -> EmicW<PcieRcSlotControlAndStatusSpec> {
        EmicW::new(self, 11)
    }
    #[doc = "Bit 12 - Data Link Layer State Changed Enable \\[DLLSCE\\]\n\nIf the Data Link Layer Link Active\n\nReporting capability is 1b, this bit\n\nenables software notification when\n\nData Link Layer Link Active bit is\n\nchanged. If the Data Link Layer\n\nLink Active Reporting Capable bit is\n\n0b, this bit is permitted to be read-\n\nonly with a value of 0b. Default\n\nvalue of this bit is 0b."]
    #[inline(always)]
    #[must_use]
    pub fn dllsce(&mut self) -> DllsceW<PcieRcSlotControlAndStatusSpec> {
        DllsceW::new(self, 12)
    }
    #[doc = "Bit 16 - Attention Button Pressed \\[ABPRSD\\]\n\nIf an Attention Button is\n\nimplemented, this bit is Set when\n\nthe attention button is pressed. If\n\nan Attention Button is not\n\nsupported, this bit must not be\n\nSet."]
    #[inline(always)]
    #[must_use]
    pub fn abprsd(&mut self) -> AbprsdW<PcieRcSlotControlAndStatusSpec> {
        AbprsdW::new(self, 16)
    }
    #[doc = "Bit 17 - Power Fault Detected \\[PFD\\]\n\nIf a Power Controller that supports\n\npower fault detection is\n\nimplemented, this bit is Set when\n\nthe Power Controller detects a\n\npower fault at this slot. Note that,\n\ndepending on hardware capability,\n\nit is possible that a power fault can\n\nbe detected at any time,\n\nindependent of the Power\n\nController Control setting or the\n\noccupancy of the slot. If power\n\nfault detection is not supported,\n\nthis bit must not be Set."]
    #[inline(always)]
    #[must_use]
    pub fn pfd(&mut self) -> PfdW<PcieRcSlotControlAndStatusSpec> {
        PfdW::new(self, 17)
    }
    #[doc = "Bit 18 - MRL Sensor Changed \\[MRLSC\\]\n\nIf an MRL sensor is implemented,\n\nthis bit is Set when a MRL Sensor\n\nstate change is detected. If an MRL\n\nsensor is not implemented, this bit\n\nmust not be Set."]
    #[inline(always)]
    #[must_use]
    pub fn mrlsc(&mut self) -> MrlscW<PcieRcSlotControlAndStatusSpec> {
        MrlscW::new(self, 18)
    }
    #[doc = "Bit 19 - Presence Detect Changed \\[PDC\\]\n\nThis bit is set when the value\n\nreported in the Presence Detect\n\nState bit is changed."]
    #[inline(always)]
    #[must_use]
    pub fn pdc(&mut self) -> PdcW<PcieRcSlotControlAndStatusSpec> {
        PdcW::new(self, 19)
    }
    #[doc = "Bit 20 - Command Completed \\[CMDCMPL\\]\n\nIf Command Completed\n\nnotification is supported (if the No\n\nCommand Completed Support bit\n\nin the Slot Capabilities register is\n\n0b), this bit is Set when a hot-plug\n\ncommand has completed and the\n\nHot-Plug Controller is ready to\n\naccept a subsequent command.\n\nThe Command Completed status\n\nbit is Set as an indication to host\n\nsoftware that the Hot- Plug\n\nController has processed the\n\nprevious command and is ready to\n\nreceive the next command; it\n\nprovides no guarantee that the\n\naction corresponding to the\n\ncommand is complete. If\n\nCommand Completed notification\n\nis not supported, this bit must be\n\nhardwired to 0b."]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpl(&mut self) -> CmdcmplW<PcieRcSlotControlAndStatusSpec> {
        CmdcmplW::new(self, 20)
    }
    #[doc = "Bit 24 - Data Link Layer State Changed \\[DLLSC\\]\n\nThis bit is Set when the value\n\nreported in the Data Link Layer\n\nLink Active bit of the Link Status\n\nregister is changed. In response to\n\na Data Link Layer State Changed\n\nevent, software must read the\n\nData Link Layer Link Active bit of\n\nthe Link Status register to\n\ndetermine if the Link is active\n\nbefore initiating configuration\n\ncycles to the hot plugged device."]
    #[inline(always)]
    #[must_use]
    pub fn dllsc(&mut self) -> DllscW<PcieRcSlotControlAndStatusSpec> {
        DllscW::new(self, 24)
    }
}
#[doc = "Slot Control and Status Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_slot_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_slot_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcSlotControlAndStatusSpec;
impl crate::RegisterSpec for PcieRcSlotControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_slot_control_and_status::R`](R) reader structure"]
impl crate::Readable for PcieRcSlotControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_slot_control_and_status::W`](W) writer structure"]
impl crate::Writable for PcieRcSlotControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x011f_0000;
}
#[doc = "`reset()` method sets PCIE_RC_SLOT_CONTROL_AND_STATUS to value 0x07c0"]
impl crate::Resettable for PcieRcSlotControlAndStatusSpec {
    const RESET_VALUE: u32 = 0x07c0;
}
