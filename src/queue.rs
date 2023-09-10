use crate::delay::TickType;
use esp_idf_sys::{xQueueGenericSend, xQueueReceive, QueueHandle_t};
use std::ffi::{c_int, c_void};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::time::Duration;

/** FreeRTOS queue. */
#[derive(Clone)]
pub struct Queue<T> {
    handle: QueueHandle_t,
    _phantom: PhantomData<T>,
}

pub struct Timeout;

const QUEUE_SEND_TO_BACK: c_int = 0;

impl<T> Queue<T> {
    pub unsafe fn from_handle(handle: QueueHandle_t) -> Self {
        Queue {
            handle,
            _phantom: PhantomData,
        }
    }

    pub fn send(&self, data: &T) -> Result<(), Timeout> {
        let ret = unsafe {
            xQueueGenericSend(
                self.handle,
                data as *const T as *const c_void,
                0,
                QUEUE_SEND_TO_BACK,
            )
        };
        if ret != 0 {
            Ok(())
        } else {
            Err(Timeout)
        }
    }

    pub fn receive(&self, timeout: Duration) -> Result<T, Timeout> {
        unsafe {
            let mut data: MaybeUninit<T> = MaybeUninit::uninit();
            let ret = xQueueReceive(
                self.handle,
                &mut data as *mut MaybeUninit<T> as *mut c_void,
                TickType::from(timeout).0,
            );
            if ret != 0 {
                Ok(data.assume_init())
            } else {
                Err(Timeout)
            }
        }
    }
}
