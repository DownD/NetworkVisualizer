use std::sync::mpsc::Receiver;
use crate::data::IPPacketInfo;
use std::sync::mpsc::TryIter;

pub struct EngineChannelListener{
    channel_recv: Receiver<IPPacketInfo>
}



impl EngineChannelListener{
    pub fn new(channel_recv: Receiver<IPPacketInfo>) -> Self {
        EngineChannelListener{
            channel_recv: channel_recv
        }
    }

    pub fn get_packets(&mut self) -> TryIter<IPPacketInfo> {
        return self.channel_recv.try_iter();
    }
}