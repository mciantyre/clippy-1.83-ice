#![no_std]

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Channel;

pub struct Write<'a> {
    _channel: &'a Channel,
}

impl Future for Write<'_> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

pub struct Periph;

impl Periph {
    pub fn dma_write<'a>(
        &'a mut self,
        _channel: &'a mut Channel,
    ) -> Write<'a> {
        Write { _channel }
    }
}
