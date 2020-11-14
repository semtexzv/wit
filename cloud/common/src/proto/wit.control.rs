#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Function {
    #[prost(bytes, tag="1")]
    pub id: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub body: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
pub struct Rule {
    #[prost(string, tag="1")]
    pub spec: std::string::String,
    #[prost(bytes, tag="2")]
    pub funid: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticData {
    #[prost(message, optional, tag="1")]
    pub function: ::std::option::Option<Function>,
    #[prost(message, optional, tag="2")]
    pub rule: ::std::option::Option<Rule>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub spec: std::string::String,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoke {
    #[prost(bytes, tag="1")]
    pub funid: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub funbody: std::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub event: ::std::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeResult {
    #[prost(bytes, tag="1")]
    pub response: std::vec::Vec<u8>,
}

use quix::derive::*;
pub struct Update(pub StaticData);
impl From<StaticData> for Update {
    fn from(a: StaticData) -> Self {
        Self(a)
    }
}
impl Into<StaticData> for Update {
    fn into(self) -> StaticData {
        self.0
    }
}
impl ::core::ops::Deref for Update {
    type Target = StaticData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Update {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl quix::derive::Service for Update {
    const NAME: &'static str = "wit.control.Data.update";
    const ID: u64 = 1970139209675510007;
    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        prost::Message::encode(&self.0, b).map_err(|_| ())
    }
    fn read(b: impl bytes::Buf) -> Result<Self, ()> {
        Ok(Self(prost::Message::decode(b).map_err(|_| ())?))
    }

    fn read_result(b: impl bytes::Buf) -> Result<Self::Result, ()> {
        Ok(())
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        ();
        Ok(())
    }
}
impl actix::Message for Update {
    type Result = ();
}
            
use quix::derive::*;
pub struct FunctionUpdate(pub Function);
impl From<Function> for FunctionUpdate {
    fn from(a: Function) -> Self {
        Self(a)
    }
}
impl Into<Function> for FunctionUpdate {
    fn into(self) -> Function {
        self.0
    }
}
impl ::core::ops::Deref for FunctionUpdate {
    type Target = Function;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for FunctionUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl quix::derive::Service for FunctionUpdate {
    const NAME: &'static str = "wit.control.Data.function_update";
    const ID: u64 = 9919633882927141631;
    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        prost::Message::encode(&self.0, b).map_err(|_| ())
    }
    fn read(b: impl bytes::Buf) -> Result<Self, ()> {
        Ok(Self(prost::Message::decode(b).map_err(|_| ())?))
    }

    fn read_result(b: impl bytes::Buf) -> Result<Self::Result, ()> {
        Ok(())
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        ();
        Ok(())
    }
}
impl actix::Message for FunctionUpdate {
    type Result = ();
}
            
use quix::derive::*;
pub struct RuleUpdate(pub Rule);
impl From<Rule> for RuleUpdate {
    fn from(a: Rule) -> Self {
        Self(a)
    }
}
impl Into<Rule> for RuleUpdate {
    fn into(self) -> Rule {
        self.0
    }
}
impl ::core::ops::Deref for RuleUpdate {
    type Target = Rule;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for RuleUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl quix::derive::Service for RuleUpdate {
    const NAME: &'static str = "wit.control.Data.rule_update";
    const ID: u64 = 15862458845705750889;
    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        prost::Message::encode(&self.0, b).map_err(|_| ())
    }
    fn read(b: impl bytes::Buf) -> Result<Self, ()> {
        Ok(Self(prost::Message::decode(b).map_err(|_| ())?))
    }

    fn read_result(b: impl bytes::Buf) -> Result<Self::Result, ()> {
        Ok(())
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        ();
        Ok(())
    }
}
impl actix::Message for RuleUpdate {
    type Result = ();
}
            
use quix::derive::*;
pub struct EventRecvd(pub Event);
impl From<Event> for EventRecvd {
    fn from(a: Event) -> Self {
        Self(a)
    }
}
impl Into<Event> for EventRecvd {
    fn into(self) -> Event {
        self.0
    }
}
impl ::core::ops::Deref for EventRecvd {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for EventRecvd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl quix::derive::Service for EventRecvd {
    const NAME: &'static str = "wit.control.Work.event_recvd";
    const ID: u64 = 5770144640345414593;
    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        prost::Message::encode(&self.0, b).map_err(|_| ())
    }
    fn read(b: impl bytes::Buf) -> Result<Self, ()> {
        Ok(Self(prost::Message::decode(b).map_err(|_| ())?))
    }

    fn read_result(b: impl bytes::Buf) -> Result<Self::Result, ()> {
        Ok(Ok(InvokeResult::decode(b).unwrap()))
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        let a: &InvokeResult = res.as_ref().unwrap(); a.encode(b).unwrap();
        Ok(())
    }
}
impl actix::Message for EventRecvd {
    type Result = Result<InvokeResult, ()>;
}
            
use quix::derive::*;
pub struct InvokeFun(pub Invoke);
impl From<Invoke> for InvokeFun {
    fn from(a: Invoke) -> Self {
        Self(a)
    }
}
impl Into<Invoke> for InvokeFun {
    fn into(self) -> Invoke {
        self.0
    }
}
impl ::core::ops::Deref for InvokeFun {
    type Target = Invoke;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for InvokeFun {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl quix::derive::Service for InvokeFun {
    const NAME: &'static str = "wit.control.Work.invoke_fun";
    const ID: u64 = 14875656262819210624;
    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        prost::Message::encode(&self.0, b).map_err(|_| ())
    }
    fn read(b: impl bytes::Buf) -> Result<Self, ()> {
        Ok(Self(prost::Message::decode(b).map_err(|_| ())?))
    }

    fn read_result(b: impl bytes::Buf) -> Result<Self::Result, ()> {
        Ok(Ok(InvokeResult::decode(b).unwrap()))
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), ()> {
        let a: &InvokeResult = res.as_ref().unwrap(); a.encode(b).unwrap();
        Ok(())
    }
}
impl actix::Message for InvokeFun {
    type Result = Result<InvokeResult, ()>;
}
            