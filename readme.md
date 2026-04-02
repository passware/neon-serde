Neon-serde
==========

This is a fork of the official neon-serde project. The project became stale and
stopped following neon releases.

This crate is a utility to easily convert values between

A `Handle<JsValue>` from the [neon](https://github.com/neon-bindings/neon) crate
and any value implementing `serde::{Serialize, Deserialize}`

## Versions support

neon-serde is tested on node
`8` `10` `12`

## Usage

#### `neon_serde2::from_value`
Convert a `Handle<js::JsValue>` to
a type implementing `serde::Deserialize`

#### `neon_serde2::to_value`˚
Convert a value implementing `serde::Serialize` to
a `Handle<JsValue>`

## Direct Usage Example

```rust,no_run
extern crate neon_serde;
extern crate neon;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

#[derive(Serialize, Debug, Deserialize)]
struct AnObject {
    a: u32,
    b: Vec<f64>,
    c: String,
}

fn deserialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let arg0_value: AnObject = match neon_serde2::from_value(&mut cx, arg0) {
        Ok(value) => value,
        Err(e) => {
            return cx.throw_error(e.to_string());
        }
    };
    println!("{:?}", arg0_value);

    Ok(JsUndefined::new().upcast())
}

fn serialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let value = AnObject {
        a: 1,
        b: vec![2f64, 3f64, 4f64],
        c: "a string".into()
    };

    neon_serde2::to_value(&mut cx, &value)
        .or_else(|e| cx.throw_error(e.to_string()))
}
```

## Limitations

### Data ownership
All Deserialize Values must own all their data (they must have the trait `serde::DererializeOwned`)
