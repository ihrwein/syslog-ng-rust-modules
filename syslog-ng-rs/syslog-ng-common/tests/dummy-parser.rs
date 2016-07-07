// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;

use std::marker::PhantomData;

pub struct DummyParser<P: Pipe>(PhantomData<P>);

pub struct DummyParserBuilder<P: Pipe>(PhantomData<P>);

use syslog_ng_common::LogMessage;
use syslog_ng_common::{Parser, ParserBuilder, Error, Pipe, GlobalConfig};

impl<P: Pipe> ParserBuilder<P> for DummyParserBuilder<P> {
    type Parser = DummyParser<P>;
    fn new(_: GlobalConfig) -> Self {
        DummyParserBuilder(PhantomData)
    }
    fn option(&mut self, name: String, value: String) -> Result<(), Error> {
        debug!("Setting option: {}={}", name, value);
        Ok(())
    }
    fn build(self) -> Result<Self::Parser, Error> {
        debug!("Building Rust parser");
        Ok(DummyParser(PhantomData))
    }
}

impl<P: Pipe> Parser<P> for DummyParser<P> {
    fn parse(&mut self, _: &mut P, message: &mut LogMessage, input: &str) -> bool {
        debug!("Processing input in Rust Parser: {}", input);
        message.insert(&b"input"[..], input.as_bytes());
        true
    }
}

impl<P: Pipe> Clone for DummyParserBuilder<P> {
    fn clone(&self) -> Self {
        DummyParserBuilder(PhantomData)
    }
}

// this verifies that the macro can be expanded
parser_plugin!(DummyParserBuilder<LogParser>);

use syslog_ng_common::{SYSLOG_NG_INITIALIZED, syslog_ng_global_init, ParserProxy, LogParser};

struct DummyPipe;

impl Pipe for DummyPipe {
    fn forward(&mut self, _: LogMessage) {}
}

#[test]
fn test_given_parser_implementation_when_it_receives_a_message_then_it_adds_a_specific_key_value_pair_to_it
    () {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let builder = DummyParserBuilder::<DummyPipe>::new(cfg);
    let mut parser = builder.build().ok().expect("Failed to build DummyParser");
    let mut msg = LogMessage::new();
    let input = "The quick brown ...";
    let mut pipe = DummyPipe;
    let result = parser.parse(&mut pipe, &mut msg, input);
    assert!(result);
    assert_eq!(msg.get(&b"input"[..]).unwrap(), input.as_bytes());
}

#[test]
fn test_parser_proxy_can_be_deinitialized() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let mut proxy = ParserProxy::<DummyParserBuilder<LogParser>>::new(cfg);
    proxy.set_option("foo".to_owned(), "bar".to_owned());
    proxy.init();
    proxy.deinit();
    proxy.init();
    proxy.deinit();
}