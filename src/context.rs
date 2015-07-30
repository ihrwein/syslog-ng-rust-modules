use std::rc::Rc;

use super::{config, Conditions, Message, TimerEvent};

pub struct Context {
    conditions: Conditions,
    opened: bool,
    patterns: Vec<String>,
    elapsed_time: u32,
    elapsed_time_since_last_message: u32,
    messages: Vec<Rc<Message>>
}

impl Context {
    pub fn new(conditions: Conditions) -> Context {
        Context {
            conditions: conditions,
            opened: false,
            elapsed_time: 0,
            elapsed_time_since_last_message: 0,
            messages: Vec::new(),
            patterns: Vec::new()
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent) -> bool {
        if self.opened {
            println!("timer event: {}", event.0);
            self.update_timers(event.0);
            self.is_any_timer_expired()
        } else {
            false
        }
    }

    fn process_message(&mut self, event: Rc<Message>) -> bool {
        println!("message event");
        self.elapsed_time_since_last_message = 0;
        self.messages.push(event);
        self.is_closing()
    }

    pub fn on_message(&mut self, event: Rc<Message>) -> bool {
        if self.opened {
            self.process_message(event)
        } else {
            self.open_context_or_ignore_message(event)
        }
    }

    fn open_context_or_ignore_message(&mut self, event: Rc<Message>) -> bool {
        if self.is_opening(&event) {
            self.opened = true;
            self.process_message(event)
        } else {
            false
        }
    }

    fn is_max_size_reached(&self) -> bool {
        self.conditions.max_size.map_or(false, |max_size| self.messages.len() >= max_size)
    }

    fn is_closing_message(&self) -> bool {
        self.messages.last().map_or(false, |event| {
            self.patterns.last().map_or(false, |pattern| {
                pattern == event.get("uuid").unwrap()
            })
        })
    }

    fn is_opening(&self, message: &Message) -> bool {
        let found = self.patterns.contains(message.get("uuid").unwrap());
        self.conditions.first_opens.map_or(found, |first| {
            if first {
                self.patterns.first().map_or(false, |pattern| {
                    pattern == message.get("uuid").unwrap()
                })
            } else {
                found
            }
        })
    }

    fn is_closing(&self) -> bool {
       self.is_max_size_reached() || self.is_closing_message()
    }

    fn is_timeout_expired(&self) -> bool {
        self.elapsed_time >= self.conditions.timeout
    }

    fn is_renew_timeout_expired(&self) -> bool {
        self.conditions.renew_timeout.map_or(false, |renew_timeout| {
            self.elapsed_time_since_last_message >= renew_timeout
        })
    }

    fn is_any_timer_expired(&self) -> bool {
        self.is_timeout_expired() || self.is_renew_timeout_expired()
    }

    fn update_timers(&mut self, elapsed_time: u32) {
        self.elapsed_time += elapsed_time;
        self.elapsed_time_since_last_message += elapsed_time;
    }
}

impl From<config::Context> for Context {
    fn from(context: config::Context) -> Context {
        let mut ctx = Context::new(context.conditions);
        ctx.patterns = context.patterns;
        ctx
    }
}

use conditions::Builder;

#[test]
fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
    let timeout = 100;
    let mut context = Context::new(Builder::new(timeout).build());
    assert_false!(context.on_timer(&mut TimerEvent(50)));
    assert_false!(context.on_timer(&mut TimerEvent(49)));
    assert_true!(context.on_timer(&mut TimerEvent(1)));
}

#[test]
fn test_given_close_condition_with_max_size_when_the_max_size_reached_then_the_condition_is_met() {
    let timeout = 100;
    let max_size = 3;
    let mut context = Context::new(Builder::new(timeout).max_size(max_size).build());
    let msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
    };
    let event = Rc::new(msg1);
    assert_false!(context.on_message(event.clone()));
    assert_false!(context.on_message(event.clone()));
    assert_true!(context.on_message(event.clone()));
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_without_renewing_messages_then_the_condition_is_met() {
    let timeout = 100;
    let renew_timeout = 10;
    let mut context = Context::new(Builder::new(timeout).renew_timeout(renew_timeout).build());
    let msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
    };
    let event = Rc::new(msg1);
    assert_false!(context.on_message(event.clone()));
    assert_false!(context.on_timer(&mut TimerEvent(8)));
    assert_false!(context.on_timer(&mut TimerEvent(1)));
    assert_true!(context.on_timer(&mut TimerEvent(1)));
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_with_renewing_messages_then_the_context_is_not_closed() {
    let timeout = 100;
    let renew_timeout = 10;
    let mut context = Context::new(Builder::new(timeout).renew_timeout(renew_timeout).build());
    let msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
    };
    let event = Rc::new(msg1);
    assert_false!(context.on_message(event.clone()));
    assert_false!(context.on_timer(&mut TimerEvent(8)));
    assert_false!(context.on_timer(&mut TimerEvent(1)));
    assert_false!(context.on_message(event.clone()));
    assert_false!(context.on_timer(&mut TimerEvent(1)));
}
