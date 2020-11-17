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
use quix::derive::*;
pub struct Update(pub StaticData);

pub trait UpdateAddr {
    fn update(&self, arg: StaticData) -> BoxFuture<'static, Result<(), DispatchError>>;
}

impl<A> UpdateAddr for Pid<A> where A: Handler<Update> + DynHandler {
    fn update(&self, arg: StaticData) -> BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(Update(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl UpdateAddr for PidRecipient<Update> {
    fn update(&self, arg: StaticData) -> BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(Update(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl UpdateAddr for NodeId {
    fn update(&self, arg: StaticData) ->BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(Update(arg)))
    }
}

impl actix::Message for Update {
    type Result = Result<(), DispatchError>;
}

impl quix::derive::RpcMethod for Update {
    const NAME: &'static str = "wit.control.Data.update";
    const ID: u32 = 2033402647;


    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        prost::Message::encode(&self.0, b).map_err(|_| DispatchError::MessageFormat)
    }
    fn read(b: impl bytes::Buf) -> Result<Self, DispatchError> {
        Ok(Self(prost::Message::decode(b).map_err(|_| DispatchError::MessageFormat)?))
    }

    fn read_result(b: impl bytes::Buf) -> Self::Result {
        Ok(())
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        ();
        Ok(())
    }
}

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
            
use quix::derive::*;
pub struct FunctionUpdate(pub Function);

pub trait FunctionUpdateAddr {
    fn function_update(&self, arg: Function) -> BoxFuture<'static, Result<(), DispatchError>>;
}

impl<A> FunctionUpdateAddr for Pid<A> where A: Handler<FunctionUpdate> + DynHandler {
    fn function_update(&self, arg: Function) -> BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(FunctionUpdate(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl FunctionUpdateAddr for PidRecipient<FunctionUpdate> {
    fn function_update(&self, arg: Function) -> BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(FunctionUpdate(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl FunctionUpdateAddr for NodeId {
    fn function_update(&self, arg: Function) ->BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(FunctionUpdate(arg)))
    }
}

impl actix::Message for FunctionUpdate {
    type Result = Result<(), DispatchError>;
}

impl quix::derive::RpcMethod for FunctionUpdate {
    const NAME: &'static str = "wit.control.Data.function_update";
    const ID: u32 = 2604232651;


    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        prost::Message::encode(&self.0, b).map_err(|_| DispatchError::MessageFormat)
    }
    fn read(b: impl bytes::Buf) -> Result<Self, DispatchError> {
        Ok(Self(prost::Message::decode(b).map_err(|_| DispatchError::MessageFormat)?))
    }

    fn read_result(b: impl bytes::Buf) -> Self::Result {
        Ok(())
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        ();
        Ok(())
    }
}

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
            
use quix::derive::*;
pub struct RuleUpdate(pub Rule);

pub trait RuleUpdateAddr {
    fn rule_update(&self, arg: Rule) -> BoxFuture<'static, Result<(), DispatchError>>;
}

impl<A> RuleUpdateAddr for Pid<A> where A: Handler<RuleUpdate> + DynHandler {
    fn rule_update(&self, arg: Rule) -> BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(RuleUpdate(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl RuleUpdateAddr for PidRecipient<RuleUpdate> {
    fn rule_update(&self, arg: Rule) -> BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(RuleUpdate(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl RuleUpdateAddr for NodeId {
    fn rule_update(&self, arg: Rule) ->BoxFuture<'static, Result<(), DispatchError>> {
        Box::pin(self.send(RuleUpdate(arg)))
    }
}

impl actix::Message for RuleUpdate {
    type Result = Result<(), DispatchError>;
}

impl quix::derive::RpcMethod for RuleUpdate {
    const NAME: &'static str = "wit.control.Data.rule_update";
    const ID: u32 = 158844763;


    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        prost::Message::encode(&self.0, b).map_err(|_| DispatchError::MessageFormat)
    }
    fn read(b: impl bytes::Buf) -> Result<Self, DispatchError> {
        Ok(Self(prost::Message::decode(b).map_err(|_| DispatchError::MessageFormat)?))
    }

    fn read_result(b: impl bytes::Buf) -> Self::Result {
        Ok(())
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        ();
        Ok(())
    }
}

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
            use quix::derive::*;
use quix::derive::*;
pub struct EventRecvd(pub Event);

pub trait EventRecvdAddr {
    fn event_recvd(&self, arg: Event) -> BoxFuture<'static, Result<InvokeResult, DispatchError>>;
}

impl<A> EventRecvdAddr for Pid<A> where A: Handler<EventRecvd> + DynHandler {
    fn event_recvd(&self, arg: Event) -> BoxFuture<'static, Result<InvokeResult, DispatchError>> {
        Box::pin(self.send(EventRecvd(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl EventRecvdAddr for PidRecipient<EventRecvd> {
    fn event_recvd(&self, arg: Event) -> BoxFuture<'static, Result<InvokeResult, DispatchError>> {
        Box::pin(self.send(EventRecvd(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl EventRecvdAddr for NodeId {
    fn event_recvd(&self, arg: Event) ->BoxFuture<'static, Result<InvokeResult, DispatchError>> {
        Box::pin(self.send(EventRecvd(arg)))
    }
}

impl actix::Message for EventRecvd {
    type Result = Result<InvokeResult, DispatchError>;
}

impl quix::derive::RpcMethod for EventRecvd {
    const NAME: &'static str = "wit.control.Work.event_recvd";
    const ID: u32 = 2600550877;


    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        prost::Message::encode(&self.0, b).map_err(|_| DispatchError::MessageFormat)
    }
    fn read(b: impl bytes::Buf) -> Result<Self, DispatchError> {
        Ok(Self(prost::Message::decode(b).map_err(|_| DispatchError::MessageFormat)?))
    }

    fn read_result(b: impl bytes::Buf) -> Self::Result {
        Ok(<InvokeResult>::decode(b)?)
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        let a: &InvokeResult = res.as_ref()?; a.encode(b)?;
        Ok(())
    }
}

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
            
use quix::derive::*;
pub struct InvokeFun(pub Invoke);

pub trait InvokeFunAddr {
    fn invoke_fun(&self, arg: Invoke) -> BoxFuture<'static, Result<InvokeResult, DispatchError>>;
}

impl<A> InvokeFunAddr for Pid<A> where A: Handler<InvokeFun> + DynHandler {
    fn invoke_fun(&self, arg: Invoke) -> BoxFuture<'static, Result<InvokeResult, DispatchError>> {
        Box::pin(self.send(InvokeFun(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl InvokeFunAddr for PidRecipient<InvokeFun> {
    fn invoke_fun(&self, arg: Invoke) -> BoxFuture<'static, Result<InvokeResult, DispatchError>> {
        Box::pin(self.send(InvokeFun(arg)).map(|r| r.and_then(|r|r) ))
    }
}
impl InvokeFunAddr for NodeId {
    fn invoke_fun(&self, arg: Invoke) ->BoxFuture<'static, Result<InvokeResult, DispatchError>> {
        Box::pin(self.send(InvokeFun(arg)))
    }
}

impl actix::Message for InvokeFun {
    type Result = Result<InvokeResult, DispatchError>;
}

impl quix::derive::RpcMethod for InvokeFun {
    const NAME: &'static str = "wit.control.Work.invoke_fun";
    const ID: u32 = 1970656331;


    fn write(&self, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        prost::Message::encode(&self.0, b).map_err(|_| DispatchError::MessageFormat)
    }
    fn read(b: impl bytes::Buf) -> Result<Self, DispatchError> {
        Ok(Self(prost::Message::decode(b).map_err(|_| DispatchError::MessageFormat)?))
    }

    fn read_result(b: impl bytes::Buf) -> Self::Result {
        Ok(<InvokeResult>::decode(b)?)
    }

    fn write_result(res: &Self::Result, b: &mut impl bytes::BufMut) -> Result<(), DispatchError> {
        let a: &InvokeResult = res.as_ref()?; a.encode(b)?;
        Ok(())
    }
}

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
            