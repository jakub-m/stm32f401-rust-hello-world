#[doc = "Register `FS_GINTSTS` reader"]
pub type R = crate::R<FsGintstsSpec>;
#[doc = "Register `FS_GINTSTS` writer"]
pub type W = crate::W<FsGintstsSpec>;
#[doc = "Field `CMOD` reader - Current mode of operation"]
pub type CmodR = crate::BitReader;
#[doc = "Field `MMIS` reader - Mode mismatch interrupt"]
pub type MmisR = crate::BitReader;
#[doc = "Field `MMIS` writer - Mode mismatch interrupt"]
pub type MmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTG interrupt"]
pub type OtgintR = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RxFIFO non-empty"]
pub type RxflvlR = crate::BitReader;
#[doc = "Field `NPTXFE` reader - Non-periodic TxFIFO empty"]
pub type NptxfeR = crate::BitReader;
#[doc = "Field `GINAKEFF` reader - Global IN non-periodic NAK effective"]
pub type GinakeffR = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - Global OUT NAK effective"]
pub type GoutnakeffR = crate::BitReader;
#[doc = "Field `ESUSP` reader - Early suspend"]
pub type EsuspR = crate::BitReader;
#[doc = "Field `ESUSP` writer - Early suspend"]
pub type EsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USB suspend"]
pub type UsbsuspR = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USB suspend"]
pub type UsbsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNE` reader - Enumeration done"]
pub type EnumdneR = crate::BitReader;
#[doc = "Field `ENUMDNE` writer - Enumeration done"]
pub type EnumdneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRP` reader - Isochronous OUT packet dropped interrupt"]
pub type IsoodrpR = crate::BitReader;
#[doc = "Field `ISOODRP` writer - Isochronous OUT packet dropped interrupt"]
pub type IsoodrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of periodic frame interrupt"]
pub type EopfR = crate::BitReader;
#[doc = "Field `EOPF` writer - End of periodic frame interrupt"]
pub type EopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt"]
pub type IepintR = crate::BitReader;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt"]
pub type OepintR = crate::BitReader;
#[doc = "Field `IISOIXFR` reader - Incomplete isochronous IN transfer"]
pub type IisoixfrR = crate::BitReader;
#[doc = "Field `IISOIXFR` writer - Incomplete isochronous IN transfer"]
pub type IisoixfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFR_INCOMPISOOUT` reader - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
pub type IpxfrIncompisooutR = crate::BitReader;
#[doc = "Field `IPXFR_INCOMPISOOUT` writer - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
pub type IpxfrIncompisooutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPRTINT` reader - Host port interrupt"]
pub type HprtintR = crate::BitReader;
#[doc = "Field `HCINT` reader - Host channels interrupt"]
pub type HcintR = crate::BitReader;
#[doc = "Field `PTXFE` reader - Periodic TxFIFO empty"]
pub type PtxfeR = crate::BitReader;
#[doc = "Field `CIDSCHG` reader - Connector ID status change"]
pub type CidschgR = crate::BitReader;
#[doc = "Field `CIDSCHG` writer - Connector ID status change"]
pub type CidschgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt"]
pub type DiscintR = crate::BitReader;
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt"]
pub type DiscintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRQINT` reader - Session request/new session detected interrupt"]
pub type SrqintR = crate::BitReader;
#[doc = "Field `SRQINT` writer - Session request/new session detected interrupt"]
pub type SrqintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - Resume/remote wakeup detected interrupt"]
pub type WkupintR = crate::BitReader;
#[doc = "Field `WKUPINT` writer - Resume/remote wakeup detected interrupt"]
pub type WkupintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current mode of operation"]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(&self) -> MmisR {
        MmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OtgintR {
        OtgintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO non-empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RxflvlR {
        RxflvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NptxfeR {
        NptxfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN non-periodic NAK effective"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GinakeffR {
        GinakeffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GoutnakeffR {
        GoutnakeffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esusp(&self) -> EsuspR {
        EsuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> UsbsuspR {
        UsbsuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdne(&self) -> EnumdneR {
        EnumdneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoodrp(&self) -> IsoodrpR {
        IsoodrpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EopfR {
        EopfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IisoixfrR {
        IisoixfrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline(always)]
    pub fn ipxfr_incompisoout(&self) -> IpxfrIncompisooutR {
        IpxfrIncompisooutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt"]
    #[inline(always)]
    pub fn hprtint(&self) -> HprtintR {
        HprtintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt"]
    #[inline(always)]
    pub fn hcint(&self) -> HcintR {
        HcintR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PtxfeR {
        PtxfeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(&self) -> CidschgR {
        CidschgR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn discint(&self) -> DiscintR {
        DiscintR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    pub fn srqint(&self) -> SrqintR {
        SrqintR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkupint(&self) -> WkupintR {
        WkupintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mmis(&mut self) -> MmisW<FsGintstsSpec> {
        MmisW::new(self, 1)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<FsGintstsSpec> {
        SofW::new(self, 3)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    #[must_use]
    pub fn esusp(&mut self) -> EsuspW<FsGintstsSpec> {
        EsuspW::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> UsbsuspW<FsGintstsSpec> {
        UsbsuspW::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<FsGintstsSpec> {
        UsbrstW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    #[must_use]
    pub fn enumdne(&mut self) -> EnumdneW<FsGintstsSpec> {
        EnumdneW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrp(&mut self) -> IsoodrpW<FsGintstsSpec> {
        IsoodrpW::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EopfW<FsGintstsSpec> {
        EopfW::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfr(&mut self) -> IisoixfrW<FsGintstsSpec> {
        IisoixfrW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ipxfr_incompisoout(&mut self) -> IpxfrIncompisooutW<FsGintstsSpec> {
        IpxfrIncompisooutW::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    #[must_use]
    pub fn cidschg(&mut self) -> CidschgW<FsGintstsSpec> {
        CidschgW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DiscintW<FsGintstsSpec> {
        DiscintW::new(self, 29)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn srqint(&mut self) -> SrqintW<FsGintstsSpec> {
        SrqintW::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkupint(&mut self) -> WkupintW<FsGintstsSpec> {
        WkupintW::new(self, 31)
    }
}
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsGintstsSpec;
impl crate::RegisterSpec for FsGintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gintsts::R`](R) reader structure"]
impl crate::Readable for FsGintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_gintsts::W`](W) writer structure"]
impl crate::Writable for FsGintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_GINTSTS to value 0x0400_0020"]
impl crate::Resettable for FsGintstsSpec {
    const RESET_VALUE: u32 = 0x0400_0020;
}
