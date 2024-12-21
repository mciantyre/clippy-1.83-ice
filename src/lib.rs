#![no_std]

use core::{
    future::Future,
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
    task::{Context, Poll},
};

pub enum Error {}

pub trait Element: Copy {}
impl Element for u8 {}

pub trait Destination<E: Element> {}
pub trait WorksWith<const DMA_INST: u8> {}
pub struct Channel<const DMA_INST: u8>();

pub struct Transfer<'a, const DMA_INST: u8> {
    channel: &'a Channel<DMA_INST>,
    _pinned: PhantomPinned,
}

impl<const DMA_INST: u8> Future for Transfer<'_, DMA_INST> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}

pub struct Write<'a, D, E, const DMA_INST: u8>
where
    D: Destination<E>,
    E: Element,
{
    channel: &'a Channel<DMA_INST>,
    destination: &'a mut D,
    transfer: Transfer<'a, DMA_INST>,
    _elem: PhantomData<&'a E>,
}

impl<D, E, const DMA_INST: u8> Future for Write<'_, D, E, DMA_INST>
where
    D: Destination<E>,
    E: Element,
{
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|this| &mut this.transfer) }.poll(cx)
    }
}

impl<D, E, const DMA_INST: u8> Drop for Write<'_, D, E, DMA_INST>
where
    D: Destination<E>,
    E: Element,
{
    fn drop(&mut self) {}
}

pub fn write<'a, D, E, const DMA_INST: u8>(
    channel: &'a mut Channel<DMA_INST>,
    _: &'a [E],
    destination: &'a mut D,
) -> Write<'a, D, E, DMA_INST>
where
    D: Destination<E> + WorksWith<DMA_INST>,
    E: Element,
{
    Write {
        channel,
        destination,
        transfer: Transfer {
            channel,
            _pinned: PhantomPinned,
        },
        _elem: PhantomData,
    }
}

pub struct Periph<P, const N: u8>(PhantomData<P>);

impl<P> WorksWith<3> for Periph<P, 1> {}
impl<P, const N: u8> Destination<u8> for Periph<P, N> {}

impl<P, const N: u8> Periph<P, N> {
    pub fn dma_write<'a, const DMA_INST: u8>(
        &'a mut self,
        channel: &'a mut Channel<DMA_INST>,
        buffer: &'a [u8],
    ) -> Write<'a, Self, u8, DMA_INST>
    where
        Self: WorksWith<DMA_INST>,
    {
        write(channel, buffer, self)
    }
}
