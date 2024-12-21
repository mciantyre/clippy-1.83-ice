#![no_std]

use core::{
    future::Future,
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

pub struct Periph<const N: u8>();

impl<const N: u8> Periph<N> {
    pub fn dma_write<'a, const DMA_INST: u8>(
        &'a mut self,
        _channel: &'a mut Channel<DMA_INST>,
    ) -> Write<'a, DMA_INST> {
        Write { _channel }
    }
}
