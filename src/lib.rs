#![no_std]

use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

pub trait Element: Copy {}
impl Element for u8 {}

pub trait Destination<E: Element> {}
pub struct Channel<const DMA_INST: u8>();

pub struct Write<'a, D, E, const DMA_INST: u8>
where
    D: Destination<E>,
    E: Element,
{
    _channel: &'a Channel<DMA_INST>,
    _destination: &'a mut D,
    _elem: PhantomData<&'a E>,
}

impl<D, E, const DMA_INST: u8> Future for Write<'_, D, E, DMA_INST>
where
    D: Destination<E>,
    E: Element,
{
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

pub fn write<'a, D, E, const DMA_INST: u8>(
    _channel: &'a mut Channel<DMA_INST>,
    _: &'a [E],
    _destination: &'a mut D,
) -> Write<'a, D, E, DMA_INST>
where
    D: Destination<E>,
    E: Element,
{
    Write {
        _channel,
        _destination,
        _elem: PhantomData,
    }
}

pub struct Periph<P, const N: u8>(PhantomData<P>);
impl<P, const N: u8> Destination<u8> for Periph<P, N> {}

impl<P, const N: u8> Periph<P, N> {
    pub fn dma_write<'a, const DMA_INST: u8>(
        &'a mut self,
        channel: &'a mut Channel<DMA_INST>,
        buffer: &'a [u8],
    ) -> Write<'a, Self, u8, DMA_INST>
    where
    {
        write(channel, buffer, self)
    }
}
