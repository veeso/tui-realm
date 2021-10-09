//! ## State
//!
//! This module exposes the state type and values

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use crate::props::Color;
use crate::utils::{Email, PhoneNumber};
use std::collections::{HashMap, LinkedList};

/// ## State
///
/// State describes a component state
#[derive(Debug, PartialEq, Clone)]
pub enum State {
    One(Value),
    Tup2((Value, Value)),
    Tup3((Value, Value, Value)),
    Tup4((Value, Value, Value, Value)),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>),
    Linked(LinkedList<State>),
    None,
}

/// ## Value
///
/// Value describes the value contained in a Payload
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F64(f64),
    Isize(isize),
    String(String),
    // -- input types
    Color(Color),
    Email(Email),
    PhoneNumber(PhoneNumber),
}
