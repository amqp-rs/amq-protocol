use util::*;

use handlebars::{Handlebars, Helper, RenderContext, RenderError};

#[derive(Debug)]
pub struct AMQPTemplates {
    pub main:     String,
    pub domain:   String,
    pub constant: String,
    pub klass:    String,
    pub method:   String,
    pub argument: String,
    pub property: String,
}

impl Default for AMQPTemplates {
    fn default() -> AMQPTemplates {
        AMQPTemplates {
            main:     String::new(),
            domain:   String::new(),
            constant: String::new(),
            klass:    String::new(),
            method:   String::new(),
            argument: String::new(),
            property: String::new(),
        }
    }
}

pub fn camel_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).expect("no param given to camel").value().as_str().expect("non-string param given to camel");
    rc.writer.write(camel_name(param).as_bytes())?;
    Ok(())
}

pub fn snake_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).expect("no param given to snake").value().as_str().expect("non-string param given to snake");
    rc.writer.write(snake_name(param).as_bytes())?;
    Ok(())
}
