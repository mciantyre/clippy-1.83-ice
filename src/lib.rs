#![no_std]

use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Channel<const DMA_INST: u8>();

pub struct Write<'a, const DMA_INST: u8> {
    _channel: &'a Channel<DMA_INST>,
}

impl<const DMA_INST: u8> Future for Write<'_, DMA_INST> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

pub fn write<'a, const DMA_INST: u8>(_channel: &'a mut Channel<DMA_INST>) -> Write<'a, DMA_INST> {
    Write { _channel }
}

pub struct Periph<P, const N: u8>(PhantomData<P>);

impl<P, const N: u8> Periph<P, N> {
    pub fn dma_write<'a, const DMA_INST: u8>(
        &'a mut self,
        channel: &'a mut Channel<DMA_INST>,
    ) -> Write<'a, DMA_INST> {
        write(channel)
    }
}
