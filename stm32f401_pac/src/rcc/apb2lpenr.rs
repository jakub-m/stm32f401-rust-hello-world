#[doc = "Register `APB2LPENR` reader"]
pub type R = crate::R<Apb2lpenrSpec>;
#[doc = "Register `APB2LPENR` writer"]
pub type W = crate::W<Apb2lpenrSpec>;
#[doc = "Field `TIM1LPEN` reader - TIM1 clock enable during Sleep mode"]
pub type Tim1lpenR = crate::BitReader;
#[doc = "Field `TIM1LPEN` writer - TIM1 clock enable during Sleep mode"]
pub type Tim1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during Sleep mode"]
pub type Usart1lpenR = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during Sleep mode"]
pub type Usart1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during Sleep mode"]
pub type Usart6lpenR = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during Sleep mode"]
pub type Usart6lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1LPEN` reader - ADC1 clock enable during Sleep mode"]
pub type Adc1lpenR = crate::BitReader;
#[doc = "Field `ADC1LPEN` writer - ADC1 clock enable during Sleep mode"]
pub type Adc1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode"]
pub type SdiolpenR = crate::BitReader;
#[doc = "Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode"]
pub type SdiolpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode"]
pub type Spi1lpenR = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode"]
pub type Spi1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode"]
pub type SyscfglpenR = crate::BitReader;
#[doc = "Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode"]
pub type SyscfglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9LPEN` reader - TIM9 clock enable during sleep mode"]
pub type Tim9lpenR = crate::BitReader;
#[doc = "Field `TIM9LPEN` writer - TIM9 clock enable during sleep mode"]
pub type Tim9lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10LPEN` reader - TIM10 clock enable during Sleep mode"]
pub type Tim10lpenR = crate::BitReader;
#[doc = "Field `TIM10LPEN` writer - TIM10 clock enable during Sleep mode"]
pub type Tim10lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11LPEN` reader - TIM11 clock enable during Sleep mode"]
pub type Tim11lpenR = crate::BitReader;
#[doc = "Field `TIM11LPEN` writer - TIM11 clock enable during Sleep mode"]
pub type Tim11lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> Tim1lpenR {
        Tim1lpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> Usart1lpenR {
        Usart1lpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> Usart6lpenR {
        Usart6lpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&self) -> Adc1lpenR {
        Adc1lpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&self) -> SdiolpenR {
        SdiolpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> Spi1lpenR {
        Spi1lpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SyscfglpenR {
        SyscfglpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&self) -> Tim9lpenR {
        Tim9lpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&self) -> Tim10lpenR {
        Tim10lpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&self) -> Tim11lpenR {
        Tim11lpenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> Tim1lpenW<Apb2lpenrSpec> {
        Tim1lpenW::new(self, 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> Usart1lpenW<Apb2lpenrSpec> {
        Usart1lpenW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> Usart6lpenW<Apb2lpenrSpec> {
        Usart6lpenW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1lpen(&mut self) -> Adc1lpenW<Apb2lpenrSpec> {
        Adc1lpenW::new(self, 8)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdiolpen(&mut self) -> SdiolpenW<Apb2lpenrSpec> {
        SdiolpenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> Spi1lpenW<Apb2lpenrSpec> {
        Spi1lpenW::new(self, 12)
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SyscfglpenW<Apb2lpenrSpec> {
        SyscfglpenW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim9lpen(&mut self) -> Tim9lpenW<Apb2lpenrSpec> {
        Tim9lpenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim10lpen(&mut self) -> Tim10lpenW<Apb2lpenrSpec> {
        Tim10lpenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim11lpen(&mut self) -> Tim11lpenW<Apb2lpenrSpec> {
        Tim11lpenW::new(self, 18)
    }
}
#[doc = "APB2 peripheral clock enabled in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2lpenrSpec;
impl crate::RegisterSpec for Apb2lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpenr::R`](R) reader structure"]
impl crate::Readable for Apb2lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure"]
impl crate::Writable for Apb2lpenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2LPENR to value 0x0007_5f33"]
impl crate::Resettable for Apb2lpenrSpec {
    const RESET_VALUE: u32 = 0x0007_5f33;
}
